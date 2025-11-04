---
icon: chart-simple
layout:
  width: default
  title:
    visible: true
  description:
    visible: true
  tableOfContents:
    visible: true
  outline:
    visible: true
  pagination:
    visible: true
  metadata:
    visible: true
---

# 📊 Bonding Curve

Understanding Yoink's bonding curve is essential for successful trading. This automated market-making system determines token prices, provides instant liquidity, and creates a fair launch mechanism for all creator tokens.

## What is a Bonding Curve?

A **bonding curve** is a mathematical formula that automatically determines token prices based on supply and demand. Unlike traditional markets where prices are set by order books, bonding curves use algorithmic pricing to ensure:

* **Instant liquidity** - Always able to buy or sell
* **Fair pricing** - Transparent, predictable price discovery
* **No rug pulls** - Liquidity can't be removed by developers
* **Automatic market making** - No need for manual liquidity provision

{% hint style="info" %}
**Simple explanation**: The more tokens that exist (higher supply), the higher the price becomes. It's like a vending machine that automatically adjusts prices based on how many items have been sold.
{% endhint %}

## The Mathematical Formula

Yoink uses a **linear bonding curve** with the following formula:

```
Price = Base Price + (Current Supply × Price Slope)
```

### Formula Components

| Component | Description | Example Value |
|-----------|-------------|---------------|
| **Base Price** | Starting price for the first token | 0.000001 SOL |
| **Current Supply** | Number of tokens already minted | 1,000,000 tokens |
| **Price Slope** | Rate at which price increases | 0.00000001 SOL per token |
| **Resulting Price** | Price for the next token | 0.000011 SOL |

### Visual Representation

```
Price (SOL)
     │
     │                                    ╱
     │                                ╱
     │                            ╱
     │                        ╱
     │                    ╱
     │                ╱
     │            ╱
     │        ╱
     │    ╱
     │╱
     └────────────────────────────────── Supply (tokens)
     0    2M    4M    6M    8M    10M
```

As you can see, the price increases linearly as more tokens are minted, creating predictable price appreciation.

## How Trading Works

### Buying Tokens

When you buy tokens on the bonding curve:

1. **Specify SOL amount** you want to spend
2. **Formula calculates** how many tokens you'll receive
3. **Price increases** as tokens are minted
4. **Your SOL** enters the bonding curve contract
5. **New tokens** are created and sent to your wallet

**Example Buy Transaction:**
```
Before: 1,000,000 tokens exist at 0.000010 SOL each
You buy: 10 SOL worth of tokens
Result: ~900,000 new tokens minted to you
After: 1,900,000 tokens exist at 0.000019 SOL each
Price Impact: Your buy increased the token price by 90%
```

### Selling Tokens

When you sell tokens on the bonding curve:

1. **Specify token amount** you want to sell
2. **Formula calculates** how much SOL you'll receive
3. **Price decreases** as tokens are burned
4. **Your tokens** are permanently destroyed
5. **SOL** is released from the contract to your wallet

**Example Sell Transaction:**
```
Before: 1,900,000 tokens exist at 0.000019 SOL each
You sell: 400,000 tokens
Result: ~7.6 SOL received
After: 1,500,000 tokens exist at 0.000015 SOL each
Price Impact: Your sell decreased the token price by 21%
```

## The Progress Bar System

Every token displays a **progress bar** showing how close it is to "graduation" (moving to a traditional DEX).

### Progress Calculation

Progress is based on the token's **market capitalization**:

```
Progress % = (Current Market Cap / Target Market Cap) × 100
Target Market Cap = ~10,000 SOL
```

### Progress Stages

| Progress | Stage | Market Cap | Characteristics |
|----------|-------|------------|-----------------|
| **0-25%** | Launch | 0-2,500 SOL | High volatility, early opportunity |
| **25-50%** | Growth | 2,500-5,000 SOL | Building momentum, community forming |
| **50-75%** | Trending | 5,000-7,500 SOL | Strong interest, higher volume |
| **75-95%** | Pre-Graduation | 7,500-9,500 SOL | Anticipation building, less volatility |
| **95-100%** | Graduation Ready | 9,500-10,000 SOL | Imminent DEX migration |

