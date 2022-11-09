package dk.alexandra.bulletproofcoffee;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;

public class RangeProof {

    private RangeProof() {}

    static {
        var arch = System.getProperty("os.arch");
        var os = System.getProperty("os.name");
        String lib = null;
        switch (os+":"+arch) {
            case "Mac OS X:aarch64" -> lib = "libjava_ffi.dylib";
            case "Mac OS X:amd64" -> lib = "libjava_ffi64.dylib";
            default -> throw new RuntimeException("Unsupported architecture: " + os + " " + arch);
        }
        try {
            var tmp = File.createTempFile("temp", "lib");
            var link = RangeProof.class.getClassLoader().getResourceAsStream("native/"+lib);
            if (link == null) {
                throw new RuntimeException("Could not find bundled native library");
            }
            Files.copy(link, tmp.toPath(), StandardCopyOption.REPLACE_EXISTING);
            System.load(tmp.getAbsolutePath());
        } catch (IOException e) {
            throw new RuntimeException("Could not load native library", e);
        }
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
    public native static boolean verify(Proof proof, Commitment commitment, int bound);

}
