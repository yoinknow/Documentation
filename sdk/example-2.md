# 📝 Example Script 2: Portfolio Analytics Dashboard

This example demonstrates how to build a comprehensive portfolio analytics dashboard that tracks your bonding curve token holdings, performance, and provides real-time insights using the Yoink SDK.

## Overview

This script will:
- Track all bonding curve token holdings in real-time
- Calculate portfolio performance and P&L based on actual market data
- Monitor market cap changes and buyer activity
- Generate analytics reports with bonding curve statistics
- Send alerts for significant price movements

## Prerequisites

- Yoink SDK installed: `npm install yoink-sdk`
- Funded Solana wallet to track
- Basic understanding of bonding curves and portfolio management

## Script Code

```typescript
import { YoinkSDK } from "yoink-sdk";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { AnchorProvider } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { getAccount, getAssociatedTokenAddress } from "@solana/spl-token";
import fs from 'fs';
import path from 'path';

interface TokenHolding {
  mint: string;
  balance: number;
  currentPrice: number;
  marketCap: number;
  totalBuyers: number;
  isComplete: boolean;
  value: number;
  change24h?: number;
}

interface PortfolioSnapshot {
  timestamp: number;
  totalValue: number;
  holdings: TokenHolding[];
  topPerformer: string;
  bottomPerformer: string;
}

class PortfolioAnalytics {
  private sdk: YoinkSDK;
  private config: any;
  private previousSnapshot?: PortfolioSnapshot;
  private isRunning: boolean = false;

  constructor(config: any) {
    // Initialize connection and provider
    const connection = new Connection(config.rpcUrl || "https://staging-rpc.dev2.eclipsenetwork.xyz");
    const wallet = new NodeWallet(config.keypair || Keypair.generate());
    const provider = new AnchorProvider(connection, wallet, { commitment: "confirmed" });
    
    this.sdk = new YoinkSDK(provider);
    
    this.config = {
      walletAddress: config.walletAddress,
      updateInterval: config.updateInterval || 60000, // 1 minute
      alertThresholds: {
        gainPercent: config.alertThresholds?.gainPercent || 20,
        lossPercent: config.alertThresholds?.lossPercent || -10,
        marketCapChange: config.alertThresholds?.marketCapChange || 50,
      },
      exportPath: config.exportPath || './portfolio_data.json',
      tokenMints: config.tokenMints || [], // Specific tokens to track
    };
  }

  async start() {
    console.log('📊 Starting Portfolio Analytics Dashboard...');
    
    try {
      // Test connection to Yoink protocol
      const global = await this.sdk.getGlobalAccount();
      console.log('✅ Connected to Yoink protocol');
      console.log('Protocol fee:', Number(global.feeBasisPoints) / 100, '%');
      console.log('📈 Tracking wallet:', this.config.walletAddress);
      
      // Initial portfolio load
      await this.updatePortfolio();
      
      this.isRunning = true;
      this.startMonitoring();
      
    } catch (error) {
      console.error('❌ Failed to start analytics:', error);
    }
  }

  async startMonitoring() {
    while (this.isRunning) {
      try {
        await this.updatePortfolio();
        await this.generateReport();
        await this.checkAlerts();
        
        await this.sleep(this.config.updateInterval);
      } catch (error) {
        console.error('❌ Monitoring error:', error.message);
        await this.sleep(5000);
      }
    }
  }

  async updatePortfolio() {
    try {
      console.log('🔄 Updating portfolio data...');
      
      const holdings: TokenHolding[] = [];
      let totalValue = 0;
      
      // Check holdings for each configured token mint
      for (const mintStr of this.config.tokenMints) {
        try {
          const mint = new PublicKey(mintStr);
          const balance = await this.getTokenBalance(mint, new PublicKey(this.config.walletAddress));
          
          if (balance > 0) {
            // Get bonding curve data
            const curve = await this.sdk.getBondingCurveAccount(mint);
            if (curve) {
              const currentPrice = curve.getPricePerToken();
              const marketCap = Number(curve.getMarketCapSOL()) / LAMPORTS_PER_SOL;
              const value = balance * currentPrice;
              
              const holding: TokenHolding = {
                mint: mintStr,
                balance,
                currentPrice,
                marketCap,
                totalBuyers: Number(curve.totalBuyers),
                isComplete: curve.complete,
                value,
              };
              
              holdings.push(holding);
              totalValue += value;
              
              console.log(`🪙 ${mintStr.slice(0, 8)}...: ${balance.toFixed(2)} tokens (${value.toFixed(6)} SOL)`);
            }
          }
        } catch (error) {
          console.error(`❌ Error checking ${mintStr}:`, error);
        }
      }
      
      // Create portfolio snapshot
      const snapshot: PortfolioSnapshot = {
        timestamp: Date.now(),
        totalValue,
        holdings,
        topPerformer: this.findTopPerformer(holdings),
        bottomPerformer: this.findBottomPerformer(holdings),
      };
      
      // Calculate changes from previous snapshot
      if (this.previousSnapshot) {
        this.calculateChanges(snapshot, this.previousSnapshot);
        this.checkAlerts(snapshot, this.previousSnapshot);
      }
      
      
      console.log(`💼 Total Portfolio Value: ${totalValue.toFixed(6)} SOL`);
      console.log('─'.repeat(50));
      
    } catch (error) {
      console.error('❌ Error updating portfolio:', error);
    }
  }

  async getTokenBalance(mint: PublicKey, owner: PublicKey): Promise<number> {
    try {
      const ata = await getAssociatedTokenAddress(mint, owner);
      const account = await getAccount(this.sdk.connection, ata);
      return Number(account.amount) / Math.pow(10, 6); // Assuming 6 decimals
    } catch (error) {
      return 0; // No token account or zero balance
    }
  }

  findTopPerformer(holdings: TokenHolding[]): string {
    if (holdings.length === 0) return 'None';
    const top = holdings.reduce((prev, current) => 
      (current.value > prev.value) ? current : prev
    );
    return `${top.mint.slice(0, 8)}... (${top.value.toFixed(6)} SOL)`;
  }

  findBottomPerformer(holdings: TokenHolding[]): string {
    if (holdings.length === 0) return 'None';
    const bottom = holdings.reduce((prev, current) => 
      (current.value < prev.value) ? current : prev
    );
    return `${bottom.mint.slice(0, 8)}... (${bottom.value.toFixed(6)} SOL)`;
  }

  calculateChanges(current: PortfolioSnapshot, previous: PortfolioSnapshot) {
    const valueChange = current.totalValue - previous.totalValue;
    const percentChange = previous.totalValue > 0 ? 
      (valueChange / previous.totalValue) * 100 : 0;
    
    console.log(`📈 Portfolio Change: ${valueChange > 0 ? '+' : ''}${valueChange.toFixed(6)} SOL (${percentChange.toFixed(2)}%)`);
    
    // Add 24h change to holdings
    current.holdings.forEach(holding => {
      const prevHolding = previous.holdings.find(h => h.mint === holding.mint);
      if (prevHolding) {
        const priceChange = ((holding.currentPrice - prevHolding.currentPrice) / prevHolding.currentPrice) * 100;
        holding.change24h = priceChange;
      }
    });
  }

    // Check for significant portfolio changes
    const valueChange = current.totalValue - previous.totalValue;
    const percentChange = previous.totalValue > 0 ? 
      (valueChange / previous.totalValue) * 100 : 0;
    
    if (Math.abs(percentChange) >= this.config.alertThresholds.gainPercent) {
      console.log(`🚨 ALERT: Portfolio ${percentChange > 0 ? 'gained' : 'lost'} ${Math.abs(percentChange).toFixed(2)}%`);
    }
    
    // Check individual token alerts
    current.holdings.forEach(holding => {
      const prevHolding = previous.holdings.find(h => h.mint === holding.mint);
      if (prevHolding) {
        const marketCapChange = ((holding.marketCap - prevHolding.marketCap) / prevHolding.marketCap) * 100;
        
        if (Math.abs(marketCapChange) >= this.config.alertThresholds.marketCapChange) {
          console.log(`🚨 ALERT: ${holding.mint.slice(0, 8)}... market cap ${marketCapChange > 0 ? 'increased' : 'decreased'} by ${Math.abs(marketCapChange).toFixed(2)}%`);
        }
      }
    });
  }

  displayPortfolio(snapshot: PortfolioSnapshot) {
    console.clear();
    console.log('🚀 YOINK PORTFOLIO ANALYTICS DASHBOARD');
    console.log('=====================================');
    console.log(`Last Update: ${new Date(snapshot.timestamp).toLocaleString()}`);
    console.log();
    
    // Portfolio Overview
    console.log('📊 PORTFOLIO OVERVIEW');
    console.log('---------------------');
    console.log(`Total Value: ${snapshot.totalValue.toFixed(6)} SOL`);
    console.log(`Top Performer: ${snapshot.topPerformer}`);
    console.log(`Bottom Performer: ${snapshot.bottomPerformer}`);
    console.log();
    
    // Holdings
    console.log('💼 CURRENT HOLDINGS');
    console.log('-------------------');
    if (snapshot.holdings.length === 0) {
      console.log('No tokens in portfolio');
    } else {
      snapshot.holdings.forEach(holding => {
        const changeColor = (holding.change24h || 0) >= 0 ? '🟢' : '🔴';
        const changeStr = holding.change24h ? `(${holding.change24h > 0 ? '+' : ''}${holding.change24h.toFixed(2)}%)` : '';
        
        console.log(`${changeColor} ${holding.mint.slice(0, 8)}...`);
        console.log(`   Balance: ${holding.balance.toFixed(2)} tokens`);
        console.log(`   Value: ${holding.value.toFixed(6)} SOL ${changeStr}`);
        console.log(`   Price: ${holding.currentPrice.toFixed(8)} SOL/token`);
        console.log(`   Market Cap: ${holding.marketCap.toFixed(4)} SOL`);
        console.log(`   Buyers: ${holding.totalBuyers}`);
        console.log(`   Complete: ${holding.isComplete ? 'Yes' : 'No'}`);
        console.log();
      });
    }
    
    console.log('─'.repeat(50));
  }

  exportData(snapshot: PortfolioSnapshot) {
    try {
      const exportData = {
        timestamp: snapshot.timestamp,
        totalValue: snapshot.totalValue,
        holdings: snapshot.holdings,
        topPerformer: snapshot.topPerformer,
        bottomPerformer: snapshot.bottomPerformer,
      };
      
      fs.writeFileSync(this.config.exportPath, JSON.stringify(exportData, null, 2));
      console.log(`📄 Data exported to ${this.config.exportPath}`);
    } catch (error) {
      console.error('❌ Export failed:', error);
    }
  }

    console.log('� Starting continuous monitoring...');
    setInterval(async () => {
      await this.updatePortfolio();
    }, this.config.updateInterval);
  }

  stop() {
    console.log('🛑 Stopping portfolio analytics...');
    this.isRunning = false;
  }
}

// Usage Example
async function main() {
  // Replace with your actual keypair and configuration
  const analytics = new PortfolioAnalytics({
    walletAddress: "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU", // Your wallet address
    keypair: Keypair.generate(), // Your keypair for API access
    rpcUrl: "https://staging-rpc.dev2.eclipsenetwork.xyz",
    
    // Token mints to track
    tokenMints: [
      "9BSxAV9iRuiT3W7kwhFEkmzfoMo7xZTBdFGRF793JRbC",
      // Add more token mint addresses
    ],
    
    updateInterval: 30000, // Update every 30 seconds
    exportPath: './portfolio_analytics.json',
    
    alertThresholds: {
      gainPercent: 25,        // Alert on 25%+ gains
      lossPercent: -15,       // Alert on 15%+ losses
      marketCapChange: 100,   // Alert on 100%+ market cap changes
    },
  });
  
  await analytics.start();
  
  // Stop after 1 hour (for demo)
  setTimeout(() => {
    analytics.stop();
  }, 3600000);
}

// Handle graceful shutdown
process.on('SIGINT', () => {
  console.log('\\n� Shutting down analytics...');
  process.exit(0);
});

main().catch(console.error);
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `walletAddress` | String | Required | Wallet address to track holdings |
| `tokenMints` | Array | `[]` | List of token mint addresses to monitor |
| `updateInterval` | Number | `60000` | Update interval in milliseconds |
| `exportPath` | String | `'./portfolio_data.json'` | Path for data export |
| `alertThresholds.gainPercent` | Number | `20` | Alert threshold for gains (%) |
| `alertThresholds.lossPercent` | Number | `-10` | Alert threshold for losses (%) |
| `alertThresholds.marketCapChange` | Number | `50` | Market cap change alert threshold (%) |

## Features

- **Real-time Portfolio Tracking**: Uses actual SDK methods to fetch token balances and bonding curve data
- **Market Data Integration**: Displays market cap, buyer count, and curve completion status
- **Change Detection**: Tracks value changes between updates
- **Alert System**: Configurable alerts for significant changes
- **Data Export**: JSON export for further analysis
- **Performance Metrics**: Portfolio value tracking and top/bottom performers

## Running the Script

1. **Install dependencies**:
   ```bash
   npm install yoink-sdk @solana/web3.js @solana/spl-token @coral-xyz/anchor
   ```

2. **Configure your settings**:
   ```typescript
   const analytics = new PortfolioAnalytics({
     walletAddress: "YOUR_WALLET_ADDRESS",
     tokenMints: ["MINT1", "MINT2"], // Token mints to track
     updateInterval: 30000, // 30 seconds
   });
   ```

3. **Run the script**:
   ```bash
   npx ts-node portfolio-analytics.ts
   ```

## Example Output

```
🚀 YOINK PORTFOLIO ANALYTICS DASHBOARD
=====================================
Last Update: 11/4/2025, 2:30:15 PM

