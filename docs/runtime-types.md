# Runtime Types

Detailed explanation of different runtime types and when to use them.

## Overview

Blockchain Runtime supports four types of runtime environments, each with different trade-offs:

```
RuntimeType::Docker        - Containerized blockchain nodes
RuntimeType::LocalProcess  - Native process execution
RuntimeType::CloudInstance - Cloud-hosted services
RuntimeType::InMemory      - In-memory simulation
```

## Docker Runtime

### Description

Runs blockchain node in a Docker container with complete isolation.

### Architecture

```
Host Machine
└── Docker Container
    ├── Blockchain Node (geth, anvil, etc.)
    ├── Isolated Filesystem
    ├── Isolated Network
    └── Port Forwarding (8545, etc.)
```

### Advantages

- ✅ **Complete Isolation**: No impact on host system
- ✅ **Reproducible**: Same environment every time
- ✅ **Clean Cleanup**: Remove container, all state gone
- ✅ **Parallel Execution**: Run multiple containers simultaneously
- ✅ **Resource Limits**: Docker memory/CPU limits
- ✅ **Security**: Sandboxed execution

### Disadvantages

- ❌ **Slower Startup**: 2-5 seconds to start container
- ❌ **Resource Overhead**: Docker daemon overhead
- ❌ **Requires Docker**: External dependency
- ❌ **Complexity**: Container management

### Use Cases

- CI/CD pipelines
- Security analysis (untrusted code)
- Parallel test execution
- Production-like testing

### Example

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::Local,
    ..Default::default()
};

let env = runtime.create_environment(config).await?;
assert_eq!(env.runtime_type, RuntimeType::Docker);
```

---

## LocalProcess Runtime

### Description

Runs blockchain node as a native process on the host machine.

### Architecture

```
Host Machine
├── Blockchain Node Process (ganache-cli, hardhat node, anvil)
│   ├── Listens on localhost:8545
│   └── Shares host resources
└── Your Application
    └── Connects via HTTP RPC
```

### Advantages

- ✅ **Fast Startup**: 1-2 seconds
- ✅ **Low Overhead**: No containerization overhead
- ✅ **Easy Debugging**: Direct process access
- ✅ **Simple**: No Docker required
- ✅ **Good Performance**: Native execution

### Disadvantages

- ❌ **Less Isolation**: Shares host resources
- ❌ **Port Conflicts**: May conflict with other services
- ❌ **Cleanup Issues**: Process may not terminate cleanly
- ❌ **State Persistence**: May leave state on filesystem

### Use Cases

- Local development
- Quick testing
- Debugging
- IDE integration

### Example

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::Local,
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("use_process".to_string(), serde_json::json!(true));
        map
    },
    ..Default::default()
};

let env = runtime.create_environment(config).await?;
assert_eq!(env.runtime_type, RuntimeType::LocalProcess);
```

---

## CloudInstance Runtime

### Description

Connects to cloud-hosted blockchain services (Alchemy, Infura, QuickNode).

### Architecture

```
Your Application
    │
    │ HTTPS RPC
    ▼
Cloud Provider (Alchemy, Infura)
    │
    └── Managed Blockchain Node
        ├── Testnet
        ├── Mainnet Fork
        └── Archive Node
```

### Advantages

- ✅ **No Local Setup**: No node management
- ✅ **Fast Startup**: <1 second (just connection)
- ✅ **Scalable**: Cloud provider handles load
- ✅ **Reliable**: Professional infrastructure
- ✅ **Archive Access**: Historical state available
- ✅ **Mainnet Fork**: Test against real state

### Disadvantages

- ❌ **Requires API Key**: External dependency
- ❌ **Rate Limits**: API call limits
- ❌ **Network Latency**: Internet connection required
- ❌ **Cost**: May have usage fees
- ❌ **Less Control**: Provider limitations

### Use Cases

- Integration testing
- Mainnet fork testing
- Testing against real contracts
- Team collaboration

### Example

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::MainnetFork,
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("api_key".to_string(), serde_json::json!(env::var("ALCHEMY_API_KEY")?));
        map.insert("fork_block".to_string(), serde_json::json!(17000000));
        map
    },
    ..Default::default()
};

let env = runtime.create_environment(config).await?;
assert_eq!(env.runtime_type, RuntimeType::CloudInstance);
```

---

## InMemory Runtime

### Description

Simulates blockchain in-memory without external processes.

### Architecture

```
Your Application Process
└── In-Memory Blockchain Simulator
    ├── Memory State Store
    ├── Mock Transaction Processing
    └── Simulated Events
