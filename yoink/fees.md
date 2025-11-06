---
icon: receipt
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
---

# Fee Structure

Yoink uses a transparent, fair fee system that benefits all participants in the ecosystem: the platform, creators, early supporters, and token holders.

## Total Transaction Fee

{% hint style="info" %}
**4% fee on all buy and sell transactions**

This single fee is automatically split among multiple parties to create a sustainable, community-driven ecosystem.
{% endhint %}

## Fee Distribution Breakdown

Every transaction on Yoink incurs a **4% fee** that is distributed as follows:

| Recipient | Percentage | Purpose |
|-----------|------------|---------|
| **Platform** | 1.0% | Platform operations & $YOINK buyback |
| **Creator** | 0.8% | Token creator earnings |
| **Early Bird Pool** | 1.2% | Rewards for first 20 buyers |
| **Treasury** | 1.0% | Automatic buyback mechanism |

### 1. Platform Fee (1%)

The 1% platform fee is further split 50-50:

* **0.5%** → Platform Treasury (operations, development, maintenance)
* **0.5%** → $YOINK Buyback Wallet (flywheel mechanism)

{% hint style="success" %}
**Flywheel Effect**: Half of all platform fees automatically buy back $YOINK tokens, creating constant buying pressure and value accrual for $YOINK holders.
{% endhint %}

### 2. Creator Fee (0.8%)

* Goes directly to the token creator
* Same rate as pump.fun (we compensate via user pool)
* Continues even after token graduates to Raydium
* Incentivizes quality token creation

### 3. Early Bird Pool (1.2%)

* Shared among the first 20 buyers of each token
* Rewards early supporters and risk-takers
* Claimable after holding for required period
* Creates incentive for early discovery

### 4. Treasury Pool (1%)

* Funds the automatic buyback mechanism
* Used to buy and burn tokens when price drops below EMA
* Grows with trading volume
* Creates price support for all tokens

## Fee Examples

### Example 1: 10 SOL Buy Transaction

```
Total Fee: 0.4 SOL (4%)

Distribution:
├─ Platform: 0.1 SOL (1%)
│  ├─ Platform Treasury: 0.05 SOL (0.5%)
│  └─ $YOINK Buyback: 0.05 SOL (0.5%)
├─ Creator: 0.08 SOL (0.8%)
├─ Early Bird Pool: 0.12 SOL (1.2%)
└─ Treasury (Buyback): 0.1 SOL (1%)
```

### Example 2: 100 SOL Sell Transaction

```
Total Fee: 4 SOL (4%)

Distribution:
├─ Platform: 1 SOL (1%)
│  ├─ Platform Treasury: 0.5 SOL (0.5%)
│  └─ $YOINK Buyback: 0.5 SOL (0.5%)
├─ Creator: 0.8 SOL (0.8%)
├─ Early Bird Pool: 1.2 SOL (1.2%)
└─ Treasury (Buyback): 1 SOL (1%)
```

## Volume = Fees = Ecosystem Growth

The fee system creates a positive feedback loop:

```
More Trading Volume
        ↓
More Fees Generated
        ↓
Larger Buyback Treasuries
        ↓
Stronger Price Support
        ↓
More Confident Traders
        ↓
More Trading Volume
        ↓
(cycle continues)
```

### Benefits by Participant

**For Traders:**
* Transparent, predictable fees
* Automatic price support via buybacks
* No hidden costs or surprise charges

**For Creators:**
* Earn 0.8% on all trades forever
* Incentive to promote and grow your token
* Revenue continues even after Raydium graduation

**For Early Birds:**
* 1.2% of all volume shared among first 20 buyers
* Significant rewards for early discovery
* Risk-reward balanced incentive

**For Platform:**
* Sustainable operations funding
* Continuous development resources
* Long-term viability

**For $YOINK Holders:**
* Constant buyback pressure from platform fees
* Value accrual from entire platform volume
* Flywheel mechanism benefits all holders

## Fee Comparison

| Platform | Total Fee | Creator Share | Buyback System | Early Bird Rewards |
|----------|-----------|---------------|----------------|-------------------|
| **Yoink** | 4% | 0.8% | ✅ Yes (2%) | ✅ Yes (1.2%) |
| **Pump.fun** | 1% | 0% | ❌ No | ❌ No |
| **Traditional DEX** | 0.25-0.3% | 0% | ❌ No | ❌ No |

