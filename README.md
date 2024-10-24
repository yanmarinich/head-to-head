```markdown
# Head to Head (H2H)

A decentralized peer-to-peer price prediction battle platform on Solana where players go head-to-head betting on price movements. Challenge others or accept challenges by predicting whether a price will go UP or DOWN - winner takes all!

## Overview

Head to Head (H2H) implements a dynamic peer-to-peer betting system where:

- Players can create challenges by predicting UP/DOWN and placing a bet
- Opponents can accept challenges by taking the opposite position
- When price moves beyond the configured threshold, the winner is determined
- Winners claim double their bet
- Challenge creators can withdraw if no one accepts their challenge

## Key Features

- Direct peer-to-peer battles
- Token-based wagering using SPL tokens
- Configurable win/join thresholds for balanced gameplay
- Secure token vault system
- Price feed integration ready
- Fair play mechanics
- Auto-settlement system

## Prerequisites

- Rust 1.70.0 or higher
- Solana Tool Suite 1.16.0 or higher
- Anchor Framework 0.30.1 or higher
- Node.js 16+ and npm
```

## Installation

1. Clone the repository:

```bash
git clone git@github.com:yanmarinich/head-to-head.git
cd head-to-head
```

2. Install dependencies:

```bash
npm install
```

## Building

Build the program:

```bash
anchor build
```

## Testing

Run Anchor tests:

```bash
anchor test
```

Run Rust tests:

```bash
cargo test
```

## Program Architecture

### Core Accounts

- `Config`: Game parameters and admin settings
- `Games`: Active and completed battles
- `Prices`: Price data
- `Vault`: Secure token holdings

### Instructions

- `create_game`: Create a new H2H challenge
- `join_game`: Accept an existing challenge
- `claim_winnings`: Claim victory rewards
- `withdraw_from_game`: Cancel an unaccepted challenge
- `add_price`: Update price data (admin only)

### Game Parameters

- Fixed bet sizes for fair competition
- Configurable win thresholds
- Join timeframe limitations
- Price movement validations

## Security Features

- PDA-based account security
- SPL token integration
- Authorization checks
- Input validation
- Secure vault architecture

## Disclaimer

This project is only for demostration purposes.
