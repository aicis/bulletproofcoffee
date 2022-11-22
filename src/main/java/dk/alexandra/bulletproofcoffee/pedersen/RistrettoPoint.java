package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.FFILoader;
import dk.alexandra.bulletproofcoffee.Util;

import java.math.BigInteger;
import java.util.Objects;

/**
 * Class representing a public commitment as a Compressed Ristretto255 Point.
 * The commitment can be verified using the provided randomness
 * {@link Scalar}
 * along with the value used in the commit phase.
 */
public class RistrettoPoint {

    static {
        FFILoader.loadLibrary();
    }

    // JNI requires that this is called bytes
    private final byte[] bytes;

    // The Native JNI calls depends on this signature
    public RistrettoPoint(byte[] commitment) {
        this.bytes = Objects.requireNonNull(commitment);
        if (commitment.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
    }

    public byte[] bytes() {
        return bytes;
    }

    /**
     * @param other Commitment to add
     * @return a new commitment being the sum of this and the other
     */
    public native RistrettoPoint add(RistrettoPoint other);

    /**
     * @param others Commitments to sum together
     * @return new Commitment representing to sum of the ptehrs
     */
    public native static RistrettoPoint sum(RistrettoPoint[] others);



}
