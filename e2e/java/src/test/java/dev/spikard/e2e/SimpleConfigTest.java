package dev.spikard.e2e;

import dev.spikard.*;

public class SimpleConfigTest {
    public static void main(String[] args) {
        try {
            System.err.println("1. Creating App...");
            App app = new App();
            System.err.println("2. Calling app.config(127.0.0.1, 8000)...");
            app.config("127.0.0.1", 8000);
            System.err.println("3. Success! Server configured.");
        } catch (Exception e) {
            System.err.println("4. Error: " + e.getMessage());
            e.printStackTrace();
        }
    }
}
