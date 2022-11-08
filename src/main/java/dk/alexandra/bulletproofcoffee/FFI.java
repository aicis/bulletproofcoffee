package dk.alexandra.bulletproofcoffee;

import java.io.File;

public class FFI {

    private FFI() {}

    static {
        // TODO: Use a more global release version.
        File lib = new File("rust-jni/target/debug/libjava_ffi.dylib");
        System.load(lib.getAbsolutePath());
    }


    public native static Pair<Proof, Commitment> prove(long secret);

    public native static boolean verify(Proof proof, Commitment commitment);


//    public native static Pair<Proof, Commitment> prove(long secret);
//
//    public native static boolean verify(Proof proof, Commitment commitment);
//
//    public native static List<Pair<Proof, Commitment>> prove(List<Long> secrets);
//
//    public native static List<Boolean> verify(List<Proof> proof, List<Commitment> commitment);
}
