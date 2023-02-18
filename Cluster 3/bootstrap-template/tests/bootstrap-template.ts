import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BootstrapTemplate } from "../target/types/bootstrap_template";
import { BN } from "bn.js";
import { assert } from "chai";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountIdempotentInstruction, createMint, createMintToCheckedInstruction, getAssociatedTokenAddress,  TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("bootstrap-template", () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.BootstrapTemplate as Program<BootstrapTemplate>;
  let { connection } = program.provider;
  const user = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

  });
  before(async () => {

    // DROP 2 SOL TO GENERATED WALLET
    let airdropBlock = await connection.getLatestBlockhash('finalized');
    try {
      const airdrop = await connection.requestAirdrop(user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
      await connection.confirmTransaction({
        signature: airdrop,
        blockhash: airdropBlock.blockhash,
        lastValidBlockHeight: airdropBlock.lastValidBlockHeight
      });
      console.log('airdrop tx', airdrop);
    } catch (error) {
      conso
      le.error("Error while requesting airdrop:", error);
    }
  })
});
