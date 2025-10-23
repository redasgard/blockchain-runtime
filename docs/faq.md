# FAQ

## General Questions

### What is Blockchain Runtime?

A blockchain-agnostic abstraction layer for testing, analyzing, and simulating smart contracts across different blockchain platforms with a unified interface.

### Why do I need this?

If you're building tools that work with multiple blockchains (security auditing, testing frameworks, analysis tools), Blockchain Runtime lets you write code once instead of implementing blockchain-specific logic 40+ times.

### Is this a blockchain implementation?

No. It's an abstraction layer over existing blockchain nodes/clients. You still need blockchain-specific runtimes (Ethereum, Solana, etc.) that implement the trait.

### Does this run on mainnet?

The library supports MainnetFork mode for testing against mainnet state, but it's designed for testing and analysis, not production transaction submission.

## Technical Questions

### How do I implement support for a new blockchain?

Implement the `BlockchainRuntime` trait. See [Implementation Guide](./implementation-guide.md) for step-by-step instructions.

### What runtime type should I use?

- **Development**: LocalProcess (fast startup)
- **CI/CD**: Docker (isolation)
- **Integration tests**: Testnet (realistic)
- **Mainnet testing**: MainnetFork (real state)
- **Unit tests**: InMemory (fastest)

See [Runtime Types](./runtime-types.md) for detailed comparison.

### How do I choose network mode?

- **Local**: Development and unit tests (free, fast, offline)
- **Testnet**: Integration tests (realistic, free tokens)
- **MainnetFork**: Testing against real contracts (realistic, free)

See [Network Modes](./network-modes.md) for details.

### Is this async-only?

Yes. All operations are async because blockchain interactions are inherently I/O-bound. Use `tokio` or another async runtime.

### Can I use this in sync code?

Yes, but you need to bridge to async:

```rust
// In sync context
let result = tokio::runtime::Runtime::new()?
    .block_on(runtime.create_environment(config))?;
```

### How do I handle errors?

All methods return `anyhow::Result<T>`. Use `?` operator:

```rust
let env = runtime.create_environment(config).await?;
let address = runtime.deploy_contract(&env, &bytecode, &[]).await?;
```

## Implementation Questions

### Do I need to implement all trait methods?

Yes, all methods are required. However, some can return simple defaults if not applicable to your blockchain.

### What should metrics_definition() return?

Return all metrics your runtime can provide. Common ones:
- `gas_used` (Ethereum, Polygon, etc.)
- `compute_units` (Solana)
- `storage_delta` (all)
- `execution_time` (all)

### How do I handle blockchain-specific features?

Use `blockchain_config` HashMap for blockchain-specific settings:

```rust
config.blockchain_config.insert(
    "solana_commitment".to_string(),
    serde_json::json!("confirmed")
);
```

### Should I use Docker or LocalProcess?

**Docker if:**
- Need strong isolation
- Running in CI/CD
- Want reproducibility
- Testing untrusted code

**LocalProcess if:**
- Local development
- Need fast startup
- Debugging
- Don't have Docker

## Performance Questions

### How fast is environment creation?

Depends on runtime type:
- Docker: 2-5s
- LocalProcess: 1-2s
- CloudInstance: <1s
- InMemory: <100ms

### Can I run tests in parallel?

Yes! Each test can have its own environment:

```rust
#[tokio::test]
async fn test_parallel() {
    // Each test creates its own environment
    let env = runtime.create_environment(config).await?;
    // ... test ...
    runtime.destroy(env).await?;
}
```

### How do I improve test performance?

1. Use InMemory runtime for unit tests
2. Reuse environments when possible
3. Run tests in parallel
4. Cache compiled contracts
5. Use Local network mode

## Troubleshooting

### Environment creation fails

**Check:**
- Runtime is available: `runtime.is_available().await`
- Docker is running (if using Docker)
- Ports are not in use
- Sufficient memory/disk space

### Deployment fails

**Check:**
- Bytecode is valid for the blockchain
- Environment is in Ready state
- Account has sufficient funds (testnet)
- Constructor arguments are correct

### Execution timeouts

**Solutions:**
- Increase timeout in config
- Optimize contract code
- Check for infinite loops
- Use faster network mode

### State changes not captured

**Check:**
- Monitoring is enabled in config
- Runtime supports state inspection
- Execution completed successfully

### Resource leaks (Docker containers not stopping)

**Solutions:**
- Always call `destroy()`
- Use helper functions that guarantee cleanup
- Implement Drop for test fixtures
- Check Docker manually: `docker ps`

## Use Case Questions

### Can I test cross-chain bridges?

Yes! Create environments for multiple chains:

```rust
let eth_env = eth_runtime.create_environment(config).await?;
let sol_env = sol_runtime.create_environment(config).await?;

// Test bridge functionality between chains
```

### Can I simulate MEV attacks?

Yes with MainnetFork mode:

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::MainnetFork,
    ..Default::default()
};

// Simulate frontrunning, sandwiches, etc.
```

### Can I test gas optimization?

Yes, compare metrics across implementations:

```rust
let result1 = runtime.execute(&env, impl1, &inputs).await?;
let result2 = runtime.execute(&env, impl2, &inputs).await?;

let gas1 = result1.metrics.get("gas_used").unwrap();
let gas2 = result2.metrics.get("gas_used").unwrap();

assert!(gas2 < gas1, "Optimization failed");
```

## Integration Questions

### Does this work with Hardhat?

Yes, as LocalProcess runtime:

```rust
// Start Hardhat node
let env = runtime.create_environment(config).await?;
// env.endpoint_url = "http://localhost:8545"
```

### Does this work with Foundry/Anvil?

Yes, as LocalProcess or Docker runtime.

### Does this work with Ganache?

Yes, as LocalProcess runtime.

### Does this work with Solana Test Validator?

Yes, implement SolanaRuntime trait with test validator.

## Contributing Questions

### How do I add support for a new blockchain?

1. Implement `BlockchainRuntime` trait
2. Add tests
3. Document blockchain-specific behavior
4. Submit PR

See [Implementation Guide](./implementation-guide.md).

### How do I report bugs?

- GitHub: https://github.com/redasgard/blockchain-runtime/issues
- Email: hello@redasgard.com
- Security: security@redasgard.com (private)

### What's the license?

MIT License. Free to use commercially.

## Advanced Questions

### Can I extend the trait?

The trait is sealed for stability. For blockchain-specific features, use the `blockchain_config` HashMap or implement additional traits.

### How do I handle chain-specific metrics?

Use `MetricType::Custom(String)`:

```rust
RuntimeMetricDefinition {
    name: "solana_rent".to_string(),
    description: "Rent cost in lamports".to_string(),
    unit: "lamports".to_string(),
    metric_type: MetricType::Custom("solana_specific".to_string()),
}
```

### Can I use this for production transactions?

Not recommended. This is designed for testing and analysis. Use official blockchain clients for production.

## Next Steps

- Read [Getting Started](./getting-started.md) for quick start
- Check [User Guide](./user-guide.md) for comprehensive usage
- See [Implementation Guide](./implementation-guide.md) to implement custom runtimes
- Review [Best Practices](./best-practices.md) for optimal usage

## Still Have Questions?

- Email: hello@redasgard.com
- GitHub Issues: https://github.com/redasgard/blockchain-runtime/issues
- Documentation: `/docs/` directory

