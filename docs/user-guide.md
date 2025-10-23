# User Guide

Comprehensive guide for using Blockchain Runtime.

## Environment Management

### Creating Environments

```rust
use blockchain_runtime::{RuntimeConfig, NetworkMode};

// Basic environment
let config = RuntimeConfig::default();
let env = runtime.create_environment(config).await?;

// Custom configuration
let config = RuntimeConfig {
    timeout_seconds: 600,      // 10 minutes
    memory_limit_mb: 2048,     // 2GB
    network_mode: NetworkMode::Local,
    enable_monitoring: true,
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("chain_id".to_string(), serde_json::json!(1337));
        map
    },
};

let env = runtime.create_environment(config).await?;
```

### Environment Lifecycle

```rust
// Create
let env = runtime.create_environment(config).await?;
assert_eq!(env.state, EnvironmentState::Ready);

// Use
let result = runtime.execute(&env, code, &inputs).await?;

// Destroy
runtime.destroy(env).await?;
```

## Contract Operations

### Deploying Contracts

```rust
// Read compiled bytecode
let bytecode = std::fs::read("MyContract.bin")?;

// Encode constructor arguments (blockchain-specific)
let constructor_args = encode_constructor_args(&[
    "initial_supply".to_string(),
    "1000000".to_string(),
])?;

// Deploy
let address = runtime.deploy_contract(&env, &bytecode, &constructor_args).await?;

println!("Contract deployed at: {}", address);
```

### Calling Functions

```rust
// Encode function call (blockchain-specific)
let args = encode_function_args("transfer", &[
    "recipient_address",
    "amount",
])?;

// Call function
let result = runtime.call_function(&env, &contract_address, "transfer", &args).await?;

// Decode result (blockchain-specific)
let decoded = decode_return_value(&result)?;
println!("Transfer result: {:?}", decoded);
```

### State Queries

```rust
// Call view/read-only function
let balance_args = encode_function_args("balanceOf", &["address"])?;

let result = runtime.call_function(&env, &contract_address, "balanceOf", &balance_args).await?;

let balance: u64 = decode_u64(&result)?;
println!("Balance: {}", balance);
```

## Execution and Monitoring

### Full Execution Flow

```rust
use blockchain_runtime::{ExecutionInputs, ExecutionContext};
use std::path::Path;

let inputs = ExecutionInputs {
    target_function: "complex_function".to_string(),
    parameters: {
        let mut params = HashMap::new();
        params.insert("amount".to_string(), serde_json::json!(1000));
        params.insert("recipient".to_string(), serde_json::json!("0x..."));
        params
    },
    context: ExecutionContext {
        sender: Some("0x1234...".to_string()),
        block_number: Some(1000),
        timestamp: Some(1698000000),
        extra: HashMap::new(),
    },
};

let result = runtime.execute(&env, Path::new("contract.sol"), &inputs).await?;

// Analyze results
if result.success {
    println!("✓ Execution successful");
    
    // Metrics
    if let Some(gas) = result.metrics.get("gas_used") {
        println!("Gas: {}", gas);
    }
    
    // State changes
    println!("State changes: {}", result.state_changes.len());
    for change in &result.state_changes {
        match change.change_type {
            blockchain_runtime::StateChangeType::Created => {
                println!("  Created: {} = {:?}", change.key, change.new_value);
            }
            blockchain_runtime::StateChangeType::Updated => {
                println!("  Updated: {} ({:?} → {:?})", 
                    change.key, change.old_value, change.new_value);
            }
            blockchain_runtime::StateChangeType::Deleted => {
                println!("  Deleted: {}", change.key);
            }
        }
    }
    
    // Events
    println!("Events: {}", result.events.len());
    for event in &result.events {
        println!("  {}: {:?}", event.event_type, event.data);
    }
} else {
    println!("✗ Execution failed: {:?}", result.error);
}
```

### Event Monitoring

```rust
// Execute contract
let result = runtime.execute(&env, code_path, &inputs).await?;

// Monitor events
let events = runtime.monitor(&env, &result.execution_id).await?;

for event in events {
    println!("Event: {} at timestamp {}", event.event_type, event.timestamp);
    
    // Process event data
    match event.event_type.as_str() {
        "Transfer" => {
            let from = event.data.get("from");
            let to = event.data.get("to");
            let amount = event.data.get("amount");
            println!("  Transfer: {:?} -> {:?}, amount: {:?}", from, to, amount);
        }
        "Approval" => {
            // Handle approval event
        }
        _ => {
            println!("  Unknown event: {:?}", event.data);
        }
    }
}
```

## Metrics Collection

### Available Metrics

```rust
// Get metric definitions
let metrics = runtime.metrics_definition();

for metric in metrics {
    println!("Metric: {}", metric.name);
    println!("  Description: {}", metric.description);
    println!("  Unit: {}", metric.unit);
    println!("  Type: {:?}", metric.metric_type);
}

// Common metrics:
// - gas_used (Ethereum, Polygon, BSC)
// - compute_units (Solana)
// - storage_delta (all chains)
// - execution_time (all chains)
```

### Using Metrics

