# API Reference

Complete API documentation for Blockchain Runtime.

## Core Trait

### BlockchainRuntime

Main trait for blockchain runtime implementations.

```rust
#[async_trait]
pub trait BlockchainRuntime: Send + Sync
```

#### Required Methods

##### `blockchain_id()`

```rust
fn blockchain_id(&self) -> &str
```

Get the blockchain identifier.

**Returns:** Blockchain ID (e.g., "ethereum", "solana", "near")

**Example:**
```rust
assert_eq!(runtime.blockchain_id(), "ethereum");
```

---

##### `create_environment()`

```rust
async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment>
```

Create a new runtime environment.

**Parameters:**
- `config` - Configuration for the environment

**Returns:** `Result<RuntimeEnvironment>` - Created environment

**Example:**
```rust
let config = RuntimeConfig::default();
let env = runtime.create_environment(config).await?;
```

---

##### `execute()`

```rust
async fn execute(
    &self,
    env: &RuntimeEnvironment,
    code_path: &Path,
    inputs: &ExecutionInputs,
) -> Result<ExecutionResult>
```

Execute code in the runtime environment.

**Parameters:**
- `env` - Runtime environment
- `code_path` - Path to code file
- `inputs` - Execution inputs

**Returns:** `Result<ExecutionResult>` - Execution results with metrics

**Example:**
```rust
let inputs = ExecutionInputs {
    target_function: "test".to_string(),
    parameters: HashMap::new(),
    context: ExecutionContext::default(),
};

let result = runtime.execute(&env, Path::new("contract.sol"), &inputs).await?;
println!("Success: {}", result.success);
```

---

##### `deploy_contract()`

```rust
async fn deploy_contract(
    &self,
    env: &RuntimeEnvironment,
    bytecode: &[u8],
    constructor_args: &[u8],
) -> Result<String>
```

Deploy a contract to the blockchain.

**Parameters:**
- `env` - Runtime environment
- `bytecode` - Compiled contract bytecode
- `constructor_args` - ABI-encoded constructor arguments

**Returns:** `Result<String>` - Deployed contract address

**Example:**
```rust
let bytecode = std::fs::read("contract.bin")?;
let address = runtime.deploy_contract(&env, &bytecode, &[]).await?;
println!("Deployed at: {}", address);
```

---

##### `call_function()`

```rust
async fn call_function(
    &self,
    env: &RuntimeEnvironment,
    contract_address: &str,
    function: &str,
    args: &[u8],
) -> Result<Vec<u8>>
```

Call a contract function.

**Parameters:**
- `env` - Runtime environment
- `contract_address` - Contract address
- `function` - Function name
- `args` - ABI-encoded function arguments

**Returns:** `Result<Vec<u8>>` - Raw return value

**Example:**
```rust
let result = runtime.call_function(
    &env,
    "0x742d35Cc...",
    "transfer",
    &encoded_args,
).await?;
```

---

##### `metrics_definition()`

```rust
fn metrics_definition(&self) -> Vec<RuntimeMetricDefinition>
```

Get definitions of metrics this runtime can provide.

**Returns:** `Vec<RuntimeMetricDefinition>` - Available metrics

**Example:**
```rust
let metrics = runtime.metrics_definition();
for metric in metrics {
    println!("{}: {} ({})", metric.name, metric.description, metric.unit);
}
```

---

##### `monitor()`

```rust
async fn monitor(
    &self,
    env: &RuntimeEnvironment,
    execution_id: &str,
) -> Result<Vec<RuntimeEvent>>
```

Monitor runtime events for an execution.

**Parameters:**
- `env` - Runtime environment
- `execution_id` - Execution to monitor

**Returns:** `Result<Vec<RuntimeEvent>>` - Captured events

**Example:**
```rust
let events = runtime.monitor(&env, &execution_id).await?;
for event in events {
    println!("Event: {}", event.event_type);
}
```

---

##### `destroy()`

```rust
async fn destroy(&self, env: RuntimeEnvironment) -> Result<()>
```

Destroy a runtime environment and clean up resources.

**Parameters:**
- `env` - Environment to destroy

**Returns:** `Result<()>`

**Example:**
```rust
runtime.destroy(env).await?;
```

---

##### `is_available()`

```rust
async fn is_available(&self) -> bool
```

Check if runtime is available and functional.

**Returns:** `bool` - true if available

**Example:**
```rust
if runtime.is_available().await {
    println!("Runtime is ready");
}
```

---

##### `capabilities()`

```rust
fn capabilities(&self) -> RuntimeCapabilities
```

Get runtime capabilities.

**Returns:** `RuntimeCapabilities` - What this runtime can do

**Example:**
```rust
let caps = runtime.capabilities();
if caps.supports_gas_estimation {
    // Use gas estimation features
}
```

---

## Configuration Types

### RuntimeConfig

