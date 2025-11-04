# 🔄 Auto Buyback

Yoink features an **algorithmic buyback mechanism** for all coins created on the platform.  
When market conditions are favorable, the system automatically purchases and burns tokens using the **treasury pool**, helping to stabilize prices and reduce circulating supply.  

This mechanism supports long-term **token health and sustainability**, ensuring a more stable experience for traders.  
**Treasury pools grow over time** through the fees accumulated from trading volume (see the [Fees](fees.md) page for details).


## 🎯 How Autobuyback Works

### Smart Price Detection

{% hint style="info" %}
**🔍 Market Analysis**

- Monitors token prices on every trade using **bonding curve data**  
- Tracks price trends through **Exponential Moving Average (EMA)**  
- Calculates **backing value** based on treasury reserves  
- Automatically identifies **optimal buyback opportunities**
{% endhint %}

### Trigger Conditions

{% hint style="warning" %}
**📊 Buyback Triggers When:**
- The current price falls below the **backing threshold** (250% of backing value)  
- **OR** the price drops below the **EMA threshold** (90% of the moving average)  
- **AND** sufficient **treasury funds** are available  
- **AND** the **total burn limit** hasn’t been reached (25% of max supply)

{% endhint %}



## ⚙️ Technical Implementation
### Dynamic Parameters

| Parameter | Value | Purpose |
|-----------|--------|---------|
| **Backing Multiplier** | 250% (25,000 bps) | Price threshold above backing value |
| **EMA Drop Threshold** | 90% (9,000 bps) | Trigger level for moving average deviation |
| **EMA Response Speed** | 50% (5,000 bps) | Weight factor in price trend calculation |
| **Treasury Spend** | 60% (6,000 bps) | Portion of treasury used per buyback |
| **Max Burn Total** | 25% (2,500 bps) | Lifetime burn limit relative to total supply |

{% hint style="info" %}
**💡 Parameter Balance:**  
These parameters are designed to maintain **long-term sustainability** by blending conservative trigger thresholds (250% backing, 90% EMA) with decisive response actions (60% treasury spend, 40% supply cap).  
The elevated backing multiplier minimizes unnecessary activations during normal volatility, while the treasury’s significant allocation ensures **meaningful buyback impact** when market conditions warrant.  
Finally, the **25% lifetime burn cap** prevents over-deflation while still allowing a healthy reduction in circulating supply over time.
{% endhint %}


## 💰 Benefits for Token Holders

{% hint style="success" %}
**📈 Price Stabilization**
- **Automatic support** during price drops
- **Reduces volatility** through algorithmic intervention  
- **Creates price floor** based on treasury backing
{% endhint %}

{% hint style="info" %}
**🔥 Supply Reduction**
- **Permanently burns** purchased tokens
- **Reduces circulating supply** over time
- **Increases scarcity** for remaining holders
{% endhint %}

{% hint style="warning" %}
**⚡ Treasury Efficiency**
- **Utilizes accumulated fees** productively
- **Provides value** back to token community
- **No manual intervention** required
{% endhint %}

## 📊 Monitoring & Transparency

### Buyback Activity Tab

{% hint style="info" %}
**📈 Track Buybacks on Token Pages:**
- Each coin page includes a dedicated **"Buybacks" tab**
- View complete history of all buyback events
- See exact timing, amounts, and transaction links
- Monitor treasury utilization and burn statistics

**🔗 Transaction Visibility:**
- **Burn transactions** are fully visible on Solana blockchain
- **Buy operations** happen internally within the program
- Only the burn will show as an external transaction
- The "purchase" is actually an internal ledger update
{% endhint %}

### Internal Mechanics Explained

{% hint style="warning" %}
**🔄 How the "Buy" Works Internally:**
- SOL moves from treasury vault to bonding curve liquidity pool
- Token quotation changes to reflect increased SOL reserves
- **No actual SOL leaves the bonding curve** - it's an internal transfer
- This creates tokens for burning without external market impact
- Result: Treasury balance decreases, curve liquidity increases, tokens get burned

**💡 Why You Only See Burns:**
- The purchase is an internal AMM ledger adjustment
- No external buy/sell pressure on the market
- Only the final burn transaction appears on-chain
- This maintains price stability during buyback operations
{% endhint %}

**🔥 Autobuyback is enabled by default** for all tokens and operates automatically without any user intervention required.

**📊 Monitor your token's buyback activity** on the token page to see real-time burn statistics and treasury utilization.

