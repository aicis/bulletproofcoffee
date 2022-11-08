package dk.alexandra.bulletproofcoffee;

public class Main {
    public static void main(String[] args) {
        var pair = RangeProof.proveRange(256, 16);
        var proof = pair.fst();
        var commitment = pair.snd();

        var success = RangeProof.verify(proof, commitment, 16);
        System.out.println("Successfully called function!");
        if (success) {
            System.out.println("Verified proof");
        } else {
            System.err.println("Proof failed");
        }
    }
}