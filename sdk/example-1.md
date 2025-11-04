# 📝 Example Script 1: Basic Token Trading Bot

This example demonstrates how to create a simple trading bot that monitors token prices and executes trades based on basic criteria.

## Overview

This script will:
- Monitor multiple tokens for price changes
- Execute buy orders when price drops below a threshold
- Execute sell orders when price rises above a threshold
- Log all trading activity

## Prerequisites

- Yoink SDK installed
- Wallet with SOL for trading
- Basic understanding of trading concepts

## Script Code

```javascript
import { YoinkSDK } from 'yoink-sdk';
import { Connection, PublicKey } from '@solana/web3.js';

class BasicTradingBot {
  constructor(config) {
    this.yoink = new YoinkSDK({
      network: config.network || 'mainnet-beta',
      wallet: config.wallet,
    });
    
    this.config = {
      tokens: config.tokens || [],
      buyThreshold: config.buyThreshold || -5, // Buy when price drops 5%
      sellThreshold: config.sellThreshold || 10, // Sell when price rises 10%
      tradeAmount: config.tradeAmount || 0.1, // SOL amount per trade
      checkInterval: config.checkInterval || 30000, // Check every 30 seconds
    };
    
    this.positions = new Map(); // Track current positions
    this.isRunning = false;
  }

  async start() {
    console.log('🤖 Starting Basic Trading Bot...');
    
    try {
      await this.yoink.connect();
      console.log('✅ Connected to wallet:', this.yoink.publicKey);
      
      this.isRunning = true;
      this.monitorTokens();
      
    } catch (error) {
      console.error('❌ Failed to start bot:', error.message);
    }
  }

  async monitorTokens() {
    while (this.isRunning) {
      try {
        for (const tokenAddress of this.config.tokens) {
          await this.checkToken(tokenAddress);
        }
        
        await this.sleep(this.config.checkInterval);
      } catch (error) {
        console.error('❌ Monitoring error:', error.message);
        await this.sleep(5000); // Wait 5 seconds before retrying
      }
    }
  }

  async checkToken(tokenAddress) {
    try {
      // Get current token data
      const token = await this.yoink.getToken(tokenAddress);
      const currentPrice = token.price;
      
      // Get or initialize position
      let position = this.positions.get(tokenAddress) || {
        entryPrice: null,
        amount: 0,
        lastPrice: currentPrice,
      };
      
      // Calculate price change
      const priceChange = position.lastPrice 
        ? ((currentPrice - position.lastPrice) / position.lastPrice) * 100
        : 0;
      
      console.log(`📊 ${token.symbol}: $${currentPrice} (${priceChange.toFixed(2)}%)`);
      
      // Trading logic
      if (position.amount === 0 && priceChange <= this.config.buyThreshold) {
        await this.buyToken(tokenAddress, token);
      } else if (position.amount > 0 && priceChange >= this.config.sellThreshold) {
        await this.sellToken(tokenAddress, token);
      }
      
      // Update position
      position.lastPrice = currentPrice;
      this.positions.set(tokenAddress, position);
      
    } catch (error) {
      console.error(`❌ Error checking ${tokenAddress}:`, error.message);
    }
  }

  async buyToken(tokenAddress, token) {
    try {
      console.log(`🟢 Buying ${token.symbol} at $${token.price}`);
      
      const transaction = await this.yoink.buyTokens({
        tokenAddress,
        solAmount: this.config.tradeAmount,
        slippage: 5,
      });
      
      const signature = await this.yoink.sendTransaction(transaction);
      console.log(`✅ Buy order executed: ${signature}`);
      
      // Update position
      const position = this.positions.get(tokenAddress) || { amount: 0 };
      position.entryPrice = token.price;
      position.amount += this.config.tradeAmount / token.price;
      this.positions.set(tokenAddress, position);
      
    } catch (error) {
      console.error(`❌ Buy failed for ${token.symbol}:`, error.message);
    }
  }

  async sellToken(tokenAddress, token) {
    try {
      const position = this.positions.get(tokenAddress);
      if (!position || position.amount <= 0) return;
      
      console.log(`🔴 Selling ${token.symbol} at $${token.price}`);
      
      const transaction = await this.yoink.sellTokens({
        tokenAddress,
        tokenAmount: position.amount,
        slippage: 5,
      });
      
      const signature = await this.yoink.sendTransaction(transaction);
      console.log(`✅ Sell order executed: ${signature}`);
      
      // Calculate profit/loss
      const exitPrice = token.price;
      const profit = (exitPrice - position.entryPrice) * position.amount;
      console.log(`💰 P&L: ${profit > 0 ? '+' : ''}${profit.toFixed(4)} SOL`);
      
      // Reset position
      position.amount = 0;
      position.entryPrice = null;
      this.positions.set(tokenAddress, position);
      
    } catch (error) {
      console.error(`❌ Sell failed for ${token.symbol}:`, error.message);
    }
  }

  stop() {
    console.log('🛑 Stopping trading bot...');
    this.isRunning = false;
  }

  sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// Usage Example
async function main() {
  const bot = new BasicTradingBot({
    network: 'devnet', // Use devnet for testing
    wallet: yourWalletAdapter,
    tokens: [
      'TOKEN_ADDRESS_1',
      'TOKEN_ADDRESS_2',
    ],
    buyThreshold: -3,  // Buy on 3% dip
    sellThreshold: 5,  // Sell on 5% gain
    tradeAmount: 0.05, // 0.05 SOL per trade
    checkInterval: 15000, // Check every 15 seconds
  });
  
  await bot.start();
  
  // Stop bot after 1 hour (for demo)
  setTimeout(() => {
    bot.stop();
  }, 3600000);
}

// Run the bot
main().catch(console.error);
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `tokens` | Array | `[]` | List of token addresses to monitor |
| `buyThreshold` | Number | `-5` | Price drop percentage to trigger buy |
| `sellThreshold` | Number | `10` | Price rise percentage to trigger sell |
| `tradeAmount` | Number | `0.1` | SOL amount per trade |
| `checkInterval` | Number | `30000` | Monitoring interval in milliseconds |

## Safety Features

- **Slippage Protection**: 5% maximum slippage on all trades
- **Error Handling**: Continues monitoring even if individual trades fail
- **Position Tracking**: Prevents duplicate orders
- **Logging**: Comprehensive trade logging for analysis

## Running the Script

1. **Install dependencies**:
   ```bash
   npm install yoink-sdk @solana/web3.js
   ```

2. **Configure your settings**:
   - Replace `yourWalletAdapter` with your actual wallet adapter
   - Add real token addresses to monitor
   - Adjust thresholds and amounts for your strategy

3. **Run the script**:
   ```bash
   node trading-bot.js
   ```

## ⚠️ Important Notes

- **Use devnet first**: Always test on devnet before using mainnet
- **Start small**: Begin with small amounts to test your strategy
- **Monitor closely**: Keep an eye on the bot's performance
- **Risk management**: Never trade more than you can afford to lose

## Next Steps

- [🛠️ Try Example Script 2](example-2.md)
- [📖 Read the full Usage Guide](usage.md)
- [❓ Check the FAQ](../support/faq.md)