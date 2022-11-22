package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.FFILoader;
import dk.alexandra.bulletproofcoffee.Pair;
import dk.alexandra.bulletproofcoffee.Util;
import org.jetbrains.annotations.NotNull;

import java.lang.annotation.Native;
import java.math.BigInteger;

public class Generator {

    static {
        FFILoader.loadLibrary();
    }


    @Native
    @SuppressWarnings("unused")
    private RistrettoPoint basePoint;

    @Native
    @SuppressWarnings("unused")
    private RistrettoPoint blindingBasePoint;

    @Native
    @SuppressWarnings("unused")
    private final boolean useDefault;

    public Generator(@NotNull RistrettoPoint basePoint, @NotNull RistrettoPoint blindingBasePoint) {
        this.basePoint = basePoint;
        this.blindingBasePoint = blindingBasePoint;
        useDefault = false;
    };

    public Generator() {
        useDefault = true;
    }



    /**
     * @param value value to commit to using OS randomness.
     * @return a pair consisting of generated commitment and the secret randomness to verify it
     * @throws IllegalArgumentException if value is negative.
     */
    public native Pair<RistrettoPoint, Scalar> commit(long value);


    /**
     * @param value Value to commit using OS randomness
     * @return a pair consisting of generated commitment and the secret randomness to verify it
     * @throws IllegalArgumentException if the supplied BigInteger is negative or zero,
     * or over 32 bytes long.
     */
    public Pair<RistrettoPoint, Scalar> commit(BigInteger value) {
        return commit(Util.convertBigInteger(value));
    }
    private native Pair<RistrettoPoint, Scalar> commit(byte[] value);

    /**
     * @param value Value to commit using OS randomness
     * @param blinding randomness to hide the commitment later used to open it again
     * @return a pair consisting of generated commitment and provided randomness
     * @throws IllegalArgumentException if the supplied BigInteger is negative or zero,
     * or over 32 bytes long.
     */
    public Pair<RistrettoPoint, Scalar> commit(BigInteger value, BigInteger blinding) {
        var v = Util.convertBigInteger(value);
        var b = Util.convertBigInteger(blinding);
        return commit(v,b);
    }

    private native Pair<RistrettoPoint, Scalar> commit(byte[] value, byte[] blinding);


    public native boolean verify(RistrettoPoint commitment, long value, Scalar blinding);

    private native boolean verify(RistrettoPoint commitment, byte[] value, Scalar blinding);


    public boolean verify(RistrettoPoint commitment, BigInteger value, Scalar blinding) {
        return verify(commitment, Util.convertBigInteger(value), blinding);
    }

}
