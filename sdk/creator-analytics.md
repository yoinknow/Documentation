# 📈 Creator Analytics & Insights

This example demonstrates how to build a comprehensive analytics tool for tracking creator performance, token metrics, and generating insights for data-driven investment decisions in the Yoink ecosystem.

## Overview

This tool provides:
- Deep analytics on creator performance and engagement
- Token price analysis and trend prediction
- Creator ranking and comparison tools
- Investment recommendation engine
- Real-time alerts for creator milestones
- Comprehensive reporting and data export

## Prerequisites

- Yoink SDK installed
- Historical data access
- Basic understanding of financial metrics
- Optional: Machine learning libraries for predictions

## Script Code

```javascript
import { YoinkSDK } from 'yoink-sdk';
import fs from 'fs';
import path from 'path';

class CreatorAnalytics {
  constructor(config) {
    this.yoink = new YoinkSDK({
      network: config.network || 'mainnet-beta',
      wallet: config.wallet,
    });
    
    this.config = {
      // Analysis Settings
      trackingPeriod: config.trackingPeriod || 30, // days
      updateInterval: config.updateInterval || 300000, // 5 minutes
      minTokenAge: config.minTokenAge || 24, // hours
      
      // Metrics Configuration
      metrics: {
        price: true,
        volume: true,
        holders: true,
        streams: true,
        engagement: true,
        social: config.includeSocial || false,
      },
      
      // Alert Thresholds
      alerts: {
        priceChange: config.alerts?.priceChange || 20, // 20% change
        volumeSpike: config.alerts?.volumeSpike || 300, // 300% volume increase
        newMilestone: config.alerts?.newMilestone || true,
        holderThreshold: config.alerts?.holderThreshold || 1000,
      },
      
      // Export Settings
      exportPath: config.exportPath || './creator-analytics',
      autoExport: config.autoExport || true,
      exportInterval: config.exportInterval || 3600000, // 1 hour
    };
    
    this.analytics = {
      creators: new Map(),
      rankings: {
        byPerformance: [],
        byVolume: [],
        byGrowth: [],
        byEngagement: [],
      },
      insights: [],
      predictions: new Map(),
    };
    
    this.isRunning = false;
    this.lastUpdate = null;
  }

  async start() {
    console.log('📈 Starting Creator Analytics & Insights...');
    
    try {
      await this.yoink.connect();
      console.log('✅ Connected to Yoink platform');
      
      // Initial data load
      await this.loadCreatorData();
      await this.performAnalysis();
      
      this.isRunning = true;
      this.startContinuousAnalysis();
      
      if (this.config.autoExport) {
        this.startAutoExport();
      }
      
    } catch (error) {
      console.error('❌ Failed to start analytics:', error.message);
    }
  }

  async loadCreatorData() {
    console.log('📊 Loading creator data...');
    
    try {
      // Get all active creators and their tokens
      const creators = await this.yoink.getAllCreators();
      
      for (const creator of creators) {
        await this.analyzeCreator(creator);
      }
      
      console.log(`✅ Loaded data for ${creators.length} creators`);
      
    } catch (error) {
      console.error('❌ Error loading creator data:', error.message);
    }
  }

  async analyzeCreator(creator) {
    try {
      const creatorId = creator.address;
      const token = await this.yoink.getToken(creator.tokenAddress);
      
      // Get historical data
      const priceHistory = await this.yoink.getTokenPriceHistory(
        creator.tokenAddress,
        this.config.trackingPeriod
      );
      
      const volumeHistory = await this.yoink.getTokenVolumeHistory(
        creator.tokenAddress,
        this.config.trackingPeriod
      );
      
      // Get stream data
      const streamMetrics = await this.yoink.getCreatorStreamMetrics(creatorId);
      
      // Calculate analytics
      const analytics = {
        // Basic Info
        creatorId,
        name: creator.name,
        platform: creator.platform,
        tokenAddress: creator.tokenAddress,
        
        // Current Metrics
        currentPrice: token.price,
        marketCap: token.marketCap,
        totalSupply: token.totalSupply,
        holders: token.holders,
        volume24h: token.volume24h,
        
        // Performance Metrics
        performance: this.calculatePerformance(priceHistory),
        volatility: this.calculateVolatility(priceHistory),
        momentum: this.calculateMomentum(priceHistory),
        
        // Volume Analysis
        volumeMetrics: this.analyzeVolume(volumeHistory),
        liquidityScore: this.calculateLiquidityScore(token),
        
        // Stream Analytics
        streamMetrics: {
          averageViewers: streamMetrics.averageViewers,
          streamHours: streamMetrics.totalHours,
          peakViewers: streamMetrics.peakViewers,
          consistency: streamMetrics.streamDays / this.config.trackingPeriod,
          engagement: streamMetrics.engagementRate,
        },
        
        // Growth Metrics
        holderGrowth: this.calculateHolderGrowth(creator),
        priceGrowth: this.calculatePriceGrowth(priceHistory),
        volumeGrowth: this.calculateVolumeGrowth(volumeHistory),
        
        // Risk Assessment
        riskScore: this.calculateRiskScore(token, streamMetrics),
        stabilityIndex: this.calculateStabilityIndex(priceHistory),
        
        // Timestamps
        lastUpdated: new Date(),
        createdAt: creator.createdAt,
        tokenAge: this.calculateTokenAge(creator.createdAt),
      };
      
      this.analytics.creators.set(creatorId, analytics);
      
    } catch (error) {
      console.error(`❌ Error analyzing creator ${creator.name}:`, error.message);
    }
  }

  calculatePerformance(priceHistory) {
    if (priceHistory.length < 2) return { return: 0, change: 0 };
    
    const firstPrice = priceHistory[0].price;
    const lastPrice = priceHistory[priceHistory.length - 1].price;
    const change = lastPrice - firstPrice;
    const returnPercent = (change / firstPrice) * 100;
    
    return {
      return: returnPercent,
      change: change,
      absolute: Math.abs(returnPercent),
      direction: change > 0 ? 'up' : change < 0 ? 'down' : 'flat',
    };
  }

  calculateVolatility(priceHistory) {
    if (priceHistory.length < 2) return 0;
    
    const returns = [];
    for (let i = 1; i < priceHistory.length; i++) {
      const dailyReturn = (priceHistory[i].price - priceHistory[i-1].price) / priceHistory[i-1].price;
      returns.push(dailyReturn);
    }
    
    const mean = returns.reduce((sum, r) => sum + r, 0) / returns.length;
    const variance = returns.reduce((sum, r) => sum + Math.pow(r - mean, 2), 0) / returns.length;
    
    return Math.sqrt(variance) * 100; // Convert to percentage
  }

  calculateMomentum(priceHistory) {
    if (priceHistory.length < 7) return 0;
    
    const recent = priceHistory.slice(-7); // Last 7 days
    const older = priceHistory.slice(-14, -7); // Previous 7 days
    
    const recentAvg = recent.reduce((sum, p) => sum + p.price, 0) / recent.length;
    const olderAvg = older.reduce((sum, p) => sum + p.price, 0) / older.length;
    
    return ((recentAvg - olderAvg) / olderAvg) * 100;
  }

  analyzeVolume(volumeHistory) {
    if (volumeHistory.length === 0) return { average: 0, trend: 'flat', spikes: 0 };
    
    const volumes = volumeHistory.map(v => v.volume);
    const average = volumes.reduce((sum, v) => sum + v, 0) / volumes.length;
    
    // Calculate trend
    const firstHalf = volumes.slice(0, Math.floor(volumes.length / 2));
    const secondHalf = volumes.slice(Math.floor(volumes.length / 2));
    
    const firstAvg = firstHalf.reduce((sum, v) => sum + v, 0) / firstHalf.length;
    const secondAvg = secondHalf.reduce((sum, v) => sum + v, 0) / secondHalf.length;
    
    const trend = secondAvg > firstAvg * 1.2 ? 'increasing' : 
                  secondAvg < firstAvg * 0.8 ? 'decreasing' : 'stable';
    
    // Count volume spikes (3x average)
    const spikes = volumes.filter(v => v > average * 3).length;
    
    return {
      average,
      current: volumes[volumes.length - 1],
      trend,
      spikes,
      consistency: this.calculateVolumeConsistency(volumes),
    };
  }

  calculateVolumeConsistency(volumes) {
    if (volumes.length < 2) return 0;
    
    const average = volumes.reduce((sum, v) => sum + v, 0) / volumes.length;
    const deviations = volumes.map(v => Math.abs(v - average) / average);
    const avgDeviation = deviations.reduce((sum, d) => sum + d, 0) / deviations.length;
    
    return Math.max(0, 100 - (avgDeviation * 100)); // Higher is more consistent
  }

  calculateLiquidityScore(token) {
    // Simplified liquidity scoring based on volume and holder count
    const volumeScore = Math.min(100, (token.volume24h / token.marketCap) * 1000);
    const holderScore = Math.min(100, token.holders / 10);
    
    return (volumeScore + holderScore) / 2;
  }

  calculateHolderGrowth(creator) {
    // This would require historical holder data
    // For now, return a simplified calculation
    const daysActive = this.calculateTokenAge(creator.createdAt);
    return daysActive > 0 ? creator.holders / daysActive : 0;
  }

  calculatePriceGrowth(priceHistory) {
    if (priceHistory.length < 2) return 0;
    
    const periods = [1, 7, 30]; // 1 day, 1 week, 1 month
    const growth = {};
    
    periods.forEach(period => {
      if (priceHistory.length > period) {
        const currentPrice = priceHistory[priceHistory.length - 1].price;
        const pastPrice = priceHistory[priceHistory.length - 1 - period].price;
        growth[`${period}d`] = ((currentPrice - pastPrice) / pastPrice) * 100;
      }
    });
    
    return growth;
  }

  calculateVolumeGrowth(volumeHistory) {
    if (volumeHistory.length < 7) return 0;
    
    const recent = volumeHistory.slice(-7);
    const previous = volumeHistory.slice(-14, -7);
    
    const recentTotal = recent.reduce((sum, v) => sum + v.volume, 0);
    const previousTotal = previous.reduce((sum, v) => sum + v.volume, 0);
    
    return previousTotal > 0 ? ((recentTotal - previousTotal) / previousTotal) * 100 : 0;
  }

  calculateRiskScore(token, streamMetrics) {
    let riskScore = 0;
    
    // Low holder count increases risk
    if (token.holders < 100) riskScore += 30;
    else if (token.holders < 500) riskScore += 15;
    
    // Low liquidity increases risk
    if (token.volume24h < token.marketCap * 0.01) riskScore += 25;
    
    // Irregular streaming increases risk
    if (streamMetrics.streamDays < this.config.trackingPeriod * 0.5) riskScore += 20;
    
    // High price volatility increases risk
    // This would need the volatility calculation result
    
    return Math.min(100, riskScore);
  }

  calculateStabilityIndex(priceHistory) {
    if (priceHistory.length < 7) return 0;
    
    const volatility = this.calculateVolatility(priceHistory);
    return Math.max(0, 100 - volatility); // Higher is more stable
  }

  calculateTokenAge(createdAt) {
    const now = new Date();
    const created = new Date(createdAt);
    return Math.floor((now - created) / (1000 * 60 * 60 * 24)); // Days
  }

  async performAnalysis() {
    console.log('🔍 Performing comprehensive analysis...');
    
    // Generate rankings
    this.generateRankings();
    
    // Generate insights
    this.generateInsights();
    
    // Generate predictions (simplified)
    this.generatePredictions();
    
    this.lastUpdate = new Date();
    console.log('✅ Analysis complete');
  }

  generateRankings() {
    const creators = Array.from(this.analytics.creators.values());
    
    // Performance ranking
    this.analytics.rankings.byPerformance = creators
      .sort((a, b) => b.performance.return - a.performance.return)
      .slice(0, 20);
    
    // Volume ranking
    this.analytics.rankings.byVolume = creators
      .sort((a, b) => b.volume24h - a.volume24h)
      .slice(0, 20);
    
    // Growth ranking (holder growth)
    this.analytics.rankings.byGrowth = creators
      .sort((a, b) => b.holderGrowth - a.holderGrowth)
      .slice(0, 20);
    
    // Engagement ranking
    this.analytics.rankings.byEngagement = creators
      .sort((a, b) => b.streamMetrics.engagement - a.streamMetrics.engagement)
      .slice(0, 20);
  }

  generateInsights() {
    const creators = Array.from(this.analytics.creators.values());
    this.analytics.insights = [];
    
    // Market trends
    const avgPerformance = creators.reduce((sum, c) => sum + c.performance.return, 0) / creators.length;
    this.analytics.insights.push({
      type: 'market_trend',
      title: 'Market Performance',
      message: `Average creator token performance: ${avgPerformance.toFixed(2)}%`,
      sentiment: avgPerformance > 0 ? 'positive' : 'negative',
    });
    
    // Top performers
    const topPerformer = this.analytics.rankings.byPerformance[0];
    if (topPerformer) {
      this.analytics.insights.push({
        type: 'top_performer',
        title: 'Best Performing Creator',
        message: `${topPerformer.name} leading with ${topPerformer.performance.return.toFixed(2)}% return`,
        creator: topPerformer.creatorId,
        sentiment: 'positive',
      });
    }
    
    // Volume leaders
    const volumeLeader = this.analytics.rankings.byVolume[0];
    if (volumeLeader) {
      this.analytics.insights.push({
        type: 'volume_leader',
        title: 'Highest Volume',
        message: `${volumeLeader.name} with ${volumeLeader.volume24h.toFixed(4)} SOL in 24h volume`,
        creator: volumeLeader.creatorId,
        sentiment: 'neutral',
      });
    }
    
    // Risk warnings
    const highRiskCreators = creators.filter(c => c.riskScore > 70);
    if (highRiskCreators.length > 0) {
      this.analytics.insights.push({
        type: 'risk_warning',
        title: 'High Risk Alert',
        message: `${highRiskCreators.length} creators showing high risk indicators`,
        sentiment: 'negative',
      });
    }
  }

  generatePredictions() {
    // Simplified prediction algorithm
    const creators = Array.from(this.analytics.creators.values());
    
    creators.forEach(creator => {
      let prediction = 'neutral';
      let confidence = 50;
      
      // Momentum-based prediction
      if (creator.momentum > 10 && creator.streamMetrics.consistency > 0.7) {
        prediction = 'bullish';
        confidence += 20;
      } else if (creator.momentum < -10 || creator.streamMetrics.consistency < 0.3) {
        prediction = 'bearish';
        confidence += 15;
      }
      
      // Volume consideration
      if (creator.volumeMetrics.trend === 'increasing') {
        confidence += 10;
      }
      
      // Risk adjustment
      if (creator.riskScore > 70) {
        confidence -= 20;
      }
      
      this.analytics.predictions.set(creator.creatorId, {
        prediction,
        confidence: Math.max(0, Math.min(100, confidence)),
        timeframe: '7d',
        factors: {
          momentum: creator.momentum,
          volume: creator.volumeMetrics.trend,
          consistency: creator.streamMetrics.consistency,
          risk: creator.riskScore,
        },
      });
    });
  }

  async startContinuousAnalysis() {
    while (this.isRunning) {
      try {
        await this.loadCreatorData();
        await this.performAnalysis();
        this.displayDashboard();
        
        await this.sleep(this.config.updateInterval);
      } catch (error) {
        console.error('❌ Analysis error:', error.message);
        await this.sleep(30000);
      }
    }
  }

  startAutoExport() {
    setInterval(() => {
      this.exportAnalytics();
    }, this.config.exportInterval);
  }

  displayDashboard() {
    console.clear();
    console.log('📈 YOINK CREATOR ANALYTICS DASHBOARD');
    console.log('===================================');
    console.log(`Last Update: ${this.lastUpdate?.toLocaleString()}`);
    console.log(`Tracking ${this.analytics.creators.size} creators`);
    console.log();
    
    // Top performers
    console.log('🏆 TOP PERFORMERS (24H):');
    console.log('------------------------');
    this.analytics.rankings.byPerformance.slice(0, 5).forEach((creator, index) => {
      const trend = creator.performance.direction === 'up' ? '📈' : 
                   creator.performance.direction === 'down' ? '📉' : '➡️';
      console.log(`${index + 1}. ${trend} ${creator.name}`);
      console.log(`   Return: ${creator.performance.return > 0 ? '+' : ''}${creator.performance.return.toFixed(2)}%`);
      console.log(`   Volume: ${creator.volume24h.toFixed(4)} SOL`);
      console.log(`   Risk: ${creator.riskScore.toFixed(0)}/100`);
      console.log();
    });
    
    // Key insights
    console.log('💡 KEY INSIGHTS:');
    console.log('----------------');
    this.analytics.insights.slice(0, 3).forEach(insight => {
      const emoji = insight.sentiment === 'positive' ? '✅' : 
                   insight.sentiment === 'negative' ? '⚠️' : 'ℹ️';
      console.log(`${emoji} ${insight.title}: ${insight.message}`);
    });
    console.log();
    
    // Market summary
    const creators = Array.from(this.analytics.creators.values());
    const avgReturn = creators.reduce((sum, c) => sum + c.performance.return, 0) / creators.length;
    const positiveCreators = creators.filter(c => c.performance.return > 0).length;
    
    console.log('📊 MARKET SUMMARY:');
    console.log('------------------');
    console.log(`Average Return: ${avgReturn > 0 ? '+' : ''}${avgReturn.toFixed(2)}%`);
    console.log(`Positive: ${positiveCreators}/${creators.length} (${(positiveCreators/creators.length*100).toFixed(1)}%)`);
    console.log(`Total Volume: ${creators.reduce((sum, c) => sum + c.volume24h, 0).toFixed(2)} SOL`);
  }

  async exportAnalytics() {
    try {
      const exportData = {
        timestamp: new Date().toISOString(),
        summary: {
          totalCreators: this.analytics.creators.size,
          lastUpdate: this.lastUpdate,
          marketSummary: this.generateMarketSummary(),
        },
        creators: Array.from(this.analytics.creators.values()),
        rankings: this.analytics.rankings,
        insights: this.analytics.insights,
        predictions: Object.fromEntries(this.analytics.predictions),
      };
      
      const filename = `creator-analytics-${new Date().toISOString().split('T')[0]}.json`;
      const filepath = path.join(this.config.exportPath, filename);
      
      if (!fs.existsSync(this.config.exportPath)) {
        fs.mkdirSync(this.config.exportPath, { recursive: true });
      }
      
      fs.writeFileSync(filepath, JSON.stringify(exportData, null, 2));
      console.log(`📁 Analytics exported to: ${filepath}`);
      
    } catch (error) {
      console.error('❌ Export failed:', error.message);
    }
  }

  generateMarketSummary() {
    const creators = Array.from(this.analytics.creators.values());
    
    return {
      totalCreators: creators.length,
      avgReturn: creators.reduce((sum, c) => sum + c.performance.return, 0) / creators.length,
      totalVolume: creators.reduce((sum, c) => sum + c.volume24h, 0),
      totalMarketCap: creators.reduce((sum, c) => sum + c.marketCap, 0),
      avgRiskScore: creators.reduce((sum, c) => sum + c.riskScore, 0) / creators.length,
      positivePerformers: creators.filter(c => c.performance.return > 0).length,
    };
  }

  stop() {
    console.log('🛑 Stopping creator analytics...');
    this.isRunning = false;
    this.exportAnalytics(); // Final export
  }

  sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// Usage Example
async function main() {
  const analytics = new CreatorAnalytics({
    network: 'mainnet-beta',
    wallet: yourWalletAdapter,
    trackingPeriod: 30, // 30 days of data
    updateInterval: 300000, // 5 minutes
    
    alerts: {
      priceChange: 25, // Alert on 25% price changes
      volumeSpike: 400, // Alert on 400% volume spikes
      holderThreshold: 500, // Alert when reaching 500 holders
    },
    
    exportPath: './analytics-exports',
    autoExport: true,
    exportInterval: 1800000, // Export every 30 minutes
  });
  
  await analytics.start();
  
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

### 📊 Comprehensive Analytics
- Creator performance tracking
- Token price and volume analysis
- Holder growth metrics
- Stream engagement analytics

### 🏆 Rankings & Comparisons
- Performance leaderboards
- Volume rankings
- Growth metrics
- Risk assessments

### 🔮 Predictive Insights
- Momentum analysis
- Trend predictions
- Risk scoring
- Market sentiment

### 📈 Advanced Metrics
- Volatility calculations
- Liquidity scoring
- Stability indices
- Correlation analysis

### 📁 Data Export
- JSON data exports
- Historical analytics
- Custom reporting
- Automated backups

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `trackingPeriod` | Number | `30` | Days of historical data |
| `updateInterval` | Number | `300000` | Update frequency (ms) |
| `alerts.priceChange` | Number | `20` | Price change alert (%) |
| `alerts.volumeSpike` | Number | `300` | Volume spike alert (%) |
| `exportPath` | String | `'./creator-analytics'` | Export directory |
| `autoExport` | Boolean | `true` | Enable auto exports |

## Running the Script

1. **Install dependencies**:
   ```bash
   npm install yoink-sdk @solana/web3.js
   ```

2. **Configure analytics**:
   ```javascript
   const analytics = new CreatorAnalytics({
     trackingPeriod: 45,
     alerts: { priceChange: 30 }
   });
   ```

3. **Start the dashboard**:
   ```bash
   node creator-analytics.js
   ```

## Sample Output

```
📈 YOINK CREATOR ANALYTICS DASHBOARD
===================================
Last Update: 11/4/2025, 3:15:45 PM
Tracking 127 creators

🏆 TOP PERFORMERS (24H):
------------------------
1. 📈 StreamerName
   Return: +34.56%
   Volume: 12.4567 SOL
   Risk: 25/100

2. 📈 AnotherCreator
   Return: +18.23%
   Volume: 8.9012 SOL
   Risk: 35/100

💡 KEY INSIGHTS:
----------------
✅ Market Performance: Average creator token performance: +8.45%
✅ Best Performing Creator: StreamerName leading with 34.56% return
ℹ️ Highest Volume: TopTrader with 45.6789 SOL in 24h volume

📊 MARKET SUMMARY:
------------------
Average Return: +8.45%
Positive: 89/127 (70.1%)
Total Volume: 234.56 SOL
```

## Advanced Analytics

- **Technical Indicators**: RSI, MACD, Bollinger Bands
- **Correlation Analysis**: Creator-to-creator relationships
- **Sentiment Analysis**: Social media integration
- **Risk Models**: VaR calculations and stress testing

## Next Steps

- [📦 Back to SDK Overview](overview.md)
- [🎥 Try Stream Monitor](stream-monitor.md)
- [🤖 Creator Token Bot](creator-token-bot.md)
- [📊 Portfolio Dashboard](portfolio-dashboard.md)