package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.FFILoader;
import dk.alexandra.bulletproofcoffee.Util;

import java.math.BigInteger;
import java.util.Objects;

public class Commitment {

    static {
        FFILoader.loadLibrary();
    }
    private final byte[] commitment;

    // The Native JNI calls depends on this signature
    public Commitment(byte[] commitment) {
        Objects.requireNonNull(commitment);
        if (commitment.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
        this.commitment = commitment;
    }

    public native boolean verify(long value, Blinding blinding);

    private native boolean verify(byte[] value, Blinding blinding);

    public boolean verify(BigInteger value, Blinding blinding) {
        return verify(Util.convertBigInteger(value), blinding);
    }

    // The Native JNI calls depends on this signature
    public byte[] asBytes() {
        return commitment;
    }

}
