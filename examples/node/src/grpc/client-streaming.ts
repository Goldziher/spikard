/**
 * Client Streaming gRPC Example: Log Aggregation Service
 *
 * This example demonstrates client streaming RPC where a client streams
 * log entries to the server, and the server returns aggregated statistics.
 *
 * Current Implementation Note:
 * The Node.js implementation supports consuming client streams via async
 * iteration using GrpcMessageStream. The handler collects all messages
 * and returns a single aggregated response.
 *
 * @see crates/spikard-node/src/grpc/handler.rs for implementation details
 */

import type { GrpcRequest, GrpcResponse } from '@spikard/node';

// Mock protobuf types (in production, use generated proto files)
interface LogEntry {
  level: 'DEBUG' | 'INFO' | 'WARN' | 'ERROR' | 'FATAL';
  message: string;
  timestamp: number;
  service: string;
  traceId?: string;
}

interface LogAggregationResult {
  totalLogs: number;
  byLevel: Record<string, number>;
  timeRange: {
    start: number;
    end: number;
  };
  services: string[];
  errorRate: number;
}

/**
 * Log Aggregation Service Handler
 *
 * Demonstrates client streaming pattern where server consumes a stream
 * of log entries and returns aggregated statistics.
 */
class LogAggregationServiceHandler {
  /**
   * Client Streaming RPC: Aggregate logs from client stream
   *
   * The handler receives an async iterator of GrpcRequest objects,
   * each containing a serialized LogEntry. After consuming all entries,
   * it returns aggregated statistics.
   *
   * @param requestStream AsyncIterator yielding GrpcRequest objects
   * @returns Promise<GrpcResponse> with aggregated statistics
   */
  async handleClientStream(
    requestStream: AsyncIterator<GrpcRequest>
  ): Promise<GrpcResponse> {
    console.log('[Client Streaming] AggregateLogs called');

    const logs: LogEntry[] = [];
    const levelCounts: Record<string, number> = {
      DEBUG: 0,
      INFO: 0,
      WARN: 0,
      ERROR: 0,
      FATAL: 0,
    };
    const services = new Set<string>();
    let minTimestamp = Infinity;
    let maxTimestamp = 0;

    // Consume all log entries from the stream
    let entryCount = 0;
    try {
      for await (const request of requestStream) {
        entryCount++;
        console.log(`  Received log entry ${entryCount}`);

        // Deserialize log entry
        const logEntry = JSON.parse(request.payload.toString('utf-8')) as LogEntry;
        logs.push(logEntry);

        // Update aggregation statistics
        levelCounts[logEntry.level]++;
        services.add(logEntry.service);
        minTimestamp = Math.min(minTimestamp, logEntry.timestamp);
        maxTimestamp = Math.max(maxTimestamp, logEntry.timestamp);

        console.log(`    [${logEntry.level}] ${logEntry.service}: ${logEntry.message}`);
      }
    } catch (error) {
      console.error('Error consuming stream:', error);
      throw new Error('INTERNAL: Failed to consume log stream');
    }

    console.log(`\nProcessed ${logs.length} log entries`);

    // Calculate error rate
    const errorCount = levelCounts.ERROR + levelCounts.FATAL;
    const errorRate = logs.length > 0 ? errorCount / logs.length : 0;

    // Build aggregation result
    const result: LogAggregationResult = {
      totalLogs: logs.length,
      byLevel: levelCounts,
      timeRange: {
        start: minTimestamp === Infinity ? 0 : minTimestamp,
        end: maxTimestamp,
      },
      services: Array.from(services),
      errorRate,
    };

    // Serialize response
    const responsePayload = Buffer.from(JSON.stringify(result), 'utf-8');

    return {
      payload: responsePayload,
      metadata: {
        'content-type': 'application/json',
        'x-service': 'log-aggregation-service',
        'x-total-logs': String(result.totalLogs),
        'x-error-rate': String(result.errorRate.toFixed(4)),
      },
    };
  }

