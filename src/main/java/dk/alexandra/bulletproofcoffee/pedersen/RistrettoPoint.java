package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.FFILoader;
import dk.alexandra.bulletproofcoffee.Util;
import org.jetbrains.annotations.NotNull;

import java.lang.annotation.Native;
import java.math.BigInteger;
import java.util.Arrays;
import java.util.HexFormat;
import java.util.List;
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
    @Native
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
     * Constructs a RistrettoPoint from uniformly distributed bytes, which maps using elligator
     * to a uniformly distributed RistrettoPoint.
     * @param bytes a byte array of size 64
     * @return uniformly distributed RistrettoPoint
     * @throws IllegalArgumentException if bytes is any other length than 64
     */
    public static native RistrettoPoint fromUniformBytes(byte[] bytes);

    /**
     * @param other Scalar to multiply with
     * @return a new commitment being the sum of this and the other
     */
    public native RistrettoPoint mul(Scalar other);

    /**
     * @param other RistrettoPoint to add
     * @return a new commitment being the sum of this and the other
     */
    public native RistrettoPoint add(RistrettoPoint other);

    /**
     * @param others RistrettoPoints to sum together
     * @return new Commitment representing to sum of the ptehrs
     */
    public native static RistrettoPoint sum(List<RistrettoPoint> others);

    @Override
    public boolean equals(Object obj) {
        if (super.equals(obj)) {
            return true;
        }
        if (obj instanceof RistrettoPoint other) {
            return Arrays.equals(this.bytes, other.bytes);
        }
        return false;
    }

    @Override
    public String toString() {
        String hex = new BigInteger(1, bytes).toString(16);
        return "RistrettoPoint{" +
                "bytes=" + hex +
                '}';
    }
}
