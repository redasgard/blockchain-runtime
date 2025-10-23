# Implementation Guide

Guide for implementing the `BlockchainRuntime` trait for your blockchain.

## Overview

To add support for a new blockchain, implement the `BlockchainRuntime` trait. This guide walks you through each method with examples.

## Template

Start with this template:

```rust
use blockchain_runtime::*;
use async_trait::async_trait;
use anyhow::Result;

pub struct MyBlockchainRuntime {
    // Your runtime state (client connections, config, etc.)
}

impl MyBlockchainRuntime {
    pub fn new() -> Self {
        Self {
            // Initialize
        }
    }
}

#[async_trait]
impl BlockchainRuntime for MyBlockchainRuntime {
    fn blockchain_id(&self) -> &str {
        "mychain"
    }
    
    // Implement remaining methods...
}
```

## Method Implementation Guide

### 1. blockchain_id()

Return a unique identifier for your blockchain.

```rust
fn blockchain_id(&self) -> &str {
    "ethereum"  // or "solana", "near", "polygon", etc.
}
```

**Requirements:**
- Lowercase
- Unique across all blockchains
- Consistent with Valkra blockchain enum

**Examples:**
- `"ethereum"`, `"solana"`, `"near"`, `"polygon"`, `"avalanche"`

### 2. create_environment()

Create an isolated runtime environment.

```rust
async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment> {
    // 1. Choose runtime type based on config
    let runtime_type = match config.network_mode {
        NetworkMode::Local => RuntimeType::Docker,
        NetworkMode::Testnet => RuntimeType::CloudInstance,
        NetworkMode::MainnetFork => RuntimeType::LocalProcess,
    };
    
    // 2. Start blockchain node/service
    let endpoint = match runtime_type {
        RuntimeType::Docker => {
            // Start Docker container
            let container = start_docker_container(&config)?;
            format!("http://localhost:{}", container.port)
        }
        RuntimeType::LocalProcess => {
            // Start local process (ganache, hardhat, etc.)
            let process = start_local_node(&config)?;
            process.endpoint.clone()
        }
        RuntimeType::CloudInstance => {
            // Use cloud provider
            let api_key = std::env::var("ALCHEMY_API_KEY")?;
            format!("https://eth-goerli.alchemyapi.io/v2/{}", api_key)
        }
        RuntimeType::InMemory => {
            "memory://local".to_string()
        }
    };
    
    // 3. Wait for readiness
    wait_for_ready(&endpoint, Duration::from_secs(30)).await?;
    
    // 4. Return environment
    Ok(RuntimeEnvironment {
        environment_id: uuid::Uuid::new_v4().to_string(),
        blockchain_id: self.blockchain_id().to_string(),
        runtime_type,
        endpoint_url: endpoint,
        state: EnvironmentState::Ready,
        metadata: HashMap::new(),
    })
}
```

### 3. execute()

Execute code and collect comprehensive results.

```rust
async fn execute(
    &self,
    env: &RuntimeEnvironment,
    code_path: &Path,
    inputs: &ExecutionInputs,
) -> Result<ExecutionResult> {
    let start = std::time::Instant::now();
    
    // 1. Read and compile code
    let code = std::fs::read_to_string(code_path)?;
    let bytecode = compile_code(&code)?;
    
    // 2. Deploy contract
    let address = self.deploy_contract(env, &bytecode, &[]).await?;
    
    // 3. Prepare function call
    let function_args = encode_function_call(
        &inputs.target_function,
        &inputs.parameters,
    )?;
    
    // 4. Execute function
    let result = self.call_function(
        env,
        &address,
        &inputs.target_function,
        &function_args,
    ).await;
    
    // 5. Collect metrics
    let metrics = collect_metrics(env, &address).await?;
    
    // 6. Get state changes
    let state_changes = get_state_changes(env, &address).await?;
    
    // 7. Get events
    let events = get_events(env, &address).await?;
    
    // 8. Build result
    Ok(ExecutionResult {
        execution_id: uuid::Uuid::new_v4().to_string(),
        success: result.is_ok(),
        return_value: result.ok().map(|v| serde_json::to_value(v).ok()).flatten(),
        error: result.err().map(|e| e.to_string()),
        metrics,
        state_changes,
        events,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}
```

### 4. deploy_contract()

Deploy a contract and return its address.

