package dk.alexandra.bulletproofcoffee;

public class Proof {

    private final byte[] proof;

    // The Native JNI calls depends on this signature
    public Proof(byte[] proof) {
        this.proof = proof;
    }

    // The Native JNI calls depends on this signature
    public byte[] asBytes() {
        return proof;
    }
}
