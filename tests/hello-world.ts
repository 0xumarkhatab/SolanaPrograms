import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorld } from "../target/types/hello_world";



describe("hello-world",async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HelloWorld as Program<HelloWorld>;
  const signer = anchor.web3.Keypair.generate();
  const data_account = anchor.web3.Keypair.generate();

  // await program.provider.connection.confirmTransaction(await program.provider.connection.requestAirdrop(signer.publicKey,100*anchor.web3.LAMPORTS_PER_SOL),"finalized")

  

  it("Is initialized!", async () => {
  let x =await program.provider.connection.requestAirdrop(signer.publicKey,100*anchor.web3.LAMPORTS_PER_SOL)
  console.log("airdrop tx is ",x);
  let conf_tx = await program.provider.connection.confirmTransaction(x,
    "finalized"
  )
  console.log("conf ",conf_tx);
  

    // Add your test here.
    const tx = await program.methods.initialize("Hello World").accounts({
      signer:signer.publicKey,
      dataAccount:data_account.publicKey,
        systemProgram:anchor.web3.SystemProgram.programId,
      
    }).signers([signer,data_account]).rpc();
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
