import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Yulugi01 } from "../target/types/yulugi01";

describe("yulugi01", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Yulugi01 as Program<Yulugi01>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
