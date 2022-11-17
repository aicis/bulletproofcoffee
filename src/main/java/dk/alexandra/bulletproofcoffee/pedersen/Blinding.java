package dk.alexandra.bulletproofcoffee.pedersen;

import java.util.Objects;

public record Blinding(byte[] bytes) {

    public Blinding {
        Objects.requireNonNull(bytes);
        if (bytes.length != 32) {
            throw new IllegalArgumentException("Commitment length has to be 32");
        }
    }

}
