# Blockchain Runtime

[![Crates.io](https://img.shields.io/crates/v/blockchain-runtime.svg)](https://crates.io/crates/blockchain-runtime)
[![Documentation](https://docs.rs/blockchain-runtime/badge.svg)](https://docs.rs/blockchain-runtime)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

Blockchain-agnostic runtime abstraction for dynamic analysis, testing, and simulation.

## Features

- **Blockchain-Agnostic**: Unified interface for any blockchain
- **Dynamic Analysis**: Execute smart contracts in isolated environments
- **Testing**: Spin up test networks for contract testing
- **Simulation**: Simulate transactions and monitor behavior
- **Metrics**: Track gas, compute units, state changes
- **Event Monitoring**: Capture blockchain events and logs
- **Async-First**: Non-blocking operations

## Installation

```toml
[dependencies]
blockchain-runtime = "0.1"

# With tracing support
blockchain-runtime = { version = "0.1", features = ["tracing"] }
```

## Quick Start

```rust
use blockchain_runtime::{
    BlockchainRuntime, RuntimeConfig, RuntimeEnvironment,
    ExecutionInputs, ExecutionContext, NetworkMode
};
use std::collections::HashMap;

#[async_trait::async_trait]
impl BlockchainRuntime for MyBlockchainRuntime {
    fn blockchain_id(&self) -> &str {
        "ethereum"
    }

    async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment> {
        // Create a local test network
        Ok(RuntimeEnvironment {
            environment_id: "env-1".to_string(),
            blockchain_id: "ethereum".to_string(),
            runtime_type: RuntimeType::Docker,
            endpoint_url: "http://localhost:8545".to_string(),
            state: EnvironmentState::Ready,
            metadata: HashMap::new(),
        })
    }

    async fn deploy_contract(&self, env: &RuntimeEnvironment, bytecode: &[u8], args: &[u8]) -> Result<String> {
        // Deploy contract and return address
        Ok("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string())
    }

    // ... implement other methods
}
```

## Use Cases

### Smart Contract Testing

```rust
// Create test environment
let config = RuntimeConfig {
    network_mode: NetworkMode::Local,
    enable_monitoring: true,
    ..Default::default()
};

let env = runtime.create_environment(config).await?;

// Deploy contract
let address = runtime.deploy_contract(&env, bytecode, &constructor_args).await?;

// Call contract function
let result = runtime.call_function(&env, &address, "transfer", &args).await?;

// Check results
assert!(result.success);
```

### Gas Estimation

```rust
let result = runtime.execute(&env, code_path, &inputs).await?;

if let Some(gas) = result.metrics.get("gas_used") {
    println!("Gas used: {}", gas);
}
```

### State Inspection

```rust
let result = runtime.execute(&env, code_path, &inputs).await?;

for change in result.state_changes {
    println!("State change: {} -> {:?}", change.key, change.new_value);
}
```

### Event Monitoring

```rust
let events = runtime.monitor(&env, &execution_id).await?;

for event in events {
    println!("Event: {} at {}", event.event_type, event.timestamp);
}
```

## Runtime Types

- **Docker**: Containerized blockchain nodes
- **LocalProcess**: Native process (e.g., Ganache, Hardhat)
- **CloudInstance**: Cloud-hosted test networks
- **InMemory**: In-memory simulation

## Network Modes

- **Local**: Local test network (no real funds)
- **Testnet**: Public test network
- **MainnetFork**: Mainnet fork for realistic testing

## Supported Capabilities

```rust
let caps = runtime.capabilities();

if caps.supports_contract_deployment {
    // Can deploy contracts
}

if caps.supports_gas_estimation {
    // Can estimate gas costs
}

if caps.supports_time_travel {
    // Can manipulate block time/number
}
```

## Configuration

```rust
use blockchain_runtime::RuntimeConfig;
use std::collections::HashMap;

let mut config = RuntimeConfig::default();

// Set limits
config.timeout_seconds = 600;
config.memory_limit_mb = 2048;

// Network mode
config.network_mode = NetworkMode::Testnet;

// Blockchain-specific config
config.blockchain_config.insert(
    "chain_id".to_string(),
    serde_json::json!(1337)
);
```

## Metrics

Track blockchain-specific metrics:

```rust
let metrics = runtime.metrics_definition();

for metric in metrics {
    println!("{}: {} ({})", metric.name, metric.description, metric.unit);
}

// Common metrics:
// - gas_used: Gas consumed (units)
// - compute_units: Compute units (Solana)
// - storage_delta: Storage change (bytes)
// - execution_time: Time taken (ms)
```

## Origin

Extracted from [Valkra](https://github.com/asgardtech/valkra), a blockchain security auditing platform where it provides runtime execution environments for dynamic smart contract analysis across 40+ blockchains.

## License

Licensed under MIT License. See [LICENSE-MIT](LICENSE-MIT) for details.

## Contributing

Contributions welcome! Areas of interest:
- Additional blockchain implementations
- Performance optimizations
- Testing utilities
- Documentation improvements

## Contact

- **Author**: Red Asgard
- **Email**: hello@redasgard.com
- **GitHub**: https://github.com/redasgard

