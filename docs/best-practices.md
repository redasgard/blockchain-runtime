# Best Practices

Best practices for using Blockchain Runtime effectively and safely.

## Resource Management

### Always Clean Up Environments

```rust
// ❌ Bad: Resource leak
async fn bad_example(runtime: &dyn BlockchainRuntime) {
    let env = runtime.create_environment(config).await.unwrap();
    // If error occurs, environment never destroyed
    let result = runtime.execute(&env, code, &inputs).await.unwrap();
}

// ✅ Good: Always clean up
async fn good_example(runtime: &dyn BlockchainRuntime) -> Result<()> {
    let env = runtime.create_environment(config).await?;
    
    let result = runtime.execute(&env, code, &inputs).await;
    
    // Clean up regardless of success/failure
    runtime.destroy(env).await?;
    
    result?;
    Ok(())
}

// ✅ Better: Use helper
async fn with_environment<F, T>(
    runtime: &dyn BlockchainRuntime,
    config: RuntimeConfig,
    f: F,
) -> Result<T>
where
    F: FnOnce(&RuntimeEnvironment) -> Result<T>,
{
    let env = runtime.create_environment(config).await?;
    let result = f(&env);
    runtime.destroy(env).await?;
    result
}
```

### Check Availability Before Use

```rust
// ✅ Good: Check first
async fn safe_execution(runtime: &dyn BlockchainRuntime) -> Result<()> {
    if !runtime.is_available().await {
        anyhow::bail!("Runtime not available");
    }
    
    // Proceed with execution
    Ok(())
}
```

## Configuration

### Use Appropriate Timeouts

```rust
// ❌ Bad: Too short
let config = RuntimeConfig {
    timeout_seconds: 10,  // May timeout prematurely
    ..Default::default()
};

// ✅ Good: Based on complexity
let config = RuntimeConfig {
    timeout_seconds: match contract_complexity {
        Complexity::Simple => 60,
        Complexity::Medium => 300,
        Complexity::Complex => 600,
    },
    ..Default::default()
};
```

### Set Memory Limits

```rust
// ✅ Good: Prevent OOM
let config = RuntimeConfig {
    memory_limit_mb: 2048,  // 2GB limit
    ..Default::default()
};
```

### Choose Appropriate Network Mode

```rust
// Development
let dev_config = RuntimeConfig {
    network_mode: NetworkMode::Local,
    ..Default::default()
};

// Integration testing
let integration_config = RuntimeConfig {
    network_mode: NetworkMode::Testnet,
    ..Default::default()
};

// Pre-production testing
let staging_config = RuntimeConfig {
    network_mode: NetworkMode::MainnetFork,
    ..Default::default()
};
```

## Error Handling

### Handle All Error Cases

```rust
// ✅ Good: Comprehensive error handling
async fn robust_deployment(runtime: &dyn BlockchainRuntime) -> Result<String> {
    let env = runtime.create_environment(config).await
        .context("Failed to create environment")?;
    
    let address = runtime.deploy_contract(&env, &bytecode, &args).await
        .context("Failed to deploy contract")?;
    
    // Verify deployment
    let code = runtime.call_function(&env, &address, "code", &[]).await
        .context("Failed to verify deployment")?;
    
    if code.is_empty() {
        anyhow::bail!("Contract deployment failed: no code at address");
    }
    
    runtime.destroy(env).await
        .context("Failed to clean up environment")?;
    
    Ok(address)
}
```

### Don't Panic

```rust
// ❌ Bad: Panics on error
let env = runtime.create_environment(config).await.unwrap();

// ✅ Good: Propagates error
let env = runtime.create_environment(config).await?;
```

## Performance

### Reuse Environments When Possible

```rust
// ❌ Bad: Create new environment for each test
for test in tests {
    let env = runtime.create_environment(config).await?;
    run_test(env, test).await?;
    runtime.destroy(env).await?;
}

// ✅ Good: Reuse environment
let env = runtime.create_environment(config).await?;
for test in tests {
    run_test(&env, test).await?;
}
runtime.destroy(env).await?;
```

