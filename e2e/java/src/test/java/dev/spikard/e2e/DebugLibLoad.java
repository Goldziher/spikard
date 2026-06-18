package dev.spikard.e2e;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

public class DebugLibLoad {
    public static void main(String[] args) throws Throwable {
        // Load NativeLib to trigger extraction
        System.out.println("Loading NativeLib...");
        try {
            Class.forName("dev.spikard.NativeLib");
        } catch (ClassNotFoundException ignored) {}
        
        System.out.println("NativeLib loaded");
        
        // Find the library path from the temp directory
        String tmpDir = System.getProperty("java.io.tmpdir");
        System.out.println("Temp dir: " + tmpDir);
        
        // Search for libspikard_ffi in temp
        java.nio.file.Files.walk(java.nio.file.Paths.get(tmpDir))
            .filter(p -> p.toString().contains("spikard_ffi"))
            .forEach(p -> {
                System.out.println("Found: " + p + " (size: " + new java.io.File(p.toString()).length() + ")");
            });
    }
}
