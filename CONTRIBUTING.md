# Contributing to Blockchain Runtime

Thank you for your interest in contributing to Blockchain Runtime! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Testing](#testing)
- [Security](#security)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- Understanding of blockchain technology and smart contracts
- Familiarity with async programming and network protocols
- Basic knowledge of blockchain APIs and Web3 protocols

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/blockchain-runtime.git
   cd blockchain-runtime
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/redasgard/blockchain-runtime.git
   ```

## How to Contribute

### Reporting Issues

Before creating an issue, please:

1. **Search existing issues** to avoid duplicates
2. **Check the documentation** in the `docs/` folder
3. **Verify the issue** with the latest version
4. **Test with minimal examples**

When creating an issue, include:

- **Clear description** of the problem
- **Steps to reproduce** with code examples
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, blockchain network)
- **Blockchain-specific details** (if related to specific blockchains)

### Suggesting Enhancements

For feature requests:

1. **Check existing issues** and roadmap
2. **Describe the use case** clearly
3. **Explain the blockchain benefit**
4. **Consider implementation complexity**
5. **Provide blockchain examples** if applicable

### Pull Requests

#### Before You Start

1. **Open an issue first** for significant changes
2. **Discuss the approach** with maintainers
3. **Ensure the change aligns** with project goals
4. **Consider blockchain compatibility** implications

#### PR Process

1. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our guidelines

3. **Test thoroughly**:
   ```bash
   cargo test
   cargo test --features tracing
   cargo clippy
   cargo fmt
   ```

4. **Update documentation** if needed

5. **Commit with clear messages**:
   ```bash
   git commit -m "Add support for new blockchain network"
   ```

6. **Push and create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

#### PR Requirements

- **All tests pass** (CI will check)
- **Code is formatted** (`cargo fmt`)
- **No clippy warnings** (`cargo clippy`)
- **Documentation updated** if needed
- **Clear commit messages**
- **PR description** explains the change
- **Blockchain compatibility** maintained

## Development Setup

### Project Structure

```
blockchain-runtime/
├── src/                 # Source code
│   ├── lib.rs          # Main library interface
│   ├── runtime.rs      # Runtime management
│   ├── config.rs       # Configuration
│   ├── security.rs     # Security features
│   └── types.rs        # Type definitions
├── tests/              # Integration tests
├── examples/           # Usage examples
└── docs/               # Documentation
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with tracing
cargo test --features tracing

# Run specific test
cargo test test_blockchain_runtime

# Run examples
cargo run --example simple_runtime
```

### Code Style

We follow standard Rust conventions:

- **Format code**: `cargo fmt`
- **Check linting**: `cargo clippy`
- **Use meaningful names**
- **Add documentation** for public APIs
- **Write tests** for new functionality
- **Consider async performance**

## Testing

### Test Categories

1. **Unit Tests**: Test individual functions
2. **Integration Tests**: Test complete workflows
3. **Blockchain Tests**: Test with real blockchain networks
4. **Mock Tests**: Test with mocked blockchain data
5. **Performance Tests**: Test async operations

### Adding Tests

When adding new functionality:

1. **Write unit tests** for each function
2. **Add integration tests** for workflows
3. **Test with real blockchains** (if applicable)
4. **Test error handling** and edge cases
5. **Test async operations**

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blockchain_runtime() {
        let runtime = BlockchainRuntime::new("ethereum").await?;
        
        // Test runtime initialization
        assert!(runtime.is_initialized());
    }

    #[tokio::test]
    async fn test_contract_deployment() {
        let runtime = BlockchainRuntime::new("ethereum").await?;
        
        // Test contract deployment
        let contract = runtime.deploy_contract(bytecode, constructor_args).await?;
        assert!(contract.address().is_some());
    }
}
```

## Security

### Security Considerations

Blockchain Runtime is a security-critical library. When contributing:

1. **Understand blockchain security** before making changes
2. **Test with real blockchain networks** (safely)
3. **Consider private key security**
4. **Review security implications** of changes
5. **Test with various blockchain types**

### Security Testing

```bash
# Run security tests
cargo test test_private_key_security
cargo test test_transaction_validation
cargo test test_contract_security

# Test with examples
cargo run --example simple_runtime
```

### Blockchain Security

When adding security features:

1. **Research blockchain security** best practices
2. **Understand private key management**
3. **Test with malicious inputs**
4. **Consider transaction security**
5. **Document security implications**

### Reporting Security Issues

**Do not open public issues for security vulnerabilities.**

Instead:
1. Email security@redasgard.com
2. Include detailed description
3. Include blockchain examples
4. Wait for response before disclosure

## Documentation

### Documentation Standards

- **Public APIs** must have doc comments
- **Examples** in doc comments should be runnable
- **Security implications** should be documented
- **Performance characteristics** should be noted
- **Blockchain concepts** should be explained

### Documentation Structure

```
docs/
├── README.md              # Main documentation
├── getting-started.md      # Quick start guide
├── api-reference.md       # Complete API docs
├── blockchain-guide.md    # Blockchain integration guide
├── best-practices.md      # Usage guidelines
└── faq.md                 # Frequently asked questions
```

### Writing Documentation

1. **Use clear, concise language**
2. **Include practical examples**
3. **Explain security implications**
4. **Document blockchain concepts**
5. **Link to related resources**
6. **Keep it up to date**

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking API changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

Before releasing:

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Security review completed
- [ ] Performance benchmarks updated
- [ ] Blockchain compatibility tested

### Release Steps

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md**
3. **Create release PR**
4. **Review and merge**
5. **Tag release** on GitHub
6. **Publish to crates.io**

## Areas for Contribution

### High Priority

- **New blockchain support**: Add support for additional blockchains
- **Performance improvements**: Optimize async operations and blockchain interactions
- **Security enhancements**: Better private key management and transaction security
- **Contract management**: Improve smart contract deployment and management

### Medium Priority

- **Configuration options**: More flexible blockchain configuration
- **Error handling**: Better error messages and recovery
- **Testing**: More comprehensive test coverage
- **Documentation**: Improve examples and guides

### Low Priority

- **CLI tools**: Command-line utilities for blockchain operations
- **Monitoring**: Blockchain monitoring and observability
- **Visualization**: Blockchain data visualization tools
- **Hot reloading**: Runtime blockchain configuration updates

## Blockchain Development

### Blockchain Categories

1. **Ethereum**: Smart contract platform
2. **Bitcoin**: Digital currency and payment system
3. **Solana**: High-performance blockchain
4. **Polkadot**: Multi-chain blockchain platform

### Blockchain Development Process

1. **Research**: Understand the blockchain and its API
2. **Implement**: Create blockchain integration
3. **Test**: Test with real blockchain networks
4. **Validate**: Ensure security and performance
5. **Document**: Document the blockchain and its capabilities
6. **Deploy**: Make the blockchain available

### Blockchain Testing

```rust
// Test new blockchain
#[tokio::test]
async fn test_new_blockchain() {
    let runtime = BlockchainRuntime::new("new_blockchain").await?;
    
    // Test blockchain initialization
    assert!(runtime.is_initialized());
    
    // Test basic operations
    let balance = runtime.get_balance("0x123...").await?;
    assert!(balance >= 0);
}
```

## Getting Help

### Resources

- **Documentation**: Check the `docs/` folder
- **Examples**: Look at `examples/` folder
- **Issues**: Search existing GitHub issues
- **Discussions**: Use GitHub Discussions for questions

### Contact

- **Email**: hello@redasgard.com
- **GitHub**: [@redasgard](https://github.com/redasgard)
- **Security**: security@redasgard.com

## Recognition

Contributors will be:

- **Listed in CONTRIBUTORS.md**
- **Mentioned in release notes** for significant contributions
- **Credited in documentation** for major features
- **Acknowledged** for blockchain development

Thank you for contributing to Blockchain Runtime! ⛓️🛡️
