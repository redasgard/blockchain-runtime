# Network Modes

Detailed explanation of network modes and when to use each.

## Overview

Blockchain Runtime supports three network modes for different testing and development scenarios:

```rust
pub enum NetworkMode {
    Local,          // Local test network
    Testnet,        // Public testnet
    MainnetFork,    // Mainnet fork at specific block
}
```

## Local Mode

### Description

Creates a local blockchain network with full control and no external dependencies.

### Characteristics

- **Chain ID**: Typically 1337 or custom
- **Accounts**: Pre-funded accounts (free ETH/SOL)
- **Block Time**: Instant or configurable
- **State**: Fresh state on each run
- **Network**: No internet required
- **Cost**: Free (no gas fees)

### Architecture

```
Your Application
    │
    └── Local Blockchain Node
        ├── Genesis Block
        ├── Pre-funded Accounts
        │   ├── Account 0: 1000 ETH
        │   ├── Account 1: 1000 ETH
        │   └── Account N: 1000 ETH
        └── Instant Mining
```

### Configuration

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::Local,
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("chain_id".to_string(), serde_json::json!(1337));
        map.insert("block_time".to_string(), serde_json::json!(0)); // Instant
        map.insert("accounts".to_string(), serde_json::json!(10));
        map.insert("balance".to_string(), serde_json::json!("1000000000000000000000")); // 1000 ETH
        map
    },
    ..Default::default()
};
```

### Best For

- ✅ Unit testing
- ✅ Development
- ✅ Fast iteration
- ✅ Deterministic tests
- ✅ Offline development

### Example

```rust
#[tokio::test]
async fn test_local_deployment() {
    let config = RuntimeConfig {
        network_mode: NetworkMode::Local,
        ..Default::default()
    };
    
    let env = runtime.create_environment(config).await?;
    
    // Deploy without real funds
    let address = runtime.deploy_contract(&env, &bytecode, &[]).await?;
    
    // Test for free
    let result = runtime.call_function(&env, &address, "test", &[]).await?;
    
    runtime.destroy(env).await?;
}
```

---

## Testnet Mode

### Description

Connects to a public testnet with free test tokens from faucets.

### Characteristics

- **Chain ID**: Official testnet ID (e.g., 5 for Goerli)
- **Accounts**: Need to fund via faucets
- **Block Time**: Real network block time (12s for Ethereum)
- **State**: Shared with other testnet users
- **Network**: Internet required
- **Cost**: Free (testnet tokens)
- **Persistence**: State persists across sessions

### Architecture

```
Your Application
    │
    │ HTTPS RPC
    ▼
Public Testnet (Goerli, Sepolia, Devnet)
    ├── Real Network Consensus
    ├── Other Users' Transactions
    ├── Public Block Explorer
    └── Real Network Latency
```

### Configuration

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::Testnet,
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("network".to_string(), serde_json::json!("goerli"));
        map.insert("rpc_url".to_string(), 
            serde_json::json!("https://goerli.infura.io/v3/YOUR_KEY"));
        map
    },
    ..Default::default()
};
```

### Popular Testnets

**Ethereum:**
- Sepolia (recommended)
- Goerli (deprecated but still used)
- Holesky

**Solana:**
- Devnet
- Testnet

**Polygon:**
- Mumbai

**Avalanche:**
- Fuji

### Best For

- ✅ Integration testing
- ✅ Staging environment
- ✅ Public testing
- ✅ Testing real network conditions
- ✅ Bug reproduction

### Example

```rust
#[tokio::test]
#[ignore] // Ignore by default (requires network)
async fn test_testnet_deployment() {
    let config = RuntimeConfig {
        network_mode: NetworkMode::Testnet,
        blockchain_config: {
            let mut map = HashMap::new();
            map.insert("network".to_string(), serde_json::json!("goerli"));
            map
        },
        ..Default::default()
    };
    
    let env = runtime.create_environment(config).await?;
    
    // Deploy to testnet (requires funded account)
    let address = runtime.deploy_contract(&env, &bytecode, &[]).await?;
    
    println!("Deployed to testnet: {}", address);
    // Contract persists on testnet
}
```

### Getting Testnet Tokens

**Ethereum (Sepolia):**
- https://sepoliafaucet.com/
- https://faucet.quicknode.com/

**Solana (Devnet):**
```bash
solana airdrop 2 --url devnet
```

**Polygon (Mumbai):**
- https://faucet.polygon.technology/

---

## MainnetFork Mode

### Description

