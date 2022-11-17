package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.FFILoader;
import dk.alexandra.bulletproofcoffee.Util;

import java.math.BigInteger;
import java.util.Objects;

/**
 * Class representing a public commitment as a Compressed Ristretto255 Point.
 * The commitment can be verified using the provided randomness
 * {@link dk.alexandra.bulletproofcoffee.pedersen.Blinding}
 * along with the value used in the commit phase.
 */
public class Commitment {

    static {
        FFILoader.loadLibrary();
    }

    // JNI requires that this is called bytes
    private final byte[] bytes;

    // The Native JNI calls depends on this signature
    public Commitment(byte[] commitment) {
        this.bytes = Objects.requireNonNull(commitment);
        if (commitment.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
    }

    public native boolean verify(long value, Blinding blinding);

    private native boolean verify(byte[] value, Blinding blinding);

    public boolean verify(BigInteger value, Blinding blinding) {
        return verify(Util.convertBigInteger(value), blinding);
    }

    // The Native JNI calls depends on this signature
    public byte[] asBytes() {
        return bytes;
    }

    /**
     * @param other Commitment to add
     * @return a new commitment being the sum of this and the other
     */
    public native Commitment add(Commitment other);

    /**
     * @param others Commitments to sum together
     * @return new Commitment representing to sum of the ptehrs
     */
    public native static Commitment sum(Commitment[] others);
}