  /**
   * Generate sample log entries for testing
   */
  static generateSampleLogs(count: number, service: string): LogEntry[] {
    const levels: LogEntry['level'][] = ['DEBUG', 'INFO', 'WARN', 'ERROR', 'FATAL'];
    const messages = [
      'Service started successfully',
      'Processing request',
      'Database connection established',
      'Cache miss, fetching from database',
      'Request completed',
      'High memory usage detected',
      'Connection timeout',
      'Invalid request payload',
      'Authentication failed',
      'System overload',
    ];

    const logs: LogEntry[] = [];
    const baseTimestamp = Date.now() - 60000; // Start 1 minute ago

    for (let i = 0; i < count; i++) {
      // Bias towards INFO and WARN levels
      const levelWeights = [10, 40, 30, 15, 5]; // DEBUG, INFO, WARN, ERROR, FATAL
      const rand = Math.random() * 100;
      let level: LogEntry['level'] = 'INFO';
      let cumulative = 0;
      for (let j = 0; j < levelWeights.length; j++) {
        cumulative += levelWeights[j];
        if (rand < cumulative) {
          level = levels[j];
          break;
        }
      }

      const log: LogEntry = {
        level,
        message: messages[Math.floor(Math.random() * messages.length)],
        timestamp: baseTimestamp + i * 1000, // 1 second apart
        service,
        traceId: `trace-${Math.random().toString(36).substring(2, 15)}`,
      };

      logs.push(log);
    }

    return logs;
  }
}

/**
 * Mock async generator to simulate client stream
 */
async function* createLogStream(logs: LogEntry[]): AsyncGenerator<GrpcRequest> {
  for (const log of logs) {
    // Simulate network delay
    await new Promise((resolve) => setTimeout(resolve, 50));

    yield {
      serviceName: 'logs.v1.LogAggregationService',
      methodName: 'AggregateLogs',
      payload: Buffer.from(JSON.stringify(log), 'utf-8'),
      metadata: {},
    };
  }
}

/**
 * Client: Test the log aggregation service
 */
async function runClient() {
  console.log('=== Log Aggregation Client ===\n');

  const service = new LogAggregationServiceHandler();

  // Generate sample logs from multiple services
  const logs1 = LogAggregationServiceHandler.generateSampleLogs(10, 'api-gateway');
  const logs2 = LogAggregationServiceHandler.generateSampleLogs(8, 'auth-service');
  const logs3 = LogAggregationServiceHandler.generateSampleLogs(6, 'db-service');

  const allLogs = [...logs1, ...logs2, ...logs3].sort((a, b) => a.timestamp - b.timestamp);

  console.log(`Generated ${allLogs.length} sample log entries from 3 services\n`);
  console.log('Streaming logs to aggregation service...\n');

  // Create client stream
  const logStream = createLogStream(allLogs);

  // Call client streaming RPC
  const response = await service.handleClientStream(logStream);

  // Parse and display results
  const result = JSON.parse(response.payload.toString('utf-8')) as LogAggregationResult;

  console.log('\n=== Aggregation Results ===');
  console.log(`Total logs processed: ${result.totalLogs}`);
  console.log(`\nLogs by level:`);
  Object.entries(result.byLevel).forEach(([level, count]) => {
    const percentage = result.totalLogs > 0 ? ((count / result.totalLogs) * 100).toFixed(1) : '0.0';
    console.log(`  ${level.padEnd(5)}: ${String(count).padStart(3)} (${percentage}%)`);
  });
  console.log(`\nTime range:`);
  console.log(`  Start: ${new Date(result.timeRange.start).toISOString()}`);
  console.log(`  End:   ${new Date(result.timeRange.end).toISOString()}`);
  console.log(`\nServices: ${result.services.join(', ')}`);
  console.log(`Error rate: ${(result.errorRate * 100).toFixed(2)}%`);

  // Metadata from response
  console.log(`\nResponse metadata:`);
  Object.entries(response.metadata || {}).forEach(([key, value]) => {
    console.log(`  ${key}: ${value}`);
  });

  console.log('\n=== Demo Complete ===');
}

/**
 * Server: Start the gRPC server (when Spikard supports it)
 */
async function runServer() {
  console.log('=== Starting Log Aggregation Service ===\n');

  // Note: This is a conceptual example. Actual server implementation
  // depends on Spikard's gRPC server API being finalized.
  //
  // Expected usage:
  // const app = new Spikard();
  // app.registerGrpcHandler('logs.v1.LogAggregationService', new LogAggregationServiceHandler());
  // await app.listen(50051);

  console.log('Server would run on port 50051');
  console.log('Service: logs.v1.LogAggregationService');
  console.log('Methods: AggregateLogs (client streaming)\n');

  console.log('⚠️  Server implementation pending Spikard gRPC server API');
}

/**
 * Main entry point
 */
async function main() {
  const mode = process.argv[2] || 'client';

  if (mode === 'server') {
    await runServer();
  } else {
    await runClient();
  }
}

// Run if executed directly
if (require.main === module) {
  main().catch((error) => {
    console.error('Error:', error);
    process.exit(1);
  });
}

export {
  LogAggregationServiceHandler,
  LogEntry,
  LogAggregationResult,
  createLogStream,
};
