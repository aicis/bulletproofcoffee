package dk.alexandra.bulletproofcoffee;

import dk.alexandra.bulletproofcoffee.pedersen.Scalar;
import dk.alexandra.bulletproofcoffee.pedersen.RistrettoPoint;

public class RangeProof {

    static {
        FFILoader.loadLibrary();
    }


    // JNI calls depend on this being called bytes
    private final byte[] bytes;

    // The Native JNI calls depends on this signature
    public RangeProof(byte[] proof) {
        this.bytes = proof;
    }

    public byte[] getBytes() {
        return bytes;
    }


    /**
     * Construct a range proof
     * @param secret that lies in range 0 <= secret <= bound
     * @param bound upper bound `n`, must be a power of 2
     * @return a pair consisting of a proof and commitment
     */
    public native static Triple<RangeProof, RistrettoPoint, Scalar> proveRange(long secret, int bound);

    /**
     *  Verify a range proof
     * @param proof that is to be verified
     * @param commitment used for verifying
     * @return true if the proof is valid
     */
    public native static boolean verify(RangeProof proof, RistrettoPoint commitment, int bound);

}
