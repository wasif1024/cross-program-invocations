import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { Keypair } from '@solana/web3.js'
import { expect } from 'chai'
import { Child } from "../target/types/child";
import { Master } from "../target/types/master";
//import { Child } from '../target/types/master'

describe("child", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)


  const childProgram = anchor.workspace.Child as Program<Child>
  const masterProgram = anchor.workspace
    .Master as Program<Master>


  const childKeypair = Keypair.generate()
  const authorityKeypair = Keypair.generate()

  it('Does CPI!', async () => {
    await childProgram.methods
      .initialize(authorityKeypair.publicKey)
      .accounts({
        child: childKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([childKeypair])
      .rpc()


    await masterProgram.methods
      .pullStrings(new anchor.BN(42))
      .accounts({
        childProgram: childProgram.programId,
        child: childKeypair.publicKey,
        authority: authorityKeypair.publicKey
      }).signers([authorityKeypair])
      .rpc()


    expect(
      (
        await childProgram.account.data.fetch(childKeypair.publicKey)
      ).data.toNumber()
    ).to.equal(42)
  })
});