```rust
pub struct RuntimeConfig {
    pub timeout_seconds: u64,
    pub memory_limit_mb: u64,
    pub network_mode: NetworkMode,
    pub enable_monitoring: bool,
    pub blockchain_config: HashMap<String, serde_json::Value>,
}
```

**Default:**
```rust
RuntimeConfig {
    timeout_seconds: 300,
    memory_limit_mb: 1024,
    network_mode: NetworkMode::Local,
    enable_monitoring: true,
    blockchain_config: HashMap::new(),
}
```

### NetworkMode

```rust
pub enum NetworkMode {
    Local,          // Local test network
    Testnet,        // Public testnet
    MainnetFork,    // Mainnet fork
}
```

### RuntimeType

```rust
pub enum RuntimeType {
    Docker,         // Containerized
    LocalProcess,   // Native process
    CloudInstance,  // Cloud-hosted
    InMemory,       // In-memory simulation
}
```

### EnvironmentState

```rust
pub enum EnvironmentState {
    Creating,       // Being created
    Ready,          // Ready for use
    Running,        // Currently executing
    Stopped,        // Stopped
    Error,          // Error state
}
```

---

## Result Types

### ExecutionResult

```rust
pub struct ExecutionResult {
    pub execution_id: String,
    pub success: bool,
    pub return_value: Option<serde_json::Value>,
    pub error: Option<String>,
    pub metrics: HashMap<String, serde_json::Value>,
    pub state_changes: Vec<StateChange>,
    pub events: Vec<RuntimeEvent>,
    pub execution_time_ms: u64,
}
```

### StateChange

```rust
pub struct StateChange {
    pub key: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: serde_json::Value,
    pub change_type: StateChangeType,
}

pub enum StateChangeType {
    Created,
    Updated,
    Deleted,
}
```

### RuntimeEvent

```rust
pub struct RuntimeEvent {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: u64,
    pub data: HashMap<String, serde_json::Value>,
}
```

---

## Input Types

### ExecutionInputs

```rust
pub struct ExecutionInputs {
    pub target_function: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub context: ExecutionContext,
}
```

### ExecutionContext

```rust
pub struct ExecutionContext {
    pub sender: Option<String>,           // Calling address
    pub block_number: Option<u64>,        // Block number
    pub timestamp: Option<u64>,           // Block timestamp
    pub extra: HashMap<String, serde_json::Value>, // Extra context
}
```

---

## Capability Types

### RuntimeCapabilities

```rust
pub struct RuntimeCapabilities {
    pub supports_contract_deployment: bool,
    pub supports_function_calls: bool,
    pub supports_state_inspection: bool,
    pub supports_event_monitoring: bool,
    pub supports_gas_estimation: bool,
    pub supports_time_travel: bool,
    pub max_execution_time_seconds: u64,
}
```

### RuntimeMetricDefinition

```rust
pub struct RuntimeMetricDefinition {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub metric_type: MetricType,
}

pub enum MetricType {
    Gas,
    ComputeUnits,
    StorageBytes,
    Time,
    Custom(String),
}
```

---

## Thread Safety

All trait methods are async and require `Send + Sync`:

```rust
pub trait BlockchainRuntime: Send + Sync {
    // All methods can be called from any thread
}
```

Safe for concurrent operations across threads.

---

## Performance Characteristics

| Operation | Typical Time | Notes |
|-----------|-------------|-------|
| `create_environment()` | 1-5s | Depends on runtime type |
| `deploy_contract()` | 0.5-2s | Network dependent |
| `call_function()` | 100-500ms | Function complexity |
| `execute()` | 200ms-2s | Code complexity |
| `monitor()` | 100ms-1s | Event count |
| `destroy()` | 500ms-2s | Cleanup overhead |

---

## Example Patterns

### Resource Management

```rust
struct ManagedRuntime {
    runtime: Box<dyn BlockchainRuntime>,
}

impl ManagedRuntime {
    async fn with_environment<F, T>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(&blockchain_runtime::RuntimeEnvironment) -> anyhow::Result<T>,
    {
        let config = RuntimeConfig::default();
        let env = self.runtime.create_environment(config).await?;
        
        let result = f(&env)?;
        
        self.runtime.destroy(env).await?;
        
        Ok(result)
    }
}
```

### Error Recovery

```rust
async fn robust_execution(runtime: &dyn BlockchainRuntime) -> anyhow::Result<()> {
    let config = RuntimeConfig::default();
    
    let env = match runtime.create_environment(config).await {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to create environment: {}", e);
            return Err(e);
        }
    };
    
    // Use environment...
    
    // Always clean up
    if let Err(e) = runtime.destroy(env).await {
        eprintln!("Warning: Failed to destroy environment: {}", e);
    }
    
    Ok(())
}
```

---

## Version Compatibility

Current version: `0.1.0`

**Breaking changes:** Will use semantic versioning (0.x.0 for breaking changes)

**Stability:** API is in development, expect changes in v0.x releases

