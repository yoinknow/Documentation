# Autobuyback

An intelligent system that automatically purchases and burns tokens from the treasury pool when market conditions are favorable, helping stabilize token prices and reduce circulating supply.

## 🎯 How Autobuyback Works

### Smart Price Detection

{% hint style="info" %}
**🔍 Market Analysis**
- Monitors real-time token price using bonding curve data
- Tracks price trends with Exponential Moving Average (EMA)
- Calculates backing value based on treasury reserves
- Identifies optimal buyback opportunities automatically
{% endhint %}

### Trigger Conditions

{% hint style="warning" %}
**📊 Buyback Triggers When:**
- Current price falls below backing threshold (250% of backing value)
- **OR** price drops below EMA threshold (90% of moving average)
- **AND** sufficient treasury funds are available
- **AND** total burn limit hasn't been reached (25% max supply)

**📈 Safety Guards:**
- Maximum 40% of circulating supply per buyback
- Minimum backing value required to prevent dust triggers
- Emergency stop when 50% price drop occurs
{% endhint %}

## ⚙️ Technical Implementation

### Dynamic Parameters

| Parameter | Value | Purpose |
|-----------|--------|---------|
| **Backing Multiplier** | 250% (25000 bps) | Price threshold above backing value |
| **EMA Drop Threshold** | 90% (9000 bps) | Moving average trigger level |
| **EMA Response Speed** | 50% (5000 bps) | Price trend calculation weight |
| **Treasury Spend** | 60% (6000 bps) | Portion of treasury used per buyback |
| **Supply Cap** | 40% (4000 bps) | Maximum tokens bought per transaction |
| **Max Burn Total** | 25% (2500 bps) | Lifetime burn limit of total supply |

{% hint style="info" %}
**💡 Parameter Balance:** These settings create sustainable long-term tokenomics by combining conservative trigger thresholds (250% backing, 90% EMA) with aggressive response actions (60% treasury spend, 40% supply cap). The high backing multiplier prevents frequent triggering during normal volatility, while substantial spending ensures meaningful impact when conditions warrant intervention. The 25% lifetime burn limit protects against excessive deflation while allowing significant supply reduction over time.
{% endhint %}

### Process Flow

{% hint style="success" %}
**1️⃣ Price Analysis**
- Calculate current market price for 1 token lot
- Update EMA with latest price data
- Determine backing value from treasury + reserves

**2️⃣ Trigger Check**
- Compare market price to trigger thresholds
- Verify treasury has sufficient funds
- Check total burn limit hasn't been exceeded

**3️⃣ Buyback Execution**
- Calculate optimal purchase amount from treasury budget
- Execute buy order through bonding curve mechanics
- **Immediately burn all purchased tokens**
- Update circulating supply and burn totals

**4️⃣ State Updates**
- Reduce virtual and real token reserves
- Increase SOL reserves with purchase amount
- Deduct cost from treasury pool
- Log complete transaction details
{% endhint %}

## � Benefits for Token Holders

### Price Stabilization
- **Automatic support** during price drops
- **Reduces volatility** through algorithmic intervention
- **Creates price floor** based on treasury backing

### Supply Reduction
- **Permanently burns** purchased tokens
- **Reduces circulating supply** over time
- **Increases scarcity** for remaining holders

### Treasury Efficiency
- **Utilizes accumulated fees** productively
- **Provides value** back to token community
- **No manual intervention** required

## 📊 Monitoring & Transparency

### Real-Time Tracking

{% hint style="info" %}
**📈 Available Metrics:**
- Total tokens burned through buybacks
- Treasury funds spent on buybacks
- Buyback frequency and timing
- Price impact of each buyback
- Remaining burn capacity (25% limit)

**🔍 Transaction Logs:**
- Every buyback emits detailed event data
- Complete before/after state comparison
- Price calculations and trigger reasons
- Burn amounts and supply updates
{% endhint %}

### Public Verification
- All buyback transactions are **on-chain and verifiable**
- **Real-time price data** available through bonding curve
- **Treasury balance** publicly auditable
- **Burn statistics** tracked permanently

## 🛡️ Safety Features

### Burn Limits
- **Maximum 25% of total supply** can ever be burned
- **Per-transaction limit** of 10% circulating supply
- **Automatic shutdown** when limit reached

### Price Protection
- **Significant drop protection** (50% threshold)
- **Minimum backing requirements** prevent manipulation
- **EMA smoothing** reduces noise and false triggers

### Treasury Management
- **Aggressive spending** (60% per trigger maximum)
- **Multiple fallback mechanisms** if primary calculation fails
- **Dust protection** prevents tiny, inefficient buybacks

---

**🔥 Autobuyback is enabled by default** for all tokens and operates automatically without any user intervention required.

**📊 Monitor your token's buyback activity** on the token page to see real-time burn statistics and treasury utilization.

