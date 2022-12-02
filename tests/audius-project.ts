import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
const assert = require('assert');
import { Myepicproject } from "../target/types/myepicproject";
import { create } from 'ipfs-core';
//import  main  from './main'



describe("audius-project", async () => {
  // Configure the client to use the local cluster.
  
  const provider = anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Myepicproject as Program<Myepicproject>;
  it('Checks that a track was uploaded', async () => {
    const baseAccount = anchor.web3.Keypair.generate();
    await program.rpc.startStuffOff({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [ baseAccount ],
    });

    await program.rpc.addGif('TOPIC HERE', "hellp",{
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    });

    // await program.rpc.printCid(1, {
    //   accounts: {
    //     baseAccount: baseAccount.publicKey,
    //     user: program.provider.wallet.publicKey,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //   }
    // });
  
    // The account should have been created.
    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    const count = account.cidCount.toNumber()
    console.log('Track info ', account.cidList[0]);

});

});
