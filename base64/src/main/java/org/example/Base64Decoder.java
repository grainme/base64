package org.example;

public class Base64Decoder {
    public static String decode(String text) {
        // First I need to remove the padding.
        text = text.replace("=", "");

        // Convert each "Base64 char" to it corresponding number!
        // I will implement a very bad brute force idea for now!
        StringBuilder sixGroupBits = new StringBuilder();
        for (int k = 0; k < text.length(); k++) {
            for (int i = 0; i < 64; i++) {
                if (Base64.toBase64[i] == text.charAt(k)) {
                    String current = String.format("%6s", Integer.toString(i, 2)).replace(" ", "0");
                    sixGroupBits.append(current);
                    break;
                }
            }
        }

        StringBuilder decoded = new StringBuilder();
        for(int i = 0; i + 8 <= sixGroupBits.length(); ) {
            StringBuilder currentWindow = new StringBuilder();
            int k;
            for(k = 0; k < 8 && i + k < sixGroupBits.length(); k++) {
                currentWindow.append(sixGroupBits.charAt(i + k));
            }
            i += k;
            decoded.append((char) Integer.parseInt(String.valueOf(currentWindow), 2));
        }

        return String.valueOf(decoded);
    }
}