```rust
async fn deploy_contract(
    &self,
    env: &RuntimeEnvironment,
    bytecode: &[u8],
    constructor_args: &[u8],
) -> Result<String> {
    // 1. Connect to blockchain
    let client = self.get_client(&env.endpoint_url)?;
    
    // 2. Prepare deployment transaction
    let tx = create_deployment_tx(bytecode, constructor_args)?;
    
    // 3. Sign and send
    let tx_hash = client.send_transaction(tx).await?;
    
    // 4. Wait for confirmation
    let receipt = client.wait_for_transaction(tx_hash).await?;
    
    // 5. Extract contract address
    let address = receipt.contract_address
        .ok_or_else(|| anyhow::anyhow!("No contract address in receipt"))?;
    
    Ok(address)
}
```

### 5. call_function()

Call a deployed contract function.

```rust
async fn call_function(
    &self,
    env: &RuntimeEnvironment,
    contract_address: &str,
    function: &str,
    args: &[u8],
) -> Result<Vec<u8>> {
    // 1. Connect to blockchain
    let client = self.get_client(&env.endpoint_url)?;
    
    // 2. Encode function call
    let call_data = encode_function_call(function, args)?;
    
    // 3. Create transaction
    let tx = create_call_tx(contract_address, call_data)?;
    
    // 4. Send transaction
    let tx_hash = client.send_transaction(tx).await?;
    
    // 5. Wait for result
    let receipt = client.wait_for_transaction(tx_hash).await?;
    
    // 6. Decode return value
    Ok(receipt.return_data)
}
```

### 6. metrics_definition()

Define which metrics your runtime provides.

```rust
fn metrics_definition(&self) -> Vec<RuntimeMetricDefinition> {
    vec![
        RuntimeMetricDefinition {
            name: "gas_used".to_string(),
            description: "Total gas consumed by transaction".to_string(),
            unit: "units".to_string(),
            metric_type: MetricType::Gas,
        },
        RuntimeMetricDefinition {
            name: "gas_price".to_string(),
            description: "Gas price in gwei".to_string(),
            unit: "gwei".to_string(),
            metric_type: MetricType::Gas,
        },
        RuntimeMetricDefinition {
            name: "storage_delta".to_string(),
            description: "Change in storage size".to_string(),
            unit: "bytes".to_string(),
            metric_type: MetricType::StorageBytes,
        },
        RuntimeMetricDefinition {
            name: "execution_time".to_string(),
            description: "Time taken to execute".to_string(),
            unit: "ms".to_string(),
            metric_type: MetricType::Time,
        },
    ]
}
```

### 7. monitor()

Monitor events for an execution.

```rust
async fn monitor(
    &self,
    env: &RuntimeEnvironment,
    execution_id: &str,
) -> Result<Vec<RuntimeEvent>> {
    // 1. Get client
    let client = self.get_client(&env.endpoint_url)?;
    
    // 2. Fetch transaction receipt
    let receipt = client.get_transaction_receipt(execution_id).await?;
    
    // 3. Parse events/logs
    let events = receipt.logs.iter().map(|log| {
        RuntimeEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: decode_event_type(log),
            timestamp: receipt.block_timestamp,
            data: decode_event_data(log),
        }
    }).collect();
    
    Ok(events)
}
```

### 8. destroy()

Clean up environment resources.

```rust
async fn destroy(&self, env: RuntimeEnvironment) -> Result<()> {
    match env.runtime_type {
        RuntimeType::Docker => {
            // Stop and remove Docker container
            stop_docker_container(&env.environment_id).await?;
        }
        RuntimeType::LocalProcess => {
            // Kill local process
            kill_local_process(&env.environment_id).await?;
        }
        RuntimeType::CloudInstance => {
            // Disconnect (no cleanup needed)
        }
        RuntimeType::InMemory => {
            // Free memory (automatic)
        }
    }
    
    Ok(())
}
```

### 9. is_available()

Check if runtime dependencies are available.

```rust
async fn is_available(&self) -> bool {
    // Check required tools/services
    match env::var("CHAIN_RPC_URL") {
        Ok(_) => true,
        Err(_) => {
            // Try to start local node
            self.can_start_local_node()
        }
    }
}
```

### 10. capabilities()

Declare what your runtime can do.

```rust
fn capabilities(&self) -> RuntimeCapabilities {
    RuntimeCapabilities {
        supports_contract_deployment: true,
        supports_function_calls: true,
        supports_state_inspection: true,
        supports_event_monitoring: true,
        supports_gas_estimation: true,    // If your chain supports gas
        supports_time_travel: false,       // If you can manipulate block time
        max_execution_time_seconds: 600,
    }
}
```

