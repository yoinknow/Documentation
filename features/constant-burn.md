# 🔥 Constant Burn

Along with every buyback, a simultaneous **burn** of the acquired tokens occurs as part of the process.  
This core **deflationary mechanism** permanently removes tokens from circulation through strategic burning, creating long-term value for all holders by gradually reducing total supply over time.

## ⚙️ How Constant Burn Works

### Integrated with Auto Buyback

<figure><img src="../.gitbook/assets/burnandbuyback.png" alt=""><figcaption></figcaption></figure>

{% hint style="info" %}
**🔄 Burn Process**
- The Auto Buyback system purchases tokens from the bonding curve using treasury funds  
- **All purchased tokens are immediately burned** — never resold  
- Circulating supply decreases permanently with each buyback
{% endhint %}

### Smart Burn Triggers

{% hint style="success" %}
**🛡️ Built-in Limit:**
- A maximum of **25% of total supply** can ever be burned  
- Once this threshold is reached, no further buybacks or burns occur  
{% endhint %}

## 📊 Technical Implementation

### Burn Mechanics

| Component | Value | Purpose |
|-----------|--------|---------|
| **Max Total Burn** | 25% of supply (2,500 bps) | Lifetime burn limit across all buybacks |
| **Supply Cap** | 40% per transaction (4,000 bps) | Maximum tokens burned in a single buyback |

### Process Flow

{% hint style="warning" %}
**🧠 Step 1: Algorithm Decision**  
- Price falls below one or more **trigger thresholds from buybacks**  
- The **treasury** has sufficient funds for a meaningful buyback  
- The **total burn limit** has not been exceeded  
- **All purchased tokens are immediately burned** as part of the buyback process  

**🔥 Step 2: Supply Impact**  
- Permanently reduces **the token reserves on the curve**  
- Decreases **circulating supply** tracked on-chain  
- Creates **deflationary pressure** for remaining holders
{% endhint %}

## 🔍 Transparency & Tracking

### Real-Time Monitoring

<figure><img src="../.gitbook/assets/burnandbuyback.png" alt=""><figcaption></figcaption></figure>

{% hint style="info" %}
**📊 On-Chain Metrics:**
- Total tokens burned through all buybacks  
- Treasury funds spent on burn activities  
- Burn frequency and market timing  

**🔗 Public Verification:**
- All burn transactions are visible on the **Solana blockchain**  
- Treasury balances are auditable in real time  
- Complete transaction history available with before/after states  
{% endhint %}

### Burn Statistics

<figure><img src="../.gitbook/assets/burndash.png" alt=""><figcaption></figcaption></figure>

Each coin page displays its **current burned supply percentage**, updated in real time.
