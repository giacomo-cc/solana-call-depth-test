import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { P1 } from "../target/types/p1";
import { P2 } from "../target/types/p2";
import { P3 } from "../target/types/p3";
import { P4 } from "../target/types/p4";
import { P5 } from "../target/types/p5";
import { P6 } from "../target/types/p6";

describe("ping", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const programP1 = anchor.workspace.P1 as Program<P1>;
  const programP2 = anchor.workspace.P2 as Program<P2>;
  const programP3 = anchor.workspace.P3 as Program<P3>;
  const programP4 = anchor.workspace.P4 as Program<P4>;
  const programP5 = anchor.workspace.P5 as Program<P5>;
  const programP6 = anchor.workspace.P6 as Program<P6>;

  it("Is initialized!", async () => {
    for (let i = 0; i < 8; i++) {
      console.log("# # # # # # # # # # # # # # # # # # ");
      console.log("iteration: " + i);
      try {
        const tx = await programP1.rpc.initialize(new anchor.BN(i), {
          accounts: {
            p1: programP1.programId,
            p2: programP2.programId,
            p3: programP3.programId,
            p4: programP4.programId,
            p5: programP5.programId,
            p6: programP6.programId,
          },
        });
        console.log("tx: ", tx);
      } catch (err) {
        console.error(err);
      }
    }
  });
});