{% hint style="info" %}
**Why Higher Fees?**

Yoink's 4% fee might seem higher, but it includes:
- Automatic buyback protection (2% total)
- Creator compensation (0.8%)
- Early supporter rewards (1.2%)
- Sustainable platform development

You're not just paying for a trade—you're investing in ecosystem features that protect your investment.
{% endhint %}

## Fee Collection & Distribution

### Automatic & Trustless

* All fees collected automatically via smart contract
* No manual intervention required
* Transparent on-chain tracking
* Immediate distribution to respective pools

### Real-Time Tracking

* View fee accumulation on token pages
* Monitor treasury growth
* Track early bird pool size
* See creator earnings in real-time

### On-Chain Verification

All fee transactions are publicly verifiable on Solana blockchain:
* Platform fees → Visible in platform wallet
* Creator fees → Visible in creator wallet
* Early bird pool → Visible in pool account
* Treasury pool → Visible in bonding curve account

---

## $YOINK Buyback Flywheel

{% hint style="success" %}
**🔄 Platform Fees Power $YOINK Growth**

**50% of all platform fees are used to buyback $YOINK tokens**, creating constant buying pressure regardless of individual token performance.

**How it works:**
1. Every transaction generates 1% platform fee
2. 0.5% (half) goes directly to $YOINK buyback wallet
3. Buyback wallet automatically purchases $YOINK from market
4. Creates perpetual buying pressure for $YOINK token
5. More platform volume = more $YOINK buybacks

**Example:**
- $1M daily platform volume → $10,000 in platform fees → $5,000 buys $YOINK
- $10M daily platform volume → $100,000 in platform fees → $50,000 buys $YOINK

This percentage can increase as the platform establishes stronger backing infrastructure and achieves sustainable profitability.
{% endhint %}

### Why This Matters for $YOINK Holders

* **Volume Alignment**: Your $YOINK value grows with total platform success, not just individual tokens
* **Consistent Demand**: Constant buyback regardless of market conditions
* **Scalable**: More users = more volume = more buybacks
* **Transparent**: All buyback transactions visible on-chain
* **Sustainable**: Grows naturally with platform adoption

## FAQs

### Are fees the same for buys and sells?

Yes, both buy and sell transactions incur the same 4% fee with identical distribution.

### Can fees change in the future?

Fees are set in the smart contract at token creation. Each token's fee structure is permanent and cannot be changed.

### Do I pay fees when claiming early bird rewards?

No, claiming rewards does not incur trading fees. You only pay standard Solana network fees (gas).

### What happens to creator fees if creator abandons token?

Creator fees continue accumulating in the creator wallet. They can claim anytime, even years later.

### How is the $YOINK buyback executed?

The 0.5% platform fee accumulates in a dedicated wallet, which automatically purchases $YOINK from the market at regular intervals.

### Can I see where my fees went?

Yes! Every fee transaction is on-chain. You can track exactly where each portion went by viewing the transaction on Solana Explorer.

### Are there any additional hidden fees?

No. The 4% transaction fee is the only fee. You also pay standard Solana network fees (usually < $0.01), but those go to Solana validators, not Yoink.

### Why is Yoink's creator fee lower than pump.fun?

We compensate by allocating more to the early bird pool, which benefits the community and incentivizes early discovery and support.

## Next Steps

<table data-view="cards"><thead><tr><th></th><th></th><th data-hidden data-card-target data-type="content-ref"></th></tr></thead><tbody><tr><td><strong>Early Birds</strong></td><td>Learn how to earn from the early bird pool</td><td><a href="../features/early-seats.md">early-seats.md</a></td></tr><tr><td><strong>Autobuyback</strong></td><td>Understand how treasury fees create price support</td><td><a href="../features/autobuyback.md">autobuyback.md</a></td></tr><tr><td><strong>$YOINK Flywheel</strong></td><td>See how platform fees power $YOINK</td><td><a href="../usdyoink/constant-buyback.md">constant-buyback.md</a></td></tr></tbody></table>
