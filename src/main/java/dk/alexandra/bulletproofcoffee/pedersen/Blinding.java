package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.Util;

import java.math.BigInteger;
import java.util.Objects;

public record Blinding(byte[] bytes) {

    public Blinding {
        Objects.requireNonNull(bytes);
        if (bytes.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
    }

    public static Blinding from(BigInteger integer) {
        return new Blinding(Util.convertBigInteger(integer));
    }

}