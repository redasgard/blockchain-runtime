# Use Cases

Real-world applications and scenarios for Blockchain Runtime.

## 1. Smart Contract Testing Framework

### Scenario: Automated Contract Testing

Test smart contracts across multiple blockchains with a unified interface.

**Implementation:**

```rust
use blockchain_runtime::{BlockchainRuntime, RuntimeConfig, NetworkMode};
use std::path::Path;

struct ContractTester {
    runtime: Box<dyn BlockchainRuntime>,
}

impl ContractTester {
    async fn test_contract(&self, contract_path: &Path) -> anyhow::Result<TestReport> {
        // Create test environment
        let config = RuntimeConfig {
            network_mode: NetworkMode::Local,
            enable_monitoring: true,
            ..Default::default()
        };
        
        let env = self.runtime.create_environment(config).await?;
        
        // Read and deploy contract
        let bytecode = std::fs::read(contract_path.with_extension("bin"))?;
        let address = self.runtime.deploy_contract(&env, &bytecode, &[]).await?;
        
        // Run test cases
        let mut report = TestReport::new();
        
        // Test case 1: Basic functionality
        let result = self.runtime.call_function(&env, &address, "initialize", &[]).await;
        report.add_test("initialize", result.is_ok());
        
        // Test case 2: Transfer
        let result = self.runtime.call_function(&env, &address, "transfer", &[]).await;
        report.add_test("transfer", result.is_ok());
        
        // Clean up
        self.runtime.destroy(env).await?;
        
        Ok(report)
    }
}

struct TestReport {
    tests: Vec<(String, bool)>,
}

impl TestReport {
    fn new() -> Self {
        Self { tests: vec![] }
    }
    
    fn add_test(&mut self, name: &str, passed: bool) {
        self.tests.push((name.to_string(), passed));
    }
}
```

**Benefits:**
- Blockchain-agnostic tests
- Automated testing
- Isolated environments
- Easy cleanup

## 2. Gas Optimization Analysis

### Scenario: Optimize Contract Gas Usage

Analyze and optimize gas consumption across different implementations.

**Implementation:**

```rust
use blockchain_runtime::{BlockchainRuntime, ExecutionInputs, ExecutionContext};
use std::path::Path;

struct GasOptimizer {
    runtime: Box<dyn BlockchainRuntime>,
}

impl GasOptimizer {
    async fn compare_implementations(&self, implementations: Vec<&Path>) -> anyhow::Result<OptimizationReport> {
        let config = blockchain_runtime::RuntimeConfig::default();
        let env = self.runtime.create_environment(config).await?;
        
        let mut report = OptimizationReport::new();
        
        for impl_path in implementations {
            let inputs = ExecutionInputs {
                target_function: "test_function".to_string(),
                parameters: std::collections::HashMap::new(),
                context: ExecutionContext {
                    sender: None,
                    block_number: None,
                    timestamp: None,
                    extra: std::collections::HashMap::new(),
                },
            };
            
            let result = self.runtime.execute(&env, impl_path, &inputs).await?;
            
            if let Some(gas) = result.metrics.get("gas_used") {
                report.add_implementation(
                    impl_path.display().to_string(),
                    gas.as_u64().unwrap_or(0),
                );
            }
        }
        
        self.runtime.destroy(env).await?;
        
        Ok(report)
    }
}

struct OptimizationReport {
    implementations: Vec<(String, u64)>,
}

impl OptimizationReport {
    fn new() -> Self {
        Self { implementations: vec![] }
    }
    
    fn add_implementation(&mut self, name: String, gas: u64) {
        self.implementations.push((name, gas));
    }
    
    fn best_implementation(&self) -> Option<&(String, u64)> {
        self.implementations.iter().min_by_key(|(_, gas)| gas)
    }
}
```

**Benefits:**
- Quantitative comparison
- Optimization opportunities
- Cost reduction
- Performance insights

## 3. Security Fuzzing

### Scenario: Fuzz Test Smart Contracts

Discover vulnerabilities through automated fuzzing.

**Implementation:**

