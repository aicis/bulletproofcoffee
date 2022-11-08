package dk.alexandra.bulletproofcoffee;

public class BulletProofException extends Exception {

    // The Native JNI calls depends on this signature
    public BulletProofException(String message) {
        super(message);
    }

}
