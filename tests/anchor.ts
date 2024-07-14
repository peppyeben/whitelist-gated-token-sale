import * as anchor from "@coral-xyz/anchor";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import type { WhitelistGatedTokenSale } from "../target/types/whitelist_gated_token_sale";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.WhitelistGatedTokenSale as anchor.Program<WhitelistGatedTokenSale>;
  
  it("initialize", async () => {
    // Generate keypair for the new account
    const newAccountKp = new web3.Keypair();

    console.log(newAccountKp.publicKey);

    // Send transaction
    const admin = newAccountKp.publicKey;
    // const txHash = await program.methods
    //   .initialize(admin)
    //   .accounts({
    //     admin,
    //     systemProgram: web3.SystemProgram.programId,
    //   })
    //   .signers([newAccountKp])
    //   .rpc();
    // console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    // await program.provider.connection.confirmTransaction(txHash);

    // // Fetch the created account
    // const newAccount = await program.account.newAccount.fetch(
    //   newAccountKp.publicKey
    // );

    // assert(data.eq(newAccount.data));
  });
});
