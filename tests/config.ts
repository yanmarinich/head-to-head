import { assert } from "chai";
import "dotenv/config";
import { D, getKeypair } from "./utils";

export const SIGNER_KEY_PATH = process.env.SIGNER_KEY_PATH!;
assert(SIGNER_KEY_PATH, "no signer env");

export const THRESHOLD_DECIMALS = 2;
export const MINT_DECIMALS = 9;
export const PRICE_DECIMALS = 9;

export const BET_SIZE = 1_000;

export const D_BET_SIZE = D(BET_SIZE, MINT_DECIMALS);
export const D_JOIN_THRESHOLD_PERCENT = D(1, THRESHOLD_DECIMALS);
export const D_WIN_THRESHOLD_PERCENT = D(5, THRESHOLD_DECIMALS);

export const SIGNER = getKeypair(SIGNER_KEY_PATH);
