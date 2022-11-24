package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.Util;

import java.lang.annotation.Native;
import java.math.BigInteger;
import java.util.Arrays;
import java.util.Objects;

/**
 * Blinding or randomness used for verifying a commitment.
 */
public final class Scalar {

    public final static BigInteger ORDER
            = new BigInteger("7237005577332262213973186563042994240857116359379907606001950938285454250989");

    @Native
    private final byte[] bytes;

    public Scalar(byte[] bytes) {
        Objects.requireNonNull(bytes);
        if (bytes.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
        this.bytes = bytes;
    }

    public Scalar(BigInteger integer) {
        this.bytes = Util.convertBigInteger(integer.mod(ORDER));
    }

    public BigInteger toBigInteger() {
        return new BigInteger(bytes);
    }

    public native Scalar add(Scalar other);

    public byte[] bytes() {
        return bytes;
    }


    @Override
    public boolean equals(Object obj) {
        if (super.equals(obj)) {
            return true;
        }
        if (obj instanceof Scalar other) {
            return Arrays.equals(this.bytes, other.bytes);
        }
        return false;
    }

    @Override
    public String toString() {
        return new BigInteger(1, bytes).toString();
    }
}
