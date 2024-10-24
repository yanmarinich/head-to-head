import * as anchor from "@coral-xyz/anchor";
import { assert, expect } from "chai";
import { BN, Program } from "@coral-xyz/anchor";
import { HeadToHead } from "../target/types/head_to_head";
import {
  D_BET_SIZE,
  MINT_DECIMALS,
  PRICE_DECIMALS,
  D_WIN_THRESHOLD_PERCENT,
  D_JOIN_THRESHOLD_PERCENT,
  SIGNER,
  THRESHOLD_DECIMALS,
  BET_SIZE,
} from "./config";
import { Connection, PublicKey } from "@solana/web3.js";
import {
  mockMintKeypair,
  mockPlayerAKeypair,
  mockPlayerBKeypair,
} from "./mocks";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { airdrop, assertDeepEqual, D } from "./utils";
import { Game } from "./types";

describe("Head to Head Game", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.HeadToHead as Program<HeadToHead>;
  const connection = new Connection("http://localhost:8899", "confirmed");

  const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );
  const [pricesPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("prices")],
    program.programId
  );
  const [gamesPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("games")],
    program.programId
  );

  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    program.programId
  );

  let playerATokenAccount: PublicKey;
  let playerBTokenAccount: PublicKey;

  const mint = mockMintKeypair.publicKey;

  async function credit(destination: PublicKey, amount: number) {
    await mintTo(
      connection,
      SIGNER,
      mockMintKeypair.publicKey,
      destination,
      SIGNER,
      amount
    );
  }

  before(async () => {
    await createMint(
      connection,
      SIGNER,
      SIGNER.publicKey,
      SIGNER.publicKey,
      MINT_DECIMALS,
      mockMintKeypair
    );

    const playerATokenAccountInfo = await getOrCreateAssociatedTokenAccount(
      connection,
      SIGNER,
      mint,
      mockPlayerAKeypair.publicKey
    );
    playerATokenAccount = playerATokenAccountInfo.address;

    const playerBTokenAccountInfo = await getOrCreateAssociatedTokenAccount(
      connection,
      SIGNER,
      mockMintKeypair.publicKey,
      mockPlayerBKeypair.publicKey
    );
    playerBTokenAccount = playerBTokenAccountInfo.address;
  });

  it("successfully initializes config", async () => {
    const configArgs = {
      betSize: new BN(D_BET_SIZE),
      winThresholdPercent: D_WIN_THRESHOLD_PERCENT,
      joinThresholdPercent: D_JOIN_THRESHOLD_PERCENT,
      thresholdDecimals: THRESHOLD_DECIMALS,
    };

    await program.methods
      .initializeConfig(configArgs)
      .accounts({
        signer: SIGNER.publicKey,
        mint,
      })
      .signers([SIGNER])
      .rpc();

    const configAccount = await program.account.config.fetch(configPda);

    expect(configAccount.admin.toString()).to.equal(
      SIGNER.publicKey.toString()
    );
    expect(configAccount.mint.toString()).to.equal(mint.toString());
    expect(configAccount.betSize.toNumber()).to.equal(D_BET_SIZE);
    expect(configAccount.thresholdDecimals).to.equal(THRESHOLD_DECIMALS);
    expect(configAccount.joinThresholdPercent).to.equal(
      D_JOIN_THRESHOLD_PERCENT
    );
    expect(configAccount.winThresholdPercent).to.equal(D_WIN_THRESHOLD_PERCENT);
  });

  it("prevents double initialization", async () => {
    const configArgs = {
      betSize: new BN(D_BET_SIZE),
      winThresholdPercent: D_WIN_THRESHOLD_PERCENT,
      joinThresholdPercent: D_JOIN_THRESHOLD_PERCENT,
      thresholdDecimals: THRESHOLD_DECIMALS,
    };

    try {
      // Try to initialize config again
      await program.methods
        .initializeConfig(configArgs)
        .accounts({
          signer: SIGNER.publicKey,
          mint,
        })
        .signers([SIGNER])
        .rpc();

      assert.fail("Should not allow double initialization");
    } catch (error) {
      expect(error.message).to.include("custom program error: 0x0");
    }
  });

  it("successfully initializes prices", async () => {
    const pricesArgs = {
      initialPrice: new BN(D(1500, PRICE_DECIMALS)),
      priceDecimals: PRICE_DECIMALS,
    };

    await program.methods
      .initializePrices(pricesArgs)
      .accounts({
        signer: SIGNER.publicKey,
      })
      .signers([SIGNER])
      .rpc();

    const {
      prices: [initialPrice],
      decimals,
    } = await program.account.prices.fetch(pricesPda);
    expect(initialPrice.toNumber()).to.equal(D(1500, PRICE_DECIMALS));
    expect(decimals).to.equal(PRICE_DECIMALS);
  });

  it("prevents double initialization of prices", async () => {
    const pricesArgs = {
      initialPrice: new BN(D(1500, PRICE_DECIMALS)),
      priceDecimals: PRICE_DECIMALS,
    };

    try {
      await program.methods
        .initializePrices(pricesArgs)
        .accounts({
          signer: SIGNER.publicKey,
        })
        .signers([SIGNER])
        .rpc();

      assert.fail("Should not allow double initialization");
    } catch (error) {
      expect(error.message).to.include("custom program error: 0x0");
    }
  });

  it("successfully initializes games", async () => {
    await program.methods
      .initializeGames()
      .accounts({
        signer: SIGNER.publicKey,
      })
      .signers([SIGNER])
      .rpc();

    const { games } = await program.account.games.fetch(gamesPda);

    expect(games.length).to.equal(0);
  });

  it("prevents double initialization of games", async () => {
    try {
      await program.methods
        .initializeGames()
        .accounts({
          signer: SIGNER.publicKey,
        })
        .signers([SIGNER])
        .rpc();

      assert.fail("Should not allow double initialization");
    } catch (error) {
      expect(error.message).to.include("custom program error: 0x0");
    }
  });

  it("successfully initializes vault", async () => {
    await program.methods
      .initializeVault()
      .accounts({
        signer: SIGNER.publicKey,
        mint,
      })
      .signers([SIGNER])
      .rpc();
  });

  it("prevents double initialization of vault", async () => {
    try {
      await program.methods
        .initializeVault()
        .accounts({
          signer: SIGNER.publicKey,
          mint,
        })
        .signers([SIGNER])
        .rpc();

      assert.fail("Should not allow double initialization");
    } catch (error) {
      expect(error.message).to.include("custom program error: 0x0");
    }
  });

  it("successfully adds multiple prices and verifies state", async () => {
    const priceAccount = await program.account.prices.fetch(pricesPda);
    const newPrices = [1501, 1502, 1503, 1504, 1505].map(
      (price) => new BN(D(price, PRICE_DECIMALS))
    );

    for (const price of newPrices) {
      await program.methods.addPrice(price).signers([SIGNER]).rpc();
    }

    const updatedPriceAccount = await program.account.prices.fetch(pricesPda);

    expect(updatedPriceAccount.prices.length).to.equal(
      priceAccount.prices.length + newPrices.length
    );

    updatedPriceAccount.prices.forEach((storedPrice, index) => {
      expect(storedPrice.toString()).to.equal(
        [...priceAccount.prices, ...newPrices][index].toString()
      );
    });
  });
  it("prevents non-admin from adding price", async () => {
    const nonAdmin = mockPlayerAKeypair;

    const price = new BN(1500);

    try {
      await program.methods
        .addPrice(price)
        .accounts({ admin: nonAdmin.publicKey })
        .signers([nonAdmin])
        .rpc();

      assert.fail("Should not allow non-admin to add price");
    } catch (error) {
      expect(error.message).to.include("Only admin can perform this action");
    }
  });

  it("prevents adding invalid price", async () => {
    const invalidPrice = new BN(0);

    try {
      await program.methods.addPrice(invalidPrice).signers([SIGNER]).rpc();

      assert.fail("Should not allow invalid price");
    } catch (error) {
      expect(error.message).to.include("InvalidPrice");
    }
  });

  it("allows player to create game with UP prediction", async () => {
    await airdrop(mockPlayerAKeypair.publicKey, 5, connection);
    await credit(playerATokenAccount, D_BET_SIZE);
    const priceAccount = await program.account.prices.fetch(pricesPda);
    const vaultBalanceBefore = await connection.getTokenAccountBalance(
      vaultPda
    );

    const playerBalanceBefore = await connection.getTokenAccountBalance(
      playerATokenAccount
    );

    expect(D(playerBalanceBefore.value.uiAmount)).to.equal(D_BET_SIZE);

    await program.methods
      .createGame(true)
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc({ commitment: "confirmed" });

    const gamesAccount = await program.account.games.fetch(gamesPda);

    assert.equal(gamesAccount.games.length, 1);
    const [game] = gamesAccount.games;

    const expectedGame: Game = {
      host: mockPlayerAKeypair.publicKey,
      opponent: null,
      hostPrediction: true,
      amount: new BN(D_BET_SIZE),
      priceIndex: priceAccount.prices.length - 1,
      result: null,
      isClosed: false,
    };

    assertDeepEqual(game, expectedGame);

    const vaultBalanceAfter = await connection.getTokenAccountBalance(vaultPda);
    const playerBalanceAfter = await connection.getTokenAccountBalance(
      playerATokenAccount
    );

    expect(D(playerBalanceAfter.value.uiAmount)).to.equal(0);

    expect(
      D(vaultBalanceAfter.value.uiAmount - vaultBalanceBefore.value.uiAmount)
    ).to.equal(D_BET_SIZE);
  });

  it("allows player to create game with DOWN prediction as second game", async () => {
    await credit(playerATokenAccount, D_BET_SIZE);

    const priceAccount = await program.account.prices.fetch(pricesPda);

    await program.methods
      .createGame(false)
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc({ commitment: "confirmed" });

    const gamesAccount = await program.account.games.fetch(gamesPda);

    assert.equal(gamesAccount.games.length, 2);
    const [_, game] = gamesAccount.games;

    const expectedGame: Game = {
      host: mockPlayerAKeypair.publicKey,
      opponent: null,
      hostPrediction: false,
      amount: new BN(D_BET_SIZE),
      priceIndex: priceAccount.prices.length - 1,
      result: null,
      isClosed: false,
    };

    assertDeepEqual(game, expectedGame);
  });

  it("prevents non-host from withdrawing", async () => {
    try {
      await program.methods
        .withdrawFromGame(0)
        .accounts({
          player: mockPlayerBKeypair.publicKey,
          playerTokenAccount: playerBTokenAccount,
        })
        .signers([mockPlayerBKeypair])
        .rpc({ commitment: "confirmed" });

      assert.fail("Should not allow non-host to withdraw");
    } catch (error) {
      expect(error.message).to.include("UnauthorizedWithdrawal");
    }
  });

  it("should successfully widthdraw money from game without opponent", async () => {
    const gameindex = 0;
    const vaultBalanceBefore = await connection.getTokenAccountBalance(
      vaultPda
    );

    const playerBalanceBefore = await connection.getTokenAccountBalance(
      playerATokenAccount
    );

    expect(playerBalanceBefore.value.uiAmount).to.equal(0);
    expect(D(vaultBalanceBefore.value.uiAmount)).to.equal(D_BET_SIZE * 2);

    await program.methods
      .withdrawFromGame(gameindex)
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc({ commitment: "confirmed" });

    const vaultBalanceAfter = await connection.getTokenAccountBalance(vaultPda);
    const playerBalanceAfter = await connection.getTokenAccountBalance(
      playerATokenAccount
    );

    expect(D(vaultBalanceAfter.value.uiAmount)).to.equal(D_BET_SIZE);
    expect(D(playerBalanceAfter.value.uiAmount)).to.equal(D_BET_SIZE);

    const gamesAccount = await program.account.games.fetch(gamesPda);

    const game = gamesAccount.games[gameindex];
    expect(game.isClosed).to.equal(true);
  });

  it("fails when trying to withdraw from closed game", async () => {
    try {
      await program.methods
        .withdrawFromGame(0)
        .accounts({
          player: mockPlayerAKeypair.publicKey,
          playerTokenAccount: playerATokenAccount,
        })
        .signers([mockPlayerAKeypair])
        .rpc({ commitment: "confirmed" });

      assert.fail("Should not allow withdrawal from closed game");
    } catch (error) {
      expect(error.message).to.include("GameAlreadyClosed");
    }
  });

  it("prevents host from joining their own game", async () => {
    try {
      await program.methods
        .joinGame(1)
        .accounts({
          player: mockPlayerAKeypair.publicKey,
          playerTokenAccount: playerATokenAccount,
        })
        .signers([mockPlayerAKeypair])
        .rpc();

      assert.fail("Should not allow host to join their own game");
    } catch (error) {
      expect(error.message).to.include("CannotJoinOwnGame");
    }
  });

  it("prevents joining closed game game", async () => {
    try {
      await program.methods
        .joinGame(0)
        .accounts({
          player: mockPlayerBKeypair.publicKey,
          playerTokenAccount: playerBTokenAccount,
        })
        .signers([mockPlayerBKeypair])
        .rpc();

      assert.fail("Should not allow to join closed game");
    } catch (error) {
      expect(error.message).to.include("GameAlreadyClosed");
    }
  });

  it("allows opponent to join an existing game", async () => {
    const game_index = 1;
    await credit(playerBTokenAccount, D_BET_SIZE);
    const vaultBalanceBefore = await connection.getTokenAccountBalance(
      vaultPda
    );

    await program.methods
      .joinGame(game_index)
      .accounts({
        player: mockPlayerBKeypair.publicKey,
        playerTokenAccount: playerBTokenAccount,
      })
      .signers([mockPlayerBKeypair])
      .rpc({ commitment: "confirmed" });

    const gamesAccount = await program.account.games.fetch(gamesPda);
    const game = gamesAccount.games[game_index];
    expect(game.opponent.toString()).to.equal(
      mockPlayerBKeypair.publicKey.toString()
    );

    const opponentBalanceAfter = await connection.getTokenAccountBalance(
      playerBTokenAccount
    );
    const vaultBalanceAfter = await connection.getTokenAccountBalance(vaultPda);

    expect(D(opponentBalanceAfter.value.uiAmount)).to.equal(0);
    expect(
      D(vaultBalanceAfter.value.uiAmount - vaultBalanceBefore.value.uiAmount)
    ).to.equal(D_BET_SIZE);
  });

  it("prevents joining after opponent has joined", async () => {
    try {
      await program.methods
        .joinGame(1)
        .accounts({
          player: mockPlayerBKeypair.publicKey,
          playerTokenAccount: playerBTokenAccount,
        })
        .signers([mockPlayerBKeypair])
        .rpc();

      assert.fail("Should not allow to join after opponent joined");
    } catch (error) {
      expect(error.message).to.include("GameAlreadyJoined");
    }
  });

  it("prevents withdrawal after opponent has joined", async () => {
    try {
      await program.methods
        .withdrawFromGame(1)
        .accounts({
          player: mockPlayerAKeypair.publicKey,
          playerTokenAccount: playerATokenAccount,
        })
        .signers([mockPlayerAKeypair])
        .rpc();

      assert.fail("Should not allow to withdraw after opponent joined");
    } catch (error) {
      expect(error.message).to.include("WithdrawalNotAllowed");
    }
  });

  it("prevents joining when price moved more than 1%", async () => {
    await credit(playerATokenAccount, D_BET_SIZE);

    await program.methods
      .createGame(true)
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc();

    const priceAccount = await program.account.prices.fetch(pricesPda);
    const gamesAccount = await program.account.games.fetch(gamesPda);
    const gameIndex = gamesAccount.games.length - 1;
    const game = gamesAccount.games[gameIndex];
    const gamePrice = priceAccount.prices[game.priceIndex];

    const newPrice = gamePrice.mul(new BN(101)).div(new BN(100));

    await program.methods.addPrice(newPrice).signers([SIGNER]).rpc();

    await credit(playerBTokenAccount, D_BET_SIZE);
    try {
      await program.methods
        .joinGame(gameIndex)
        .accounts({
          player: mockPlayerBKeypair.publicKey,
          playerTokenAccount: playerBTokenAccount,
        })
        .signers([mockPlayerBKeypair])
        .rpc();

      assert.fail("Should not allow joining when price moved too much");
    } catch (error) {
      expect(error.message).to.include("PriceMovedTooMuch");
    }
  });

  it("prevents claiming from non-existent game", async () => {
    try {
      await program.methods
        .claimWinnings(999)
        .accounts({
          player: mockPlayerAKeypair.publicKey,
          playerTokenAccount: playerATokenAccount,
        })
        .signers([mockPlayerAKeypair])
        .rpc();

      assert.fail("Should not allow claiming from non-existent game");
    } catch (error) {
      expect(error.message).to.include("GameNotFound");
    }
  });

  it("prevents claiming from game without opponent", async () => {
    await credit(playerATokenAccount, D_BET_SIZE);
    await program.methods
      .createGame(true)
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc();

    const gamesAccount = await program.account.games.fetch(gamesPda);
    const gameIndex = gamesAccount.games.length - 1;

    try {
      await program.methods
        .claimWinnings(gameIndex)
        .accounts({
          player: mockPlayerAKeypair.publicKey,
          playerTokenAccount: playerATokenAccount,
        })
        .signers([mockPlayerAKeypair])
        .rpc();

      assert.fail("Should not allow claiming from game without opponent");
    } catch (error) {
      expect(error.message).to.include("GameNotStarted");
    }
  });

  it("prevents claiming from already closed game", async () => {
    const gameIndex = 0;

    try {
      await program.methods
        .claimWinnings(gameIndex)
        .accounts({
          player: mockPlayerAKeypair.publicKey,
          playerTokenAccount: playerATokenAccount,
        })
        .signers([mockPlayerAKeypair])
        .rpc();

      assert.fail("Should not allow claiming from closed game");
    } catch (error) {
      expect(error.message).to.include("GameAlreadyClosed");
    }
  });

  it("prevents claiming when price threshold not reached", async () => {
    await credit(playerATokenAccount, D_BET_SIZE);
    await program.methods
      .createGame(true) // Host predicts UP
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc();

    const gamesAccount = await program.account.games.fetch(gamesPda);
    const gameIndex = gamesAccount.games.length - 1;

    // Player B joins predicting DOWN
    await credit(playerBTokenAccount, D_BET_SIZE);
    await program.methods
      .joinGame(gameIndex)
      .accounts({
        player: mockPlayerBKeypair.publicKey,
        playerTokenAccount: playerBTokenAccount,
      })
      .signers([mockPlayerBKeypair])
      .rpc();

    // Set price to only move 3% up (below 5% threshold)
    const priceAccount = await program.account.prices.fetch(pricesPda);
    const game = gamesAccount.games[gameIndex];
    const gamePrice = priceAccount.prices[game.priceIndex];

    const newPrice = gamePrice.mul(new BN(103)).div(new BN(100));
    await program.methods.addPrice(newPrice).signers([SIGNER]).rpc();

    // Host tries to claim before threshold is reached
    try {
      await program.methods
        .claimWinnings(gameIndex)
        .accounts({
          player: mockPlayerAKeypair.publicKey,
          playerTokenAccount: playerATokenAccount,
        })
        .signers([mockPlayerAKeypair])
        .rpc();

      assert.fail(
        "Should not allow claiming before price threshold is reached"
      );
    } catch (error) {
      expect(error.message).to.include("GameNotFinished");
    }

    // Opponent tries to claim before threshold is reached
    try {
      await program.methods
        .claimWinnings(gameIndex)
        .accounts({
          player: mockPlayerBKeypair.publicKey,
          playerTokenAccount: playerBTokenAccount,
        })
        .signers([mockPlayerBKeypair])
        .rpc();

      assert.fail(
        "Should not allow claiming before price threshold is reached"
      );
    } catch (error) {
      expect(error.message).to.include("GameNotFinished");
    }
  });

  it("prevents non-winner host from claiming when price threshold is reached", async () => {
    await credit(playerATokenAccount, D_BET_SIZE);
    await program.methods
      .createGame(true) // Host predicts UP
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc();

    const gamesAccount = await program.account.games.fetch(gamesPda);
    const gameIndex = gamesAccount.games.length - 1;

    // Player B joins predicting DOWN
    await credit(playerBTokenAccount, D_BET_SIZE);
    await program.methods
      .joinGame(gameIndex)
      .accounts({
        player: mockPlayerBKeypair.publicKey,
        playerTokenAccount: playerBTokenAccount,
      })
      .signers([mockPlayerBKeypair])
      .rpc();

    // Set price to move 5% down making Player B the winner
    const priceAccount = await program.account.prices.fetch(pricesPda);
    const game = gamesAccount.games[gameIndex];
    const gamePrice = priceAccount.prices[game.priceIndex];

    const newPrice = gamePrice.mul(new BN(95)).div(new BN(100)); // 5% decrease
    await program.methods.addPrice(newPrice).signers([SIGNER]).rpc();

    // Host (Player A) tries to claim despite losing
    try {
      await program.methods
        .claimWinnings(gameIndex)
        .accounts({
          player: mockPlayerAKeypair.publicKey,
          playerTokenAccount: playerATokenAccount,
        })
        .signers([mockPlayerAKeypair])
        .rpc();

      assert.fail("Should not allow losing player to claim winnings");
    } catch (error) {
      expect(error.message).to.include("SignerNotWinner");
    }
  });

  it("prevents non-winner opponent from claiming when price threshold is reached", async () => {
    await credit(playerATokenAccount, D_BET_SIZE);
    await program.methods
      .createGame(true) // Host predicts UP
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc();

    const gamesAccount = await program.account.games.fetch(gamesPda);
    const gameIndex = gamesAccount.games.length - 1;

    // Player B joins predicting DOWN
    await credit(playerBTokenAccount, D_BET_SIZE);
    await program.methods
      .joinGame(gameIndex)
      .accounts({
        player: mockPlayerBKeypair.publicKey,
        playerTokenAccount: playerBTokenAccount,
      })
      .signers([mockPlayerBKeypair])
      .rpc();

    // Set price to move 5% up making Player A the winner
    const priceAccount = await program.account.prices.fetch(pricesPda);
    const game = gamesAccount.games[gameIndex];
    const gamePrice = priceAccount.prices[game.priceIndex];

    const newPrice = gamePrice.mul(new BN(105)).div(new BN(100)); // 5% increase
    await program.methods.addPrice(newPrice).signers([SIGNER]).rpc();

    // Opponent (Player B) tries to claim despite losing
    try {
      await program.methods
        .claimWinnings(gameIndex)
        .accounts({
          player: mockPlayerBKeypair.publicKey,
          playerTokenAccount: playerBTokenAccount,
        })
        .signers([mockPlayerBKeypair])
        .rpc();

      assert.fail("Should not allow losing player to claim winnings");
    } catch (error) {
      expect(error.message).to.include("SignerNotWinner");
    }
  });

  it("allows winning host to successfully claim", async () => {
    await credit(playerATokenAccount, D_BET_SIZE);
    const initialHostBalance = await connection.getTokenAccountBalance(
      playerATokenAccount
    );

    const initialVaultBalance = await connection.getTokenAccountBalance(
      vaultPda
    );

    await program.methods
      .createGame(true)
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc({ commitment: "confirmed" });

    const gamesAccount = await program.account.games.fetch(gamesPda);
    const gameIndex = gamesAccount.games.length - 1;

    const hostBalanceAfterCreating = await connection.getTokenAccountBalance(
      playerATokenAccount
    );

    expect(hostBalanceAfterCreating.value.uiAmount).to.equal(
      initialHostBalance.value.uiAmount - BET_SIZE
    );

    const vaultBalanceAfterCreating = await connection.getTokenAccountBalance(
      vaultPda
    );

    expect(vaultBalanceAfterCreating.value.uiAmount).to.equal(
      initialVaultBalance.value.uiAmount + BET_SIZE
    );

    // Player B joins predicting DOWN
    await credit(playerBTokenAccount, D_BET_SIZE);
    await program.methods
      .joinGame(gameIndex)
      .accounts({
        player: mockPlayerBKeypair.publicKey,
        playerTokenAccount: playerBTokenAccount,
      })
      .signers([mockPlayerBKeypair])
      .rpc({ commitment: "confirmed" });

    // Check vault balance after opponent joins
    const vaultBalanceAfterJoin = await connection.getTokenAccountBalance(
      vaultPda
    );

    expect(vaultBalanceAfterJoin.value.uiAmount).to.equal(
      vaultBalanceAfterCreating.value.uiAmount + BET_SIZE
    );

    // Set price to move 5% up making host (Player A) the winner
    const priceAccount = await program.account.prices.fetch(pricesPda);
    const game = gamesAccount.games[gameIndex];
    const gamePrice = priceAccount.prices[game.priceIndex];

    const newPrice = gamePrice.mul(new BN(106)).div(new BN(100)); // 5% increase
    await program.methods
      .addPrice(newPrice)
      .signers([SIGNER])
      .rpc({ commitment: "confirmed" });

    const newPrices = await program.account.prices.fetch(pricesPda);

    // Host claims winnings
    await program.methods
      .claimWinnings(gameIndex)
      .accounts({
        player: mockPlayerAKeypair.publicKey,
        playerTokenAccount: playerATokenAccount,
      })
      .signers([mockPlayerAKeypair])
      .rpc({ commitment: "confirmed" });

    // Check final balances
    const finalHostBalance = await connection.getTokenAccountBalance(
      playerATokenAccount
    );
    const finalVaultBalance = await connection.getTokenAccountBalance(vaultPda);

    expect(finalHostBalance.value.uiAmount).to.equal(
      initialHostBalance.value.uiAmount + BET_SIZE
    );

    expect(finalVaultBalance.value.uiAmount).to.equal(
      initialVaultBalance.value.uiAmount - BET_SIZE * 2
    );

    // Verify game is closed
    const finalGamesAccount = await program.account.games.fetch(gamesPda);
    const finalGame = finalGamesAccount.games[gameIndex];
    expect(finalGame.isClosed).to.be.true;
    expect(finalGame.result).to.be.true;
  });
});
