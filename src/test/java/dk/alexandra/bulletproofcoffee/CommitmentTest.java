package dk.alexandra.bulletproofcoffee;

import org.junit.jupiter.api.Test;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.*;

class CommitmentTest {

    @Test
    void verifyLong() {
        var c = new Commitment(123);
        assertTrue(c.verify(123));
    }

    @Test
    void verifyBigInteger() {
        var c = new Commitment(BigInteger.TEN);
        assertTrue(c.verify(BigInteger.TEN));
    }
}