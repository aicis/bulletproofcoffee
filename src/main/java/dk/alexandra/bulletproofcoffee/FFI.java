package dk.alexandra.bulletproofcoffee;

import java.io.File;

public class FFI {

    private FFI() {}

    static {
        // TODO: Use a more global release version.
        File lib = new File("rust-jni/target/debug/libjava_ffi.dylib");
        System.load(lib.getAbsolutePath());
    }

    public native static Pair<Proof, Commitment> proveRange(long secret, int bound);

    public native static boolean verify(Proof proof, Commitment commitment);

}