## Complete Example

Here's a complete minimal implementation:

```rust
use blockchain_runtime::*;
use async_trait::async_trait;
use anyhow::{Result, Context};
use std::path::Path;
use std::collections::HashMap;

pub struct SimpleRuntime {
    node_url: String,
}

impl SimpleRuntime {
    pub fn new(node_url: String) -> Self {
        Self { node_url }
    }
}

#[async_trait]
impl BlockchainRuntime for SimpleRuntime {
    fn blockchain_id(&self) -> &str {
        "simple-chain"
    }
    
    async fn create_environment(&self, _config: RuntimeConfig) -> Result<RuntimeEnvironment> {
        Ok(RuntimeEnvironment {
            environment_id: uuid::Uuid::new_v4().to_string(),
            blockchain_id: "simple-chain".to_string(),
            runtime_type: RuntimeType::LocalProcess,
            endpoint_url: self.node_url.clone(),
            state: EnvironmentState::Ready,
            metadata: HashMap::new(),
        })
    }
    
    async fn execute(
        &self,
        _env: &RuntimeEnvironment,
        _code_path: &Path,
        _inputs: &ExecutionInputs,
    ) -> Result<ExecutionResult> {
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
        _env: &RuntimeEnvironment,
        _bytecode: &[u8],
        _constructor_args: &[u8],
    ) -> Result<String> {
        Ok("0x0000000000000000000000000000000000000000".to_string())
    }
    
    async fn call_function(
        &self,
        _env: &RuntimeEnvironment,
        _contract_address: &str,
        _function: &str,
        _args: &[u8],
    ) -> Result<Vec<u8>> {
        Ok(vec![])
    }
    
    fn metrics_definition(&self) -> Vec<RuntimeMetricDefinition> {
        vec![]
    }
    
    async fn monitor(
        &self,
        _env: &RuntimeEnvironment,
        _execution_id: &str,
    ) -> Result<Vec<RuntimeEvent>> {
        Ok(vec![])
    }
    
    async fn destroy(&self, _env: RuntimeEnvironment) -> Result<()> {
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

## Testing Your Implementation

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_environment_creation() {
        let runtime = SimpleRuntime::new("http://localhost:8545".to_string());
        let config = RuntimeConfig::default();
        
        let env = runtime.create_environment(config).await.unwrap();
        
        assert_eq!(env.blockchain_id, "simple-chain");
        assert_eq!(env.state, EnvironmentState::Ready);
        
        runtime.destroy(env).await.unwrap();
    }
    
    #[tokio::test]
    async fn test_contract_deployment() {
        let runtime = SimpleRuntime::new("http://localhost:8545".to_string());
        let config = RuntimeConfig::default();
        let env = runtime.create_environment(config).await.unwrap();
        
        let bytecode = vec![0x60, 0x60, 0x60]; // Sample bytecode
        let address = runtime.deploy_contract(&env, &bytecode, &[]).await.unwrap();
        
        assert!(!address.is_empty());
        
        runtime.destroy(env).await.unwrap();
    }
}
```

## Integration Patterns

### With Web3 Libraries (Ethereum)

```rust
use ethers::prelude::*;

pub struct EthereumRuntime {
    provider: Provider<Http>,
}

#[async_trait]
impl BlockchainRuntime for EthereumRuntime {
    async fn deploy_contract(
        &self,
        env: &RuntimeEnvironment,
        bytecode: &[u8],
        constructor_args: &[u8],
    ) -> Result<String> {
        // Use ethers-rs for deployment
        let factory = ContractFactory::new(
            abi.clone(),
            bytecode.to_vec().into(),
            self.provider.clone(),
        );
        
        let contract = factory.deploy(constructor_args)?.send().await?;
        
        Ok(format!("{:?}", contract.address()))
    }
}
```

### With Solana SDK

```rust
use solana_sdk::signature::Keypair;
use solana_client::rpc_client::RpcClient;

pub struct SolanaRuntime {
    client: RpcClient,
    payer: Keypair,
}

#[async_trait]
impl BlockchainRuntime for SolanaRuntime {
    async fn deploy_contract(
        &self,
        env: &RuntimeEnvironment,
        bytecode: &[u8],
        _constructor_args: &[u8],
    ) -> Result<String> {
        // Deploy Solana program
        let program_id = deploy_program(&self.client, &self.payer, bytecode)?;
        
        Ok(program_id.to_string())
    }
}
```

## Best Practices

