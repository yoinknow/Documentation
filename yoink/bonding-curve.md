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

<figure><img src="../.gitbook/assets/curve.jpg" alt=""><figcaption></figcaption></figure>

Understanding Yoink's bonding curve is essential for successful trading. This automated market-making system determines token prices, provides instant liquidity, and creates a fair launch mechanism for all creator tokens.

## What is a Bonding Curve?

A **bonding curve** is a mathematical formula that automatically determines token prices based on supply and demand. Unlike traditional markets where prices are set by order books, bonding curves use algorithmic pricing to ensure:

* **Instant liquidity** - Always able to buy or sell
* **Fair pricing** - Transparent, predictable price discovery
* **No rug pulls** - Liquidity can't be removed by creators
* **Automatic market making** - No need for manual liquidity provision


## The Mathematical Formula

Yoink uses a **constant product market maker (CPMM)** bonding curve.


## The Progress Bar System

<figure><img src="../.gitbook/assets/6.png" alt=""><figcaption></figcaption></figure>

Every token displays a **progress bar** showing how close it is to "graduation" (moving to a traditional DEX).


### When Graduation Occurs

Whena a coin reaches 100% progress, it triggers:

✅ **Automatic migration** to Raydium DEX  
✅ **Liquidity pool creation** with all accumulated SOL  

{% hint style="info" %}
**Current Process**: At the moment, migration to Raydium is handled manually by our team and can take a few minutes to complete after a token reaches 100% progress. However, we're working on making this process fully automatic for a seamless user experience. This manual step ensures proper liquidity setup and prevents any technical issues during the critical migration phase.
{% endhint %}  


