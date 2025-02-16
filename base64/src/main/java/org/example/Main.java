package org.example;

import java.util.Scanner;

import static org.example.Base64Decoder.decode;

public class Main {
    public static void main(String... args) {
        Scanner in = new Scanner(System.in);
        for (int i = 0; i < 16; i++) {
            String base64EncodedString = in.next();
            System.out.println(decode(base64EncodedString));
        }
    }
}