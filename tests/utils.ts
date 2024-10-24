import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  TransactionConfirmationStrategy,
} from "@solana/web3.js";
import fs from "fs";
import { MINT_DECIMALS } from "./config";
import BN from "bn.js";
import { expect } from "chai";

export function getKeypair(secretKeyJsonPath: string): Keypair {
  const keyStr = fs.readFileSync(secretKeyJsonPath, "utf8");
  const privateKey = JSON.parse(keyStr);

  return Keypair.fromSecretKey(new Uint8Array(privateKey));
}

export async function airdrop(
  pubkey: PublicKey,
  amount: number,
  connection: Connection
) {
  const airdropSignature = await connection.requestAirdrop(
    pubkey,
    amount * LAMPORTS_PER_SOL
  );

  await connection.confirmTransaction(
    { signature: airdropSignature } as TransactionConfirmationStrategy,
    "confirmed"
  );
}

export function D(amount: number, decimals: number = MINT_DECIMALS): number {
  return amount * 10 ** decimals;
}

export function d(amount: number, decimals: number = MINT_DECIMALS): number {
  return amount / 10 ** decimals;
}

export function assertDeepEqual<T extends Record<string, any>>(
  actual: T,
  expected: T,
  path: string = "",
  tolerance: BN = new BN(1)
) {
  Object.entries(expected).forEach(([key, expectedValue]) => {
    const actualValue = actual[key];
    const currentPath = path ? `${path}.${key}` : key;

    if (BN.isBN(expectedValue)) {
      const difference = expectedValue.sub(actualValue).abs();
      expect(
        closeTo(actualValue, expectedValue),
        `${currentPath} mismatch. Expected: ${expectedValue}, Actual: ${actualValue}, Difference: ${difference}, Tolerance: ${tolerance}`
      ).to.be.true;
    } else if (typeof expectedValue === "object" && expectedValue !== null) {
      assertDeepEqual(actualValue, expectedValue, currentPath, tolerance);
    } else {
      expect(
        actualValue,
        `${currentPath} mismatch. Expected: ${expectedValue}, Actual: ${actualValue}`
      ).to.equal(expectedValue);
    }
  });
}

type Number = bigint | number | BN;

export function closeTo(
  actual: Number,
  expected: Number,
  tolerance: Number = new BN(1)
) {
  const difference = new BN(`${expected}`).sub(new BN(`${actual}`)).abs();

  return difference.lte(new BN(`${tolerance}`));
}
