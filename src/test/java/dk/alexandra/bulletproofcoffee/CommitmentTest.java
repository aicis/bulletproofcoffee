package dk.alexandra.bulletproofcoffee;

import dk.alexandra.bulletproofcoffee.pedersen.Commitment;
import dk.alexandra.bulletproofcoffee.pedersen.Committer;
import org.junit.jupiter.api.Test;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.*;

class CommitmentTest {

    @Test
    void verifyLong() {
        var pair = Committer.commit(123);
        var commit = pair.fst();
        var blinding = pair.snd();
        assertTrue(commit.verify(123, blinding));
    }

    @Test
    void verifyBigInteger() {
        var pair = Committer.commit(BigInteger.TEN);
        var commit = pair.fst();
        var blinding = pair.snd();
        assertTrue(commit.verify(BigInteger.TEN, blinding));
    }

    @Test
    void negativeVerifyLong() {
        var pair = Committer.commit(2323);
        var commit = pair.fst();
        var blinding = pair.snd();
        assertFalse(commit.verify(2322, blinding));
    }
}