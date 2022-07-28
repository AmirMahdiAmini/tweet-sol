import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { Voolibow } from "../target/types/voolibow";
import {clusterApiUrl,LAMPORTS_PER_SOL} from "@solana/web3.js";

describe("voolibow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Voolibow  as Program<Voolibow>


  let user = provider.wallet;
  const tweetKeypair = anchor.web3.Keypair.generate();


  const sendTweet = async (user: any, tag: string, content: string) => {
		await program.methods.sendTweet(tag, content)
			.accounts({
				tweet: tweetKeypair.publicKey,
				user: user.publicKey,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.signers([tweetKeypair])
			.rpc();

		const tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
		return { publicKey: tweetKeypair.publicKey, account: tweet }
	};

  it("can send tweet", async () => {
    const tweet = await sendTweet(user, "game", "let's play warzone");
    assert.equal(tweet.account.user.toBase58(), provider.wallet.publicKey.toBase58());
			assert.equal(tweet.account.tag, "game");
			assert.equal(tweet.account.content, "let's play warzone");
			assert.ok(tweet.account.timestamp);
  });
  it("can update tweet",async ()=>{
    const tweet = await sendTweet(user, "game", "let's play warzone");
    assert.equal(tweet.account.user.toBase58(), provider.wallet.publicKey.toBase58());
			assert.equal(tweet.account.tag, "game");
			assert.equal(tweet.account.content, "let's play warzone");
			assert.ok(tweet.account.timestamp);


    await program.methods.updateTweet("COD", "warzone")
      .accounts({ tweet: tweet.publicKey, user: tweetKeypair.publicKey })
      .signers([tweetKeypair])
      .rpc();

    const updatedTweet = await program.account.tweet.fetch(tweet.publicKey);
    assert.equal(updatedTweet.tag, "COD");
    assert.equal(updatedTweet.content, "warzone");
  })
});
