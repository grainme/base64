package org.example;

import java.util.ArrayList;
import java.util.List;

public class Base64Encoder {
    public static String encode(String text) {
        // convert every char of text to its ascii code first
        byte[] asciiCodes = text.getBytes();

        // we build a sequence of bits as a String, then we'll divide
        // this string into groups of 6bits (per Base64 encoding algo)
        StringBuilder bitsSequence = new StringBuilder();
        for (byte b: asciiCodes) {
            bitsSequence.append(Integer.toBinaryString((b & 0xFF) + 0x100).substring(1));
        }

        List<String> sixGroupBits = getSixGroupsBits(bitsSequence);

        StringBuilder encoded = new StringBuilder();
        for (String currString: sixGroupBits) {
            int asciiCode = Integer.valueOf(currString, 2);
            encoded.append(Base64.toBase64[asciiCode]);
        }

        while (encoded.length() % 4 != 0) {
            encoded.append('=');
        }
        // Output: TG9ndG8=
        return String.valueOf(encoded);
    }

    private static List<String> getSixGroupsBits(StringBuilder bitsSequence) {
        char[] bitSequenceChars = bitsSequence.toString().toCharArray();

        int currentWindowSize = 0;
        StringBuilder currentGroup = new StringBuilder();
        List<String> sixGroupBits = new ArrayList<>();

        for (char c: bitSequenceChars) {
            currentGroup.append(c);
            currentWindowSize++;
            if (currentWindowSize == Base64.SIX_BITS_GROUP) {
                sixGroupBits.add(currentGroup.toString());
                currentGroup.setLength(0);
                currentWindowSize = 0;
            }
        }

        if (!currentGroup.isEmpty()) {
            while (currentGroup.length() < 6) {
                currentGroup.append('0');
            }
            sixGroupBits.add(currentGroup.toString());
        }
        return sixGroupBits;
    }
}
