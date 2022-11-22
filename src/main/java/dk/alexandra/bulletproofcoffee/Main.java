package dk.alexandra.bulletproofcoffee;

import dk.alexandra.bulletproofcoffee.pedersen.Generator;

import java.math.BigInteger;

public class Main {

    public static void main(String[] args) {
        var committer = new Generator();
        var p1 = committer.commit(new BigInteger("1"));
        var p2 = committer.commit(new BigInteger("2"));
        var c1 = p1.fst();
        var b1 = p1.snd();
        var c2 = p2.fst();
        var b2 = p2.snd();

        var c3 = c1.add(c2);
        var b3 = b1.add(b2);
        System.out.println(b3.toBigInteger());
        System.out.println(committer.verify(c1, new BigInteger("3"), b3));

    }
}