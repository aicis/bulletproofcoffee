package dk.alexandra.bulletproofcoffee;

import java.io.File;

public class FFI {

    private FFI() {}

    static {
        // TODO: Use a more global release version.
        File lib = new File("rust-jni/target/debug/libjava_ffi.dylib");
        System.load(lib.getAbsolutePath());
    }

    /**
     * Construct a range proof
     * @param secret that lies in range 0 <= secret <= bound
     * @param bound upper bound `n`, must be a power of 2
     * @return a pair consisting of a proof and commitment
     */
    public native static Pair<Proof, Commitment> proveRange(long secret, int bound);

    /**
     *  Verify a range proof
     * @param proof that is to be verified
     * @param commitment used for verifying
     * @return true if the proof is valid
     */
    public native static boolean verify(Proof proof, Commitment commitment);

}
