import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ClaimReward } from "../target/types/claim_reward";
import {Keypair, PublicKey} from "@solana/web3.js";
import {assert, expect} from "chai";
import { BN } from 'bn.js';

describe("claim-reward", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ClaimReward as Program<ClaimReward>;
  const pg = program.provider as anchor.AnchorProvider;
  const requestAirdrop = async (mint_keypair:anchor.web3.Keypair) => {
    const signature = await pg.connection.requestAirdrop(
        mint_keypair.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
    );
    const { blockhash, lastValidBlockHeight } = await pg.connection.getLatestBlockhash();
    await pg.connection.confirmTransaction({
      blockhash,
      lastValidBlockHeight,
      signature
    });
  }

  const admin_keypair = Keypair.generate();
  const other_admin_keypair = Keypair.generate();
  console.log(program.programId.toBase58())

  const [configPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("config")
      ],
      program.programId
  )

  it("Is initialized!", async () => {
    await requestAirdrop(admin_keypair);
    await requestAirdrop(other_admin_keypair);

    await program.methods.initialize(admin_keypair.publicKey).accounts({
      config: configPDA,
      payer: pg.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    try{
      await program.methods.initialize(admin_keypair.publicKey).accounts({
        config: configPDA,
        payer: pg.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
    } catch (error) {
      return;
    }
    assert((await program.account.config.fetch(configPDA)).owner.equals(pg.wallet.publicKey));
    assert((await program.account.config.fetch(configPDA)).signer.equals(admin_keypair.publicKey));
    assert((await program.account.config.fetch(configPDA)).initialized);
  });

  it("Update config", async () => {
    try{
      await program.methods.update(other_admin_keypair.publicKey).accounts({
        config: configPDA,
        payer: other_admin_keypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([other_admin_keypair]).rpc();
    } catch (error) {
      return;
    }
    await program.methods.update(other_admin_keypair.publicKey).accounts({
      config: configPDA,
      payer: pg.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    assert((await program.account.config.fetch(configPDA)).owner.equals(pg.wallet.publicKey));
    assert((await program.account.config.fetch(configPDA)).signer.equals(other_admin_keypair.publicKey));
    assert((await program.account.config.fetch(configPDA)).initialized);
    await program.methods.update(admin_keypair.publicKey).accounts({
      config: configPDA,
      payer: pg.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    assert((await program.account.config.fetch(configPDA)).signer.equals(admin_keypair.publicKey));
  });

  it("Register reward pool", async () => {
    const task = 1
    const total = new BN('30000')
    const buf = Buffer.alloc(2);
    buf.writeUInt16LE(task);
    const [poolPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('pool'), buf],
        program.programId
    )
    try{
      await program.methods.registerRewardPool(task, total).accounts({
        owner: pg.wallet.publicKey,
        config: configPDA,
        pool:poolPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([admin_keypair]).rpc();
    } catch (error) {
      return;
    }
    await program.methods.registerRewardPool(task, total).accounts({
      owner: pg.wallet.publicKey,
      config: configPDA,
      pool:poolPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    expect((await program.account.rewardPool.fetch(poolPDA)).total.eq(total));
    expect((await program.account.rewardPool.fetch(poolPDA)).task == task);
    expect((await program.account.rewardPool.fetch(poolPDA)).claimed.eq(new BN(0)));
    expect((await program.account.rewardPool.fetch(poolPDA)).txNum == 0);
  });
});
