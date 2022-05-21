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
JSON_RPC_URL=<polygon-mainnet-json-rpc-url>
EXPRESS_SERVER_URL=<bot-util-api-url>
```

## Check disk space
```bash
cargo run -- --disk
```

## Delete logs
```bash
cargo run -- --clean
```

## Check if bor is fully synced
```bash
cargo run -- --bor
```

## Send a message with telegram
```bash
cargo run -- --msg "message"
```