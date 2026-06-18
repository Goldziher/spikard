package dev.spikard.e2e;

import dev.spikard.*;
import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

public class MinimalCrashTest {
    public static void main(String[] args) throws Throwable {
        System.err.println("1. Creating App...");
        App app = new App();
        System.err.println("2. Testing spikard_server_config_from_json directly...");
        
        // Test the function directly
        Linker linker = Linker.nativeLinker();
        SymbolLookup lookup = SymbolLookup.loaderLookup();
        
        try {
            Class.forName("dev.spikard.NativeLib");
        } catch (ClassNotFoundException ignored) {}
        
        String testJson = "{\"host\": \"127.0.0.1\", \"port\": 8000}";
        System.err.println("   JSON: " + testJson);
        
        // Allocate the JSON string
        Arena arena = Arena.ofShared();
        MemorySegment jsonSegment = arena.allocateFrom(testJson);
        
        // Call spikard_server_config_from_json
        MemorySegment addr = lookup.find("spikard_server_config_from_json")
            .or(() -> lookup.find("_spikard_server_config_from_json"))
            .orElseThrow();
        System.err.println("   Found function address");
        
        FunctionDescriptor desc = FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS);
        MethodHandle handle = linker.downcallHandle(addr, desc);
        System.err.println("   Created downcall handle");
        
        MemorySegment result = (MemorySegment) handle.invoke(jsonSegment);
        System.err.println("   Invoked C function, result: " + (result == null ? "NULL" : "0x" + Long.toHexString(result.address())));
        
        // Check error code and message
        MemorySegment errorAddr = lookup.find("spikard_last_error_code")
            .or(() -> lookup.find("_spikard_last_error_code"))
            .orElseThrow();
        FunctionDescriptor errorDesc = FunctionDescriptor.of(ValueLayout.JAVA_INT);
        MethodHandle errorHandle = linker.downcallHandle(errorAddr, errorDesc);
        int errorCode = (int) errorHandle.invoke();
        System.err.println("   Error code: " + errorCode);
        
        // Get error message and dump its raw bytes
        MemorySegment msgAddr = lookup.find("spikard_last_error_context")
            .or(() -> lookup.find("_spikard_last_error_context"))
            .orElseThrow();
        FunctionDescriptor msgDesc = FunctionDescriptor.of(ValueLayout.ADDRESS);
        MethodHandle msgHandle = linker.downcallHandle(msgAddr, msgDesc);
        MemorySegment errMsg = (MemorySegment) msgHandle.invoke();
        System.err.println("   Error message segment: " + errMsg + ", address: 0x" + Long.toHexString(errMsg == null ? 0 : errMsg.address()));
        
        // Try to manually read the string
        if (errMsg != null && errMsg.address() != 0) {
            try {
                byte first = errMsg.get(ValueLayout.JAVA_BYTE, 0);
                System.err.println("   First byte at offset 0: " + first + " (0x" + Integer.toHexString(first & 0xFF) + ")");
                String msg = errMsg.getString(0);
                System.err.println("   Error message: " + msg);
            } catch (Exception e) {
                System.err.println("   Could not read error message: " + e);
            }
        }
    }
}