📊 PORTFOLIO OVERVIEW
---------------------
Total Value: 0.125430 SOL
Top Performer: HbiDw6U5... (0.085230 SOL)
Bottom Performer: 9WzDXwBb... (0.040200 SOL)

💼 CURRENT HOLDINGS
-------------------
🟢 HbiDw6U5...
   Balance: 150000.00 tokens
   Value: 0.085230 SOL (+12.30%)
   Price: 0.00000568 SOL/token
   Market Cap: 45.2341 SOL
   Buyers: 87
   Complete: false

🔴 9WzDXwBb...
   Balance: 75000.00 tokens
   Value: 0.040200 SOL (-5.20%)
   Price: 0.00000536 SOL/token
   Market Cap: 22.1204 SOL
   Buyers: 34
   Complete: false
```

## Next Steps

- [🤖 Build a Trading Bot](creator-token-bot.md)
- [📖 Read the full Usage Guide](usage.md)
- [📝 Try Example Script 1](example-1.md)
    
    // Export final data
    this.exportData();
  }

  sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// Usage Example
async function main() {
  const analytics = new PortfolioAnalytics({
    network: 'mainnet-beta',
    wallet: yourWalletAdapter,
    walletAddress: 'YOUR_WALLET_ADDRESS_HERE',
    updateInterval: 30000, // 30 seconds
    alertThresholds: {
      gainPercent: 15,  // Alert on 15% gains
      lossPercent: -8,  // Alert on 8% losses
    },
    exportPath: './portfolio-exports',
  });
  
  await analytics.start();
  
  // Export data every hour
  setInterval(() => {
    analytics.exportData();
  }, 3600000);
  
  // Graceful shutdown
  process.on('SIGINT', () => {
    analytics.stop();
    process.exit(0);
  });
}

// Run the analytics dashboard
main().catch(console.error);
```

