# � Portfolio Analytics Dashboard

This example demonstrates how to build a comprehensive portfolio analytics dashboard that tracks your creator token holdings, performance metrics, and provides insights into your Yoink ecosystem investments.

## Overview

This dashboard will:
- Track all creator token holdings in real-time
- Monitor streamer performance and engagement metrics
- Calculate portfolio performance and P&L across all creator investments
- Generate analytics reports with creator-specific insights
- Send alerts for significant creator events or price movements
- Export data for tax reporting and further analysis

## Prerequisites

- Yoink SDK installed
- Wallet address to track
- Basic understanding of portfolio management

## Script Code

```javascript
import { YoinkSDK } from 'yoink-sdk';
import fs from 'fs';
import path from 'path';

class CreatorTokenPortfolio {
  constructor(config) {
    this.yoink = new YoinkSDK({
      network: config.network || 'mainnet-beta',
      wallet: config.wallet,
    });
    
    this.config = {
      walletAddress: config.walletAddress,
      updateInterval: config.updateInterval || 60000, // 1 minute
      alertThresholds: {
        gainPercent: config.alertThresholds?.gainPercent || 20,
        lossPercent: config.alertThresholds?.lossPercent || -10,
        viewerChange: config.alertThresholds?.viewerChange || 50, // Alert on 50% viewer change
        streamOffline: config.alertThresholds?.streamOffline || true,
      },
      exportPath: config.exportPath || './creator-portfolio-data',
    };
    
    this.portfolio = {
      creatorTokens: new Map(),
      totalValue: 0,
      totalCost: 0,
      performance: {
        totalGainLoss: 0,
        totalGainLossPercent: 0,
        bestPerformer24h: null,
        worstPerformer24h: null,
        topStreamerByViews: null,
      },
      streamerMetrics: new Map(),
      history: [],
    };
    
    this.isRunning = false;
    this.lastUpdate = null;
  }

  async start() {
    console.log('📊 Starting Portfolio Analytics Dashboard...');
    
    try {
      await this.yoink.connect();
      console.log('✅ Connected, tracking wallet:', this.config.walletAddress);
      
      // Initial portfolio load
      await this.updatePortfolio();
      
      this.isRunning = true;
      this.startMonitoring();
      
    } catch (error) {
      console.error('❌ Failed to start analytics:', error.message);
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
      
      // Get user profile and holdings
      const profile = await this.yoink.getUserProfile(this.config.walletAddress);
      const holdings = profile.tokensOwned || [];
      
      let totalValue = 0;
      let totalCost = 0;
      
      for (const holding of holdings) {
        const token = await this.yoink.getToken(holding.tokenAddress);
        const currentValue = holding.amount * token.price;
        
        // Get or create token entry
        let tokenData = this.portfolio.tokens.get(holding.tokenAddress) || {
          symbol: token.symbol,
          name: token.name,
          amount: holding.amount,
          costBasis: holding.costBasis || 0,
          firstPurchase: holding.firstPurchase || new Date(),
          priceHistory: [],
        };
        
        // Update token data
        tokenData.currentPrice = token.price;
        tokenData.currentValue = currentValue;
        tokenData.gainLoss = currentValue - tokenData.costBasis;
        tokenData.gainLossPercent = tokenData.costBasis > 0 
          ? (tokenData.gainLoss / tokenData.costBasis) * 100 
          : 0;
        tokenData.change24h = token.priceChange24h || 0;
        tokenData.volume24h = token.volume24h || 0;
        
        // Add to price history
        tokenData.priceHistory.push({
          timestamp: new Date(),
          price: token.price,
          volume: token.volume24h,
        });
        
        // Keep only last 100 price points
        if (tokenData.priceHistory.length > 100) {
          tokenData.priceHistory.shift();
        }
        
        this.portfolio.tokens.set(holding.tokenAddress, tokenData);
        
        totalValue += currentValue;
        totalCost += tokenData.costBasis;
      }
      
      // Update portfolio totals
      this.portfolio.totalValue = totalValue;
      this.portfolio.totalCost = totalCost;
      this.portfolio.performance.totalGainLoss = totalValue - totalCost;
      this.portfolio.performance.totalGainLossPercent = totalCost > 0 
        ? (this.portfolio.performance.totalGainLoss / totalCost) * 100 
        : 0;
      
      // Find best and worst performers
      this.findTopPerformers();
      
      // Add to history
      this.portfolio.history.push({
        timestamp: new Date(),
        totalValue,
        totalCost,
        gainLoss: this.portfolio.performance.totalGainLoss,
        gainLossPercent: this.portfolio.performance.totalGainLossPercent,
      });
      
      // Keep only last 1000 history points
      if (this.portfolio.history.length > 1000) {
        this.portfolio.history.shift();
      }
      
      this.lastUpdate = new Date();
      
    } catch (error) {
      console.error('❌ Error updating portfolio:', error.message);
    }
  }

  findTopPerformers() {
    const tokens = Array.from(this.portfolio.tokens.values());
    
    // Best 24h performer
    this.portfolio.performance.best24h = tokens.reduce((best, token) => {
      return (!best || token.change24h > best.change24h) ? token : best;
    }, null);
    
    // Worst 24h performer
    this.portfolio.performance.worst24h = tokens.reduce((worst, token) => {
      return (!worst || token.change24h < worst.change24h) ? token : worst;
    }, null);
  }

  async generateReport() {
    console.clear();
    console.log('🚀 YOINK PORTFOLIO ANALYTICS DASHBOARD');
    console.log('=====================================');
    console.log(`Last Update: ${this.lastUpdate?.toLocaleString() || 'Never'}`);
    console.log();
    
    // Portfolio Overview
    console.log('📊 PORTFOLIO OVERVIEW');
    console.log('---------------------');
    console.log(`Total Value: ${this.portfolio.totalValue.toFixed(4)} SOL`);
    console.log(`Total Cost: ${this.portfolio.totalCost.toFixed(4)} SOL`);
    console.log(`Total P&L: ${this.portfolio.performance.totalGainLoss > 0 ? '+' : ''}${this.portfolio.performance.totalGainLoss.toFixed(4)} SOL`);
    console.log(`Total P&L %: ${this.portfolio.performance.totalGainLossPercent > 0 ? '+' : ''}${this.portfolio.performance.totalGainLossPercent.toFixed(2)}%`);
    console.log();
    
    // Holdings
    console.log('💼 CURRENT HOLDINGS');
    console.log('-------------------');
    if (this.portfolio.tokens.size === 0) {
      console.log('No tokens in portfolio');
    } else {
      this.portfolio.tokens.forEach(token => {
        const pnlColor = token.gainLoss >= 0 ? '🟢' : '🔴';
        console.log(`${pnlColor} ${token.symbol}`);
        console.log(`   Amount: ${token.amount.toFixed(2)}`);
        console.log(`   Price: ${token.currentPrice.toFixed(6)} SOL`);
        console.log(`   Value: ${token.currentValue.toFixed(4)} SOL`);
        console.log(`   P&L: ${token.gainLoss > 0 ? '+' : ''}${token.gainLoss.toFixed(4)} SOL (${token.gainLossPercent > 0 ? '+' : ''}${token.gainLossPercent.toFixed(2)}%)`);
        console.log(`   24h: ${token.change24h > 0 ? '+' : ''}${token.change24h.toFixed(2)}%`);
        console.log();
      });
    }
    
    // Top Performers
    if (this.portfolio.performance.best24h) {
      console.log('🏆 TOP PERFORMERS (24H)');
      console.log('-----------------------');
      console.log(`Best: ${this.portfolio.performance.best24h.symbol} (+${this.portfolio.performance.best24h.change24h.toFixed(2)}%)`);
      console.log(`Worst: ${this.portfolio.performance.worst24h.symbol} (${this.portfolio.performance.worst24h.change24h.toFixed(2)}%)`);
      console.log();
    }
    
    // Performance Chart (ASCII)
    this.drawPerformanceChart();
  }

  drawPerformanceChart() {
    if (this.portfolio.history.length < 2) return;
    
    console.log('📈 PERFORMANCE CHART (Last 20 Updates)');
    console.log('---------------------------------------');
    
    const recentHistory = this.portfolio.history.slice(-20);
    const values = recentHistory.map(h => h.gainLossPercent);
    const min = Math.min(...values);
    const max = Math.max(...values);
    const range = max - min || 1;
    
    recentHistory.forEach((point, index) => {
      const normalized = ((point.gainLossPercent - min) / range) * 20;
      const bar = '█'.repeat(Math.max(1, Math.floor(normalized)));
      const time = point.timestamp.toLocaleTimeString().slice(0, 5);
      console.log(`${time} |${bar} ${point.gainLossPercent.toFixed(2)}%`);
    });
    console.log();
  }

  async checkAlerts() {
    try {
      this.portfolio.tokens.forEach(token => {
        const { gainPercent, lossPercent } = this.config.alertThresholds;
        
        // Gain alert
        if (token.gainLossPercent >= gainPercent) {
          this.sendAlert(`🚀 ${token.symbol} is up ${token.gainLossPercent.toFixed(2)}%! Current value: ${token.currentValue.toFixed(4)} SOL`);
        }
        
        // Loss alert
        if (token.gainLossPercent <= lossPercent) {
          this.sendAlert(`⚠️ ${token.symbol} is down ${Math.abs(token.gainLossPercent).toFixed(2)}%. Current value: ${token.currentValue.toFixed(4)} SOL`);
        }
      });
    } catch (error) {
      console.error('❌ Error checking alerts:', error.message);
    }
  }

  sendAlert(message) {
    console.log(`🔔 ALERT: ${message}`);
    // Here you could integrate with Discord, Telegram, email, etc.
  }

  async exportData() {
    try {
      const exportData = {
        timestamp: new Date().toISOString(),
        portfolio: {
          totalValue: this.portfolio.totalValue,
          totalCost: this.portfolio.totalCost,
          performance: this.portfolio.performance,
        },
        holdings: Array.from(this.portfolio.tokens.entries()).map(([address, data]) => ({
          address,
          ...data,
        })),
        history: this.portfolio.history,
      };
      
      const filename = `portfolio-${new Date().toISOString().split('T')[0]}.json`;
      const filepath = path.join(this.config.exportPath, filename);
      
      // Ensure directory exists
      if (!fs.existsSync(this.config.exportPath)) {
        fs.mkdirSync(this.config.exportPath, { recursive: true });
      }
      
      fs.writeFileSync(filepath, JSON.stringify(exportData, null, 2));
      console.log(`📁 Data exported to: ${filepath}`);
      
    } catch (error) {
      console.error('❌ Export failed:', error.message);
    }
  }

  stop() {
    console.log('🛑 Stopping portfolio analytics...');
    this.isRunning = false;
    
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
- [🤖 Try Creator Token Bot](creator-token-bot.md)
- [🎥 Stream Monitor & Token Launcher](stream-monitor.md)
- [📈 Creator Analytics & Insights](creator-analytics.md)
- [📖 Read the Usage Guide](usage.md)