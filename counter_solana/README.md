# DePIN Rewards Tracker (D-R-T)

D-R-T provides a robust solution for managing reward points in decentralized applications with the following key capabilities:

- 🔐 Secure per-user reward tracking through Program Derived Addresses (PDAs)
- 📊 Configurable minimum and maximum point boundaries
- ⚡ Efficient on-chain point management
- 🛡️ Built-in protection against overflow and underflow
- 🔍 Transaction-level validation and error handling

### Features

- 🎯 Personal Reward Points tracker with user-specific PDAs
- 🎯 Customizable point boundaries with minimum and maximum thresholds
- 🎯 Atomic point operations (increment/decrement)
- 🎯 Safe point resets to minimum value

### Architecture

### User Stories

**Initialization:**

- As a signer when I initialize my tracker with minimum and maximum points, my tracker is initialized with minimum points
- As a signer when I try to initialize with maximum value less than minimum value, the transaction fails with "Invalid Arguments"
- As a signer, I cannot initialize multiple trackers with the same public key

**Operations:**

- As a signer when I increment my tracker, the count increases by one point until it reaches the maximum value
- As a signer when I decrement my tracker, the count decreases by one point until it reaches the minimum value
- As a signer when I reset my tracker, the count returns to the minimum value

**Validation:**

- As a signer when I increment above maximum value, the transaction fails with "Counter at its maximum"
- As a signer when I decrement below minimum value, the transaction fails with "Counter at its minimum"

