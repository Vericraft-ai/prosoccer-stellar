# Prosoccer Project Technical Documentation

## Overview

Prosoccer is the first multi-chain digital football platform that empowers soccer fans to own, trade, manage soccer teams and play in competitions.

## **Technical Architecture Diagram**

```mermaid
sequenceDiagram
    participant User
    participant Prosoccer Platform
    participant Multichain Blockchain
    participant Stellar Blockchain
    participant Soroban Smart Contract

    User->>+Prosoccer Platform: Register and Create Account

    User->>+Prosoccer Platform: Purchase NFT (Players, coaches or team)
    Prosoccer Platform->>+Multichain Blockchain: Check blockchain network user is accessing from
    Prosoccer Platform->>+Stellar Blockchain: Transaction Request (Purchase NFT)
    Stellar Blockchain->>+Soroban Smart Contract: Execute NFT Purchase
    Soroban Smart Contract-->>-Stellar Blockchain: NFT Purchase Executed
    Stellar Blockchain-->>-Prosoccer Platform: Transaction Confirmation (NFT Purchased)
    Prosoccer Platform-->>User: NFT Ownership Confirmed

    User->>+Prosoccer Platform: Manage Football Team
    Prosoccer Platform-->>-User: Team management executed

    User->>+Prosoccer Platform: Enter Competition
    Prosoccer Platform-->>-User: Competition entry executed

    User->>+Prosoccer Platform: Receive Rewards
    Prosoccer Platform->>+Stellar Blockchain: Transaction Request (Distribute Rewards)
    Stellar Blockchain->>+Soroban Smart Contract: Execute Rewards Distribution
    Soroban Smart Contract-->>-Stellar Blockchain: Rewards Distribution Executed
    Stellar Blockchain-->>-Prosoccer Platform: Transaction Confirmation (Rewards Distributed)
    Prosoccer Platform-->>User: Rewards Credited
```

# Explanation

## User Registration and Wallet Creation

1. The user registers on the Prosoccer platform using their preferred chain (Stellar blockchain)
2.

## Purchasing a Football Team NFT

1. The user purchases their digital assets (players, coaches or team) on the platform.
2. The platform check the chain the user is accessing from (Stellar blockchain)
3. The platform sends a transaction request to the Stellar blockchain.
4. The Soroban smart contract executes the NFT purchase.
5. The transaction is confirmed, and the platform notifies the user of their NFT ownership.

## Managing Football Team

1. The user manages their football team through the platform.
2. The platform execute the team management request.

## Entering Competitions

1. The user enters a competition.
2. The platform execute the competition entry request.

## Receiving Rewards

1. The user receives rewards for participating in competitions.
2. The platform sends a transaction request to the Stellar blockchain.
3. The Soroban smart contract executes the rewards distribution.
4. The transaction is confirmed, and the platform notifies the user.
