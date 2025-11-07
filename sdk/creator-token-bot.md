# 🤖 Example - Trading Bot

This example demonstrates how to create an intelligent trading bot specifically designed for Yoink bonding curve tokens, with features for monitoring market conditions, price movements, and automated trading strategies using the real SDK.

## Overview

This bot will:
- Monitor bonding curve tokens across the Yoink protocol
- Track market cap, buyer activity, and price movements
- Execute buy orders based on momentum and market conditions
- Implement stop-loss and take-profit strategies with slippage protection
- Log all trading activity with detailed analytics
- Send notifications for important price movements

## Prerequisites

- Yoink SDK installed: `npm install yoink-sdk`
- Funded wallet with SOL for trading
- Basic understanding of bonding curves and trading concepts

## Script Code

```typescript
import { YoinkSDK } from "yoink-sdk";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { AnchorProvider } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { getAccount, getAssociatedTokenAddress } from "@solana/spl-token";

interface BotConfig {
  // Target Token Settings
  targetTokens: string[];           // Specific token mints to monitor
  maxMarketCap: number;            // SOL - max market cap to trade
  minBuyers: number;               // Minimum number of buyers required
  onlyIncompleteCarves: boolean;   // Only trade incomplete bonding curves
  
  // Trading Parameters
  buyAmount: number;               // SOL amount per buy
  maxSlippage: number;            // Slippage tolerance in basis points
  maxPriceImpact: number;         // Maximum price impact percentage
  profitTarget: number;           // Take profit percentage
  stopLoss: number;               // Stop loss percentage
  
  // Bot Behavior
  checkInterval: number;          // Check interval in milliseconds
  maxPositions: number;           // Maximum concurrent positions
  enableMomentumTrading: boolean; // Enable momentum-based trading
}

interface Position {
  mint: string;
  balance: number;
  entryPrice: number;
  entryTime: number;
  entryMarketCap: number;
  stopLossPrice: number;
  takeProfitPrice: number;
}

class BondingCurveTradingBot {
  private sdk: YoinkSDK;
  private keypair: Keypair;
  private config: BotConfig;
  private positions: Map<string, Position>;
  private isRunning: boolean = false;
  private marketData: Map<string, any[]> = new Map(); // Price history

  constructor(keypair: Keypair, config: Partial<BotConfig>) {
    // Initialize connection and provider
    const connection = new Connection(config.rpcUrl || "https://staging-rpc.dev2.eclipsenetwork.xyz");
    const wallet = new NodeWallet(keypair);
    const provider = new AnchorProvider(connection, wallet, { commitment: "confirmed" });
    
    this.sdk = new YoinkSDK(provider);
    this.keypair = keypair;
    this.positions = new Map();
    
    this.config = {
      targetTokens: config.targetTokens || [],
      maxMarketCap: config.maxMarketCap || 50, // 50 SOL max market cap
      minBuyers: config.minBuyers || 10,
      onlyIncompleteCarves: config.onlyIncompleteCarves !== false,
      buyAmount: config.buyAmount || 0.01, // 0.01 SOL per trade
      maxSlippage: config.maxSlippage || 500, // 5%
      maxPriceImpact: config.maxPriceImpact || 3, // 3%
      profitTarget: config.profitTarget || 25, // 25% profit target
      stopLoss: config.stopLoss || -15, // 15% stop loss
      checkInterval: config.checkInterval || 10000, // 10 seconds
      maxPositions: config.maxPositions || 5,
      enableMomentumTrading: config.enableMomentumTrading !== false,
  }

  async start() {
    console.log('🤖 Starting Bonding Curve Trading Bot...');
    console.log('Wallet:', this.keypair.publicKey.toBase58());
    
    try {
      // Test connection
      const global = await this.sdk.getGlobalAccount();
      console.log('✅ Connected to Yoink protocol');
      console.log('Protocol fee:', Number(global.feeBasisPoints) / 100, '%');
      
      console.log('📊 Bot Configuration:');
      console.log('  Max Market Cap:', this.config.maxMarketCap, 'SOL');
      console.log('  Min Buyers:', this.config.minBuyers);
      console.log('  Buy Amount:', this.config.buyAmount, 'SOL');
      console.log('  Profit Target:', this.config.profitTarget, '%');
      console.log('  Stop Loss:', this.config.stopLoss, '%');
      console.log('  Max Positions:', this.config.maxPositions);
      
      this.isRunning = true;
      await this.monitorAndTrade();
      
    } catch (error) {
      console.error('❌ Failed to start bot:', error);
    }
  }

  async monitorAndTrade() {
    while (this.isRunning) {
      try {
        console.log('🔍 Scanning market...');
        
        // Check existing positions first
        await this.checkExistingPositions();
        
        // Look for new opportunities
        if (this.positions.size < this.config.maxPositions) {
          await this.scanForOpportunities();
        }
        
        await this.sleep(this.config.checkInterval);
        
      } catch (error) {
        console.error('❌ Monitoring error:', error);
        await this.sleep(5000);
      }
  }

  async checkExistingPositions() {
    for (const [mintStr, position] of this.positions.entries()) {
      try {
        const mint = new PublicKey(mintStr);
        const curve = await this.sdk.getBondingCurveAccount(mint);
        
        if (!curve) continue;
        
        const currentPrice = curve.getPricePerToken();
        const profitLoss = ((currentPrice - position.entryPrice) / position.entryPrice) * 100;
        
        console.log(`📈 ${mintStr.slice(0, 8)}... P&L: ${profitLoss.toFixed(2)}%`);
        
        // Check exit conditions
        const shouldSell = 
          profitLoss >= this.config.profitTarget ||
          profitLoss <= this.config.stopLoss ||
          curve.complete;
        
        if (shouldSell) {
          const reason = curve.complete ? 'Curve completed' : 
                        profitLoss >= this.config.profitTarget ? 'Profit target hit' :
                        'Stop loss triggered';
          
          console.log(`💰 Selling ${mintStr.slice(0, 8)}... (${reason})`);
          await this.sellPosition(mint, position);
        }
        
      } catch (error) {
        console.error(`❌ Error checking position ${mintStr}:`, error);
      }
    }
  }

  async scanForOpportunities() {
    for (const mintStr of this.config.targetTokens) {
      try {
        const mint = new PublicKey(mintStr);
        await this.analyzeToken(mint);
      } catch (error) {
        console.error(`❌ Error analyzing ${mintStr}:`, error);
      }
    }
  }

  async analyzeToken(mint: PublicKey) {
    try {
      const curve = await this.sdk.getBondingCurveAccount(mint);
      if (!curve) return;
      
      const marketCap = Number(curve.getMarketCapSOL()) / LAMPORTS_PER_SOL;
      const totalBuyers = Number(curve.totalBuyers);
      const isComplete = curve.complete;
      const currentPrice = curve.getPricePerToken();
      
      // Store price history for momentum analysis
      const history = this.marketData.get(mint.toBase58()) || [];
      history.push({
        timestamp: Date.now(),
        price: currentPrice,
        marketCap,
        buyers: totalBuyers
      });
      
      // Keep only last 20 data points
      if (history.length > 20) history.shift();
      this.marketData.set(mint.toBase58(), history);
      
      console.log(`🔍 ${mint.toBase58().slice(0, 8)}...`);
      console.log(`   Market Cap: ${marketCap.toFixed(4)} SOL`);
      console.log(`   Buyers: ${totalBuyers}`);
      console.log(`   Complete: ${isComplete}`);
      
      // Check if we should buy
      const shouldBuy = this.shouldBuyToken(curve, history);
      
      if (shouldBuy && !this.positions.has(mint.toBase58())) {
        await this.buyToken(mint, curve);
      }
      
    } catch (error) {
      console.error(`❌ Analysis error for ${mint.toBase58()}:`, error);
    }
  }

  shouldBuyToken(curve: any, history: any[]): boolean {
    const marketCap = Number(curve.getMarketCapSOL()) / LAMPORTS_PER_SOL;
    const totalBuyers = Number(curve.totalBuyers);
    
    // Basic filters
    if (marketCap > this.config.maxMarketCap) return false;
    if (totalBuyers < this.config.minBuyers) return false;
    if (this.config.onlyIncompleteCarves && curve.complete) return false;
    
    // Momentum analysis
    if (this.config.enableMomentumTrading && history.length >= 3) {
      const recent = history.slice(-3);
      const buyerGrowth = recent[2].buyers - recent[0].buyers;
      const priceGrowth = ((recent[2].price - recent[0].price) / recent[0].price) * 100;
      
      // Look for positive momentum
      if (buyerGrowth > 2 && priceGrowth > 5) {
        console.log(`📈 Momentum detected: +${buyerGrowth} buyers, +${priceGrowth.toFixed(2)}% price`);
        return true;
      }
    }
    
    // Basic entry conditions
    return marketCap < 20 && totalBuyers > 15; // Conservative entry
  }

  async buyToken(mint: PublicKey, curve: any) {
    try {
      const buyAmount = BigInt(this.config.buyAmount * LAMPORTS_PER_SOL);
      
      // Get quote first
      const quote = await this.sdk.getBuyQuote(mint, buyAmount, BigInt(this.config.maxSlippage));
      
      if (quote.priceImpact > this.config.maxPriceImpact) {
        console.log(`⚠️ Price impact too high: ${quote.priceImpact.toFixed(2)}%`);
        return;
      }
      
      console.log(`🟢 Buying ${mint.toBase58().slice(0, 8)}...`);
      console.log(`   Quote: ${quote.tokenAmount} tokens`);
      console.log(`   Price impact: ${quote.priceImpact.toFixed(2)}%`);
      
      const result = await this.sdk.buy(
        this.keypair,
        mint,
        buyAmount,
        BigInt(this.config.maxSlippage),
        { unitLimit: 400000, unitPrice: 100000 }
      );
      
      if (result.success) {
        const position: Position = {
          mint: mint.toBase58(),
          balance: Number(quote.tokenAmount) / Math.pow(10, 6),
          entryPrice: curve.getPricePerToken(),
          entryTime: Date.now(),
          entryMarketCap: Number(curve.getMarketCapSOL()) / LAMPORTS_PER_SOL,
          stopLossPrice: curve.getPricePerToken() * (1 + this.config.stopLoss / 100),
          takeProfitPrice: curve.getPricePerToken() * (1 + this.config.profitTarget / 100),
        };
        
        this.positions.set(mint.toBase58(), position);
        console.log(`✅ Position opened! Signature: ${result.signature}`);
        
      } else {
        console.error(`❌ Buy failed:`, result.error);
      }
      
    } catch (error) {
      console.error('❌ Buy execution error:', error);
    }
  }

  async sellPosition(mint: PublicKey, position: Position) {
    try {
      const tokenAmount = BigInt(Math.floor(position.balance * Math.pow(10, 6)));
      
      const result = await this.sdk.sell(
        this.keypair,
        mint,
        tokenAmount,
        BigInt(this.config.maxSlippage)
      );
      
      if (result.success) {
        const curve = await this.sdk.getBondingCurveAccount(mint);
        const exitPrice = curve?.getPricePerToken() || 0;
        const profitLoss = ((exitPrice - position.entryPrice) / position.entryPrice) * 100;
        const holdTime = (Date.now() - position.entryTime) / 1000 / 60; // minutes
        
        console.log(`✅ Position closed! Signature: ${result.signature}`);
        console.log(`💰 Final P&L: ${profitLoss.toFixed(2)}%`);
        console.log(`⏱️ Hold time: ${holdTime.toFixed(1)} minutes`);
        
        this.positions.delete(mint.toBase58());
        
      } else {
        console.error(`❌ Sell failed:`, result.error);
      }
      
    } catch (error) {
      console.error('❌ Sell execution error:', error);
    }
  }

  displayStatus() {
    console.log('\\n📊 Bot Status:');
    console.log(`Active positions: ${this.positions.size}/${this.config.maxPositions}`);
    console.log(`Monitoring ${this.config.targetTokens.length} tokens`);
    
    if (this.positions.size > 0) {
      console.log('\\n💼 Current Positions:');
      this.positions.forEach(position => {
        const holdTime = (Date.now() - position.entryTime) / 1000 / 60;
        console.log(`  ${position.mint.slice(0, 8)}... (${holdTime.toFixed(1)}m ago)`);
      });
    }
    console.log('─'.repeat(50));
  }

  stop() {
    console.log('🛑 Stopping trading bot...');
    this.isRunning = false;
  }

  sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// Usage Example
async function main() {
  const keypair = Keypair.generate(); // Use your funded keypair
  
  const bot = new BondingCurveTradingBot(keypair, {
    targetTokens: [
      "9BSxAV9iRuiT3W7kwhFEkmzfoMo7xZTBdFGRF793JRbC",
      // Add more token mints to monitor
    ],
    
    maxMarketCap: 30,           // Max 30 SOL market cap
    minBuyers: 8,               // Min 8 buyers
    onlyIncompleteCarves: true, // Only trade incomplete curves
    
    buyAmount: 0.005,           // 0.005 SOL per trade
    maxSlippage: 400,           // 4% slippage
    maxPriceImpact: 2.5,        // 2.5% max price impact
    profitTarget: 20,           // 20% profit target
    stopLoss: -12,              // 12% stop loss
    
    checkInterval: 8000,        // Check every 8 seconds
    maxPositions: 3,            // Max 3 concurrent positions
    enableMomentumTrading: true, // Enable momentum analysis
  });
  
  await bot.start();
  
  // Display status every 30 seconds
  setInterval(() => {
    bot.displayStatus();
  }, 30000);
  
  // Stop after 2 hours (for demo)
  setTimeout(() => {
    bot.stop();
  }, 2 * 60 * 60 * 1000);
}

main().catch(console.error);
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `targetTokens` | Array | `[]` | Token mint addresses to monitor |
| `maxMarketCap` | Number | `50` | Maximum market cap in SOL |
| `minBuyers` | Number | `10` | Minimum number of buyers required |
| `onlyIncompleteCarves` | Boolean | `true` | Only trade incomplete bonding curves |
| `buyAmount` | Number | `0.01` | SOL amount per trade |
| `maxSlippage` | Number | `500` | Maximum slippage in basis points |
| `maxPriceImpact` | Number | `3` | Maximum price impact percentage |
| `profitTarget` | Number | `25` | Profit target percentage |
| `stopLoss` | Number | `-15` | Stop loss percentage |
| `maxPositions` | Number | `5` | Maximum concurrent positions |
| `enableMomentumTrading` | Boolean | `true` | Enable momentum analysis |

## Bot Features

- **Real Market Analysis**: Uses actual bonding curve data for decision making
- **Momentum Detection**: Analyzes buyer growth and price trends
- **Risk Management**: Automatic stop-loss and take-profit execution
- **Position Tracking**: Comprehensive position management
- **Slippage Protection**: Built-in price impact and slippage controls
- **Market Cap Filtering**: Focuses on tokens within specified market cap range

## Safety Features

- **Max Position Limits**: Prevents over-exposure
- **Price Impact Checks**: Avoids high-impact trades
- **Stop Loss Protection**: Automatic risk management
- **Curve Completion Detection**: Exits positions when curves complete
- **Comprehensive Logging**: Detailed trade history and performance

## Running the Bot

1. **Set up your keypair and configuration**
2. **Add target token mints to monitor**
3. **Adjust risk parameters for your strategy**
4. **Run with** `npx ts-node trading-bot.ts`

**⚠️ Important**: Test thoroughly on testnet before using real funds!

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

## Next Steps

- [📦 SDK Overview](overview.md)
- [📖 Read the full Usage Guide](usage.md)
- [🌐 Join Community](../support/socials.md)