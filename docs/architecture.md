# Architecture

## System Overview

Blockchain Runtime implements a **blockchain-agnostic abstraction layer** that provides a unified interface for executing, testing, and analyzing smart contracts across different blockchain platforms.

```
┌─────────────────────────────────────────────────────────────┐
│                  Application Layer                           │
│         (Security Tools, Testing Frameworks)                 │
└───────────────────┬──────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────────┐
│              BlockchainRuntime Trait                         │
│           (Blockchain-Agnostic Interface)                    │
├─────────────────────────────────────────────────────────────┤
│  - create_environment()                                      │
│  - execute()                                                 │
│  - deploy_contract()                                         │
│  - call_function()                                           │
│  - monitor()                                                 │
│  - capabilities()                                            │
└───────────────────┬──────────────────────────────────────────┘
                    │
       ┌────────────┴────────────┬────────────┬────────────┐
       │                         │            │            │
       ▼                         ▼            ▼            ▼
┌──────────────┐        ┌──────────────┐  ┌───────┐  ┌───────┐
│   Ethereum   │        │   Solana     │  │ Near  │  │Custom │
│   Runtime    │        │   Runtime    │  │Runtime│  │Runtime│
└──────┬───────┘        └──────┬───────┘  └───┬───┘  └───┬───┘
       │                       │              │          │
       ▼                       ▼              ▼          ▼
┌──────────────┐        ┌──────────────┐  ┌──────────────┐
│ Docker       │        │ LocalProcess │  │CloudInstance │
│ Container    │        │ (ganache)    │  │ (Alchemy)    │
└──────────────┘        └──────────────┘  └──────────────┘
```

## Core Components

### 1. BlockchainRuntime Trait

Main interface for all blockchain operations.

**Responsibilities:**
- Environment lifecycle management
- Code execution
- Contract deployment
- Function calls
- Event monitoring
- Metrics collection

**Definition:**
```rust
#[async_trait]
pub trait BlockchainRuntime: Send + Sync {
    fn blockchain_id(&self) -> &str;
    async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment>;
    async fn execute(&self, env: &RuntimeEnvironment, code: &Path, inputs: &ExecutionInputs) -> Result<ExecutionResult>;
    async fn deploy_contract(&self, env: &RuntimeEnvironment, bytecode: &[u8], args: &[u8]) -> Result<String>;
    async fn call_function(&self, env: &RuntimeEnvironment, address: &str, function: &str, args: &[u8]) -> Result<Vec<u8>>;
    fn metrics_definition(&self) -> Vec<RuntimeMetricDefinition>;
    async fn monitor(&self, env: &RuntimeEnvironment, execution_id: &str) -> Result<Vec<RuntimeEvent>>;
    async fn destroy(&self, env: RuntimeEnvironment) -> Result<()>;
    async fn is_available(&self) -> bool;
    fn capabilities(&self) -> RuntimeCapabilities;
}
```

**Location:** `src/lib.rs`

### 2. Runtime Environment

Represents an isolated blockchain execution environment.

**Structure:**
```rust
pub struct RuntimeEnvironment {
    pub environment_id: String,          // Unique identifier
    pub blockchain_id: String,           // ethereum, solana, etc.
    pub runtime_type: RuntimeType,       // Docker, LocalProcess, etc.
    pub endpoint_url: String,            // RPC endpoint
    pub state: EnvironmentState,         // Creating, Ready, Running, etc.
    pub metadata: HashMap<String, serde_json::Value>,
}
```

**States:**
```
Creating → Ready → Running → Stopped
              ↓
            Error
```

### 3. Runtime Configuration

Configuration for creating runtime environments.

**Structure:**
```rust
pub struct RuntimeConfig {
    pub timeout_seconds: u64,            // Max execution time
    pub memory_limit_mb: u64,            // Memory limit
    pub network_mode: NetworkMode,       // Local, Testnet, MainnetFork
    pub enable_monitoring: bool,         // Enable event monitoring
    pub blockchain_config: HashMap<String, serde_json::Value>,
}
```

**Default Values:**
- timeout_seconds: 300 (5 minutes)
- memory_limit_mb: 1024 (1GB)
- network_mode: Local
- enable_monitoring: true

### 4. Execution Flow

#### Contract Deployment

