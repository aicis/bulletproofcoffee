package dk.alexandra.bulletproofcoffee;

import javax.management.RuntimeMBeanException;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;
import java.util.Locale;

public class FFILoader {
    private static boolean isLoaded;

    public enum OS {
        WINDOWS, LINUX, MAC, SOLARIS
    }

    public enum ARCH {
        X86_64,
        AARCH64,
    }

    public static void loadLibrary() {
        if (isLoaded) {
            return;
        }

        String lib = null;
        switch (getOS()) {
            case LINUX -> {switch (getArch()) {
                case X86_64 -> lib = "libjava_ffi_x86.so";
                case AARCH64 -> lib = "libjava_ffi_aarch64.so";
            }}
            case MAC -> {switch (getArch()) {
                case X86_64 -> lib = "libjava_ffi_x86.dylib";
                case AARCH64 -> lib = "libjava_ffi_aarch64.dylib";
            }}
            case WINDOWS -> {switch (getArch()) {
                case X86_64 -> lib = "libjava_ffi_x86.dll";
                case AARCH64 -> throw new RuntimeException("No windows library for "+getArch()+" found");
            }}
        }

        try {
            ClassLoader classLoader = Thread.currentThread().getContextClassLoader();
            var link = classLoader.getResourceAsStream("native/"+lib);
            if (link == null) {
                link = classLoader.getResourceAsStream("native/libjava_ffi.dylib");
                if (link == null) {
                    throw new RuntimeException("Could not find bundled native library: native/"+lib);
                }
                System.err.println("Could not find target library, using default");
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

    private static ARCH getArch() {
        var arch =  System.getProperty("os.arch", "generic").toLowerCase(Locale.ENGLISH);
        if (arch.contains("x86")) {
            return ARCH.X86_64;
        } else if (arch.contains("aarch64")) {
            return ARCH.AARCH64;
        }
        throw new RuntimeException("Unknown architecture: "+arch);
    }
}
