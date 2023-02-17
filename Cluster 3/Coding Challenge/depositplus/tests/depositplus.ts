import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BN } from "bn.js";
import { assert } from "chai";
import { Depositplus } from "../target/types/depositplus";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountIdempotentInstruction, createMint, createMintToCheckedInstruction, getAssociatedTokenAddress, getAssociatedTokenAddressSync, getMint, getOrCreateAssociatedTokenAccount, mintToChecked, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("depositplus", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  //let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");

  const program = anchor.workspace.Depositplus as Program<Depositplus>;
  let { connection } = program.provider;
  const user = anchor.web3.Keypair.generate();
  const [vaultPda, vaultBump] = await anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    program.programId
  );
  before(async () => {

    // Request an airdrop of 2 Sol to the user's account.
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
      console.error("Error while requesting airdrop:", error);
    }

  })
  it("Is initialized!", async () => {
    // Add your test here.
    assert(true, 'this is true');
    let latestBlockhash = await connection.getLatestBlockhash('finalized');
    try {
      const tx = await program.methods.initialize()
        .accountsStrict({
          initializer: user.publicKey,
          vault: vaultPda,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([user])
        .rpc();
      await connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
      });
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
      // Fetch the vault account and make assertions.
    } catch (error) {
      console.error("Error while creating a vault account:", error);
    } finally {
      let vault;
      try {
        vault = await program.account.vault.fetch(vaultPda);
      } catch (error) {
        console.error("Error while fetching the vault account:", error);
      }

      assert(vault.owner.toBase58() === user.publicKey.toBase58(), "The vault owner should be the user.");
      assert(vaultBump == vault.bump, "Vault bump as expected.");
      assert(vault.balance.toNumber() == 0, "Vault balance is 0.");
    }

  });
  it("Deposits 1 SOL", async () => {

    let latestBlockhash = await connection.getLatestBlockhash('finalized');
    let depositAmt = new BN(anchor.web3.LAMPORTS_PER_SOL);
    try {
      const tx = await program.methods.deposit(depositAmt)
        .accounts({
          owner: user.publicKey,
          vault: vaultPda,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([user])
        .rpc();
      await connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      }, 'confirmed');
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
    } catch (error) {
      console.error("Error making deposit:", error);
    } finally {
      // Fetch the vault account and make assertions.
      let vault;
      try {
        vault = await program.account.vault.fetch(vaultPda);
      } catch (error) {
        console.error("Error while fetching the vault account:", error);
      }
      assert(vault.balance.toNumber() == anchor.web3.LAMPORTS_PER_SOL, "Vault balance is 1 SOL.");
    }
  });
  it("Withdraws 0.5 SOL", async () => {

    let latestBlockhash = await connection.getLatestBlockhash('finalized');
    let withdrawAmt = new BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);
    try {
      const tx = await program.methods.withdraw(withdrawAmt)
        .accounts({
          owner: user.publicKey,
          vault: vaultPda,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([user])
        .rpc();
      await connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
      });
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
    } catch (error) {
      console.error("Error making withdraw:");
    }

    // Fetch the vault account and make assertions.
    let vault;
    try {
      vault = await program.account.vault.fetch(vaultPda);
    } catch (error) {
      console.error("Error while fetching the vault account:", error);
    } finally {
      assert(vault.balance.toNumber() == 0.5 * anchor.web3.LAMPORTS_PER_SOL, "Vault balance is 0.5 SOL.");
    }

  });
  it("Create and deposit mock SPL Token", async () => {

    let MINT_KEY = await createMint(
      program.provider.connection,
      user,
      user.publicKey,
      null,
      6
    );

    let user_ata = await getAssociatedTokenAddress(MINT_KEY, user.publicKey);
    let ix1 = await createAssociatedTokenAccountIdempotentInstruction(
      user.publicKey, user_ata, user.publicKey, MINT_KEY
    );
    let ix2 = await createMintToCheckedInstruction(MINT_KEY, user_ata, user.publicKey, 100e6, 6);
    let transaction = new anchor.web3.Transaction();
    transaction.add(ix1, ix2);
    try {
      let blockhash = await program.provider.connection.getLatestBlockhash();
      transaction.recentBlockhash = blockhash.blockhash;
      transaction.lastValidBlockHeight = blockhash.lastValidBlockHeight;
      let tx = await anchor.web3.sendAndConfirmTransaction(program.provider.connection, transaction, [user]);
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
    }
    catch (error) {
      console.error("Error while minting fungible:", error);
    }

    let latestBlockhash = await connection.getLatestBlockhash('finalized');
    let depositAmount = new BN(100e6);
    let dest_ata = await getAssociatedTokenAddress(MINT_KEY, vaultPda, true, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
    try {
      const tx = await program.methods.depositSpl(depositAmount)
        .accounts({
          payer: user.publicKey,
          vault: vaultPda,
          fromAta: user_ata,
          toAta: dest_ata,
          tokenMint: MINT_KEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([user])
        .rpc();
      await connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
      });
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
    } catch (error) {
      console.error("Error making spl deposit", error);
    }

    try {
      let latestBlockhash = await connection.getLatestBlockhash('finalized');
      let withdrawAmt = new BN(10e6);
      const tx = await program.methods.withdrawSpl(withdrawAmt)
        .accounts({
          payer: user.publicKey,
          vault: vaultPda,
          fromAta: dest_ata,
          toAta: user_ata,
          tokenMint: MINT_KEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([user])
        .rpc();
      await connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
      });
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
    } catch (error) {
      console.error("Error making spl deposit", error);
    } finally {
      let vault;
      try {
        vault = await program.account.vault.fetch(vaultPda);
        let [vault_spl_balance,user_spl_balance] = await Promise.allSettled([connection.getTokenAccountBalance(dest_ata), connection.getTokenAccountBalance(user_ata)]);
        if (vault_spl_balance.status === 'fulfilled') {
          assert(vault_spl_balance.value.value.uiAmount == 90, 'vault has 90 tokens')
        } else {
          console.log(`Error: ${vault_spl_balance.reason}`);
        }
        
        if (user_spl_balance.status === 'fulfilled') {
          assert(user_spl_balance.value.value.uiAmount == 10, 'user has 10 tokens')
        } else {
          console.log(`Error: ${user_spl_balance.reason}`);
        }
        
      } catch (error) {
        console.error("Error while fetching the vault account:", error);
      }
      assert(vault.balance.toNumber() == 0.5 * anchor.web3.LAMPORTS_PER_SOL, "Vault balance is 0.5 SOL.");
    }

  });

});
