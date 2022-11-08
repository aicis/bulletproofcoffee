package dk.alexandra.bulletproofcoffee;

public class Proof {

    private final byte[] proof;

    public Proof(byte[] proof) {
        this.proof = proof;
    }

    public byte[] asBytes() {
        return proof;
    }
}