```rust
use blockchain_runtime::{BlockchainRuntime, ExecutionInputs, ExecutionContext};
use std::path::Path;

struct ContractFuzzer {
    runtime: Box<dyn BlockchainRuntime>,
}

impl ContractFuzzer {
    async fn fuzz_contract(&self, contract_path: &Path, iterations: usize) -> anyhow::Result<FuzzReport> {
        let config = blockchain_runtime::RuntimeConfig::default();
        let env = self.runtime.create_environment(config).await?;
        
        let mut report = FuzzReport::new();
        
        for i in 0..iterations {
            // Generate random inputs
            let random_inputs = self.generate_random_inputs();
            
            let inputs = ExecutionInputs {
                target_function: "fuzz_target".to_string(),
                parameters: random_inputs,
                context: ExecutionContext {
                    sender: None,
                    block_number: None,
                    timestamp: None,
                    extra: std::collections::HashMap::new(),
                },
            };
            
            // Execute
            let result = self.runtime.execute(&env, contract_path, &inputs).await?;
            
            // Analyze result
            if !result.success {
                report.add_crash(i, result.error.clone());
            }
            
            // Check for suspicious patterns
            if self.is_suspicious(&result) {
                report.add_suspicious(i, result.clone());
            }
        }
        
        self.runtime.destroy(env).await?;
        
        Ok(report)
    }
    
    fn generate_random_inputs(&self) -> std::collections::HashMap<String, serde_json::Value> {
        // Generate random test inputs
        std::collections::HashMap::new()
    }
    
    fn is_suspicious(&self, result: &blockchain_runtime::ExecutionResult) -> bool {
        // Check for suspicious patterns (e.g., unexpected state changes)
        result.state_changes.len() > 100
    }
}

struct FuzzReport {
    crashes: Vec<(usize, Option<String>)>,
    suspicious: Vec<(usize, blockchain_runtime::ExecutionResult)>,
}

impl FuzzReport {
    fn new() -> Self {
        Self { crashes: vec![], suspicious: vec![] }
    }
    
    fn add_crash(&mut self, iteration: usize, error: Option<String>) {
        self.crashes.push((iteration, error));
    }
    
    fn add_suspicious(&mut self, iteration: usize, result: blockchain_runtime::ExecutionResult) {
        self.suspicious.push((iteration, result));
    }
}
```

**Benefits:**
- Automated vulnerability discovery
- Edge case detection
- Crash analysis
- Security hardening

## 4. Multi-Chain Integration Testing

### Scenario: Test Cross-Chain Functionality

Test contracts that interact across multiple blockchains.

**Implementation:**

```rust
use blockchain_runtime::{BlockchainRuntime, RuntimeConfig};
use std::collections::HashMap;

struct MultiChainTester {
    ethereum_runtime: Box<dyn BlockchainRuntime>,
    solana_runtime: Box<dyn BlockchainRuntime>,
}

impl MultiChainTester {
    async fn test_bridge_functionality(&self) -> anyhow::Result<()> {
        let config = RuntimeConfig::default();
        
        // Create environments on both chains
        let eth_env = self.ethereum_runtime.create_environment(config.clone()).await?;
        let sol_env = self.solana_runtime.create_environment(config).await?;
        
        // Deploy bridge contracts on both sides
        let eth_bridge = self.ethereum_runtime
            .deploy_contract(&eth_env, &eth_bytecode(), &[])
            .await?;
        
        let sol_bridge = self.solana_runtime
            .deploy_contract(&sol_env, &sol_bytecode(), &[])
            .await?;
        
        // Test lock on Ethereum
        let lock_result = self.ethereum_runtime
            .call_function(&eth_env, &eth_bridge, "lock", &encode_args(100))
            .await?;
        
        // Verify corresponding mint on Solana
        let mint_result = self.solana_runtime
            .call_function(&sol_env, &sol_bridge, "mint", &encode_args(100))
            .await?;
        
        // Clean up
        self.ethereum_runtime.destroy(eth_env).await?;
        self.solana_runtime.destroy(sol_env).await?;
        
        Ok(())
    }
}

fn eth_bytecode() -> Vec<u8> { vec![] }
fn sol_bytecode() -> Vec<u8> { vec![] }
fn encode_args(_amount: u64) -> Vec<u8> { vec![] }
```

**Benefits:**
- Cross-chain testing
- Bridge validation
- Interoperability verification
- Unified test framework

## 5. Performance Benchmarking

### Scenario: Benchmark Contract Performance

Compare performance across different blockchain platforms.