### Use Parallel Execution

```rust
// ✅ Good: Parallel tests
use futures::future::join_all;

let tests = vec![test1, test2, test3];

let futures = tests.iter().map(|test| {
    let runtime = runtime.clone();
    async move {
        let env = runtime.create_environment(config).await?;
        let result = test(&runtime, &env).await;
        runtime.destroy(env).await?;
        result
    }
});

join_all(futures).await;
```

### Cache Compiled Contracts

```rust
use std::sync::OnceLock;

static COMPILED_TOKEN: OnceLock<Vec<u8>> = OnceLock::new();

fn get_token_bytecode() -> &'static Vec<u8> {
    COMPILED_TOKEN.get_or_init(|| {
        compile_contract("Token.sol").expect("Compilation failed")
    })
}
```

## Security

### Validate Environment State

```rust
// ✅ Good: Check state before use
async fn safe_execute(runtime: &dyn BlockchainRuntime, env: &RuntimeEnvironment) -> Result<()> {
    if env.state != EnvironmentState::Ready {
        anyhow::bail!("Environment not ready: {:?}", env.state);
    }
    
    runtime.execute(env, code, &inputs).await?;
    Ok(())
}
```

### Sanitize Inputs

```rust
// ✅ Good: Validate before execution
async fn validated_call(
    runtime: &dyn BlockchainRuntime,
    env: &RuntimeEnvironment,
    address: &str,
    function: &str,
    args: &[u8],
) -> Result<Vec<u8>> {
    // Validate address format
    if !is_valid_address(address) {
        anyhow::bail!("Invalid address format");
    }
    
    // Validate function name
    if function.is_empty() || function.len() > 100 {
        anyhow::bail!("Invalid function name");
    }
    
    // Validate args size
    if args.len() > 1_000_000 {
        anyhow::bail!("Arguments too large");
    }
    
    runtime.call_function(env, address, function, args).await
}
```

### Use Monitoring in Production

```rust
// ✅ Good: Enable monitoring for important operations
let config = RuntimeConfig {
    enable_monitoring: is_production || is_critical,
    ..Default::default()
};
```

## Testing

### Write Deterministic Tests

```rust
// ✅ Good: Deterministic
#[tokio::test]
async fn test_deterministic() {
    let config = RuntimeConfig {
        network_mode: NetworkMode::Local,
        blockchain_config: {
            let mut map = HashMap::new();
            map.insert("seed".to_string(), serde_json::json!(12345));
            map
        },
        ..Default::default()
    };
    
    let env = runtime.create_environment(config).await?;
    // Always produces same result
}
```

### Isolate Tests

```rust
// ✅ Good: Each test gets own environment
#[tokio::test]
async fn test_isolated() {
    let env = create_test_environment().await?;
    // This test's state doesn't affect others
    runtime.destroy(env).await?;
}
```

### Test Error Cases

```rust
#[tokio::test]
async fn test_invalid_deployment() {
    let env = create_test_environment().await?;
    
    // Test with invalid bytecode
    let invalid_bytecode = vec![0xFF; 100];
    let result = runtime.deploy_contract(&env, &invalid_bytecode, &[]).await;
    
    assert!(result.is_err());
    
    runtime.destroy(env).await?;
}
```

## Monitoring and Metrics

### Collect Comprehensive Metrics

```rust
// ✅ Good: Track all relevant metrics
let result = runtime.execute(&env, code, &inputs).await?;

println!("Execution Metrics:");
for (name, value) in &result.metrics {
    println!("  {}: {}", name, value);
}

println!("State Changes: {}", result.state_changes.len());
println!("Events: {}", result.events.len());
println!("Time: {}ms", result.execution_time_ms);
```

### Monitor Critical Executions

```rust
// ✅ Good: Monitor important operations
if is_critical_operation {
    let result = runtime.execute(&env, code, &inputs).await?;
    let events = runtime.monitor(&env, &result.execution_id).await?;
    
    for event in events {
        log_event(&event);
    }
}
```

## Deployment

### Version Your Contracts

