package dev.spikard.e2e;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

public class DirectFFITest {
    public static void main(String[] args) throws Throwable {
        // Force native lib load
        try {
            Class.forName("dev.spikard.NativeLib");
        } catch (ClassNotFoundException ignored) {}

        Linker linker = Linker.nativeLinker();
        SymbolLookup lookup = SymbolLookup.loaderLookup();
        Arena arena = Arena.ofShared();

        System.out.println("Testing direct FFI call to spikard_server_config_from_json");

        // First, test with FULL JSON (all fields)
        String fullJson = "{\"host\": \"127.0.0.1\", \"port\": 8000, \"workers\": 1, \"enable_request_id\": false, \"max_body_size\": null, \"request_timeout\": null, \"compression\": null, \"rate_limit\": null, \"jwt_auth\": null, \"api_key_auth\": null, \"static_files\": [], \"graceful_shutdown\": true, \"shutdown_timeout\": 30, \"asyncapi\": null, \"openapi\": null, \"jsonrpc\": null, \"grpc\": null, \"background_tasks\": {\"max_concurrent\": 10, \"max_queue_size\": 1000, \"shutdown_timeout\": 30}, \"enable_http_trace\": false}";

        MemorySegment fullJsonSeg = arena.allocateFrom(fullJson);
        System.out.println("Full JSON allocated, calling function...");

        MemorySegment addr = lookup.find("spikard_server_config_from_json")
        .or(() -> lookup.find("_spikard_server_config_from_json"))
        .orElseThrow();
        FunctionDescriptor desc = FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS);
        MethodHandle handle = linker.downcallHandle(addr, desc);

        MemorySegment result = (MemorySegment) handle.invoke(fullJsonSeg);
        System.out.println("Result for FULL JSON: " + (result == null || result.address() == 0 ? "NULL" : "0x" + Long.toHexString(result.address())));

        // Now test with PARTIAL JSON (only host/port)
        String partialJson = "{\"host\": \"127.0.0.1\", \"port\": 8000}";
        MemorySegment partialJsonSeg = arena.allocateFrom(partialJson);
        System.out.println("Partial JSON allocated, calling function...");

        result = (MemorySegment) handle.invoke(partialJsonSeg);
        System.out.println("Result for PARTIAL JSON: " + (result == null || result.address() == 0 ? "NULL" : "0x" + Long.toHexString(result.address())));

        // Check error if partial failed
        if (result == null || result.address() == 0) {
            MemorySegment errorAddr = lookup.find("spikard_last_error_code")
            .or(() -> lookup.find("_spikard_last_error_code"))
            .orElseThrow();
            FunctionDescriptor errorDesc = FunctionDescriptor.of(ValueLayout.JAVA_INT);
            MethodHandle errorHandle = linker.downcallHandle(errorAddr, errorDesc);
            int errorCode = (int) errorHandle.invoke();
            System.out.println("Error code: " + errorCode);
        }
    }
}
