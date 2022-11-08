package dk.alexandra.bulletproofcoffee;

public class Commitment {

    private final byte[] commitment;

    // The Native JNI calls depends on this signature
    public Commitment(byte[] commitment) {
        this.commitment = commitment;
    }

    // The Native JNI calls depends on this signature
    public byte[] asBytes() {
        return commitment;
    }
}
