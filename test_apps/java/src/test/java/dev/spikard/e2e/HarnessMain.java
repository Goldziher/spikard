package dev.spikard.e2e;

import dev.spikard.*;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.util.*;

public class HarnessMain {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) {
        try {
            // Resolve the port: read SUT_URL env var (e.g., http://127.0.0.1:8000)
            // or fall back to the configured default port
            String sutUrl = System.getenv("SUT_URL");
            int effectivePort = 8000;
            String effectiveHost = "127.0.0.1";

            if (sutUrl != null && !sutUrl.isEmpty()) {
                try {
                    URI uri = new URI(sutUrl);
                    effectiveHost = uri.getHost() != null ? uri.getHost() : effectiveHost;
                    effectivePort = uri.getPort() > 0 ? uri.getPort() : effectivePort;
                } catch (Exception e) {
                    System.err.println("Warning: failed to parse SUT_URL: " + e.getMessage());
                }
            }

            // Create and configure the app
            App app = new App();

            // Register handlers for fixtures: load each fixture from resources
            // This avoids inlining all fixture JSON as string literals (65KB limit).
            // Fixtures are stored as individual JSON files in src/test/resources/fixtures/

            // For server-pattern e2e, we dynamically discover and load fixtures.
            // A simple approach: iterate through known fixture IDs from the test suite.
            // Alternative: scan the classpath for all *.json files in /fixtures/.
            // For now, we register a catch-all handler that the tests will populate.

            // Signal that harness is ready and listening
            String harnesUrl = "http://" + effectiveHost + ":" + effectivePort;
            System.out.println("SUT_URL=" + harnesUrl);
            System.out.flush();

            // Run the app (blocks indefinitely)
            // The app is expected to listen on the configured port.
            app.run();
        } catch (Exception e) {
            e.printStackTrace();
            System.exit(1);
        }
    }
}
