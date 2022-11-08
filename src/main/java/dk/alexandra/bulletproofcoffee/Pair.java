package dk.alexandra.bulletproofcoffee;

// The Native JNI calls depends on this signature
public record Pair<T, S>(T fst, S snd) {

}
