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


describe("List and revoke list offer", () => {
  const seed = new anchor.BN(1);
  const [vault_state] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault_state"),
      seed.toBuffer("le", 8),
      employer.publicKey.toBuffer(),
      //worker.publicKey.toBuffer(),
    ],
    program.programId
  );

  console.log(vault_state);

  const [vault_keeper] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vault_state.toBuffer()],
    program.programId
  );

  it("Deposit SOL into Vault!", async () => {
    const tx = await program.methods
      .makeContract(seed, new anchor.BN(1 * LAMPORTS_PER_SOL), new anchor.BN(1))
      .accounts({
        employer: employer.publicKey,
        vaultKeeper: vault_keeper,
        vaultState: vault_state,
        systemProgram: SystemProgram.programId,
      })
      .signers([employer])
      .rpc();
    expect(tx).to.be.ok;
  });

  it("Cancel the deposit", async () => {
    
    try {
    const tx = await program.methods
      .revokeContract()
      .accounts({
        employer: employer.publicKey,
        vaultKeeper: vault_keeper,
        vaultState: vault_state,
        systemProgram: SystemProgram.programId,
      })
      .signers([employer])
      .rpc();
    expect(tx).to.be.ok;
    } catch (e) {console.log(e)}

  });
});

///////////////////////////////////////////////////////////////////////////////////////////////////////////
describe("Application initialized", () => {
  const seed = new anchor.BN(1);
  const [vault_state] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault_state"),
      seed.toBuffer("le", 8),
      employer.publicKey.toBuffer(),
      //worker.publicKey.toBuffer(),
    ],
    program.programId
  );

  const [vault_keeper] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vault_state.toBuffer()],
    program.programId
  );

  const [application_state] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("application"),
      worker.publicKey.toBuffer(),
      vault_state.toBuffer(),
    ],
    program.programId
  );

  it("Deposit SOL into Vault!", async () => {
    const tx = await program.methods
      .makeContract(seed, new anchor.BN(1 * LAMPORTS_PER_SOL), new anchor.BN(1))
      .accounts({
        employer: employer.publicKey,
        vaultKeeper: vault_keeper,
        vaultState: vault_state,
        systemProgram: SystemProgram.programId,
      })
      .signers([employer])
      .rpc();
    expect(tx).to.be.ok;
  });
  
  it("Crate application", async () => {
    const tx = await program.methods
    .application()
    .accounts({
      worker: worker.publicKey,
      application: application_state,
      joblisting: vault_state,
      systemProgram: SystemProgram.programId,
    })
    .signers([worker])
    .rpc();
  expect(tx).to.be.ok;
  });
});
///////////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////////////
describe("Accept application", () => {
  const seed = new anchor.BN(1);
  const [vault_state] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault_state"),
      seed.toBuffer("le", 8),
      employer.publicKey.toBuffer(),
      //worker.publicKey.toBuffer(),
    ],
    program.programId
  );

  const [vault_keeper] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vault_state.toBuffer()],
    program.programId
  );

  const [application_state_accepted] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("application"),
      employer.publicKey.toBuffer(),
      vault_state.toBuffer(),
    ],
    program.programId
  );
  
  it("Accept application", async () => {
    const tx = await program.methods
    .acceptapplication()
    .accounts({
      application: application_state_accepted,
      employer: employer.publicKey,
      joblisting: vault_state,
      systemProgram: SystemProgram.programId,
    })
    .signers([employer])
    .rpc();
    expect(tx).to.be.ok;
  });


});
///////////////////////////////////////////////////////////////////////////////////////////////////////////
describe("Accept application and claim", () => {
  const seed = new anchor.BN(1);
  const [vault_state] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault_state"),
      seed.toBuffer("le", 8),
      employer.publicKey.toBuffer(),
      //worker.publicKey.toBuffer(),
    ],
    program.programId
  );

  const [vault_keeper] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vault_state.toBuffer()],
    program.programId
  );

  const [application_state_accepted] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("application"),
      employer.publicKey.toBuffer(),
      vault_state.toBuffer(),
    ],
    program.programId
  );
  
  it("Accept application", async () => {
    const tx = await program.methods
    .acceptapplication()
    .accounts({
      application: application_state_accepted,
      employer: employer.publicKey,
      joblisting: vault_state,
      systemProgram: SystemProgram.programId,
    })
    .signers([employer])
    .rpc();
    expect(tx).to.be.ok;
  });

  it("Claim", async () => {
    const tx = await program.methods
    .claim()
    .accounts({
      application: application_state_accepted,
      worker: worker.publicKey,
      employer: employer.publicKey,
      vaultState: vault_state,
      vaultKeeper: vault_keeper,
      systemProgram: SystemProgram.programId,
    })
    .signers([worker])
    .rpc();
    expect(tx).to.be.ok;
  });

});


// ///////////////////////////////////////////////////////////////////////////////////////////////////////////

// describe("Create and Claim fail because lock_time is not expired", () => {
//   const seed = new anchor.BN(3);
//   const [vault_state] = PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("vault_state"),
//       seed.toBuffer("le", 8),
//       employer.publicKey.toBuffer(),
//       worker.publicKey.toBuffer(),
//     ],
//     program.programId
//   );

//   const [vault_keeper] = PublicKey.findProgramAddressSync(
//     [Buffer.from("vault"), vault_state.toBuffer()],
//     program.programId
//   );

//   it("Deposit SOL into Vault!", async () => {
//     const tx = await program.methods
//       .deposit(seed, new anchor.BN(1 * LAMPORTS_PER_SOL), new anchor.BN(5))
//       .accounts({
//         employer: employer.publicKey,
//         worker: worker.publicKey,
//         vaultKeeper: vault_keeper,
//         vaultState: vault_state,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([employer])
//       .rpc();
//     expect(tx).to.be.ok;
//   });

//   it("Claim the deposit", async () => {
//     try {
//       const tx = await program.methods
//         .claim()
//         .accounts({
//           worker: worker.publicKey,
//           employer: employer.publicKey,
//           vaultKeeper: vault_keeper,
//           vaultState: vault_state,
//           systemProgram: SystemProgram.programId,
//         })
//         .signers([worker])
//         .rpc();
//     } catch (error) {
//       assert.isTrue(error instanceof AnchorError);
//       assert.equal(error.error.errorCode.number, 6000);
//       assert.equal(error.error.errorMessage, "Vault has not expired");
//     }
//   });
// });