### What the Progress Bar Shows

The progress bar provides instant visual feedback about:

* **Investment timing** - Earlier = higher risk/reward
* **Community strength** - Higher progress = more believers
* **Liquidity depth** - More progress = deeper liquidity
* **Price stability** - Higher progress = less volatility
* **Graduation proximity** - How close to DEX migration

## Graduation and DEX Integration

### When Graduation Occurs

A token automatically graduates when it reaches **~10,000 SOL market cap** (100% progress). This triggers:

✅ **Automatic migration** to Raydium DEX  
✅ **Liquidity pool creation** with permanent liquidity  
✅ **Standard SPL token** deployment  
✅ **Professional trading** features activation  
✅ **Ecosystem integration** with all Solana DEXs  

### The Migration Process

**Step 1: Trigger Detection**
- Smart contract monitors market cap
- Graduation automatically triggered at threshold
- No manual intervention required

**Step 2: Liquidity Migration**
- All SOL from bonding curve → Raydium liquidity pool
- Token supply remains unchanged
- LP tokens locked permanently (no rug pulls)

**Step 3: Trading Transition**
- Bonding curve trading stops
- Raydium DEX trading begins
- Price discovery becomes market-driven
- All holders keep their tokens

**Step 4: Platform Integration**
- Token remains visible on Yoink
- Early Bird rewards continue
- Creator fees still collected
- All platform features maintained

### Benefits of Graduation

**For Token Holders:**
* **Permanent liquidity** that can't be removed
* **Professional trading** tools and features
* **Wider market access** across Solana ecosystem
* **Institutional participation** potential
* **Continued platform** benefits and rewards

**For Creators:**
* **Legitimacy milestone** - proven market demand
* **Revenue growth** - higher volume = more fees
* **Long-term sustainability** in DeFi ecosystem
* **Broader exposure** to Solana community
* **Success validation** for their brand

## Price Impact and Slippage

### Understanding Price Impact

**Price impact** is how much your trade moves the token price:

```
Price Impact = (New Price - Old Price) / Old Price × 100
```

**Factors affecting price impact:**
* **Trade size** - Larger trades = higher impact
* **Current supply** - Lower supply = higher impact
* **Curve steepness** - Steeper curve = higher impact
* **Market cap** - Lower market cap = higher impact

### Slippage Tolerance

**Slippage** is the difference between expected and actual execution price:

**Recommended Settings:**
* **Small trades** (<1 SOL): 1-2% slippage
* **Medium trades** (1-10 SOL): 2-5% slippage
* **Large trades** (>10 SOL): 5-15% slippage
* **Low liquidity** tokens: 10-20% slippage

### Minimizing Trading Costs

**Strategies for better execution:**
* **Split large trades** into smaller chunks
* **Trade during high volume** periods
* **Monitor market cap** - higher = less impact
* **Use appropriate slippage** settings
* **Time your entries** around market activity

## Bonding Curve Advantages

### Compared to Traditional Liquidity Pools

| Feature | Bonding Curve | Traditional LP |
|---------|---------------|----------------|
| **Initial Liquidity** | ✅ Not required | ❌ Required |
| **Price Discovery** | ✅ Algorithmic | ⚡ Market-driven |
| **Rug Pull Risk** | ✅ Impossible | ⚠️ Possible |
| **Impermanent Loss** | ✅ None | ❌ Possible |
| **Complexity** | ✅ Simple | ❌ Complex |
| **Fair Launch** | ✅ Guaranteed | ⚠️ Depends |

### Key Benefits

**🛡️ Security**
- No liquidity that can be pulled
- Transparent, auditable pricing
- Smart contract guarantees

**⚖️ Fairness**
- Same curve for everyone
- No pre-sales or insider access
- Transparent price discovery

**💧 Liquidity**
- Always available for trading
- No need to find counterparties
- Instant execution

