package dk.alexandra.bulletproofcoffee;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class RangeProofTest {

    @Test
    public void valid32BitProof() {
        var pair = RangeProof.proveRange(1037578891, 32);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 32);
        assertTrue(success);
    }

    @Test
    public void valid8BitProof() {
        var pair = RangeProof.proveRange(5, 8);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertTrue(success);
    }

    @Test
    public void invalid8BitProof() {
        var pair = RangeProof.proveRange(256, 8);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertFalse(success);
    }

    @Test
    public void negativeNumbersArePositive() {
        var pair = RangeProof.proveRange(-1, 32);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 32);
        assertTrue(success);
    }

    @Test
    public void negativeNumbersArePositive8Bit() {
        var pair = RangeProof.proveRange(-256, 8);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertFalse(success);
    }

    @Test
    public void boundsNeedToMatch() {
        var pair = RangeProof.proveRange(2, 8);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 16);
        assertFalse(success);
    }

    @Test
    public void tamperedCommitFails() {
        var pair = RangeProof.proveRange(1037578891, 32);
        var proof = pair.fst();
        var commitment = pair.snd();
        commitment.asBytes()[0] ^= -1;
        var success = RangeProof.verify(proof, commitment, 32);
        assertFalse(success);
    }

    @Test
    public void tamperedProofsThrowsFails() {
        var pair = RangeProof.proveRange(1037578891, 32);
        var proof = pair.fst();
        var commitment = pair.snd();
        proof.asBytes()[0] ^= -1;
        var success = RangeProof.verify(proof, commitment, 32);
        assertFalse(success);
    }

    @Test
    public void unparsebleProofsThrowsExceptions() {
        assertThrows(
                BulletProofException.class,
                () -> {
                    var pair = RangeProof.proveRange(1037578891, 32);
                    var commitment = pair.snd();
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