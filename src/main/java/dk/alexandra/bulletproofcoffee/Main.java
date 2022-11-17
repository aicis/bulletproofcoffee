package dk.alexandra.bulletproofcoffee;

import dk.alexandra.bulletproofcoffee.pedersen.Committer;

import java.util.Arrays;

public class Main {

    public static void main(String[] args) {
//        var pair = RangeProof.proveRange(16, 16);
//        var proof = pair.fst();

//
//        var success = RangeProof.verify(proof, commitment, 16);
//        System.out.println("Successfully called function!");
//        if (success) {
//            System.out.println("Verified proof");
//        } else {
//            System.err.println("Proof failed");
//        }
//        var commit = new Commitment(BigInteger.TEN);
        var pair = Committer.commit(10);
        var commit = pair.fst();
        var blinding = pair.snd();
        System.out.println(Arrays.toString(commit.asBytes()));
        System.out.println(Arrays.toString(blinding.bytes()));

        if (commit.verify(10, blinding)) {
            System.out.println("Yay");
        } else {
            System.err.println("BAD");
        }
    }
}