### 1. Resource Cleanup

Always clean up in `destroy()`:

```rust
async fn destroy(&self, env: RuntimeEnvironment) -> Result<()> {
    // Stop processes
    // Remove containers
    // Free memory
    // Close connections
    
    // Log but don't fail
    if let Err(e) = cleanup_resources(&env).await {
        eprintln!("Warning: Cleanup failed: {}", e);
    }
    
    Ok(())
}
```

### 2. Comprehensive Error Handling

```rust
async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment> {
    let container = start_container(&config)
        .await
        .context("Failed to start Docker container")?;
    
    wait_for_ready(&container.endpoint, Duration::from_secs(30))
        .await
        .context("Container failed to become ready")?;
    
    Ok(RuntimeEnvironment { /* ... */ })
}
```

### 3. Idempotent Operations

```rust
async fn destroy(&self, env: RuntimeEnvironment) -> Result<()> {
    // Check if already destroyed
    if !container_exists(&env.environment_id) {
        return Ok(());
    }
    
    stop_container(&env.environment_id).await?;
    Ok(())
}
```

### 4. Proper Metrics

```rust
fn metrics_definition(&self) -> Vec<RuntimeMetricDefinition> {
    vec![
        // Blockchain-specific metrics
        RuntimeMetricDefinition {
            name: "gas_used".to_string(),
            description: "Gas consumed".to_string(),
            unit: "units".to_string(),
            metric_type: MetricType::Gas,
        },
        // Universal metrics
        RuntimeMetricDefinition {
            name: "execution_time".to_string(),
            description: "Execution duration".to_string(),
            unit: "ms".to_string(),
            metric_type: MetricType::Time,
        },
    ]
}
```

## Helper Functions

### Docker Integration

```rust
async fn start_docker_container(config: &RuntimeConfig) -> Result<DockerContainer> {
    let docker = Docker::connect_with_local_defaults()?;
    
    let container_config = Config {
        image: Some("ethereum/client-go:latest"),
        exposed_ports: Some(HashMap::from([("8545/tcp", HashMap::new())])),
        // ... more config
    };
    
    let container = docker.create_container::<String, String>(None, container_config).await?;
    docker.start_container(&container.id, None).await?;
    
    Ok(DockerContainer {
        id: container.id,
        port: 8545,
    })
}

struct DockerContainer {
    id: String,
    port: u16,
}
```

### RPC Client

```rust
async fn get_client(&self, endpoint: &str) -> Result<Box<dyn RpcClient>> {
    match self.blockchain_id() {
        "ethereum" => Ok(Box::new(EthereumClient::new(endpoint)?)),
        "solana" => Ok(Box::new(SolanaClient::new(endpoint)?)),
        _ => anyhow::bail!("Unsupported blockchain"),
    }
}
```

## Common Pitfalls

### ❌ Forgetting to Wait for Readiness

```rust
// Bad
let env = start_container();
return Ok(env);  // May not be ready!

// Good
let env = start_container();
wait_for_ready(&env.endpoint_url).await?;
return Ok(env);
```

### ❌ Not Handling Timeouts

```rust
// Bad
let result = execute_forever().await?;

// Good
let result = timeout(
    Duration::from_secs(config.timeout_seconds),
    execute_with_timeout()
).await??;
```

### ❌ Leaking Resources

```rust
// Bad
async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment> {
    start_container();
    // If error occurs here, container leaks!
    do_something_that_might_fail()?;
}

// Good
async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment> {
    let container = start_container();
    
    match do_something_that_might_fail() {
        Ok(_) => Ok(create_env(container)),
        Err(e) => {
            stop_container(container).await?;
            Err(e)
        }
    }
}
```

## Testing Checklist

- [ ] `blockchain_id()` returns correct ID
- [ ] `create_environment()` creates usable environment
- [ ] `execute()` works with simple contract
- [ ] `deploy_contract()` returns valid address
- [ ] `call_function()` executes successfully
- [ ] `metrics_definition()` returns valid metrics
- [ ] `monitor()` captures events
- [ ] `destroy()` cleans up all resources
- [ ] `is_available()` checks dependencies
- [ ] `capabilities()` accurately reflects features
- [ ] All async operations are cancellation-safe
- [ ] No resource leaks
- [ ] Error messages are helpful

## Next Steps

- Review [Architecture](./architecture.md) for design patterns
- Check [User Guide](./user-guide.md) for usage examples
- See [Testing Guide](./testing.md) for testing strategies

