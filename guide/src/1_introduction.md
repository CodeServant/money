# Introduction

The design goal is to make some interfaces to interact with monetary amounts.

Every type in this crate have certain features guaranteed.
- **Money** is *positive* and *rounded* to available minor units
- **Balances** is either *positive or negative* and guaranteed to be *rounded* to available minor units
- **Price** is *positive* and have to be rounded according to specific price policy (which can have more minor units than the currency allows)