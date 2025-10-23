# Getting Started

## Installation

Add Blockchain Runtime to your `Cargo.toml`:

```toml
[dependencies]
blockchain-runtime = "0.1"
tokio = { version = "1", features = ["full"] }  # Required for async
async-trait = "0.1"  # For implementing the trait

# Optional: with tracing support
blockchain-runtime = { version = "0.1", features = ["tracing"] }
```

## First Steps

### 1. Understand the Trait

The `BlockchainRuntime` trait is the core interface:

```rust
use blockchain_runtime::BlockchainRuntime;
use async_trait::async_trait;

#[async_trait]
impl BlockchainRuntime for MyRuntime {
    fn blockchain_id(&self) -> &str {
        "ethereum"  // or "solana", "near", etc.
    }
    
    // Implement other methods...
}
```

### 2. Create a Runtime Environment

```rust
use blockchain_runtime::{RuntimeConfig, NetworkMode};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure the runtime
    let config = RuntimeConfig {
        timeout_seconds: 300,
        memory_limit_mb: 1024,
        network_mode: NetworkMode::Local,
        enable_monitoring: true,
        blockchain_config: HashMap::new(),
    };
    
    // Create environment (assuming you have a runtime implementation)
    // let runtime: Box<dyn BlockchainRuntime> = get_ethereum_runtime();
    // let env = runtime.create_environment(config).await?;
    
    println!("Environment created!");
    Ok(())
}
```

### 3. Deploy a Contract

```rust
use blockchain_runtime::BlockchainRuntime;

async fn deploy_example(runtime: &dyn BlockchainRuntime, env: &blockchain_runtime::RuntimeEnvironment) -> anyhow::Result<()> {
    // Your compiled contract bytecode
    let bytecode = std::fs::read("contract.bin")?;
    
    // Constructor arguments (ABI encoded)
    let constructor_args = vec![/* encoded args */];
    
    // Deploy
    let address = runtime.deploy_contract(env, &bytecode, &constructor_args).await?;
    
    println!("Contract deployed at: {}", address);
    Ok(())
}
```

### 4. Call a Contract Function

```rust
async fn call_example(runtime: &dyn BlockchainRuntime, env: &blockchain_runtime::RuntimeEnvironment) -> anyhow::Result<()> {
    let contract_address = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb";
    let function = "transfer";
    
    // Function arguments (ABI encoded)
    let args = vec![/* encoded args */];
    
    // Call function
    let result = runtime.call_function(env, contract_address, function, &args).await?;
    
    println!("Function result: {:?}", result);
    Ok(())
}
```

### 5. Execute and Monitor

```rust
use blockchain_runtime::{ExecutionInputs, ExecutionContext};
use std::path::Path;

async fn execute_example(runtime: &dyn BlockchainRuntime, env: &blockchain_runtime::RuntimeEnvironment) -> anyhow::Result<()> {
    // Prepare inputs
    let inputs = ExecutionInputs {
        target_function: "test_function".to_string(),
        parameters: HashMap::new(),
        context: ExecutionContext {
            sender: Some("0x123...".to_string()),
            block_number: Some(100),
            timestamp: Some(1234567890),
            extra: HashMap::new(),
        },
    };
    
    // Execute
    let result = runtime.execute(env, Path::new("contract.sol"), &inputs).await?;
    
    // Check results
    println!("Success: {}", result.success);
    println!("Execution time: {}ms", result.execution_time_ms);
    
    // Check metrics
    if let Some(gas) = result.metrics.get("gas_used") {
        println!("Gas used: {}", gas);
    }
    
    // Check state changes
    for change in &result.state_changes {
        println!("State changed: {} -> {:?}", change.key, change.new_value);
    }
    
    // Check events
    for event in &result.events {
        println!("Event: {} at {}", event.event_type, event.timestamp);
    }
    
    Ok(())
}
```

### 6. Clean Up

