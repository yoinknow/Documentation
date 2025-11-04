# 🎥 Stream Monitor & Token Launcher

This example shows how to create a comprehensive stream monitoring system that tracks live streams and can automatically launch creator tokens when certain conditions are met.

## Overview

This system will:
- Monitor live streams across platforms (Twitch, YouTube, etc.)
- Track viewer engagement and stream metrics
- Automatically launch creator tokens for eligible streamers
- Send notifications for important stream events
- Manage token metadata and initial distribution

## Prerequisites

- Yoink SDK installed
- Stream platform API keys (Twitch, YouTube)
- Wallet with SOL for token creation fees
- Understanding of creator token economics

## Script Code

```javascript
import { YoinkSDK } from 'yoink-sdk';
import axios from 'axios';

class StreamMonitorAndLauncher {
  constructor(config) {
    this.yoink = new YoinkSDK({
      network: config.network || 'mainnet-beta',
      wallet: config.wallet,
    });
    
    this.config = {
      // Stream Monitoring
      twitchClientId: config.twitchClientId,
      twitchClientSecret: config.twitchClientSecret,
      youtubeApiKey: config.youtubeApiKey,
      
      // Auto-Launch Criteria
      minViewersForLaunch: config.minViewersForLaunch || 100,
      minFollowersForLaunch: config.minFollowersForLaunch || 1000,
      requireVerification: config.requireVerification || true,
      
      // Token Parameters
      initialSupply: config.initialSupply || 1000000,
      initialPrice: config.initialPrice || 0.001, // SOL
      creatorShare: config.creatorShare || 10, // 10% to creator
      
      // Monitoring Settings
      checkInterval: config.checkInterval || 30000, // 30 seconds
      platforms: config.platforms || ['twitch', 'youtube'],
    };
    
    this.activeStreams = new Map();
    this.launchedTokens = new Map();
    this.streamHistory = [];
    this.isRunning = false;
    
    // Platform API instances
    this.twitchToken = null;
    this.apis = {};
  }

  async start() {
    console.log('🎥 Starting Stream Monitor & Token Launcher...');
    
    try {
      await this.yoink.connect();
      await this.initializePlatformAPIs();
      
      console.log('✅ Connected and APIs initialized');
      
      this.isRunning = true;
      this.startMonitoring();
      
    } catch (error) {
      console.error('❌ Failed to start stream monitor:', error.message);
    }
  }

  async initializePlatformAPIs() {
    // Initialize Twitch API
    if (this.config.platforms.includes('twitch')) {
      await this.initializeTwitchAPI();
    }
    
    // Initialize YouTube API
    if (this.config.platforms.includes('youtube')) {
      this.apis.youtube = {
        key: this.config.youtubeApiKey,
        baseURL: 'https://www.googleapis.com/youtube/v3',
      };
    }
  }

  async initializeTwitchAPI() {
    try {
      const response = await axios.post('https://id.twitch.tv/oauth2/token', {
        client_id: this.config.twitchClientId,
        client_secret: this.config.twitchClientSecret,
        grant_type: 'client_credentials',
      });
      
      this.twitchToken = response.data.access_token;
      this.apis.twitch = {
        token: this.twitchToken,
        baseURL: 'https://api.twitch.tv/helix',
        headers: {
          'Client-ID': this.config.twitchClientId,
          'Authorization': `Bearer ${this.twitchToken}`,
        },
      };
      
      console.log('✅ Twitch API initialized');
    } catch (error) {
      console.error('❌ Failed to initialize Twitch API:', error.message);
    }
  }

  async startMonitoring() {
    while (this.isRunning) {
      try {
        await this.checkActiveStreams();
        await this.evaluateTokenLaunches();
        await this.updateStreamMetrics();
        
        await this.sleep(this.config.checkInterval);
      } catch (error) {
        console.error('❌ Monitoring error:', error.message);
        await this.sleep(5000);
      }
    }
  }

  async checkActiveStreams() {
    // Check Twitch streams
    if (this.config.platforms.includes('twitch')) {
      await this.checkTwitchStreams();
    }
    
    // Check YouTube streams
    if (this.config.platforms.includes('youtube')) {
      await this.checkYouTubeStreams();
    }
  }

  async checkTwitchStreams() {
    try {
      const response = await axios.get(`${this.apis.twitch.baseURL}/streams`, {
        headers: this.apis.twitch.headers,
        params: {
          first: 100,
          game_id: '', // Can filter by specific games
        },
      });
      
      for (const stream of response.data.data) {
        await this.processStream({
          platform: 'twitch',
          streamerId: stream.user_id,
          streamerName: stream.user_name,
          title: stream.title,
          game: stream.game_name,
          viewers: stream.viewer_count,
          isLive: true,
          startedAt: new Date(stream.started_at),
          thumbnailUrl: stream.thumbnail_url,
        });
      }
      
    } catch (error) {
      console.error('❌ Error checking Twitch streams:', error.message);
    }
  }

  async checkYouTubeStreams() {
    try {
      // YouTube Live Stream API call
      const response = await axios.get(`${this.apis.youtube.baseURL}/search`, {
        params: {
          part: 'snippet',
          eventType: 'live',
          type: 'video',
          maxResults: 50,
          key: this.apis.youtube.key,
        },
      });
      
      for (const video of response.data.items) {
        // Get additional stream details
        const streamDetails = await this.getYouTubeStreamDetails(video.id.videoId);
        
        await this.processStream({
          platform: 'youtube',
          streamerId: video.snippet.channelId,
          streamerName: video.snippet.channelTitle,
          title: video.snippet.title,
          viewers: streamDetails.viewers,
          isLive: true,
          startedAt: new Date(video.snippet.publishedAt),
          thumbnailUrl: video.snippet.thumbnails.medium.url,
        });
      }
      
    } catch (error) {
      console.error('❌ Error checking YouTube streams:', error.message);
    }
  }

  async getYouTubeStreamDetails(videoId) {
    try {
      const response = await axios.get(`${this.apis.youtube.baseURL}/videos`, {
        params: {
          part: 'liveStreamingDetails,statistics',
          id: videoId,
          key: this.apis.youtube.key,
        },
      });
      
      const video = response.data.items[0];
      return {
        viewers: parseInt(video.liveStreamingDetails?.concurrentViewers || 0),
        startTime: video.liveStreamingDetails?.actualStartTime,
      };
    } catch (error) {
      return { viewers: 0 };
    }
  }

  async processStream(streamData) {
    const streamKey = `${streamData.platform}-${streamData.streamerId}`;
    
    // Update active streams
    const existingStream = this.activeStreams.get(streamKey);
    if (existingStream) {
      existingStream.viewers = streamData.viewers;
      existingStream.lastUpdate = new Date();
    } else {
      this.activeStreams.set(streamKey, {
        ...streamData,
        firstSeen: new Date(),
        lastUpdate: new Date(),
        peakViewers: streamData.viewers,
        hasToken: false,
      });
    }
    
    // Update peak viewers
    const stream = this.activeStreams.get(streamKey);
    if (streamData.viewers > stream.peakViewers) {
      stream.peakViewers = streamData.viewers;
    }
    
    // Add to history
    this.streamHistory.push({
      timestamp: new Date(),
      ...streamData,
    });
    
    // Keep only last 1000 entries
    if (this.streamHistory.length > 1000) {
      this.streamHistory.shift();
    }
  }

  async evaluateTokenLaunches() {
    for (const [streamKey, stream] of this.activeStreams.entries()) {
      if (stream.hasToken) continue;
      
      // Check if stream meets launch criteria
      const meetsViewerThreshold = stream.viewers >= this.config.minViewersForLaunch;
      const meetsPeakViewers = stream.peakViewers >= this.config.minViewersForLaunch;
      
      if (meetsViewerThreshold || meetsPeakViewers) {
        // Get additional streamer info
        const streamerInfo = await this.getStreamerInfo(stream);
        
        if (this.shouldLaunchToken(stream, streamerInfo)) {
          await this.launchCreatorToken(stream, streamerInfo);
        }
      }
    }
  }

  async getStreamerInfo(stream) {
    try {
      if (stream.platform === 'twitch') {
        const response = await axios.get(`${this.apis.twitch.baseURL}/users`, {
          headers: this.apis.twitch.headers,
          params: { id: stream.streamerId },
        });
        
        const user = response.data.data[0];
        return {
          followers: await this.getTwitchFollowerCount(stream.streamerId),
          description: user.description,
          profileImage: user.profile_image_url,
          verified: user.broadcaster_type === 'partner',
          createdAt: new Date(user.created_at),
        };
      }
      
      // Add YouTube logic here
      return { followers: 0, verified: false };
      
    } catch (error) {
      console.error('❌ Error getting streamer info:', error.message);
      return { followers: 0, verified: false };
    }
  }

  async getTwitchFollowerCount(userId) {
    try {
      const response = await axios.get(`${this.apis.twitch.baseURL}/users/follows`, {
        headers: this.apis.twitch.headers,
        params: { to_id: userId },
      });
      
      return response.data.total;
    } catch (error) {
      return 0;
    }
  }

  shouldLaunchToken(stream, streamerInfo) {
    // Check follower threshold
    if (streamerInfo.followers < this.config.minFollowersForLaunch) {
      return false;
    }
    
    // Check verification requirement
    if (this.config.requireVerification && !streamerInfo.verified) {
      return false;
    }
    
    // Check if token already exists for this streamer
    if (this.launchedTokens.has(stream.streamerId)) {
      return false;
    }
    
    console.log(`✅ ${stream.streamerName} meets token launch criteria!`);
    return true;
  }

  async launchCreatorToken(stream, streamerInfo) {
    try {
      console.log(`🚀 Launching creator token for ${stream.streamerName}...`);
      
      const tokenMetadata = {
        name: `${stream.streamerName} Token`,
        symbol: this.generateTokenSymbol(stream.streamerName),
        description: `Creator token for ${stream.streamerName} - ${streamerInfo.description || 'Live streamer'}`,
        image: streamerInfo.profileImage,
        initialSupply: this.config.initialSupply,
        initialPrice: this.config.initialPrice,
        creatorShare: this.config.creatorShare,
        platform: stream.platform,
        streamerId: stream.streamerId,
      };
      
      const result = await this.yoink.createCreatorToken(tokenMetadata);
      
      if (result.success) {
        const tokenData = {
          tokenAddress: result.tokenAddress,
          creatorAddress: result.creatorAddress,
          launchTime: new Date(),
          initialPrice: this.config.initialPrice,
          initialSupply: this.config.initialSupply,
          stream: stream,
          streamerInfo: streamerInfo,
        };
        
        this.launchedTokens.set(stream.streamerId, tokenData);
        stream.hasToken = true;
        
        console.log(`🎉 Token launched successfully!`);
        console.log(`Token Address: ${result.tokenAddress}`);
        console.log(`Creator Address: ${result.creatorAddress}`);
        
        // Send notification
        await this.sendLaunchNotification(stream, tokenData);
        
      } else {
        console.error(`❌ Failed to launch token: ${result.error}`);
      }
      
    } catch (error) {
      console.error('❌ Error launching creator token:', error.message);
    }
  }

  generateTokenSymbol(streamerName) {
    // Generate a token symbol from streamer name
    return streamerName
      .toUpperCase()
      .replace(/[^A-Z0-9]/g, '')
      .substring(0, 8);
  }

  async sendLaunchNotification(stream, tokenData) {
    const message = `🚀 NEW CREATOR TOKEN LAUNCHED!\n\n` +
      `Streamer: ${stream.streamerName}\n` +
      `Platform: ${stream.platform.toUpperCase()}\n` +
      `Current Viewers: ${stream.viewers}\n` +
      `Peak Viewers: ${stream.peakViewers}\n` +
      `Token: ${tokenData.tokenAddress}\n` +
      `Initial Price: ${this.config.initialPrice} SOL`;
    
    console.log('🔔 LAUNCH NOTIFICATION:');
    console.log(message);
    
    // Here you could integrate with Discord, Telegram, email, etc.
  }

  async updateStreamMetrics() {
    // Clean up offline streams
    const currentTime = new Date();
    for (const [streamKey, stream] of this.activeStreams.entries()) {
      const timeSinceUpdate = currentTime - stream.lastUpdate;
      
      // Remove streams that haven't updated in 5 minutes
      if (timeSinceUpdate > 300000) {
        console.log(`📴 ${stream.streamerName} went offline`);
        this.activeStreams.delete(streamKey);
      }
    }
    
    // Display current statistics
    this.displayStatistics();
  }

  displayStatistics() {
    console.clear();
    console.log('🎥 YOINK STREAM MONITOR & TOKEN LAUNCHER');
    console.log('======================================');
    console.log(`Active Streams: ${this.activeStreams.size}`);
    console.log(`Tokens Launched: ${this.launchedTokens.size}`);
    console.log(`Last Update: ${new Date().toLocaleTimeString()}`);
    console.log();
    
    if (this.activeStreams.size > 0) {
      console.log('📺 ACTIVE STREAMS:');
      console.log('------------------');
      this.activeStreams.forEach(stream => {
        const hasTokenIcon = stream.hasToken ? '💰' : '⭕';
        console.log(`${hasTokenIcon} ${stream.streamerName} (${stream.platform.toUpperCase()})`);
        console.log(`   Viewers: ${stream.viewers} | Peak: ${stream.peakViewers}`);
        console.log(`   Title: ${stream.title}`);
        console.log();
      });
    }
    
    if (this.launchedTokens.size > 0) {
      console.log('🚀 LAUNCHED TOKENS:');
      console.log('-------------------');
      this.launchedTokens.forEach((token, streamerId) => {
        console.log(`💰 ${token.stream.streamerName}`);
        console.log(`   Token: ${token.tokenAddress}`);
        console.log(`   Launched: ${token.launchTime.toLocaleString()}`);
        console.log();
      });
    }
  }

  stop() {
    console.log('🛑 Stopping stream monitor...');
    this.isRunning = false;
  }

  sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// Usage Example
async function main() {
  const monitor = new StreamMonitorAndLauncher({
    network: 'mainnet-beta',
    wallet: yourWalletAdapter,
    
    // Platform API Keys
    twitchClientId: 'YOUR_TWITCH_CLIENT_ID',
    twitchClientSecret: 'YOUR_TWITCH_CLIENT_SECRET',
    youtubeApiKey: 'YOUR_YOUTUBE_API_KEY',
    
    // Launch Criteria
    minViewersForLaunch: 75,
    minFollowersForLaunch: 500,
    requireVerification: false,
    
    // Token Settings
    initialSupply: 1000000,
    initialPrice: 0.002, // SOL
    creatorShare: 15, // 15%
    
    platforms: ['twitch'], // Start with just Twitch
  });
  
  await monitor.start();
  
  // Graceful shutdown
  process.on('SIGINT', () => {
    monitor.stop();
    process.exit(0);
  });
}

// Run the stream monitor
main().catch(console.error);
```

