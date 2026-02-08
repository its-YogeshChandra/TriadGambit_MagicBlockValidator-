import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TriadgambitMagicblockValidator } from "../target/types/triadgambit_magicblock_validator";

describe("triadgambit-magicblock-validator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.triadgambitMagicblockValidator as Program<TriadgambitMagicblockValidator>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