```rust
async fn cleanup_example(runtime: &dyn BlockchainRuntime, env: blockchain_runtime::RuntimeEnvironment) -> anyhow::Result<()> {
    // Destroy environment when done
    runtime.destroy(env).await?;
    
    println!("Environment cleaned up");
    Ok(())
}
```

## Complete Example

```rust
use blockchain_runtime::{
    BlockchainRuntime, RuntimeConfig, RuntimeEnvironment,
    NetworkMode, ExecutionInputs, ExecutionContext
};
use std::collections::HashMap;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Blockchain Runtime Example ===\n");
    
    // 1. Get runtime (you'll implement this)
    // let runtime = get_my_runtime();
    
    // 2. Configure
    let config = RuntimeConfig {
        timeout_seconds: 300,
        memory_limit_mb: 1024,
        network_mode: NetworkMode::Local,
        enable_monitoring: true,
        blockchain_config: HashMap::new(),
    };
    
    // 3. Create environment
    println!("Creating environment...");
    // let env = runtime.create_environment(config).await?;
    // println!("✓ Environment ready: {}", env.environment_id);
    
    // 4. Deploy contract
    println!("\nDeploying contract...");
    let bytecode = vec![/* your bytecode */];
    // let address = runtime.deploy_contract(&env, &bytecode, &[]).await?;
    // println!("✓ Contract deployed at: {}", address);
    
    // 5. Call function
    println!("\nCalling function...");
    // let result = runtime.call_function(&env, &address, "getValue", &[]).await?;
    // println!("✓ Result: {:?}", result);
    
    // 6. Execute with monitoring
    println!("\nExecuting with full monitoring...");
    let inputs = ExecutionInputs {
        target_function: "test".to_string(),
        parameters: HashMap::new(),
        context: ExecutionContext {
            sender: None,
            block_number: None,
            timestamp: None,
            extra: HashMap::new(),
        },
    };
    
    // let exec_result = runtime.execute(&env, Path::new("contract.sol"), &inputs).await?;
    // println!("✓ Execution completed:");
    // println!("  Success: {}", exec_result.success);
    // println!("  Time: {}ms", exec_result.execution_time_ms);
    // println!("  Events: {}", exec_result.events.len());
    
    // 7. Clean up
    println!("\nCleaning up...");
    // runtime.destroy(env).await?;
    // println!("✓ Environment destroyed");
    
    Ok(())
}
```

## Implementing Your Own Runtime

To create a runtime for your blockchain:

```rust
use blockchain_runtime::*;
use async_trait::async_trait;

pub struct MyBlockchainRuntime {
    // Your runtime state
}

#[async_trait]
impl BlockchainRuntime for MyBlockchainRuntime {
    fn blockchain_id(&self) -> &str {
        "mychain"
    }
    
    async fn create_environment(&self, config: RuntimeConfig) -> anyhow::Result<RuntimeEnvironment> {
        // Create a local test network or connect to existing one
        Ok(RuntimeEnvironment {
            environment_id: uuid::Uuid::new_v4().to_string(),
            blockchain_id: self.blockchain_id().to_string(),
            runtime_type: RuntimeType::LocalProcess,
            endpoint_url: "http://localhost:8545".to_string(),
            state: EnvironmentState::Ready,
            metadata: HashMap::new(),
        })
    }
    
    async fn execute(
        &self,
        env: &RuntimeEnvironment,
        code_path: &Path,
        inputs: &ExecutionInputs,
    ) -> anyhow::Result<ExecutionResult> {
        // Read code, compile, deploy, execute
        Ok(ExecutionResult {
            execution_id: uuid::Uuid::new_v4().to_string(),
            success: true,
            return_value: None,
            error: None,
            metrics: HashMap::new(),
            state_changes: vec![],
            events: vec![],
            execution_time_ms: 100,
        })
    }
    
    async fn deploy_contract(
        &self,
        env: &RuntimeEnvironment,
        bytecode: &[u8],
        constructor_args: &[u8],
    ) -> anyhow::Result<String> {
        // Deploy contract and return address
        Ok("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string())
    }
    
    async fn call_function(
        &self,
        env: &RuntimeEnvironment,
        contract_address: &str,
        function: &str,
        args: &[u8],
    ) -> anyhow::Result<Vec<u8>> {
        // Encode call, execute, decode result
        Ok(vec![])
    }
    
    fn metrics_definition(&self) -> Vec<RuntimeMetricDefinition> {
        vec![
            RuntimeMetricDefinition {
                name: "gas_used".to_string(),
                description: "Gas consumed".to_string(),
                unit: "units".to_string(),
                metric_type: MetricType::Gas,
            },
        ]
    }
    
    async fn monitor(
        &self,
        env: &RuntimeEnvironment,
        execution_id: &str,
    ) -> anyhow::Result<Vec<RuntimeEvent>> {
        Ok(vec![])
    }
    
    async fn destroy(&self, env: RuntimeEnvironment) -> anyhow::Result<()> {
        // Clean up resources
        Ok(())
    }
    
    async fn is_available(&self) -> bool {
        true
    }
    
    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities::default()
    }
}
```

