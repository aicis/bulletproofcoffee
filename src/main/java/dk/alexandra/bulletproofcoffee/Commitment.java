package dk.alexandra.bulletproofcoffee;

public class Commitment {

    private final byte[] commitment;

    public Commitment(byte[] commitment) {
        this.commitment = commitment;
    }

    public byte[] asBytes() {
        return commitment;
    }
}
