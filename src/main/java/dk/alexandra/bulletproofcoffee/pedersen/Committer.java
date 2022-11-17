package dk.alexandra.bulletproofcoffee.pedersen;

import dk.alexandra.bulletproofcoffee.FFILoader;
import dk.alexandra.bulletproofcoffee.Pair;
import dk.alexandra.bulletproofcoffee.Util;

import java.math.BigInteger;

public class Committer {

    static {
        FFILoader.loadLibrary();
    }

    /**
     * @param value value to commit to using OS randomness.
     * @return a pair consisting of generated commitment and the secret randomness to verify it
     * @throws IllegalArgumentException if value is negative.
     */
    public static native Pair<Commitment, Blinding> commit(long value);


    /**
     * @param value Value to commit using OS randomness
     * @return a pair consisting of generated commitment and the secret randomness to verify it
     * @throws IllegalArgumentException if the supplied BigInteger is negative or zero,
     * or over 32 bytes long.
     */
    public static Pair<Commitment, Blinding> commit(BigInteger value) {
        return commit(Util.convertBigInteger(value));
    }
    private static native Pair<Commitment, Blinding> commit(byte[] value);

    /**
     * @param value Value to commit using OS randomness
     * @param blinding randomness to hide the commitment later used to open it again
     * @return a pair consisting of generated commitment and provided randomness
     * @throws IllegalArgumentException if the supplied BigInteger is negative or zero,
     * or over 32 bytes long.
     */
    public static Pair<Commitment, Blinding> commit(BigInteger value, BigInteger blinding) {
        var v = Util.convertBigInteger(value);
        var b = Util.convertBigInteger(blinding);
        return commit(v,b);
    }

    private static native Pair<Commitment, Blinding> commit(byte[] value, byte[] blinding);

}