**Implementation:**

```rust
use blockchain_runtime::{BlockchainRuntime, ExecutionInputs, ExecutionContext};
use std::path::Path;
use std::time::Instant;

struct PerformanceBenchmark {
    runtimes: HashMap<String, Box<dyn BlockchainRuntime>>,
}

impl PerformanceBenchmark {
    async fn benchmark_contract(&self, contract_path: &Path) -> anyhow::Result<BenchmarkResults> {
        let mut results = BenchmarkResults::new();
        
        for (chain_name, runtime) in &self.runtimes {
            let config = blockchain_runtime::RuntimeConfig::default();
            let env = runtime.create_environment(config).await?;
            
            let inputs = ExecutionInputs {
                target_function: "benchmark_function".to_string(),
                parameters: HashMap::new(),
                context: ExecutionContext {
                    sender: None,
                    block_number: None,
                    timestamp: None,
                    extra: HashMap::new(),
                },
            };
            
            // Execute multiple times
            let iterations = 100;
            let start = Instant::now();
            
            for _ in 0..iterations {
                runtime.execute(&env, contract_path, &inputs).await?;
            }
            
            let avg_time = start.elapsed().as_millis() / iterations as u128;
            
            results.add_result(chain_name.clone(), avg_time as u64);
            
            runtime.destroy(env).await?;
        }
        
        Ok(results)
    }
}

struct BenchmarkResults {
    results: HashMap<String, u64>,
}

impl BenchmarkResults {
    fn new() -> Self {
        Self { results: HashMap::new() }
    }
    
    fn add_result(&mut self, chain: String, avg_ms: u64) {
        self.results.insert(chain, avg_ms);
    }
}
```

**Benefits:**
- Performance comparison
- Blockchain selection guidance
- Optimization targets
- Cost analysis

## 6. CI/CD Integration

### Scenario: Automated Contract Testing in CI

Run contract tests as part of continuous integration.

**Implementation:**

```rust
use blockchain_runtime::{BlockchainRuntime, RuntimeConfig, NetworkMode};

async fn ci_test_suite(runtime: &dyn BlockchainRuntime) -> anyhow::Result<()> {
    // Use Local mode for fast, deterministic tests
    let config = RuntimeConfig {
        network_mode: NetworkMode::Local,
        timeout_seconds: 60,  // Shorter for CI
        memory_limit_mb: 512, // Lower for CI
        ..Default::default()
    };
    
    let env = runtime.create_environment(config).await?;
    
    // Run all tests
    run_deployment_tests(&env, runtime).await?;
    run_functional_tests(&env, runtime).await?;
    run_security_tests(&env, runtime).await?;
    
    // Clean up
    runtime.destroy(env).await?;
    
    Ok(())
}

async fn run_deployment_tests(env: &blockchain_runtime::RuntimeEnvironment, runtime: &dyn BlockchainRuntime) -> anyhow::Result<()> {
    // Deployment tests
    Ok(())
}

async fn run_functional_tests(env: &blockchain_runtime::RuntimeEnvironment, runtime: &dyn BlockchainRuntime) -> anyhow::Result<()> {
    // Functional tests
    Ok(())
}

async fn run_security_tests(env: &blockchain_runtime::RuntimeEnvironment, runtime: &dyn BlockchainRuntime) -> anyhow::Result<()> {
    // Security tests
    Ok(())
}
```

**Benefits:**
- Automated quality checks
- Regression prevention
- Fast feedback
- Reproducible tests

## Summary

Blockchain Runtime is ideal for:

✅ **Smart Contract Testing** - Automated test suites
✅ **Gas Optimization** - Performance analysis
✅ **Security Fuzzing** - Vulnerability discovery
✅ **Multi-Chain Development** - Cross-chain testing
✅ **Performance Benchmarking** - Platform comparison
✅ **CI/CD Integration** - Automated testing

Choose Blockchain Runtime when you need:
- Blockchain-agnostic code
- Automated testing
- Dynamic analysis
- Isolated environments
- Metrics collection
- Event monitoring

## Next Steps

- Review [Architecture](./architecture.md) for system design
- Check [Getting Started](./getting-started.md) for quick start
- See [Implementation Guide](./implementation-guide.md) for creating custom runtimes
- Read [Testing Guide](./testing.md) for testing strategies