```rust
// ✅ Good: Include version in deployment
let constructor_args = encode_args(&[
    "v1.0.0",  // Version
    initial_supply,
    owner_address,
]);

let address = runtime.deploy_contract(&env, &bytecode, &constructor_args).await?;
```

### Verify Deployments

```rust
// ✅ Good: Verify after deployment
let address = runtime.deploy_contract(&env, &bytecode, &args).await?;

// Verify code exists
let code = get_code_at_address(&env, &address).await?;
if code.is_empty() {
    anyhow::bail!("Deployment verification failed");
}

// Verify initialization
let initialized = call_view_function(&env, &address, "isInitialized").await?;
if !initialized {
    anyhow::bail!("Contract not properly initialized");
}
```

## Documentation

### Document Runtime Requirements

```rust
/// Ethereum runtime implementation
///
/// # Requirements
///
/// - Docker installed and running
/// - Port 8545 available
/// - At least 2GB free memory
///
/// # Environment Variables
///
/// - `INFURA_API_KEY` - For testnet/mainnet access
pub struct EthereumRuntime {
    // ...
}
```

### Document Blockchain-Specific Behavior

```rust
impl BlockchainRuntime for EthereumRuntime {
    /// Deploys a contract to Ethereum
    ///
    /// # Arguments
    ///
    /// * `bytecode` - Compiled EVM bytecode (from solc)
    /// * `constructor_args` - ABI-encoded constructor arguments
    ///
    /// # Returns
    ///
    /// Ethereum address (0x-prefixed hex string)
    ///
    /// # Errors
    ///
    /// - If bytecode is invalid
    /// - If deployment transaction fails
    /// - If contract creation fails
    async fn deploy_contract(/* ... */) -> Result<String> {
        // ...
    }
}
```

## Common Pitfalls

### ❌ Not Checking Capabilities

```rust
// Bad: Assumes feature exists
let result = runtime.call_function(&env, &addr, "estimate_gas", &[]).await?;

// Good: Check capability first
let caps = runtime.capabilities();
if caps.supports_gas_estimation {
    let result = runtime.call_function(&env, &addr, "estimate_gas", &[]).await?;
} else {
    // Fallback or error
}
```

### ❌ Ignoring Metrics

```rust
// Bad: Ignore cost
let result = runtime.execute(&env, code, &inputs).await?;

// Good: Track and optimize
let result = runtime.execute(&env, code, &inputs).await?;
if let Some(gas) = result.metrics.get("gas_used") {
    if gas.as_u64().unwrap() > 1_000_000 {
        println!("Warning: High gas usage");
    }
}
```

### ❌ Not Testing on Multiple Networks

```rust
// Bad: Only test locally
#[tokio::test]
async fn test_only_local() {
    let config = RuntimeConfig {
        network_mode: NetworkMode::Local,
        ..Default::default()
    };
    // ...
}

// Good: Test on multiple networks
#[tokio::test]
async fn test_all_networks() {
    for mode in [NetworkMode::Local, NetworkMode::Testnet, NetworkMode::MainnetFork] {
        let config = RuntimeConfig {
            network_mode: mode,
            ..Default::default()
        };
        // ... test on each
    }
}
```

## Checklist

### Before Production

- [ ] Test on all network modes
- [ ] Verify resource cleanup
- [ ] Handle all error cases
- [ ] Set appropriate timeouts
- [ ] Enable monitoring
- [ ] Document requirements
- [ ] Test concurrent usage
- [ ] Verify metric collection
- [ ] Test failure scenarios
- [ ] Review security considerations

### For Each Implementation

- [ ] Implement all trait methods
- [ ] Return accurate capabilities
- [ ] Provide useful metrics
- [ ] Clean up resources properly
- [ ] Handle errors gracefully
- [ ] Write comprehensive tests
- [ ] Document blockchain-specific behavior

## Conclusion

Following these best practices ensures:
- Reliable operation
- Efficient resource usage
- Safe concurrent access
- Easy debugging
- Maintainable code

See [User Guide](./user-guide.md) for more patterns.

