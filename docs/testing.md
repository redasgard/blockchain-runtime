# Testing Guide

Guide for testing blockchain code using Blockchain Runtime.

## Testing Strategies

### Unit Testing

Test individual contract functions in isolation:

```rust
#[tokio::test]
async fn test_token_transfer() {
    let runtime = get_test_runtime();
    let config = RuntimeConfig::default();
    let env = runtime.create_environment(config).await.unwrap();
    
    // Deploy token contract
    let bytecode = compile_token_contract();
    let address = runtime.deploy_contract(&env, &bytecode, &[]).await.unwrap();
    
    // Test transfer
    let args = encode_args(&["recipient", "100"]);
    let result = runtime.call_function(&env, &address, "transfer", &args).await.unwrap();
    
    assert!(decode_bool(&result));
    
    runtime.destroy(env).await.unwrap();
}
```

### Integration Testing

Test contract interactions:

```rust
#[tokio::test]
async fn test_defi_protocol() {
    let runtime = get_test_runtime();
    let env = runtime.create_environment(RuntimeConfig::default()).await.unwrap();
    
    // Deploy multiple contracts
    let token = deploy_token(&runtime, &env).await.unwrap();
    let pool = deploy_pool(&runtime, &env).await.unwrap();
    let router = deploy_router(&runtime, &env).await.unwrap();
    
    // Test workflow
    approve_token(&runtime, &env, &token, &pool).await.unwrap();
    add_liquidity(&runtime, &env, &router, &pool).await.unwrap();
    swap_tokens(&runtime, &env, &router).await.unwrap();
    
    runtime.destroy(env).await.unwrap();
}
```

### Property-Based Testing

Use proptest for randomized testing:

```rust
use proptest::prelude::*;

proptest! {
    #[tokio::test]
    async fn test_transfer_properties(
        amount in 0u64..1000000,
        recipient in "[a-f0-9]{40}",
    ) {
        let runtime = get_test_runtime();
        let env = runtime.create_environment(RuntimeConfig::default()).await?;
        
        let address = deploy_contract(&runtime, &env).await?;
        let args = encode_transfer(recipient, amount)?;
        
        let result = runtime.call_function(&env, &address, "transfer", &args).await;
        
        // Properties that should always hold
        prop_assert!(result.is_ok() || is_expected_error(&result));
        
        runtime.destroy(env).await?;
    }
}
```

## Test Organization

### Test Fixtures

```rust
struct TestFixture {
    runtime: Box<dyn BlockchainRuntime>,
    env: RuntimeEnvironment,
}

impl TestFixture {
    async fn new() -> Result<Self> {
        let runtime = get_test_runtime();
        let env = runtime.create_environment(RuntimeConfig::default()).await?;
        
        Ok(Self { runtime, env })
    }
    
    async fn deploy_contract(&self, bytecode: &[u8]) -> Result<String> {
        self.runtime.deploy_contract(&self.env, bytecode, &[]).await
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        // Clean up in background (can't use async in Drop)
        // Consider using tokio::spawn or similar
    }
}
```

### Test Helpers

```rust
mod test_helpers {
    use super::*;
    
    pub async fn deploy_and_init(runtime: &dyn BlockchainRuntime, env: &RuntimeEnvironment) -> Result<String> {
        let bytecode = compile_test_contract();
        let address = runtime.deploy_contract(env, &bytecode, &[]).await?;
        
        // Initialize
        let init_args = encode_args(&["init_value"]);
        runtime.call_function(env, &address, "initialize", &init_args).await?;
        
        Ok(address)
    }
    
    pub fn encode_args(args: &[&str]) -> Vec<u8> {
        // Your encoding logic
        vec![]
    }
}
```

## Mock Runtime for Testing

Create a mock runtime for fast testing:

```rust
use blockchain_runtime::*;
use async_trait::async_trait;

pub struct MockRuntime {
    deployed_contracts: std::sync::Arc<std::sync::Mutex<HashMap<String, Vec<u8>>>>,
}

impl MockRuntime {
    pub fn new() -> Self {
        Self {
            deployed_contracts: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl BlockchainRuntime for MockRuntime {
    fn blockchain_id(&self) -> &str {
        "mock"
    }
    
    async fn create_environment(&self, _config: RuntimeConfig) -> anyhow::Result<RuntimeEnvironment> {
        Ok(RuntimeEnvironment {
            environment_id: "mock-env".to_string(),
            blockchain_id: "mock".to_string(),
            runtime_type: RuntimeType::InMemory,
            endpoint_url: "mock://localhost".to_string(),
            state: EnvironmentState::Ready,
            metadata: HashMap::new(),
        })
    }
    
    async fn deploy_contract(
        &self,
        _env: &RuntimeEnvironment,
        bytecode: &[u8],
        _constructor_args: &[u8],
    ) -> anyhow::Result<String> {
        let address = format!("0x{:040x}", rand::random::<u128>());
        self.deployed_contracts.lock().unwrap().insert(address.clone(), bytecode.to_vec());
        Ok(address)
    }
    
    // ... implement other methods with mocked behavior
    
    async fn destroy(&self, _env: RuntimeEnvironment) -> anyhow::Result<()> {
        Ok(())
    }
    
    async fn is_available(&self) -> bool {
        true
    }
    
    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            supports_contract_deployment: true,
            supports_function_calls: true,
            supports_state_inspection: true,
            supports_event_monitoring: true,
            supports_gas_estimation: true,
            supports_time_travel: true,
            max_execution_time_seconds: 60,
        }
    }
}
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Contract Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test
        
      - name: Run integration tests
        run: cargo test --test integration_tests
```

