package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.Util;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.Collections;
import java.util.Objects;

/**
 * Blinding or randomness used for verifying a commitment.
 */
public final class Blinding {
    private final byte[] bytes;

    public Blinding(byte[] bytes) {
        Objects.requireNonNull(bytes);
        if (bytes.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
        this.bytes = bytes;
    }

    public Blinding(BigInteger integer) {
        this.bytes = Util.convertBigInteger(integer);
    }

    public BigInteger toBigInteger() {
        return new BigInteger(bytes);
    }

    public native Blinding add(Blinding other);

    public byte[] bytes() {
        return bytes;
    }

}
