package dk.alexandra.bulletproofcoffee;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;

public class FFILoader {
    private static boolean isLoaded;


    public static void loadLibrary() {
        if (isLoaded) {
            return;
        }

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

        isLoaded = true;
    }
}
