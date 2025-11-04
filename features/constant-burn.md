# Constant Burn

A core deflationary mechanism that permanently removes tokens from circulation through strategic burning, creating long-term value for all token holders by reducing supply over time.

## 🔥 How Constant Burn Works

### Integrated with Autobuyback

{% hint style="info" %}
**🔄 Burn Process**
- Autobuyback system purchases tokens from bonding curve using treasury funds
- **All purchased tokens are immediately burned** - never resold
- Tokens sent to burn address: `1111...1111` (mathematically inaccessible)
- Circulating supply decreases permanently with each buyback
{% endhint %}

### Smart Burn Triggers

{% hint style="success" %}
**⚡ Automatic Burning When:**
- Market price falls below backing threshold (250% of treasury backing)
- **OR** price drops below EMA threshold (90% of moving average)
- **AND** sufficient treasury funds are available for buyback
- **AND** total burn limit hasn't been reached (25% of total supply)

**🛡️ Built-in Safety:**
- Maximum 25% of total supply can ever be burned
- Per-transaction limit of 40% of circulating supply
- Minimum backing requirements prevent dust burns
{% endhint %}

## 📊 Technical Implementation

### Burn Mechanics

| Component | Value | Purpose |
|-----------|--------|---------|
| **Max Total Burn** | 25% of supply (2500 bps) | Lifetime burn limit across all buybacks |
| **Treasury Spend** | 60% per trigger (6000 bps) | Portion of treasury used for each buyback |
| **Supply Cap** | 40% per transaction (4000 bps) | Maximum tokens burned in single buyback |
| **Backing Floor** | Dynamic minimum | Prevents manipulation through dust triggers |

### Process Flow

{% hint style="warning" %}
**1️⃣ Market Analysis**
- Monitor bonding curve price vs treasury backing value
- Track price trends with Exponential Moving Average (EMA)
- Calculate optimal burn opportunities automatically

**2️⃣ Buyback Decision**
- Price drops below trigger thresholds
- Treasury has sufficient funds for meaningful buyback
- Total burn limit hasn't been exceeded

**3️⃣ Burn Execution**
- Calculate purchase amount from treasury budget
- Buy tokens through bonding curve mechanics  
- **Immediately burn ALL purchased tokens**
- Update circulating supply and burn statistics

**4️⃣ Supply Impact**
- Reduces virtual and real token reserves permanently
- Decreases circulating supply tracked on-chain
- Creates deflationary pressure for remaining holders
{% endhint %}

## 💰 Economic Impact

### For Token Holders

**📈 Supply Reduction Benefits**
- Your percentage ownership of total supply increases
- Fewer tokens available for trading creates scarcity
- Deflationary pressure supports long-term value growth

**🎯 Price Stabilization**
- Automatic buying support during price drops
- Reduces volatility through algorithmic intervention
- Creates effective price floor based on treasury backing

### For the Ecosystem

**💪 Sustainable Tokenomics**
- Platform trading fees fund the burn mechanism
- Growth in trading volume → more treasury → more burns
- Self-reinforcing cycle of value creation

## 🔍 Transparency & Tracking

### Real-Time Monitoring

{% hint style="info" %}
**📊 On-Chain Metrics:**
- Total tokens burned through all buybacks
- Treasury funds spent on burn activities
- Burn frequency and market timing
- Remaining burn capacity (25% limit tracking)
- Current circulating supply

**🔗 Public Verification:**
- All burn transactions visible on Solana blockchain
- Real-time treasury balance auditable by anyone
- Burn statistics permanently recorded in smart contract
- Complete transaction history with before/after states
{% endhint %}

### Burn Statistics Dashboard

Track key metrics:
- **Cumulative Burned**: Total tokens removed from circulation
- **Burn Rate**: Tokens burned per day/week/month  
- **Supply Impact**: Percentage of original supply destroyed
- **Treasury Efficiency**: SOL spent vs tokens burned ratio
- **Market Impact**: Price correlation with burn events

## 🛡️ Safety & Security Features

### Burn Limits
- **Hard cap of 25% total supply** prevents excessive burning
- **Per-transaction limits** ensure gradual, sustainable burns
- **Automatic shutdown** when maximum burn reached

### Price Protection
- **Significant drop thresholds** (50%) trigger emergency protections
- **Minimum backing requirements** prevent market manipulation
- **EMA smoothing** reduces false triggers from price noise

### Treasury Management
- **Conservative spending limits** (20% maximum per trigger)
- **Multiple fallback calculations** ensure reliable execution
- **Dust protection** prevents tiny, inefficient burn attempts

## 📈 Burn Projections

### Current Parameters
Based on 25% maximum burn limit:
- **Conservative daily trading**: Moderate burn frequency
- **High volume periods**: Accelerated burn rate during active trading
- **Market downturns**: Increased burn activity when prices drop

### Growth Scenarios
- **Platform adoption growth** → More trading fees → Larger treasury → More significant burns
- **Volume milestones** → Trigger bonus burn periods
- **Community growth** → Higher trading frequency → More burn opportunities

---

**🔥 Constant burn operates fully automatically** through the smart contract with no manual intervention required.

**📊 Monitor burn activity** on your token's page to see real-time statistics and verify all burn transactions on-chain.

**🎯 Every burn permanently reduces supply** - creating lasting value for the entire token community.
