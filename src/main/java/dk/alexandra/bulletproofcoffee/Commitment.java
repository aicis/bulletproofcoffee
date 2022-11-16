package dk.alexandra.bulletproofcoffee;

import java.math.BigInteger;

public class Commitment {

    static {
        FFILoader.loadLibrary();
    }

    private byte[] commitment;
    private byte[] blinding;

    // The Native JNI calls depends on this signature
    public Commitment(byte[] commitment, byte[] blinding) {
        if (commitment.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
        if (blinding.length != 32) {
            throw new IllegalArgumentException("Blinding length has to be 32");
        }

        this.commitment = commitment;
        this.blinding = blinding;
    }

    /** Commit to value
     * @param value Value to commit using OS randomness
     * @throws IllegalArgumentException if the supplied BigInteger is negative or zero,
     * or not exactly 32 bytes long.
     */
    public Commitment(BigInteger value) {
        if (value.compareTo(BigInteger.ZERO) <= 0) {
            throw new IllegalArgumentException("Value has to be positive and non-zero");
        }
        if (value.bitLength() / 8 + 1 != 32) {
            // TODO: Fill big integer if too small
            throw new IllegalArgumentException("Big integer has to be 32 bytes");
        }
        newCommitmentFromBytes(value.toByteArray());
    }
    ;

    /**
     * @param value value to commit to using OS randomness.
     * @throws IllegalArgumentException if value is negative.
     */
    public Commitment(long value) {
        if (value <= 0) {
            throw new IllegalArgumentException("Value has to be positive and non-zero");
        }
        newCommitmentFromLong(value);
    };
    private static native Commitment newCommitmentFromBytes(byte[] value);
    private static native Commitment newCommitmentFromLong(long value);

    public native boolean verify(long value);
    private native boolean verify(byte[] value);
    public boolean verify(BigInteger value) {
        if (value.compareTo(BigInteger.ZERO) <= 0) {
            throw new IllegalArgumentException("Value has to be positive and non-zero");
        }
        if (value.bitLength() / 8 + 1 != 32) {
            // TODO: Fill big integer if too small
            throw new IllegalArgumentException("Big integer has to be 32 bytes");
        }
        return verify(value.toByteArray());
    }


    // The Native JNI calls depends on this signature
    public byte[] asBytes() {
        return commitment;
    }
    public byte[] getBlinding() {
        return blinding;
    }

}
