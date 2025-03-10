import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiA } from "../target/types/cpi_a";



describe("cpi-a",async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const programA = anchor.workspace.cpi_a as Program<CpiA>;

  const signer = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {

    const [pda_address,bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("superForgerer"),signer.publicKey.toBuffer()],
      program.programId
    )

  // Airdrop 100 lamports to signer
  let x =await program.provider.connection.requestAirdrop(pda_address,100*anchor.web3.LAMPORTS_PER_SOL)
  console.log("airdrop tx is ",x);
  await program.provider.connection.confirmTransaction(x,
    "finalized"
  )  
  //////////////////////////////////////
  

    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      pdaAccount:pda_address,
      signer:signer.publicKey,
      system_program:anchor.web3.SystemProgram.programId
      
    }).signers([signer]).rpc();
    console.log("Your transaction signature", tx);

  });


  /**
   * 
   *     try{
    const airdropTx = await program.provider.connection.requestAirdrop(
      signer.publicKey,
      100 * anchor.web3.LAMPORTS_PER_SOL
    );
    console.log("airdrop tx", airdropTx);

    const airdropConf = await program.provider.connection.confirmTransaction(
      airdropTx,
      "finalized"
    );
    // Add your test here.
    const txSignature = await program.methods
      .initialize("Hello World")
      .accounts({
        signer: signer.publicKey,
        dataAccount: data_account.publicKey,
      })
      .signers([signer, data_account])
      .rpc();
    console.log("Your transaction signature", txSignature);
  } catch (error) {
    console.error("Error:", error);
  }

   */
});
