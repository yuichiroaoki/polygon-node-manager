# polygon-node-manager

## setup

1. Install dependencies
```bash
sudo apt install pkg-config libssl-dev
```

2. Add .env file with your credentials
```
API_KEY=<telegram-bot-api-key>
CHAT_ID=<telegram-bot-chat-id>
JSON_RPC_URL=<json-rpc-url-for-your-node> (e.g. http://localhost:8545)
ALCHEMY_JSON_RPC_URL=<polygon-mainnet-json-rpc-url>
```

## Check disk space
```bash
env RUST_LOG=info cargo run -- --disk
```

## Delete logs
```bash
env RUST_LOG=info cargo run -- --clean
```

## Check if bor is fully synced
```bash
env RUST_LOG=info cargo run -- --bor
```

## Send a message with telegram
```bash
env RUST_LOG=info cargo run -- --msg "message"
```
