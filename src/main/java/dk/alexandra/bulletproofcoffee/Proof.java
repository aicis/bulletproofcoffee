package dk.alexandra.bulletproofcoffee;

public class Proof {

    // JNI calls depend on this being called bytes
    private final byte[] bytes;

    // The Native JNI calls depends on this signature
    public Proof(byte[] proof) {
        this.bytes = proof;
    }

    public byte[] getBytes() {
        return bytes;
    }
}
