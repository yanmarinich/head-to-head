Here's the updated README with the "Head to Head" branding, which better reflects the competitive nature of the project:

````markdown
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
- Anchor Framework 0.28.0 or higher
- Node.js 16+ and npm

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd head-to-head
```
````

2. Install dependencies:

```bash
yarn install
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

Run tests with logs:

```bash
anchor test -- --features "debug-logs"
```

## Program Architecture

### Core Accounts

- `Config`: Game parameters and admin settings
- `Games`: Active and completed battles
- `Prices`: Price oracle data
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

## Development Setup

1. Start local Solana validator:

```bash
solana-test-validator
```

2. Build and deploy:

```bash
anchor build
anchor deploy
```

3. Update program ID:

```bash
solana address -k target/deploy/head_to_head_keypair.json
```

Copy the output to `Anchor.toml` and `lib.rs`

## Contributing

1. Fork the repository
2. Create feature branch
3. Commit changes
4. Push to branch
5. Create Pull Request

## Security Features

- PDA-based account security
- SPL token integration
- Authorization checks
- Input validation
- Secure vault architecture

## License

MIT License

## Disclaimer

This is an experimental prediction game. Participate responsibly and at your own risk. Price movements can be unpredictable.

```

The "Head to Head" branding better captures the essence of your project - it's about direct competition, making predictions, and winning against opponents. The name suggests:
- Direct competition (1v1)
- Clear winners and losers
- Strategic decision making
- Real-time price action
- Betting elements

Would you like me to adjust any part of the README to better match your vision for the project?
```
