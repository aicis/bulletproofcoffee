package dk.alexandra.bulletproofcoffee;

import dk.alexandra.bulletproofcoffee.pedersen.RistrettoPoint;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.Random;

public class RistrettoTest {

    private static Random rng;
    @BeforeAll
    public static void setup() {
        rng = new Random();
        rng.setSeed(0);
    }

    private static RistrettoPoint getPoint() {
        var bytes = new byte[64];
        rng.nextBytes(bytes);
        return RistrettoPoint.fromUniformBytes(bytes);
    }

    @Test
    public void testFromUniformBytes() {
        var bytes = new byte[64];
        rng.nextBytes(bytes);
        var p = RistrettoPoint.fromUniformBytes(bytes);
        // assume ok if nothing fails
    }

    @Test
    public void testAdd() {
        getPoint().add(getPoint());
        // assume ok if nothing fails
    }

    @Test
    public void testSum() {
        var list = new ArrayList<RistrettoPoint>();
        for (int i = 0; i < 32; i++) {
            list.add(getPoint());
        }
        RistrettoPoint.sum(list);
        // assume ok if nothing fails
    }
}
