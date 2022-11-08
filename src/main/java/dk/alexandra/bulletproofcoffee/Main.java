package dk.alexandra.bulletproofcoffee;

public class Main {
    public static void main(String[] args) {
        var pair = FFI.prove(2 << 30);
        var proof = pair.fst();
        var commitment = pair.snd();
//        commitment.asBytes()[0] = 1;

        var success = FFI.verify(proof, commitment);
        System.out.println("Successfully called function!");
        if (success) {
            System.out.println("Verified proof");
        } else {
            System.err.println("Proof failed");
        }
    }
}