## Features

### 📊 Real-time Tracking
- Live portfolio value updates
- Individual token performance
- Historical price data
- Volume tracking

### 📈 Analytics
- P&L calculations
- Performance percentages
- Best/worst performers
- ASCII performance charts

### 🔔 Smart Alerts
- Configurable gain/loss thresholds
- Volume change notifications
- Real-time alert system

### 📁 Data Export
- JSON data exports
- Historical performance data
- Portfolio snapshots
- Custom export paths

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `walletAddress` | String | Required | Wallet address to track |
| `updateInterval` | Number | `60000` | Update frequency (ms) |
| `alertThresholds.gainPercent` | Number | `20` | Gain alert threshold (%) |
| `alertThresholds.lossPercent` | Number | `-10` | Loss alert threshold (%) |
| `exportPath` | String | `'./portfolio-data'` | Data export directory |

## Running the Script

1. **Install dependencies**:
   ```bash
   npm install yoink-sdk @solana/web3.js
   ```

2. **Configure settings**:
   ```javascript
   const analytics = new PortfolioAnalytics({
     walletAddress: 'YOUR_WALLET_ADDRESS',
     alertThresholds: {
       gainPercent: 25,
       lossPercent: -15,
     }
   });
   ```

3. **Run the dashboard**:
   ```bash
   node portfolio-analytics.js
   ```

## Sample Output

```
🚀 YOINK PORTFOLIO ANALYTICS DASHBOARD
=====================================
Last Update: 11/4/2025, 2:30:15 PM

📊 PORTFOLIO OVERVIEW
---------------------
Total Value: 12.4567 SOL
Total Cost: 10.0000 SOL
Total P&L: +2.4567 SOL
Total P&L %: +24.57%

💼 CURRENT HOLDINGS
-------------------
🟢 CREATOR1
   Amount: 1000.00
   Price: 0.005000 SOL
   Value: 5.0000 SOL
   P&L: +1.0000 SOL (+25.00%)
   24h: +5.25%

🔴 CREATOR2
   Amount: 500.00
   Price: 0.003000 SOL
   Value: 1.5000 SOL
   P&L: -0.5000 SOL (-25.00%)
   24h: -3.45%
```

## Advanced Features

- **Custom Alerts**: Integrate with Discord/Telegram webhooks
- **Risk Analysis**: Calculate portfolio risk metrics
- **Correlation Analysis**: Track token correlations
- **Automated Rebalancing**: Set target allocations

## Next Steps

- [📦 Back to SDK Overview](overview.md)
- [📝 Try Example Script 1](example-1.md)
- [📖 Read the Usage Guide](usage.md)