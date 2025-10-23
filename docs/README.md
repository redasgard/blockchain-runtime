# Blockchain Runtime Documentation

Welcome to the Blockchain Runtime documentation. This library provides blockchain-agnostic runtime abstraction for dynamic analysis, testing, and simulation.

## Documentation Structure

- **[Architecture](./architecture.md)** - System design and runtime patterns
- **[Getting Started](./getting-started.md)** - Quick start guide and basic usage
- **[User Guide](./user-guide.md)** - Comprehensive usage patterns
- **[API Reference](./api-reference.md)** - Detailed API documentation
- **[Use Cases](./use-cases.md)** - Real-world applications
- **[Implementation Guide](./implementation-guide.md)** - Implementing custom runtimes
- **[Testing Guide](./testing.md)** - Testing blockchain code
- **[FAQ](./faq.md)** - Frequently asked questions

## Quick Links

- [Why Blockchain Runtime?](./why-blockchain-runtime.md)
- [Runtime Types](./runtime-types.md)
- [Network Modes](./network-modes.md)
- [Best Practices](./best-practices.md)

## Overview

Blockchain Runtime provides a unified interface for executing, testing, and analyzing smart contracts across different blockchains without blockchain-specific code.

### Key Features

- ✅ **Blockchain-Agnostic**: Unified interface for any blockchain
- ✅ **Dynamic Analysis**: Execute contracts in isolated environments
- ✅ **Testing**: Spin up test networks for contract testing
- ✅ **Simulation**: Simulate transactions and monitor behavior
- ✅ **Metrics**: Track gas, compute units, state changes
- ✅ **Event Monitoring**: Capture blockchain events and logs
- ✅ **Async-First**: Non-blocking operations

### Quick Example

```rust
use blockchain_runtime::{BlockchainRuntime, RuntimeConfig, NetworkMode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure runtime
    let config = RuntimeConfig {
        network_mode: NetworkMode::Local,
        enable_monitoring: true,
        ..Default::default()
    };
    
    // Create environment
    let env = runtime.create_environment(config).await?;
    
    // Deploy contract
    let address = runtime.deploy_contract(&env, bytecode, &args).await?;
    
    // Call function
    let result = runtime.call_function(&env, &address, "transfer", &call_args).await?;
    
    println!("Transaction successful!");
    Ok(())
}
```

## Runtime Types

- **Docker**: Containerized blockchain nodes
- **LocalProcess**: Native process (Ganache, Hardhat)
- **CloudInstance**: Cloud-hosted test networks
- **InMemory**: In-memory simulation

## Network Modes

- **Local**: Local test network (no real funds)
- **Testnet**: Public test network
- **MainnetFork**: Mainnet fork for realistic testing

## Use Cases

- Smart contract testing
- Gas estimation and optimization
- State inspection and debugging
- Security analysis and fuzzing
- Integration testing
- Performance benchmarking

## Support

- **GitHub**: https://github.com/redasgard/blockchain-runtime
- **Email**: hello@redasgard.com
- **Security Issues**: security@redasgard.com

## License

MIT License - See [LICENSE-MIT](../LICENSE-MIT)

