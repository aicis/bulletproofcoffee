package dk.alexandra.bulletproofcoffee;

import dk.alexandra.bulletproofcoffee.pedersen.Blinding;
import dk.alexandra.bulletproofcoffee.pedersen.Commitment;
import dk.alexandra.bulletproofcoffee.pedersen.Committer;
import org.junit.jupiter.api.Test;

import java.math.BigInteger;
import java.util.Arrays;
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
        rand.setSeed(7);
        var blinding = new BigInteger(32*4, rand);
        var pair = Committer.commit(BigInteger.TEN, blinding);
        var commit = pair.fst();
        assertTrue(commit.verify(BigInteger.TEN, Blinding.from(blinding)),
                "rust blinding: "+ Arrays.toString(pair.snd().bytes()) +
                        "\njava blinding: "+ Arrays.toString(blinding.toByteArray())
        );
    }

    @Test
    void negativeVerifyLong() {
        var pair = Committer.commit(2323);
        var commit = pair.fst();
        var blinding = pair.snd();
        assertFalse(commit.verify(2322, blinding));
    }

    @Test
    void testAdd() {
        var p1 = Committer.commit(0x6666);
        var p2 = Committer.commit(0x3333);
        var c1 = p1.fst();
        var c2 = p2.fst();
        var b1 = p1.snd();
        var b2 = p2.snd();

        var c3 = c1.add(c2);
        var v3 = 0x3333 + 0x6666;

        var b3 = b1.add(b2);
        assertTrue(c3.verify(v3, b3));

    }


    @Test
    void testAddNegative() {
        var v1 = 0x6666;
        var v2 = 0x3333;
        var p1 = Committer.commit(v1);
        var p2 = Committer.commit(v2);
        var c1 = p1.fst();
        var c2 = p2.fst();
        var b1 = p1.snd();
        var b2 = p2.snd();

        var c3 = c1.add(c2);
        var v3 = v1 + v2 + 1;

        assertTrue(v3 != v1 + v2);

        var b3 = b1.add(b2);
        assertFalse(c3.verify(v3, b3));

    }

    @Test
    void testAddWithBlinding() {
        var rand = new Random();
        rand.setSeed(7);

        var v1 = new BigInteger("6666");
        var v2 = new BigInteger("3333");
        var p1 = Committer.commit(v1, new BigInteger(32*4, rand));
        var p2 = Committer.commit(v2, new BigInteger(32*4, rand));
        var c1 = p1.fst();
        var c2 = p2.fst();
        var b1 = p1.snd();
        var b2 = p2.snd();

        var c3 = c1.add(c2);
        var v3 = v1.add(v2);

        var b3 = b1.add(b2);
        assertTrue(c3.verify(v3, b3),
                "Blinds:\n"
        );
    }

}