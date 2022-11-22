package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.Util;

import java.math.BigInteger;
import java.util.Objects;

/**
 * Blinding or randomness used for verifying a commitment.
 */
public final class Scalar {
    private final byte[] bytes;

    public Scalar(byte[] bytes) {
        Objects.requireNonNull(bytes);
        if (bytes.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
        this.bytes = bytes;
    }

    public Scalar(BigInteger integer) {
        this.bytes = Util.convertBigInteger(integer);
    }

    public BigInteger toBigInteger() {
        return new BigInteger(bytes);
    }

    public native Scalar add(Scalar other);

    public byte[] bytes() {
        return bytes;
    }

}