Creates a local fork of mainnet at a specific block, allowing testing against real deployed contracts and state.

### Characteristics

- **Chain ID**: Mainnet ID (1 for Ethereum)
- **State**: Real mainnet state at fork block
- **Contracts**: All mainnet contracts available
- **Block Time**: Configurable (local control)
- **Network**: Initial fetch requires internet, then local
- **Cost**: Free (not real mainnet)
- **Persistence**: Local only

### Architecture

```
Mainnet (Block 17000000)
    │
    │ Initial State Fetch
    ▼
Local Fork (Block 17000000)
    ├── All Mainnet State
    ├── All Deployed Contracts
    ├── Impersonate Any Account
    └── Local Execution
        └── No Real Transactions
```

### Configuration

```rust
let config = RuntimeConfig {
    network_mode: NetworkMode::MainnetFork,
    blockchain_config: {
        let mut map = HashMap::new();
        map.insert("fork_url".to_string(), 
            serde_json::json!("https://eth-mainnet.alchemyapi.io/v2/YOUR_KEY"));
        map.insert("fork_block".to_string(), serde_json::json!(17000000));
        map.insert("chain_id".to_string(), serde_json::json!(1));
        map
    },
    ..Default::default()
};
```

### Best For

- ✅ Testing against real contracts
- ✅ Realistic integration tests
- ✅ Testing upgrades before mainnet
- ✅ Debugging mainnet issues
- ✅ Flashbots simulation

### Example

```rust
#[tokio::test]
async fn test_uniswap_interaction() {
    let config = RuntimeConfig {
        network_mode: NetworkMode::MainnetFork,
        blockchain_config: {
            let mut map = HashMap::new();
            map.insert("fork_block".to_string(), serde_json::json!(17000000));
            map
        },
        ..Default::default()
    };
    
    let env = runtime.create_environment(config).await?;
    
    // Interact with real Uniswap contracts
    let uniswap_router = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
    
    let result = runtime.call_function(
        &env,
        uniswap_router,
        "getAmountsOut",
        &args,
    ).await?;
    
    println!("Swap result from mainnet fork: {:?}", result);
    
    runtime.destroy(env).await?;
}
```

### Advanced Features

**Impersonate Accounts:**
```rust
// Impersonate Vitalik's address
let config blockchain_config = {
    let mut map = HashMap::new();
    map.insert("impersonate".to_string(), 
        serde_json::json!(["0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"]));
    map
};
```

**Time Travel:**
```rust
// Advance blockchain time
let config = {
    map.insert("increase_time".to_string(), serde_json::json!(86400)); // +1 day
    map
};
```

---

## Comparison Matrix

| Feature | Local | Testnet | MainnetFork |
|---------|-------|---------|-------------|
| Setup Time | Fast | Medium | Medium |
| Real Network | No | Yes | Fork only |
| Cost | Free | Free | Free |
| Internet Required | No | Yes | Yes (initial) |
| State Persistence | No | Yes | No |
| Real Contracts | No | Testnet only | Yes (mainnet) |
| Block Time | Instant | Real | Configurable |
| Network Congestion | No | Yes | No |
| Account Funding | Pre-funded | Faucet | Infinite (fork) |

## Mode Selection Guide

### Use Local When:

- Developing new features
- Running unit tests
- Need deterministic results
- Want fast feedback
- Working offline

### Use Testnet When:

- Integration testing
- Testing with real network conditions
- Need public visibility
- Want persistent state
- Preparing for mainnet

### Use MainnetFork When:

- Testing against real contracts (Uniswap, Aave, etc.)
- Debugging mainnet issues
- Need realistic state
- Testing contract upgrades
- Simulating mainnet transactions

## Switching Between Modes

```rust
async fn test_on_all_networks(runtime: &dyn BlockchainRuntime) -> Result<()> {
    let modes = vec![
        NetworkMode::Local,
        NetworkMode::Testnet,
        NetworkMode::MainnetFork,
    ];
    
    for mode in modes {
        let config = RuntimeConfig {
            network_mode: mode.clone(),
            ..Default::default()
        };
        
        let env = runtime.create_environment(config).await?;
        
        println!("Testing on {:?}", mode);
        run_test_suite(&env, runtime).await?;
        
        runtime.destroy(env).await?;
    }
    
    Ok(())
}
```

## Conclusion

Choose network mode based on your testing phase:
- **Development** → Local
- **Integration** → Testnet
- **Pre-Mainnet** → MainnetFork

See [User Guide](./user-guide.md) for practical usage.