### Docker Compose for Tests

```yaml
version: '3.8'

services:
  ethereum-node:
    image: ethereum/client-go:latest
    ports:
      - "8545:8545"
    command: --dev --http --http.api eth,web3,net
  
  test-runner:
    build: .
    depends_on:
      - ethereum-node
    command: cargo test
```

## Performance Testing

### Benchmark Suite

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_deployment(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("deploy_contract", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let blockchain_runtime = get_test_runtime();
                let env = blockchain_runtime.create_environment(RuntimeConfig::default()).await.unwrap();
                
                black_box(blockchain_runtime.deploy_contract(&env, &bytecode, &[]).await.unwrap());
                
                blockchain_runtime.destroy(env).await.unwrap();
            })
        })
    });
}

criterion_group!(benches, benchmark_deployment);
criterion_main!(benches);
```

## Fuzzing

### Contract Fuzzing

```rust
#[tokio::test]
async fn fuzz_contract() {
    let runtime = get_test_runtime();
    let env = runtime.create_environment(RuntimeConfig::default()).await.unwrap();
    let address = deploy_contract(&runtime, &env).await.unwrap();
    
    for _ in 0..1000 {
        let random_args = generate_random_args();
        
        let result = runtime.call_function(&env, &address, "fuzz_target", &random_args).await;
        
        // Should not panic or crash
        match result {
            Ok(_) => { /* expected */ }
            Err(e) => {
                // Check error is handled gracefully
                assert!(!e.to_string().contains("panic"));
            }
        }
    }
    
    runtime.destroy(env).await.unwrap();
}
```

## Coverage Analysis

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Run tests with coverage
cargo llvm-cov --html

# Open coverage report
open target/llvm-cov/html/index.html
```

## Snapshot Testing

```rust
#[tokio::test]
async fn test_contract_state_snapshot() {
    let runtime = get_test_runtime();
    let env = runtime.create_environment(RuntimeConfig::default()).await.unwrap();
    
    let result = runtime.execute(&env, Path::new("contract.sol"), &inputs).await.unwrap();
    
    // Compare state changes to snapshot
    insta::assert_json_snapshot!(result.state_changes);
    
    runtime.destroy(env).await.unwrap();
}
```

## Best Practices

### 1. Parallel Test Execution

```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_parallel_deployments() {
    let runtime = get_test_runtime();
    
    let tasks: Vec<_> = (0..10)
        .map(|i| {
            let runtime = runtime.clone();
            tokio::spawn(async move {
                let env = runtime.create_environment(RuntimeConfig::default()).await?;
                let address = runtime.deploy_contract(&env, &bytecode, &[]).await?;
                runtime.destroy(env).await?;
                anyhow::Ok(address)
            })
        })
        .collect();
    
    for task in tasks {
        task.await.unwrap().unwrap();
    }
}
```

### 2. Test Data Management

```rust
mod test_data {
    pub fn sample_token_bytecode() -> Vec<u8> {
        include_bytes!("../test_data/Token.bin").to_vec()
    }
    
    pub fn sample_nft_bytecode() -> Vec<u8> {
        include_bytes!("../test_data/NFT.bin").to_vec()
    }
}
```

### 3. Assertion Helpers

```rust
fn assert_successful_execution(result: &ExecutionResult) {
    assert!(result.success, "Execution failed: {:?}", result.error);
    assert!(result.error.is_none());
}

fn assert_gas_under(result: &ExecutionResult, max_gas: u64) {
    if let Some(gas) = result.metrics.get("gas_used") {
        let gas_used = gas.as_u64().unwrap_or(u64::MAX);
        assert!(gas_used < max_gas, "Gas {} exceeds limit {}", gas_used, max_gas);
    }
}
```

## Debugging Tests

### Enable Logging

```rust
#[tokio::test]
async fn test_with_logging() {
    tracing_subscriber::fmt::init();
    
    let runtime = get_test_runtime();
    // Test with full logging
}
```

### Inspect State

```rust
#[tokio::test]
async fn test_with_state_inspection() {
    let result = runtime.execute(&env, code, &inputs).await.unwrap();
    
    // Print all state changes
    println!("State changes:");
    for change in &result.state_changes {
        println!("  {:?}", change);
    }
    
    // Print all events
    println!("Events:");
    for event in &result.events {
        println!("  {:?}", event);
    }
}
```

## Conclusion

Effective testing with Blockchain Runtime:
- Use appropriate test types (unit, integration, property-based)
- Leverage mock runtime for fast tests
- Integrate with CI/CD
- Monitor performance
- Achieve high coverage

See [User Guide](./user-guide.md) for more usage patterns.

