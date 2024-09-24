import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplAccountCompressionIdlIssue } from "../target/types/spl_account_compression_idl_issue";

describe("spl-account-compression-idl-issue", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplAccountCompressionIdlIssue as Program<SplAccountCompressionIdlIssue>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