```rust
let result = runtime.execute(&env, code, &inputs).await?;

// Gas used (Ethereum)
if let Some(gas) = result.metrics.get("gas_used") {
    let gas_value = gas.as_u64().unwrap_or(0);
    println!("Gas used: {} units", gas_value);
    
    // Estimate cost
    let gas_price_gwei = 20;
    let cost_gwei = gas_value * gas_price_gwei;
    println!("Estimated cost: {} gwei", cost_gwei);
}

// Compute units (Solana)
if let Some(compute) = result.metrics.get("compute_units") {
    println!("Compute units: {}", compute);
}

// Storage delta (all)
if let Some(storage) = result.metrics.get("storage_delta") {
    println!("Storage change: {} bytes", storage);
}

// Execution time (all)
println!("Execution time: {}ms", result.execution_time_ms);
```

## Runtime Capabilities

### Checking Capabilities

```rust
let caps = runtime.capabilities();

// Feature detection
if caps.supports_contract_deployment {
    // Can deploy contracts
    let address = runtime.deploy_contract(&env, bytecode, &args).await?;
}

if caps.supports_gas_estimation {
    // Can estimate gas
    // (implementation specific)
}

if caps.supports_time_travel {
    // Can manipulate blockchain time
    // (useful for testing time-dependent contracts)
}

if caps.supports_state_inspection {
    // Can inspect contract state
}

if caps.supports_event_monitoring {
    // Can monitor events
}

// Limits
println!("Max execution time: {}s", caps.max_execution_time_seconds);
```

### Conditional Features

```rust
async fn smart_execution(runtime: &dyn BlockchainRuntime) -> Result<()> {
    let caps = runtime.capabilities();
    
    if caps.supports_gas_estimation {
        // Estimate gas first
        // let gas_estimate = runtime.estimate_gas(...).await?;
        // println!("Estimated gas: {}", gas_estimate);
    }
    
    // Execute
    let result = runtime.execute(&env, code, &inputs).await?;
    
    if caps.supports_event_monitoring {
        // Monitor detailed events
        let events = runtime.monitor(&env, &result.execution_id).await?;
        // Process events...
    }
    
    Ok(())
}
```

## Network Modes

### Local Development

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::Local,
    ..Default::default()
};

// Benefits:
// - Fast
// - Free
// - Deterministic
// - Full control
```

### Testnet Testing

```rust
let mut blockchain_config = HashMap::new();
blockchain_config.insert("network".to_string(), serde_json::json!("goerli"));

let config = RuntimeConfig {
    network_mode: NetworkMode::Testnet,
    blockchain_config,
    ..Default::default()
};

// Benefits:
// - Realistic network conditions
// - Free test tokens
// - Public visibility
// - Integration testing
```

### Mainnet Forking

```rust
let mut blockchain_config = HashMap::new();
blockchain_config.insert("fork_url".to_string(), 
    serde_json::json!("https://eth-mainnet.alchemyapi.io/v2/YOUR_KEY"));
blockchain_config.insert("fork_block".to_string(), serde_json::json!(17000000));

let config = RuntimeConfig {
    network_mode: NetworkMode::MainnetFork,
    blockchain_config,
    ..Default::default()
};

// Benefits:
// - Real contract state
// - Realistic testing
// - No real funds spent
// - Test against live contracts
```

## Error Handling

### Graceful Degradation

```rust
async fn robust_execution(runtime: &dyn BlockchainRuntime) -> anyhow::Result<()> {
    // Check availability
    if !runtime.is_available().await {
        anyhow::bail!("Runtime not available");
    }
    
    // Create environment with retry
    let env = retry_async(|| runtime.create_environment(config.clone()), 3).await?;
    
    // Execute with timeout
    let result = timeout(
        Duration::from_secs(300),
        runtime.execute(&env, code, &inputs)
    ).await??;
    
    // Always clean up, even on error
    let _ = runtime.destroy(env).await; // Ignore cleanup errors
    
    Ok(())
}

async fn retry_async<F, T>(f: F, retries: usize) -> anyhow::Result<T>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<T>>>>,
{
    for attempt in 0..retries {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < retries - 1 => {
                tokio::time::sleep(Duration::from_secs(2u64.pow(attempt as u32))).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

## Best Practices

### 1. Always Clean Up

```rust
async fn use_runtime(runtime: &dyn BlockchainRuntime) -> Result<()> {
    let env = runtime.create_environment(config).await?;
    
    // Use environment...
    
    // ALWAYS destroy, even on error
    runtime.destroy(env).await?;
    Ok(())
}
```

### 2. Check Availability First

```rust
if !runtime.is_available().await {
    anyhow::bail!("Runtime not available");
}
```

### 3. Use Appropriate Timeouts

```rust
let config = RuntimeConfig {
    timeout_seconds: match complexity {
        Complexity::Simple => 60,
        Complexity::Medium => 300,
        Complexity::Complex => 600,
    },
    ..Default::default()
};
```

### 4. Monitor Important Executions

```rust
let config = RuntimeConfig {
    enable_monitoring: is_production,
    ..Default::default()
};
```

## Next Steps

- Explore [Use Cases](./use-cases.md) for practical examples
- Read [Implementation Guide](./implementation-guide.md) to create custom runtimes
- Check [API Reference](./api-reference.md) for detailed documentation
- See [Testing Guide](./testing.md) for testing strategies

