# ⚡ Quick Start - Installation

Get up and running with the Yoink SDK in just a few minutes.

## Prerequisites

Before installing the Yoink SDK, make sure you have:

- **Node.js** (version 16 or higher)
- **npm** or **yarn** package manager
- A **Solana wallet** (Phantom, Solflare, etc.)
- Basic knowledge of **JavaScript/TypeScript**

## Installation

### Using npm

```bash
npm install yoink-sdk
```

### Using yarn

```bash
yarn add yoink-sdk
```

### Using pnpm

```bash
pnpm add yoink-sdk
```

## Quick Setup

### 1. Import the SDK

```javascript
import { YoinkSDK } from 'yoink-sdk';

// Or using CommonJS
const { YoinkSDK } = require('yoink-sdk');
```

### 2. Initialize the SDK

```javascript
const yoink = new YoinkSDK({
  network: 'mainnet-beta', // or 'devnet' for testing
  endpoint: 'https://api.mainnet-beta.solana.com',
  wallet: yourWalletAdapter, // Your wallet adapter
});
```

### 3. Connect to Wallet

```javascript
// Connect wallet
await yoink.connect();

// Check connection status
if (yoink.isConnected) {
  console.log('Connected to wallet:', yoink.publicKey);
}
```

## Environment Variables

Create a `.env` file in your project root:

```bash
# Solana RPC Endpoint
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com

# Yoink API Base URL
YOINK_API_URL=https://api.yoink.trade

# Your application name (optional)
APP_NAME=Your App Name
```

## Verification

Test your installation with this simple script:

```javascript
import { YoinkSDK } from 'yoink-sdk';

async function testConnection() {
  const yoink = new YoinkSDK({
    network: 'devnet', // Use devnet for testing
  });
  
  console.log('SDK Version:', yoink.version);
  console.log('Network:', yoink.network);
  console.log('Ready to build with Yoink! 🚀');
}

testConnection();
```

## Next Steps

- [📖 Read the Usage Guide](usage.md)
- [🎯 Try Example Script 1](example-1.md)
- [🛠️ Explore Example Script 2](example-2.md)

## Troubleshooting

### Common Issues

**"Module not found" error**
- Make sure you've installed the SDK: `npm install yoink-sdk`
- Check your Node.js version: `node --version`

**Wallet connection issues**
- Ensure your wallet is unlocked
- Check network settings (mainnet vs devnet)
- Verify wallet adapter compatibility

**Network errors**
- Check your internet connection
- Verify RPC endpoint is working
- Try switching to a different Solana RPC endpoint

Need help? Join our [Discord community](https://discord.gg/yoink) or check our [FAQ](../support/faq.md).