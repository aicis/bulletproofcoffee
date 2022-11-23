package dk.alexandra.bulletproofcoffee;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;
import java.util.Locale;

public class FFILoader {
    private static boolean isLoaded;

    private final static String PATH = "release/";
    private final static String NAME = "java_ffi";


    public enum OS {
        WINDOWS("pc-windows-gnu/"+PATH+NAME+".dll"),
        MAC("apple-darwin/"+PATH+"lib"+NAME+".dylib"),
        LINUX("unknown-linux-gnu/"+PATH+"lib"+NAME+".so"),
        SOLARIS("unknown-linux-gnu/"+PATH+"lib"+NAME+".so");

        public final String label;

        OS(String label) {
            this.label = label;
        }
    }

    public enum Arch {
        X86_64("x86_64"),
        AARCH64("aarch64");
        public final String prefix;

        Arch(String prefix) {
            this.prefix = prefix;
        }
    }

    public static void loadLibrary() {
        if (isLoaded) {
            return;
        }

        OS os = getOS();
        String lib = "native/" + getArch().prefix + "-" + os.label;


        try {
            ClassLoader classLoader = Thread.currentThread().getContextClassLoader();
            var link = classLoader.getResourceAsStream(lib);
            if (link == null) {
                throw new RuntimeException("Could not find bundled native library: native/"+lib);
            }
            var tmp = File.createTempFile("temp", "lib");
            tmp.deleteOnExit();
            Files.copy(link, tmp.toPath(), StandardCopyOption.REPLACE_EXISTING);
            System.load(tmp.getAbsolutePath());

            isLoaded = true;
        } catch (IOException e) {
            throw new RuntimeException("Could not load native library", e);
        }
    }

    private static OS getOS() {
        var os = System.getProperty("os.name", "generic").toLowerCase(Locale.ENGLISH);
        if (os.contains("win")) {
            return OS.WINDOWS;
        } else if (os.contains("mac")) {
            return OS.MAC;
        } else if (os.contains("nix") ||
                os.contains("nux") ||
                os.contains("aix")
        ) {
            return OS.LINUX;
        } else if (os.contains("sunos")) {
            return OS.SOLARIS;
        }
        throw new RuntimeException("Unknown OS: "+os);
    }

    private static Arch getArch() {
        var arch =  System.getProperty("os.arch", "generic").toLowerCase(Locale.ENGLISH);
        if (arch.contains("x86")) {
            return Arch.X86_64;
        } else if (arch.contains("aarch64")) {
            return Arch.AARCH64;
        }
        throw new RuntimeException("Unknown architecture: "+arch);
    }
}
