import { IdlTypes, Program } from "@coral-xyz/anchor";
import { HeadToHead } from "../target/types/head_to_head";

export type Game = IdlTypes<HeadToHead>["game"];
