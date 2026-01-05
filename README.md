# Order Book - Solana Decentralized Exchange

A complete Solana-based decentralized exchange (DEX) implementation with an on-chain order book program written in Rust using Anchor framework, and a client application for interacting with the protocol.

## Project Overview

This project implements a peer-to-peer token trading system on Solana that allows users to place limit buy and sell orders, which are then matched and executed atomically on-chain.

### Key Components

- **Smart Contract** (`programs/order_book/`): Anchor-based Solana program handling order creation, matching, and token transfers
- **Application** (`app/`): Client interface for interacting with the order book program
- **Scripts** (`scripts/`): Utility scripts for deployment and testing
- **Tests** (`tests/`): Test suite for the order book functionality
- **Migrations** (`migrations/`): Database and contract migration files

## Features

### Order Book Program
- **Market Management**: Create markets between any two token pairs (base and quote mints)
- **Limit Orders**: Place buy and sell orders with specific prices and quantities
- **Order Matching**: Automatically match compatible buy and sell orders
- **Token Transfers**: Execute atomic SPL token transfers between buyers and sellers
- **Order Tracking**: Track order status including remaining quantity, quantity filled, and completion state

### Program Instructions

1. **init_market**: Initialize a new market with base and quote token mints
2. **limit_buy_order**: Create a buy limit order with quantity and price
3. **limit_sell_order**: Create a sell limit order with quantity and price
4. **make_transfer**: Execute a match between compatible buy and sell orders

## How It Works

1. **Market Initialization**
   - Admin creates a market between two tokens
   - Market holds token vaults for buy and sell orders

2. **Placing Orders**
   - Buyers specify quantity and maximum price they'll pay
   - Sellers specify quantity and minimum price they'll accept
   - Orders are stored on-chain with their details and status

3. **Order Matching**
   - When a buy and sell order are compatible (prices overlap)
   - The match amount is the minimum of both remaining quantities
   - Quote tokens transfer from buyer's vault to seller
   - Base tokens transfer from seller's vault to buyer
   - Order quantities are updated accordingly

## Technical Stack

- **Language**: Rust
- **Framework**: Anchor (Solana development framework)
- **Token Standard**: SPL Token with CPI (Cross-Program Invocation)
- **Blockchain**: Solana

## Project Structure

```
order_book/
├── programs/order_book/          # Main Solana program
│   └── src/
│       ├── lib.rs                # Program entry point
│       ├── instructions/          # Program instructions
│       │   ├── init_market.rs
│       │   ├── buy_order.rs
│       │   ├── sell_order.rs
│       │   └── match_order.rs
│       └── state/                 # Data structures
│           ├── market.rs
│           ├── order.rs
│           └── errors.rs
├── app/                           # Client application
├── scripts/                       # Deployment and utility scripts
├── tests/                         # Test suite
└── Cargo.toml                    # Workspace configuration
```

## Getting Started

### Prerequisites
- Rust and Cargo
- Solana CLI tools
- Anchor framework

### Build

```bash
anchor build
```

### Deploy

```bash
anchor deploy
```

### Run Tests

```bash
anchor test
```

## Contract State

### Market
- `base_mint`: Token address for the base asset
- `quote_mint`: Token address for the quote asset
- `total_orders_created`: Counter for order IDs
- `bump`: PDA bump seed

### Buy Order
- `id`: Unique order identifier
- `owner`: Wallet address of the buyer
- `quantity`: Total quantity of tokens to buy
- `remaining`: Quantity still available to match
- `quantity_filled`: Quantity already matched
- `buy_price`: Price per unit in quote tokens
- `is_filled`: Whether order is completely filled
- `base_mint` & `quote_mint`: Token addresses
- `created_at`: Timestamp of order creation

### Sell Order
- Similar structure to Buy Order with `sell_price` instead of `buy_price`