```
User Request: deploy_contract(bytecode, args)
  │
  ├─ 1. Validate environment is Ready
  ├─ 2. Prepare deployment transaction
  ├─ 3. Submit to blockchain
  ├─ 4. Wait for confirmation
  ├─ 5. Extract contract address
  └─ 6. Return address

Result: Contract address string
```

#### Function Call

```
User Request: call_function(address, function, args)
  │
  ├─ 1. Validate contract exists
  ├─ 2. Encode function call
  ├─ 3. Create transaction
  ├─ 4. Execute transaction
  ├─ 5. Decode return value
  └─ 6. Return result

Result: Raw bytes (caller decodes)
```

#### Code Execution

```
User Request: execute(code_path, inputs)
  │
  ├─ 1. Read code file
  ├─ 2. Compile if needed
  ├─ 3. Deploy to environment
  ├─ 4. Call target function
  ├─ 5. Collect metrics
  ├─ 6. Capture events
  ├─ 7. Track state changes
  └─ 8. Return ExecutionResult

Result: ExecutionResult with metrics, events, state changes
```

## Runtime Types

### Docker Runtime

```
┌─────────────────────────────────────────┐
│  Docker Container                       │
├─────────────────────────────────────────┤
│  ┌───────────────────────────────────┐  │
│  │  Blockchain Node (geth, anvil)   │  │
│  │  - Local network                  │  │
│  │  - Isolated filesystem            │  │
│  │  - Port forwarding                │  │
│  └───────────────────────────────────┘  │
└────────────┬────────────────────────────┘
             │ RPC calls
             ▼
      Application Code
```

**Advantages:**
- Complete isolation
- Reproducible environments
- Easy cleanup
- No host pollution

**Use Cases:**
- CI/CD pipelines
- Parallel test execution
- Security analysis

### LocalProcess Runtime

```
┌─────────────────────────────────────────┐
│  Host Process                           │
├─────────────────────────────────────────┤
│  ganache-cli                            │
│  hardhat node                           │
│  anvil                                  │
└────────────┬────────────────────────────┘
             │ RPC calls (localhost)
             ▼
      Application Code
```

**Advantages:**
- Fast startup
- Lower resource usage
- Easy debugging
- Direct access

**Use Cases:**
- Local development
- Quick testing
- Debugging

### CloudInstance Runtime

```
┌─────────────────────────────────────────┐
│  Cloud Provider (Alchemy, Infura)      │
├─────────────────────────────────────────┤
│  Test Network or Mainnet Fork          │
└────────────┬────────────────────────────┘
             │ HTTPS RPC calls
             ▼
      Application Code
```

**Advantages:**
- No local setup
- Realistic environment
- Persistent state
- Shared access

**Use Cases:**
- Integration testing
- Mainnet forking
- Team development

### InMemory Runtime

```
┌─────────────────────────────────────────┐
│  Application Process                    │
├─────────────────────────────────────────┤
│  ┌───────────────────────────────────┐  │
│  │  Simulated Blockchain             │  │
│  │  - In-memory state                │  │
│  │  - Mock contracts                 │  │
│  │  - Fast execution                 │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

**Advantages:**
- Extremely fast
- No external dependencies
- Deterministic
- Lightweight

**Use Cases:**
- Unit testing
- Fuzzing
- Property testing

## Network Modes

### Local Mode

```
Local Network (Chain ID: 1337)
  │
  ├─ Pre-funded accounts
  ├─ No gas costs (free)
  ├─ Fast block times
  ├─ No real funds
  └─ Full control
```

**Use Case:** Development and testing

### Testnet Mode

```
Public Testnet (Goerli, Sepolia, Devnet)
  │
  ├─ Real network behavior
  ├─ Free test tokens (faucets)
  ├─ Public visibility
  ├─ Network delays
  └─ Shared state
```

**Use Case:** Integration testing, staging

### MainnetFork Mode

```
Mainnet Fork (at specific block)
  │
  ├─ Real mainnet state
  ├─ Real contracts available
  ├─ Local execution
  ├─ No real funds spent
  └─ Realistic testing
