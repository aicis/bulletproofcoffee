package dk.alexandra.bulletproofcoffee;

import dk.alexandra.bulletproofcoffee.pedersen.Generator;
import dk.alexandra.bulletproofcoffee.pedersen.Scalar;
import org.junit.jupiter.api.Test;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.Random;

import static org.junit.jupiter.api.Assertions.*;

class CommitmentTest {

    static Generator committer = new Generator();


    @Test
    void verifyLong() {
        var pair = committer.commit(123);
        var commit = pair.fst();
        var blinding = pair.snd();
        assertTrue(committer.verify(commit, 123, blinding));
    }

    @Test
    void verifyBigInteger() {
        var pair = committer.commit(BigInteger.TEN);
        var commit = pair.fst();
        var blinding = pair.snd();
        assertTrue(committer.verify(commit, BigInteger.TEN, blinding));
    }



    @Test
    void verifyWithBlinding() {
        var rand = new Random();
        rand.setSeed(7);
        var blinding = new BigInteger(32*4, rand);
        var pair = committer.commit(BigInteger.TEN, blinding);
        var commit = pair.fst();
        assertTrue(committer.verify(commit, BigInteger.TEN, new Scalar(blinding)),
                "rust blinding: "+ Arrays.toString(pair.snd().bytes()) +
                        "\njava blinding: "+ Arrays.toString(blinding.toByteArray())
        );
    }

    @Test
    void negativeVerifyLong() {
        var pair = committer.commit(2323);
        var commit = pair.fst();
        var blinding = pair.snd();
        assertFalse(committer.verify(commit, 2322, blinding));
    }

    @Test
    void testAdd() {
        var p1 = committer.commit(0x6666);
        var p2 = committer.commit(0x3333);
        var c1 = p1.fst();
        var c2 = p2.fst();
        var b1 = p1.snd();
        var b2 = p2.snd();

        var c3 = c1.add(c2);
        var v3 = 0x3333 + 0x6666;

        var b3 = b1.add(b2);
        assertTrue(committer.verify(c3, v3, b3));

    }


    @Test
    void testAddNegative() {
        var v1 = 0x6666;
        var v2 = 0x3333;
        var p1 = committer.commit(v1);
        var p2 = committer.commit(v2);
        var c1 = p1.fst();
        var c2 = p2.fst();
        var b1 = p1.snd();
        var b2 = p2.snd();

        var c3 = c1.add(c2);
        var v3 = v1 + v2 + 1;

        assertTrue(v3 != v1 + v2);

        var b3 = b1.add(b2);
        assertFalse(committer.verify(c3, v3, b3));

    }

    @Test
    void testInverseDarkMagic() {
        var m = new BigInteger("1001");
        var r = Scalar.ORDER.subtract(new BigInteger("12345"));

        var rng = new Random();
        rng.setSeed(0);
        var neg_r = Scalar.ORDER.subtract(r);
        var p1 = committer.commit(m);
        var p2 = committer.commit(r);
        var b_neg_r = Scalar.ORDER.subtract(p2.snd().toBigInteger());
        var p3 = committer.commit(neg_r, b_neg_r);

        var c_m = p1.fst();
        var c_r = p2.fst();
        var c_neg_r = p3.fst();
        var b_m = p1.snd();
        var b_r = p2.snd();

        var blinding = b_m.toBigInteger().subtract(b_r.toBigInteger()).mod(Scalar.ORDER);

        assertTrue(committer.verify(c_m.add(c_neg_r), m.subtract(r).mod(Scalar.ORDER), new Scalar(blinding)));

        var c_diff1= c_m.add(c_neg_r);
        var b_diff1= b_m.toBigInteger().add(b_neg_r);









    }

    @Test
    void testAddWithBlinding() {
        var rand = new Random();
        rand.setSeed(7);

        var v1 = new BigInteger("6666");
        var v2 = new BigInteger("3333");
        var p1 = committer.commit(v1, new BigInteger(32*4, rand));
        var p2 = committer.commit(v2, new BigInteger(32*4, rand));
        var c1 = p1.fst();
        var c2 = p2.fst();
        var b1 = p1.snd();
        var b2 = p2.snd();

        var c3 = c1.add(c2);
        var v3 = v1.add(v2);

        var b3 = b1.add(b2);
        assertTrue(committer.verify(c3, v3, b3),
                "Blinds:\n"
        );
    }

}