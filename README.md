# Stockholm: Real-World Asset Price Aggregator Protocol
Deployed Contract address : mantra1jpnrk92gm23p3nj6f7dj6m3mt52fgmmda0n2sx0yxal64cgma77q80l8q9

## 1. Introduction

![Stockholm Poster](https://github.com/ManiBAJPAI22/Stockholm-Agg/assets/103181735/4975afb7-6cfa-4725-b17b-6a40af9f4b91)


Stockholm is an innovative protocol designed to address key challenges in the decentralized finance (DeFi) space, particularly focusing on Real-World Assets (RWAs).

### Problem Statement:
1. **Fragmented Liquidity**: As real-world assets are tokenized across various platforms, liquidity becomes scattered, making it difficult for traders to find the best prices and execute large trades efficiently.
2. **High Transaction Costs**: DeFi trading often suffers from high gas fees and significant slippage, especially for larger transactions or when dealing with less liquid assets.
3. **Inefficient Price Discovery**: The lack of a unified platform for RWAs leads to inconsistent pricing across different exchanges, making it challenging for traders to determine fair market values.

### Solution:
Stockholm aims to solve these issues by:
- Aggregating liquidity from multiple sources to create deeper, more efficient markets for RWAs.
- Implementing advanced pathfinding algorithms to optimize trade routes and minimize costs.
- Providing a single interface for users to access the best prices across various decentralized exchanges (DEXs).

## 2. Market Opportunity

The DeFi and tokenized asset markets present significant growth potential for Stockholm.

### Market Size:
- The DeFi market has experienced exponential growth, with billions of dollars in Total Value Locked (TVL) across various protocols.
- The market for tokenized real-world assets is projected to reach multi-trillion dollar valuations, indicating vast untapped potential.

### Potential:
- **Integration of RWAs**: Stockholm aims to bridge the gap between traditional finance and DeFi by facilitating efficient trading of tokenized real-world assets.
- **Untapped Markets**: Significant opportunities exist in sectors such as real estate, commodities, and various financial instruments that are yet to be fully integrated into the DeFi ecosystem.

## 3. Stockholm Overview

Stockholm is designed as a comprehensive solution for trading RWAs in the DeFi space.

### Definition:
A real-world asset price aggregator protocol that leverages liquidity from various decentralized exchanges to provide optimal trading outcomes for users.

### Key Features:
1. **Liquidity Aggregation**: Combines liquidity from multiple sources to enhance market depth and efficiency, ensuring users get the most favorable prices for their trades.
2. **Price Optimization**: Utilizes advanced pathfinding algorithms to determine the best prices across various DEXs, reducing slippage and improving trading efficiency.
3. **Cost Efficiency**: Minimizes transaction costs through efficient contract execution and by combining multiple trades into single transactions where possible.
4. **User Convenience**: Simplifies the trading process by providing a single platform for accessing multiple DEXs, ensuring a smooth and efficient exchange experience.
5. **Security**: Leverages the robust security features of the Cosmos ecosystem and CosmWasm smart contracts to ensure safe and reliable transactions.

## 4. Technology Stack

Stockholm leverages CosmWasm and the MANTRA Chain to build a secure, scalable, and interoperable protocol.

### Why CosmWasm and MANTRA?
1. **Interoperability**: 
   - Facilitates cross-chain communication, allowing Stockholm to interact with multiple blockchains.
   - Enhances liquidity and market reach by tapping into various ecosystems.

2. **Scalability**: 
   - High throughput ensures fast and efficient transaction processing, critical for real-time price aggregation.
   - Low latency reduces delays, providing users with timely and accurate price information.

3. **Security**: 
   - Rust programming language ensures memory safety, preventing common vulnerabilities.
   - CosmWasm's design minimizes risks associated with smart contract execution.

4. **Modularity**: 
   - Allows for easy upgrades and feature additions through its modular architecture.
   - Facilitates seamless integration with other protocols and services within the Cosmos ecosystem.

## 5. Technical Architecture

Stockholm's architecture is designed to efficiently aggregate prices and execute trades across multiple DEXs.

### Components:
1. **Price Aggregation Engine**: 
   - Collects and aggregates real-time price data from multiple decentralized exchanges.
   - Ensures comprehensive market coverage for accurate price discovery.

2. **Transaction Executor**: 
   - Responsible for executing optimized trades on behalf of users.
   - Interacts with various DEXs to complete transactions based on the optimal path.

3. **Dijkstra's Algorithm**: 
   - Implemented to determine the most efficient trading paths.
   - Considers factors such as gas fees, slippage, and liquidity depth to optimize trades.

## 6. How it Works

Stockholm's operation can be broken down into four main steps:

1. **User Interaction**: 
   - Users submit trade requests through Stockholm's interface, specifying the RWA they wish to trade.

2. **Price Aggregation**: 
   - The price aggregation engine collects real-time price data from multiple connected DEXs.
   - Creates a comprehensive view of the market for the specified RWA.

3. **Pathfinding**: 
   - Dijkstra's algorithm analyzes the collected data to find the most efficient and cost-effective trading paths.
   - Considers various factors including gas fees, slippage, and available liquidity.

4. **Execution**: 
   - The transaction executor processes the trade based on the optimal path determined.
   - Interacts with the necessary DEXs to complete the transaction.
   - The user receives the best possible price for their trade, benefiting from Stockholm's aggregation and optimization.

## 7. Stockholm Ecosystem

Stockholm's ecosystem consists of three key components that work together to enable efficient RWA trading:

### Key Components:
1. **Portable Decentralized Identifiers (DIDs)**:
   - Fundamental component for establishing trust and authenticity in RWA tokenization.
   - Enables secure and verifiable identity management across different platforms.

2. **Real-World Asset (RWA) Oracles**:
   - Specialized data providers that bridge the gap between off-chain real-world data and on-chain smart contracts.
   - Ensure accurate and up-to-date information for RWA pricing and status.

3. **Stockholm Engine**:
   - Core component that enables trading of tokenized assets among RWA-focused DEXs.
   - Integrates the DIDs and RWA Oracles to facilitate secure and efficient transactions.

## 8. Use Cases

Stockholm caters to various stakeholders in the DeFi and RWA ecosystems:

### MANTRA Ecosystem:
- **Integration with MANTRA RWA tokens**: Provides easier access to a broader range of tokenized assets within the MANTRA ecosystem.
- **Integration with Vaults and Pools**: Enhances liquidity and trading options by connecting with existing MANTRA financial products.

### For Investors:
- **Improved Liquidity**: Access to deeper liquidity pools for trading RWAs, enabling larger trades with less price impact.
- **Better Prices**: Benefit from optimized price discovery and execution across multiple DEXs.

### For Institutions:
- **Efficient Trading**: Secure and efficient trading of high-value assets, suitable for large-scale operations.
- **Cost Savings**: Reduced transaction costs and minimized slippage, improving overall trading profitability.

### For Retail Traders:
- **Market Access**: Easier access to a broader range of tokenized assets, including those that might be difficult to trade on traditional platforms.
- **Competitive Pricing**: Benefit from competitive pricing and lower transaction fees due to Stockholm's optimization algorithms.

## 9. Team

The Stockholm project is led by two key individuals:

1. **Mani Bajpai**
2. **Prathmesh Ranjan**




