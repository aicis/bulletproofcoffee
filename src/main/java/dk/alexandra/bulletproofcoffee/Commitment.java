package dk.alexandra.bulletproofcoffee;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.Objects;

public class Commitment {

    static {
        FFILoader.loadLibrary();
    }
    private byte[] commitment;
    private byte[] blinding;

    // The Native JNI calls depends on this signature
    public Commitment(byte[] commitment, byte[] blinding) {
        Objects.requireNonNull(commitment);
        Objects.requireNonNull(blinding);

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
     * or over 32 bytes long.
     */
    public Commitment(BigInteger value) {
        // TODO: Make static method since the JNI becomes mad
        if (value.compareTo(BigInteger.ZERO) <= 0) {
            throw new IllegalArgumentException("Value has to be positive and non-zero");
        }
        var bytes = value.toByteArray();
        if (bytes.length < 32) {
            int startPos =  32  - bytes.length;
            var res = new byte[32];
            System.arraycopy(bytes, 0, res, startPos, bytes.length);
            newCommitmentFromBytes(res);
        } else if (bytes.length == 32) {
            newCommitmentFromBytes(bytes);
        } else {
            throw new IllegalArgumentException("Value is too large (over 256 bit)");
        }
    }

    /**
     * @param value value to commit to using OS randomness.
     * @throws IllegalArgumentException if value is negative.
     */
    public Commitment(long value) {
        // TODO: Make static method since the JNI becomes mad
        if (value <= 0) {
            throw new IllegalArgumentException("Value has to be positive and non-zero");
        }
        newCommitmentFromLong(value);
    };
    public static native Commitment newCommitmentFromBytes(byte[] value);

    public static native Commitment newCommitmentFromLong(long value);

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
