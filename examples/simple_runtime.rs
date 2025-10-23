//! Simple blockchain runtime example

use anyhow::Result;
use async_trait::async_trait;
use blockchain_runtime::{
    BlockchainRuntime, EnvironmentState, ExecutionContext, ExecutionInputs, ExecutionResult,
    NetworkMode, RuntimeCapabilities, RuntimeConfig, RuntimeEnvironment, RuntimeEvent,
    RuntimeMetricDefinition, RuntimeType, StateChange, MetricType,
};
use std::collections::HashMap;
use std::path::Path;

// Mock runtime implementation for demonstration
struct MockRuntime {
    blockchain_id: String,
}

impl MockRuntime {
    fn new(blockchain_id: impl Into<String>) -> Self {
        Self {
            blockchain_id: blockchain_id.into(),
        }
    }
}

#[async_trait]
impl BlockchainRuntime for MockRuntime {
    fn blockchain_id(&self) -> &str {
        &self.blockchain_id
    }

    async fn create_environment(&self, config: RuntimeConfig) -> Result<RuntimeEnvironment> {
        println!("  Creating {} environment...", self.blockchain_id);
        println!("    Timeout: {}s", config.timeout_seconds);
        println!("    Memory limit: {}MB", config.memory_limit_mb);
        println!("    Network mode: {:?}", config.network_mode);

        Ok(RuntimeEnvironment {
            environment_id: "env-123".to_string(),
            blockchain_id: self.blockchain_id.clone(),
            runtime_type: RuntimeType::InMemory,
            endpoint_url: "http://localhost:8545".to_string(),
            state: EnvironmentState::Ready,
            metadata: HashMap::new(),
        })
    }

    async fn execute(
        &self,
        _env: &RuntimeEnvironment,
        code_path: &Path,
        inputs: &ExecutionInputs,
    ) -> Result<ExecutionResult> {
        println!("  Executing code from: {}", code_path.display());
        println!("    Function: {}", inputs.target_function);

        Ok(ExecutionResult {
            execution_id: "exec-456".to_string(),
            success: true,
            return_value: Some(serde_json::json!({"result": "success"})),
            error: None,
            metrics: HashMap::from([
                ("gas_used".to_string(), serde_json::json!(21000)),
                ("execution_time".to_string(), serde_json::json!(150)),
            ]),
            state_changes: vec![],
            events: vec![],
            execution_time_ms: 150,
        })
    }

    async fn deploy_contract(
        &self,
        _env: &RuntimeEnvironment,
        bytecode: &[u8],
        _constructor_args: &[u8],
    ) -> Result<String> {
        println!("  Deploying contract ({} bytes)", bytecode.len());
        Ok("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string())
    }

    async fn call_function(
        &self,
        _env: &RuntimeEnvironment,
        contract_address: &str,
        function: &str,
        _args: &[u8],
    ) -> Result<Vec<u8>> {
        println!("  Calling {}::{}", contract_address, function);
        Ok(vec![1, 2, 3, 4])
    }

    fn metrics_definition(&self) -> Vec<RuntimeMetricDefinition> {
        vec![
            RuntimeMetricDefinition {
                name: "gas_used".to_string(),
                description: "Gas consumed by execution".to_string(),
                unit: "gas".to_string(),
                metric_type: MetricType::Gas,
            },
            RuntimeMetricDefinition {
                name: "execution_time".to_string(),
                description: "Time taken to execute".to_string(),
                unit: "ms".to_string(),
                metric_type: MetricType::Time,
            },
        ]
    }

    async fn monitor(
        &self,
        _env: &RuntimeEnvironment,
        _execution_id: &str,
    ) -> Result<Vec<RuntimeEvent>> {
        Ok(vec![])
    }

    async fn destroy(&self, env: RuntimeEnvironment) -> Result<()> {
        println!("  Destroying environment: {}", env.environment_id);
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
            supports_time_travel: false,
            max_execution_time_seconds: 600,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Blockchain Runtime Example ===\n");

    // Example 1: Create runtime
    println!("1. Creating Blockchain Runtime");
    println!("------------------------------");

    let ethereum_runtime = MockRuntime::new("ethereum");
    println!("Created runtime for: {}", ethereum_runtime.blockchain_id());

    // Example 2: Check capabilities
    println!("\n2. Runtime Capabilities");
    println!("-----------------------");

    let caps = ethereum_runtime.capabilities();
    println!("Supports:");
    println!("  Contract deployment: {}", caps.supports_contract_deployment);
    println!("  Function calls: {}", caps.supports_function_calls);
    println!("  State inspection: {}", caps.supports_state_inspection);
    println!("  Gas estimation: {}", caps.supports_gas_estimation);
    println!("  Max execution time: {}s", caps.max_execution_time_seconds);

    // Example 3: Create environment
    println!("\n3. Creating Runtime Environment");
    println!("--------------------------------");

    let config = RuntimeConfig {
        timeout_seconds: 300,
        memory_limit_mb: 512,
        network_mode: NetworkMode::Local,
        enable_monitoring: true,
        blockchain_config: HashMap::new(),
    };

    let env = ethereum_runtime.create_environment(config).await?;
    println!("Environment created:");
    println!("  ID: {}", env.environment_id);
    println!("  Type: {:?}", env.runtime_type);
    println!("  Endpoint: {}", env.endpoint_url);
    println!("  State: {:?}", env.state);

    // Example 4: Deploy contract
    println!("\n4. Deploying Contract");
    println!("---------------------");

    let bytecode = b"608060405234801561001057600080fd5b50"; // Mock bytecode
    let constructor_args = b"";

    let contract_address = ethereum_runtime
        .deploy_contract(&env, bytecode, constructor_args)
        .await?;

    println!("Contract deployed at: {}", contract_address);

    // Example 5: Execute code
    println!("\n5. Executing Code");
    println!("-----------------");

    let inputs = ExecutionInputs {
        target_function: "transfer".to_string(),
        parameters: HashMap::from([
            ("to".to_string(), serde_json::json!("0x123...")),
            ("amount".to_string(), serde_json::json!(1000)),
        ]),
        context: ExecutionContext {
            sender: Some("0xabc...".to_string()),
            block_number: Some(12345),
            timestamp: Some(1634567890),
            extra: HashMap::new(),
        },
    };

    let code_path = Path::new("./contract.sol");
    let result = ethereum_runtime.execute(&env, code_path, &inputs).await?;

    println!("Execution result:");
    println!("  Success: {}", result.success);
    println!("  Execution time: {}ms", result.execution_time_ms);

    if let Some(gas) = result.metrics.get("gas_used") {
        println!("  Gas used: {}", gas);
    }

    // Example 6: Metrics
    println!("\n6. Available Metrics");
    println!("--------------------");

    for metric in ethereum_runtime.metrics_definition() {
        println!("  - {}: {} ({})", metric.name, metric.description, metric.unit);
    }

    // Example 7: Cleanup
    println!("\n7. Cleaning Up");
    println!("--------------");

    ethereum_runtime.destroy(env).await?;
    println!("Environment destroyed");

    println!("\n=== Example completed successfully ===");

    Ok(())
}

