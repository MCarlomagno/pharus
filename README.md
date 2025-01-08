## Pharus
 
> **WARNING**: This project is experimental, expect breaking changes, this is not production ready.

Pharus is a tool for verifying smart contract deployments by comparing local compiled contracts with their on-chain counterparts across EVM-compatible (bytecode) and Stellar networks (wasm).

## Installation

```bash
cargo install pharus
```

Or build from source:

```bash
git clone https://github.com/yourusername/pharus
cd pharus
cargo build --release
```

## Usage

### Basic Usage

```bash
pharus --network <NETWORK> --local <LOCAL_PATH> --remote <CONTRACT_ADDRESS>
```

### Examples

#### Stellar Contract Verification
```bash
# Mainnet
pharus --network stellar \
       --local ./path/to/contract.wasm \
       --remote CB5HA53QWBLOCD7LQOFZ4FIOSQS2ZUA7KIBZYOV6D4CPJWXIYGX2OBAC

# Testnet
pharus --network stellar-testnet \
       --local ./path/to/contract.wasm \
       --remote CCHXQJ5YDCIRGCBUTLC5BF2V2DKHULVPTQJGD4BAHW46JQWVRQNGA2LU
```

#### EVM Contract Verification
```bash
pharus --network ethereum \
       --local ./path/to/artifact.json \
       --remote 0x1234567890123456789012345678901234567890 \
       --contract-path contracts/MyContract.sol \
       --contract-name MyContract

# Custom EVM Network
pharus --network custom-evm \
       --rpc-url https://my-network-rpc.example.com \
       --local ./path/to/artifact.json \
       --remote 0x1234567890123456789012345678901234567890 \
       --contract-path contracts/MyContract.sol \
       --contract-name MyContract
```

### Command Line Options

| Option | Description | Required |
|--------|-------------|----------|
| `--network` | Network name (e.g., stellar, ethereum, sepolia) | Yes |
| `--local` | Path to local contract file | Yes |
| `--remote` | On-chain contract address | Yes |
| `--rpc-url` | Custom RPC URL (optional for supported networks) | No |
| `--network-passphrase` | Custom network passphrase (Stellar only) | No |
| `--contract-path` | Contract path in artifact (EVM only) | Yes for EVM |
| `--contract-name` | Contract name in artifact (EVM only) | Yes for EVM |

## Supported Networks

### Stellar
- Mainnet (Default RPC: https://mainnet.sorobanrpc.com)
- Testnet (Default RPC: https://soroban-testnet.stellar.org)

### EVM
- Any custom EVM-compatible network (requires `--rpc-url`)

## Development

### Running Tests

```bash
cargo test
```