```

**Use Case:** Testing against real contracts

## Execution Result Structure

```rust
pub struct ExecutionResult {
    pub execution_id: String,                    // Unique execution ID
    pub success: bool,                           // Success/failure
    pub return_value: Option<serde_json::Value>, // Decoded return value
    pub error: Option<String>,                   // Error message
    pub metrics: HashMap<String, serde_json::Value>, // Metrics collected
    pub state_changes: Vec<StateChange>,         // State modifications
    pub events: Vec<RuntimeEvent>,               // Emitted events
    pub execution_time_ms: u64,                  // Time taken
}
```

### State Changes

```rust
pub struct StateChange {
    pub key: String,                             // Storage key
    pub old_value: Option<serde_json::Value>,   // Previous value
    pub new_value: serde_json::Value,           // New value
    pub change_type: StateChangeType,           // Created/Updated/Deleted
}
```

### Runtime Events

```rust
pub struct RuntimeEvent {
    pub event_id: String,                        // Unique event ID
    pub event_type: String,                      // Event name
    pub timestamp: u64,                          // When emitted
    pub data: HashMap<String, serde_json::Value>, // Event data
}
```

## Metrics System

### Metric Definitions

```rust
pub struct RuntimeMetricDefinition {
    pub name: String,                            // Metric name
    pub description: String,                     // What it measures
    pub unit: String,                            // Unit (gas, ms, bytes)
    pub metric_type: MetricType,                 // Type classification
}
```

### Common Metrics

| Blockchain | Metric | Unit | Description |
|------------|--------|------|-------------|
| Ethereum | gas_used | units | Gas consumed |
| Ethereum | gas_price | gwei | Gas price |
| Ethereum | storage_delta | bytes | Storage change |
| Solana | compute_units | units | Compute consumed |
| Solana | heap_memory | bytes | Heap usage |
| All | execution_time | ms | Time taken |
| All | state_changes | count | State modifications |

## Capabilities System

```rust
pub struct RuntimeCapabilities {
    pub supports_contract_deployment: bool,      // Can deploy contracts
    pub supports_function_calls: bool,           // Can call functions
    pub supports_state_inspection: bool,         // Can inspect state
    pub supports_event_monitoring: bool,         // Can monitor events
    pub supports_gas_estimation: bool,           // Can estimate gas
    pub supports_time_travel: bool,              // Can manipulate time
    pub max_execution_time_seconds: u64,         // Max execution time
}
```

**Usage:**
```rust
let caps = runtime.capabilities();

if caps.supports_time_travel {
    // Can use evm_increaseTime, evm_mine, etc.
}
```

## Error Handling

All operations return `anyhow::Result<T>` for flexibility.

### Common Errors

- `EnvironmentNotReady` - Environment not in Ready state
- `DeploymentFailed` - Contract deployment failed
- `ExecutionTimeout` - Execution exceeded timeout
- `ContractNotFound` - Contract address not found
- `InsufficientFunds` - Not enough balance
- `InvalidInput` - Malformed input data

### Error Propagation

```rust
async fn execute_safely(runtime: &dyn BlockchainRuntime) -> Result<()> {
    let env = runtime.create_environment(config).await
        .context("Failed to create environment")?;
    
    let address = runtime.deploy_contract(&env, bytecode, &args).await
        .context("Failed to deploy contract")?;
    
    Ok(())
}
```

## Thread Safety

All types implement `Send + Sync`:
- `BlockchainRuntime`: Thread-safe trait object
- `RuntimeEnvironment`: Can be shared across threads
- `ExecutionResult`: Immutable, thread-safe

## Performance Characteristics

### Environment Creation

| Runtime Type | Time | Resource Usage |
|-------------|------|----------------|
| Docker | 2-5s | High (container overhead) |
| LocalProcess | 1-2s | Medium |
| CloudInstance | <1s | Low (remote) |
| InMemory | <100ms | Very low |

### Execution

| Operation | Typical Time |
|-----------|-------------|
| Deploy contract | 500ms - 2s |
| Call function | 100ms - 500ms |
| State inspection | 50ms - 200ms |
| Event monitoring | 100ms - 1s |

## Security Considerations

### Isolation

- Docker provides strong isolation
- LocalProcess shares host resources
- CloudInstance requires API keys
- InMemory has no external access

### Resource Limits

- Timeout prevents infinite execution
- Memory limits prevent OOM
- Cleanup on destroy prevents leaks

## Future Enhancements

### v0.2
- Snapshot/restore functionality
- Time travel capabilities
- Enhanced monitoring

### v0.3
- Multi-environment orchestration
- Performance profiling
- Advanced debugging

### v0.4
- Smart contract fuzzing
- Formal verification integration
- Cross-chain testing

