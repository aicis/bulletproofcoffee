package dk.alexandra.bulletproofcoffee;

import java.math.BigInteger;

public class Util {
    public static byte[] convertBigInteger(BigInteger value) {
        if (value.compareTo(BigInteger.ZERO) < 0) {
            throw new IllegalArgumentException("Value has to be positive");
        }
        var bytes = value.toByteArray();
        if (bytes.length < 32) {
            int startPos = 32 - bytes.length;
            var res = new byte[32];
            System.arraycopy(bytes, 0, res, startPos, bytes.length);
            return res;
        } else if (bytes.length == 32) {
            return bytes;
        } else {
            throw new IllegalArgumentException("Value is too large (over 256 bit)");
        }
    }

    public static void reverseArray(byte[] array) {
        int startIndex = 0;
        int endIndex = array.length - 1;
        while(startIndex < endIndex) {
            byte temp = array[endIndex];
            array[endIndex] = array[startIndex];
            array[startIndex] = temp;
            startIndex++;
            endIndex--;
        }
    }

}