```

### Advantages

- ✅ **Extremely Fast**: <100ms startup
- ✅ **No Dependencies**: Pure Rust
- ✅ **Deterministic**: Consistent results
- ✅ **Lightweight**: Minimal resource usage
- ✅ **Portable**: Works anywhere

### Disadvantages

- ❌ **Limited Realism**: Not a real blockchain
- ❌ **Missing Features**: May not support all operations
- ❌ **No Network**: Can't test network behavior
- ❌ **Simplified**: May miss edge cases

### Use Cases

- Unit testing
- Fuzzing
- Property-based testing
- Embedded systems

### Example

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::Local,
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("use_in_memory".to_string(), serde_json::json!(true));
        map
    },
    ..Default::default()
};

let env = runtime.create_environment(config).await?;
assert_eq!(env.runtime_type, RuntimeType::InMemory);
```

---

## Comparison Matrix

| Feature | Docker | LocalProcess | CloudInstance | InMemory |
|---------|--------|--------------|---------------|----------|
| Startup Time | 2-5s | 1-2s | <1s | <100ms |
| Resource Usage | High | Medium | Low | Very Low |
| Isolation | Excellent | Good | N/A | Excellent |
| Requires Docker | Yes | No | No | No |
| Requires Network | No | No | Yes | No |
| Parallel Execution | Excellent | Good | Excellent | Excellent |
| Debugging | Good | Excellent | Limited | Excellent |
| Realism | Excellent | Excellent | Excellent | Limited |
| Cost | Free | Free | May cost | Free |

## Choosing a Runtime Type

### Development Workflow

```
┌─────────────────────────────────────────┐
│ Development Phase → Runtime Type        │
├─────────────────────────────────────────┤
│ Unit Testing      → InMemory            │
│ Local Development → LocalProcess        │
│ Integration Tests → Docker              │
│ Staging          → CloudInstance        │
│ Mainnet Testing  → CloudInstance (Fork) │
└─────────────────────────────────────────┘
```

### Decision Tree

```
Need real blockchain behavior?
├─ Yes
│  ├─ Need mainnet state?
│  │  └─ CloudInstance (MainnetFork)
│  ├─ Need isolation?
│  │  └─ Docker
│  └─ Need speed?
│     └─ LocalProcess
└─ No (unit testing)
   └─ InMemory
```

## Runtime Type Configuration

### Docker Configuration

```rust
let config = RuntimeConfig {
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("docker_image".to_string(), serde_json::json!("ethereum/client-go:latest"));
        map.insert("docker_network".to_string(), serde_json::json!("test-network"));
        map.insert("container_memory".to_string(), serde_json::json!("2g"));
        map
    },
    ..Default::default()
};
```

### LocalProcess Configuration

```rust
let config = RuntimeConfig {
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("node_binary".to_string(), serde_json::json!("hardhat"));
        map.insert("node_args".to_string(), serde_json::json!(["node", "--port", "8545"]));
        map
    },
    ..Default::default()
};
```

### CloudInstance Configuration

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::MainnetFork,
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("provider".to_string(), serde_json::json!("alchemy"));
        map.insert("api_key".to_string(), serde_json::json!(env::var("API_KEY")?));
        map.insert("network".to_string(), serde_json::json!("mainnet"));
        map.insert("fork_block".to_string(), serde_json::json!(17000000));
        map
    },
    ..Default::default()
};
```

### InMemory Configuration

```rust
let config = RuntimeConfig {
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("initial_balance".to_string(), serde_json::json!("1000000000000000000000"));
        map.insert("accounts_count".to_string(), serde_json::json!(10));
        map
    },
    ..Default::default()
};
```

## Performance Comparison

### Startup Time

```
InMemory:       ████                        (<100ms)
CloudInstance:  ████████                    (<1s)
LocalProcess:   ████████████                (1-2s)
Docker:         ████████████████████        (2-5s)
```

### Execution Speed

```
InMemory:       ████████████████████        (fastest)
LocalProcess:   ████████████████            (fast)
Docker:         ██████████████              (good)
CloudInstance:  ██████████                  (network latency)
```

### Resource Usage

```
InMemory:       ████                        (minimal)
LocalProcess:   ████████                    (low)
CloudInstance:  ████████                    (low local, high remote)
Docker:         ████████████████            (high)
```

## Conclusion

Choose runtime type based on your needs:
- **InMemory**: Fast unit tests
- **LocalProcess**: Development and debugging
- **Docker**: Isolated integration tests
- **CloudInstance**: Mainnet fork testing

See [User Guide](./user-guide.md) for usage examples.

