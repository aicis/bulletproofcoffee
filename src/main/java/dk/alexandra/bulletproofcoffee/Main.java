package dk.alexandra.bulletproofcoffee;

import java.math.BigInteger;
import java.util.Arrays;

public class Main {

    static class Test {
        private Commitment commitment;
        public Test() {
            commitment = Commitment.newCommitmentFromLong(10);

        }

        public Commitment getCommitment() {
            return commitment;
        }
    }
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
        var test = new Test();
        var commit = test.getCommitment();
        System.out.println(Arrays.toString(commit.asBytes()));
        System.out.println(Arrays.toString(commit.getBlinding()));

        if (commit.verify(10)) {
            System.out.println("Yay");
        } else {
            System.err.println("BAD");
        }
    }
}