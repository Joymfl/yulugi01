import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Chatroom } from "../target/types/chatroom";

describe("chatroom", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Chatroom as Program<Chatroom>;
  const chatRoom = anchor.web3.Keypair.generate();
  const chatRoom_user1 = anchor.web3.Keypair.generate();
  const chatRoom_user2 = anchor.web3.Keypair.generate();
  let message = "Hello";

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(message, chatRoom_user1.publicKey, chatRoom_user2.publicKey).accounts({
      chatRoom: chatRoom.publicKey,
      user: chatRoom_user1.secretKey,
    }).signers([chatRoom_user1]).rpc();
    console.log("Your transaction signature", tx);
  });
});
