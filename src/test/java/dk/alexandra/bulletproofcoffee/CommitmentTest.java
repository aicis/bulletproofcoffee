package dk.alexandra.bulletproofcoffee;

import dk.alexandra.bulletproofcoffee.pedersen.Blinding;
import dk.alexandra.bulletproofcoffee.pedersen.Commitment;
import dk.alexandra.bulletproofcoffee.pedersen.Committer;
import org.junit.jupiter.api.Test;

import java.math.BigInteger;
import java.util.Random;

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
    void verifyWithBlinding() {
        var rand = new Random();
        var blinding = new BigInteger(32*4, rand);
        var pair = Committer.commit(BigInteger.TEN, blinding);
        var commit = pair.fst();
        assertTrue(commit.verify(BigInteger.TEN, Blinding.from(blinding)));
    }

    @Test
    void negativeVerifyLong() {
        var pair = Committer.commit(2323);
        var commit = pair.fst();
        var blinding = pair.snd();
        assertFalse(commit.verify(2322, blinding));
    }
}