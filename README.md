# 🔮 Prediction Time Capsule

A smart contract built on **Stellar Soroban** that allows users to submit time-locked predictions. Predictions are sealed on-chain and can only be read after a specified unlock time — no one can cheat, edit, or peek early.

> **Network:** Testnet  
> **Contract ID:** `CCIQGZVEYMP2WHHWN7K2UDA5IC62ZGXLRUZUC7ZSJUODRKNHVMRKOA3N`  
> **Explorer:** [View on Stellar Lab](https://lab.stellar.org/smart-contracts/contract-explorer?$=network$id=testnet&label=Testnet&horizonUrl=https:////horizon-testnet.stellar.org&rpcUrl=https:////soroban-testnet.stellar.org&passphrase=Test%20SDF%20Network%20/;%20September%202015;&smartContracts$explorer$contractId=CCIQGZVEYMP2WHHWN7K2UDA5IC62ZGXLRUZUC7ZSJUODRKNHVMRKOA3N)

---

## 💡 What is this?

Traditional apps let admins edit or delete records. Blockchain doesn't. This contract leverages that immutability to create a **trustless prediction platform** — once you submit a prediction, it's locked on-chain forever. No one, not even you, can change it before the reveal date.

Use cases:
- Submit a crypto price prediction, lock it for 1 month, reveal it publicly
- Make a bold claim about a future event, prove you said it first
- Build an on-chain reputation as a forecaster

---

## 📦 Data Structures

### `Prediction`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | Auto-incremented unique identifier |
| `owner` | `Address` | Wallet address of the creator |
| `content` | `String` | The prediction text (hidden until unlock) |
| `category` | `Symbol` | Topic category e.g. `crypto`, `sports`, `tech` |
| `unlock_time` | `u64` | Unix timestamp when the prediction becomes readable |
| `created_at` | `u64` | Unix timestamp when the prediction was submitted |
| `verdict` | `Symbol` | `PENDING`, `CORRECT`, or `WRONG` |

### `Reputation`

| Field | Type | Description |
|---|---|---|
| `total` | `u32` | Total predictions made |
| `correct` | `u32` | Predictions marked as correct |
| `wrong` | `u32` | Predictions marked as wrong |

---

## 🔧 Contract Functions

### `submit_prediction`
Submit a new time-locked prediction.

**Parameters:**
| Name | Type | Description |
|---|---|---|
| `owner` | `Address` | Your wallet address (must sign the transaction) |
| `content` | `String` | Your prediction text |
| `category` | `Symbol` | Category e.g. `crypto`, `politics`, `tech` |
| `unlock_time` | `u64` | Unix timestamp — must be in the future |

**Returns:** `String` — success or error message

**Notes:**
- Requires auth from `owner`
- `unlock_time` must be greater than the current ledger timestamp
- Content is stored encrypted on-chain and only revealed after `unlock_time`

---

### `get_predictions`
Retrieve all predictions. Content of locked predictions is replaced with `"LOCKED"`.

**Parameters:** none

**Returns:** `Vec<Prediction>`

---

### `get_prediction_by_id`
Retrieve a single prediction by its ID.

**Parameters:**
| Name | Type | Description |
|---|---|---|
| `id` | `u64` | The prediction ID |

**Returns:** `Option<Prediction>` — `None` if not found

---

### `submit_verdict`
Mark your prediction as correct or wrong after it has been unlocked.

**Parameters:**
| Name | Type | Description |
|---|---|---|
| `caller` | `Address` | Your wallet address (must be the prediction owner) |
| `id` | `u64` | The prediction ID |
| `is_correct` | `bool` | `true` if the prediction was correct, `false` if wrong |

**Returns:** `String` — success or error message

**Notes:**
- Only callable by the original `owner`
- Only callable after `unlock_time` has passed
- Verdict can only be submitted once per prediction

---

### `get_reputation`
Get the prediction track record of any address.

**Parameters:**
| Name | Type | Description |
|---|---|---|
| `owner` | `Address` | The wallet address to look up |

**Returns:** `Reputation` struct with `total`, `correct`, and `wrong` counts

---

## 🚀 How to Interact

### Via Stellar Lab (Browser)

1. Open the [Contract Explorer](https://lab.stellar.org/smart-contracts/contract-explorer?$=network$id=testnet&label=Testnet&horizonUrl=https:////horizon-testnet.stellar.org&rpcUrl=https:////soroban-testnet.stellar.org&passphrase=Test%20SDF%20Network%20/;%20September%202015;&smartContracts$explorer$contractId=CCIQGZVEYMP2WHHWN7K2UDA5IC62ZGXLRUZUC7ZSJUODRKNHVMRKOA3N)
2. Connect your **Freighter Wallet** (set to Testnet)
3. Select a function from the list
4. Fill in the parameters
5. Click **Simulate** to dry-run, then **Submit** to execute

### Via CLI

```bash
# Submit a prediction
stellar contract invoke \
  --id CCIQGZVEYMP2WHHWN7K2UDA5IC62ZGXLRUZUC7ZSJUODRKNHVMRKOA3N \
  --source <your-keypair-name> \
  --network testnet \
  -- submit_prediction \
  --owner <your-address> \
  --content "BTC will hit 200k by end of 2025" \
  --category crypto \
  --unlock_time 1777000000

# Get all predictions
stellar contract invoke \
  --id CCIQGZVEYMP2WHHWN7K2UDA5IC62ZGXLRUZUC7ZSJUODRKNHVMRKOA3N \
  --source <your-keypair-name> \
  --network testnet \
  -- get_predictions
```

---

## ⏱️ Working with Unix Timestamps

`unlock_time` must be a Unix timestamp in **seconds** (not milliseconds).

Get values quickly in your browser console:

```javascript
// Current timestamp
Math.floor(Date.now() / 1000)

// 1 hour from now
Math.floor(Date.now() / 1000) + 3600

// 1 day from now
Math.floor(Date.now() / 1000) + 86400

// 1 week from now
Math.floor(Date.now() / 1000) + 604800
```

---

## 🏗️ Project Structure

```
contracts/
└── notes/
    └── src/
        ├── lib.rs      # Main contract logic
        └── test.rs     # Unit tests
Cargo.toml
Makefile
README.md
```

---

## 🛠️ Build & Deploy

```bash
# Build the contract
stellar contract build

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/notes.wasm \
  --source <your-keypair-name> \
  --network testnet
```

---

## 📄 License

MIT