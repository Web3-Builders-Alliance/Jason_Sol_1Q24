import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { CapProject } from "../target/types/cap_project";
import { Keypair, PublicKey, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { assert, expect } from "chai";

const program = anchor.workspace.CapProject as Program<CapProject>;
const connection = anchor.getProvider().connection;
// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const employer = Keypair.generate();
const worker = Keypair.generate();

const confirm = async (signature: string): Promise<string> => {
  const block = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    signature,
    ...block,
  });
  return signature;
};

const log = async (signature: string): Promise<string> => {
  console.log(
    `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
  );
  return signature;
};

it("Airdrop", async () => {
  await connection
    .requestAirdrop(employer.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm);
  await connection
    .requestAirdrop(worker.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm);
});