**🚀 Accessibility**
- No technical knowledge required
- Low barrier to entry
- Automated market making

## Advanced Concepts

### Curve Efficiency

**Measuring bonding curve health:**

```
Efficiency = Total Volume / Market Cap
```

**High efficiency indicators:**
* Steady trading volume
* Growing holder base
* Consistent price appreciation
* Active creator engagement

### Market Cap Calculation

**Real-time market cap:**

```
Market Cap = Current Supply × Current Price
```

**Market cap growth factors:**
* Net buying pressure
* Creator activity and content
* Community engagement
* Market sentiment
* Platform features (buybacks, etc.)

### Supply Dynamics

**Token supply changes:**
* **Buys** → Tokens minted → Supply increases
* **Sells** → Tokens burned → Supply decreases
* **Net effect** determines price direction
* **Graduation** → Supply becomes fixed

## Trading Strategies on Bonding Curves

### Early Entry Strategy

**Approach:**
- Enter at 0-25% progress
- Target Early Bird positions
- Hold through graduation
- Benefit from maximum price appreciation

**Risk/Reward:**
- **High risk** - unproven tokens
- **High reward** - maximum upside potential

### Momentum Trading

**Approach:**
- Enter during 25-75% progress
- Ride community momentum
- Exit before graduation or hold through
- Focus on volume and activity

**Risk/Reward:**
- **Medium risk** - some validation
- **Medium reward** - solid upside potential

### Pre-Graduation Accumulation

**Approach:**
- Enter at 75-95% progress
- Position for graduation event
- Benefit from liquidity migration
- Hold into DEX phase

**Risk/Reward:**
- **Lower risk** - graduation likely
- **Lower reward** - limited upside remaining

## Monitoring and Analytics

### Key Metrics to Track

**Price Metrics:**
* Current token price
* Market cap progression
* Price change (24h, 7d)
* All-time high/low

**Supply Metrics:**
* Current circulating supply
* Progress to graduation
* Token burn events
* Supply growth rate

**Trading Metrics:**
* Trading volume (24h)
* Number of transactions
* Unique traders
* Average trade size

**Community Metrics:**
* Total holders
* Holder growth rate
* Early Bird seats filled
* Creator activity level

### Using Analytics for Decisions

**Entry timing:**
- Low progress + high activity = opportunity
- Steady progress + volume = momentum
- High progress + volume = graduation play

**Exit timing:**
- Graduation approach = profit taking
- Volume decline = potential exit
- Community activity drop = concern

## Risk Management

### Common Risks

**⚠️ Early Stage Risk**
- Unproven creator/community
- High price volatility
- Low liquidity depth

**⚠️ Market Risk**
- General market conditions
- Solana ecosystem health
- Platform-specific risks

**⚠️ Execution Risk**
- High slippage on large trades
- Network congestion
- Smart contract risks

### Risk Mitigation

**🛡️ Diversification**
- Multiple tokens
- Different progress stages
- Various creator types

**🛡️ Position Sizing**
- Start small while learning
- Scale with experience
- Never risk more than you can afford

**🛡️ Research**
- Verify creator authenticity
- Check community engagement
- Monitor platform metrics

{% hint style="warning" %}
**Important**: Bonding curves are powerful but not risk-free. Always do your own research and only invest what you can afford to lose.
{% endhint %}

## Conclusion

Bonding curves represent a revolutionary approach to token launches and price discovery. By understanding how they work, you can:

* **Make informed trading decisions**
* **Identify opportunities** at different progress stages
* **Manage risk** through proper position sizing
* **Benefit from** the graduation process
* **Participate in** fair, transparent markets

The combination of algorithmic pricing, automatic liquidity, and graduation mechanics creates a unique trading environment that benefits creators, communities, and traders alike.

{% hint style="success" %}
**Ready to trade?** Now that you understand bonding curves, you're equipped to make smarter trading decisions on Yoink. Start small, learn the mechanics, and gradually increase your involvement as you gain experience.
{% endhint %}