## Testing Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_contract_deployment() {
        let runtime = MyBlockchainRuntime::new();
        let config = RuntimeConfig::default();
        
        let env = runtime.create_environment(config).await.unwrap();
        let bytecode = vec![/* test bytecode */];
        
        let address = runtime.deploy_contract(&env, &bytecode, &[]).await.unwrap();
        
        assert!(!address.is_empty());
        
        runtime.destroy(env).await.unwrap();
    }
}
```

## Network Mode Examples

### Local Mode (Default)

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::Local,
    ..Default::default()
};
```

**Best for:** Development, unit tests

### Testnet Mode

```rust
let mut blockchain_config = HashMap::new();
blockchain_config.insert("network".to_string(), serde_json::json!("goerli"));

let config = RuntimeConfig {
    network_mode: NetworkMode::Testnet,
    blockchain_config,
    ..Default::default()
};
```

**Best for:** Integration tests, staging

### Mainnet Fork

```rust
let mut blockchain_config = HashMap::new();
blockchain_config.insert("fork_block".to_string(), serde_json::json!(17000000));

let config = RuntimeConfig {
    network_mode: NetworkMode::MainnetFork,
    blockchain_config,
    ..Default::default()
};
```

**Best for:** Testing against real contracts

## Checking Capabilities

```rust
let caps = runtime.capabilities();

if caps.supports_contract_deployment {
    println!("Can deploy contracts");
}

if caps.supports_gas_estimation {
    // Use gas estimation features
}

if caps.supports_time_travel {
    // Can manipulate block time/number
}

println!("Max execution time: {}s", caps.max_execution_time_seconds);
```

## Error Handling

```rust
match runtime.deploy_contract(&env, &bytecode, &args).await {
    Ok(address) => println!("Deployed at: {}", address),
    Err(e) => eprintln!("Deployment failed: {}", e),
}
```

## Next Steps

- Read [Use Cases](./use-cases.md) for real-world examples
- Check [Implementation Guide](./implementation-guide.md) for detailed runtime implementation
- See [Testing Guide](./testing.md) for testing strategies
- Review [API Reference](./api-reference.md) for complete API documentation

## Troubleshooting

### Environment Creation Fails

- Check runtime is available: `runtime.is_available().await`
- Verify ports are not in use
- Check Docker is running (if using Docker runtime)

### Deployment Fails

- Verify bytecode is valid
- Check environment is in Ready state
- Ensure sufficient gas/funds

### Execution Timeout

- Increase `timeout_seconds` in config
- Optimize contract code
- Check for infinite loops

## Getting Help

- **Documentation**: See `/docs/` directory
- **Examples**: Check `examples/` directory
- **Issues**: https://github.com/redasgard/blockchain-runtime/issues
- **Email**: hello@redasgard.com

