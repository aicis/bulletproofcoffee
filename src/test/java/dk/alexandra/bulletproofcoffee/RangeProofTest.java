package dk.alexandra.bulletproofcoffee;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class RangeProofTest {

    @Test
    public void valid32BitProof() {
        var triple = RangeProof.proveRange(1037578891, 32);
        var proof = triple.fst();
        var commitment = triple.snd();
        var success = RangeProof.verify(proof, commitment, 32);
        assertTrue(success);
    }

    @Test
    public void valid8BitProof() {
        var triple = RangeProof.proveRange(5, 8);
        var proof = triple.fst();
        var commitment = triple.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertTrue(success);
    }

    @Test
    public void invalid8BitProof() {
        var triple = RangeProof.proveRange(256, 8);
        var proof = triple.fst();
        var commitment = triple.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertFalse(success);
    }

    @Test
    public void negativeNumbersArePositive() {
        var triple = RangeProof.proveRange(-1, 32);
        var proof = triple.fst();
        var commitment = triple.snd();
        var success = RangeProof.verify(proof, commitment, 32);
        assertTrue(success);
    }

    @Test
    public void negativeNumbersArePositive8Bit() {
        var triple = RangeProof.proveRange(-256, 8);
        var proof = triple.fst();
        var commitment = triple.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertFalse(success);
    }

    @Test
    public void boundsNeedToMatch() {
        var triple = RangeProof.proveRange(2, 8);
        var proof = triple.fst();
        var commitment = triple.snd();
        var success = RangeProof.verify(proof, commitment, 16);
        assertFalse(success);
    }

    @Test
    public void tamperedCommitFails() {
        var triple = RangeProof.proveRange(1037578891, 32);
        var proof = triple.fst();
        var commitment = triple.snd();
        commitment.asBytes()[0] ^= -1;
        var success = RangeProof.verify(proof, commitment, 32);
        assertFalse(success);
    }

    @Test
    public void tamperedProofsThrowsFails() {
        var triple = RangeProof.proveRange(1037578891, 32);
        var proof = triple.fst();
        var commitment = triple.snd();
        proof.getBytes()[0] ^= -1;
        var success = RangeProof.verify(proof, commitment, 32);
        assertFalse(success);
    }

    @Test
    public void unparsebleProofsThrowsExceptions() {
        assertThrows(
                BulletProofException.class,
                () -> {
                    var triple = RangeProof.proveRange(1037578891, 32);
                    var commitment = triple.snd();
                    var proof = new Proof(new byte[]{});
                    RangeProof.verify(proof, commitment, 32);
                }
        );
    }

    @Test
    public void badBoundsProduceExceptions() {
        assertThrows(
            BulletProofException.class,
            () -> {
                RangeProof.proveRange(256, 9);
            }
        );
    }

}