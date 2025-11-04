# 🎯 Usage Guide

Learn how to use the Yoink SDK to build powerful creator token applications.

## Basic Usage

### Initializing the SDK

```javascript
import { YoinkSDK } from 'yoink-sdk';

const yoink = new YoinkSDK({
  network: 'mainnet-beta',
  endpoint: process.env.SOLANA_RPC_URL,
  wallet: walletAdapter,
  options: {
    commitment: 'confirmed',
    preflightCommitment: 'processed',
  }
});
```

## Core Features

### 🪙 Token Operations

#### Get Token Information

```javascript
// Get token details
const token = await yoink.getToken('token_address_here');
console.log('Token:', token.name, token.symbol);
console.log('Price:', token.price, 'SOL');
console.log('Market Cap:', token.marketCap, 'SOL');
```

#### Create a New Token

```javascript
const tokenData = {
  name: 'My Creator Token',
  symbol: 'MCT',
  description: 'A token for my amazing content',
  image: 'https://example.com/image.png',
  // Additional metadata
};

const transaction = await yoink.createToken(tokenData);
const signature = await yoink.sendTransaction(transaction);
console.log('Token created:', signature);
```

### 💰 Trading Operations

#### Buy Tokens

```javascript
// Buy tokens with SOL
const buyTransaction = await yoink.buyTokens({
  tokenAddress: 'token_address_here',
  solAmount: 0.1, // Amount in SOL
  slippage: 5, // 5% slippage tolerance
});

const signature = await yoink.sendTransaction(buyTransaction);
console.log('Purchase completed:', signature);
```

#### Sell Tokens

```javascript
// Sell tokens for SOL
const sellTransaction = await yoink.sellTokens({
  tokenAddress: 'token_address_here',
  tokenAmount: 1000, // Amount of tokens to sell
  slippage: 5,
});

const signature = await yoink.sendTransaction(sellTransaction);
console.log('Sale completed:', signature);
```

### 📊 Data & Analytics

#### Get Token Price History

```javascript
const priceHistory = await yoink.getPriceHistory('token_address_here', {
  interval: '1h',
  limit: 100,
});

console.log('Price data points:', priceHistory.length);
```

#### Get Trading Activity

```javascript
const trades = await yoink.getRecentTrades('token_address_here', {
  limit: 50,
});

trades.forEach(trade => {
  console.log(`${trade.type}: ${trade.amount} tokens for ${trade.price} SOL`);
});
```

### 👤 Profile Management

#### Get User Profile

```javascript
const profile = await yoink.getUserProfile(walletAddress);
console.log('Username:', profile.username);
console.log('Total Tokens:', profile.tokensOwned.length);
```

#### Update Profile

```javascript
await yoink.updateProfile({
  username: 'newUsername',
  bio: 'Creator and trader',
  profilePicture: 'https://example.com/avatar.png',
});
```

### 📺 Stream Integration

#### Attach Stream to Token

```javascript
const streamData = {
  tokenAddress: 'token_address_here',
  streamUrl: 'rtmp://stream.url',
  platform: 'twitch', // or 'youtube', 'custom'
  streamerId: 'streamer_id',
};

await yoink.attachStream(streamData);
```

#### Get Live Streams

```javascript
const liveStreams = await yoink.getLiveStreams({
  limit: 20,
  sortBy: 'viewers', // or 'recent', 'price'
});

liveStreams.forEach(stream => {
  console.log(`${stream.title} - ${stream.viewers} viewers`);
});
```

## Advanced Features

### 🔄 Real-time Subscriptions

```javascript
// Subscribe to price updates
yoink.subscribeToPrice('token_address_here', (priceData) => {
  console.log('New price:', priceData.price, 'SOL');
  console.log('24h change:', priceData.change24h, '%');
});

// Subscribe to new trades
yoink.subscribeToTrades('token_address_here', (trade) => {
  console.log(`New ${trade.type}:`, trade.amount, 'tokens');
});
```

### 🛠️ Custom Transactions

```javascript
// Build custom transaction
const customTx = await yoink.buildTransaction([
  // Your custom instructions
]);

// Sign and send
const signature = await yoink.sendTransaction(customTx);
```

### ⚙️ Configuration Options

```javascript
// Update SDK configuration
yoink.updateConfig({
  slippageTolerance: 10, // Default slippage
  priorityFee: 0.001, // SOL
  timeout: 30000, // 30 seconds
});
```

## Error Handling

```javascript
try {
  const result = await yoink.buyTokens({
    tokenAddress: 'invalid_address',
    solAmount: 0.1,
  });
} catch (error) {
  if (error.code === 'INSUFFICIENT_BALANCE') {
    console.log('Not enough SOL in wallet');
  } else if (error.code === 'TOKEN_NOT_FOUND') {
    console.log('Token does not exist');
  } else {
    console.log('Transaction failed:', error.message);
  }
}
```

## Best Practices

### 🔐 Security

- Always validate user inputs
- Use appropriate slippage settings
- Implement proper error handling
- Never expose private keys

### ⚡ Performance

- Cache token data when possible
- Use batch requests for multiple operations
- Implement proper loading states
- Handle network timeouts gracefully

### 🎯 User Experience

- Provide clear transaction feedback
- Show estimated fees before transactions
- Implement retry mechanisms
- Use progressive loading for large datasets

## Next Steps

- [📝 See Example Script 1 in action](example-1.md)
- [🛠️ Try Example Script 2](example-2.md)
- [❓ Check the FAQ](../support/faq.md)