## Features

### 🎥 Multi-Platform Monitoring
- Twitch stream monitoring
- YouTube Live integration
- Real-time viewer tracking
- Stream metadata collection

### 🚀 Automated Token Launch
- Configurable launch criteria
- Automatic token creation
- Creator verification checks
- Initial distribution management

### 📊 Analytics & Tracking
- Peak viewer tracking
- Stream duration monitoring
- Token performance metrics
- Historical data collection

### 🔔 Notifications
- Launch announcements
- Stream status updates
- Performance alerts
- Integration ready for Discord/Telegram

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `minViewersForLaunch` | Number | `100` | Minimum concurrent viewers |
| `minFollowersForLaunch` | Number | `1000` | Minimum follower count |
| `requireVerification` | Boolean | `true` | Require platform verification |
| `initialSupply` | Number | `1000000` | Token initial supply |
| `initialPrice` | Number | `0.001` | Initial price in SOL |
| `creatorShare` | Number | `10` | Creator's token percentage |

## Running the Script

1. **Get API Keys**:
   - Twitch: Create app at [dev.twitch.tv](https://dev.twitch.tv)
   - YouTube: Get API key from [Google Cloud Console](https://console.cloud.google.com)

2. **Install dependencies**:
   ```bash
   npm install yoink-sdk axios @solana/web3.js
   ```

3. **Configure and run**:
   ```bash
   node stream-monitor.js
   ```

## Sample Output

```
🎥 YOINK STREAM MONITOR & TOKEN LAUNCHER
======================================
Active Streams: 3
Tokens Launched: 1
Last Update: 2:45:30 PM

📺 ACTIVE STREAMS:
------------------
💰 StreamerName (TWITCH)
   Viewers: 150 | Peak: 200
   Title: Playing New Game!

⭕ AnotherStreamer (TWITCH)
   Viewers: 75 | Peak: 85
   Title: Just Chatting

🚀 LAUNCHED TOKENS:
-------------------
💰 StreamerName
   Token: 7xK9...w2Fy
   Launched: 11/4/2025, 2:30:15 PM
```

## Next Steps

- [📦 Back to SDK Overview](overview.md)
- [🤖 Try Creator Token Bot](creator-token-bot.md)
- [📊 View Portfolio Dashboard](portfolio-dashboard.md)