package dk.alexandra.bulletproofcoffee;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class RangeProofTest {

    @Test
    public void Valid32BitProof() {
        var pair = RangeProof.proveRange(1037578891, 32);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 32);
        assertTrue(success);
    }

    @Test
    public void Valid8BitProof() {
        var pair = RangeProof.proveRange(5, 8);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertTrue(success);
    }

    @Test
    public void Invalid8BitProof() {
        var pair = RangeProof.proveRange(256, 8);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertFalse(success);
    }

    @Test
    public void NegativeNumbersArePositive() {
        var pair = RangeProof.proveRange(-1, 32);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 32);
        assertTrue(success);
    }

    @Test
    public void NegativeNumbersArePositive8Bit() {
        var pair = RangeProof.proveRange(-256, 8);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 8);
        assertFalse(success);
    }

    @Test
    public void BoundsNeedToMatch() {
        var pair = RangeProof.proveRange(2, 8);
        var proof = pair.fst();
        var commitment = pair.snd();
        var success = RangeProof.verify(proof, commitment, 16);
        assertFalse(success);
    }

    @Test
    public void TamperedCommitFails() {
        var pair = RangeProof.proveRange(1037578891, 32);
        var proof = pair.fst();
        var commitment = pair.snd();
        commitment.asBytes()[0] ^= -1;
        var success = RangeProof.verify(proof, commitment, 32);
        assertFalse(success);
    }

    @Test
    public void TamperedProofsThrowsFails() {
        var pair = RangeProof.proveRange(1037578891, 32);
        var proof = pair.fst();
        var commitment = pair.snd();
        proof.asBytes()[0] ^= -1;
        var success = RangeProof.verify(proof, commitment, 32);
        assertFalse(success);
    }

    @Test
    public void UnparsebleProofsThrowsExceptions() {
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
    public void BadBoundsProduceExceptions() {
        assertThrows(
            BulletProofException.class,
            () -> {
                RangeProof.proveRange(256, 9);
            }
        );
    }

}