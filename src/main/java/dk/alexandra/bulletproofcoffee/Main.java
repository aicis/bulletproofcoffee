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
        var c1 = Committer.commit(10).fst();
        var c2 = Committer.commit(10).fst();
        var c3 = c1.add(c2);
        System.out.println(Arrays.toString(c1.asBytes()));
        System.out.println(Arrays.toString(c2.asBytes()));
        System.out.println(Arrays.toString(c3.asBytes()));

        c1.addSelf(c2);
        System.out.println(Arrays.toString(c1.asBytes()));

    }
}