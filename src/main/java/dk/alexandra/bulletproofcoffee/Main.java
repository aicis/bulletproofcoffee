package dk.alexandra.bulletproofcoffee;

public class Main {
    public static void main(String[] args) {
        var pair = FFI.proveRange(2000, 15);
        var proof = pair.fst();
        var commitment = pair.snd();

        var success = FFI.verify(proof, commitment);
        System.out.println("Successfully called function!");
        if (success) {
            System.out.println("Verified proof");
        } else {
            System.err.println("Proof failed");
        }
    }
}