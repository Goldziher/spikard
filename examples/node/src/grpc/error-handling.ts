/**
 * Error Handling in gRPC Streaming: Rate Limiting, Timeouts, and Validation
 *
 * This example demonstrates proper error handling patterns for gRPC streaming:
 * - Input validation with structured errors
 * - Rate limiting and resource exhaustion
 * - Timeouts and deadline exceeded
 * - Permission checks and authorization
 * - Mid-stream errors and recovery
 *
 * gRPC Status Codes Used:
 * - INVALID_ARGUMENT: Bad request data
 * - PERMISSION_DENIED: Authorization failure
 * - RESOURCE_EXHAUSTED: Rate limit exceeded
 * - DEADLINE_EXCEEDED: Operation timeout
 * - INTERNAL: Server-side errors
 * - UNAVAILABLE: Service temporarily unavailable
 *
 * @see https://grpc.io/docs/guides/error/
 * @see crates/spikard-node/src/grpc/handler.rs
 */

import type { GrpcRequest, GrpcResponse } from '@spikard/node';

// gRPC Status Codes
enum GrpcStatus {
  OK = 0,
  CANCELLED = 1,
  UNKNOWN = 2,
  INVALID_ARGUMENT = 3,
  DEADLINE_EXCEEDED = 4,
  NOT_FOUND = 5,
  ALREADY_EXISTS = 6,
  PERMISSION_DENIED = 7,
  RESOURCE_EXHAUSTED = 8,
  FAILED_PRECONDITION = 9,
  ABORTED = 10,
  OUT_OF_RANGE = 11,
  UNIMPLEMENTED = 12,
  INTERNAL = 13,
  UNAVAILABLE = 14,
  DATA_LOSS = 15,
  UNAUTHENTICATED = 16,
}

// Mock protobuf types
interface DataRequest {
  userId: string;
  query: string;
  limit?: number;
}

interface DataResponse {
  result: string;
  processedAt: number;
}

/**
 * Rate limiter implementation
 */
class RateLimiter {
  private requests: Map<string, number[]> = new Map();

  /**
   * Check if user exceeded rate limit
   *
   * @param userId User identifier
   * @param maxRequests Maximum requests allowed
   * @param windowMs Time window in milliseconds
   * @returns true if within limit, false if exceeded
   */
  checkLimit(userId: string, maxRequests: number, windowMs: number): boolean {
    const now = Date.now();
    const userRequests = this.requests.get(userId) ?? [];

    // Remove requests outside the time window
    const recentRequests = userRequests.filter((timestamp) => now - timestamp < windowMs);

    // Check if limit exceeded
    if (recentRequests.length >= maxRequests) {
      return false;
    }

    // Add current request
    recentRequests.push(now);
    this.requests.set(userId, recentRequests);

    return true;
  }

  /**
   * Get remaining requests for user
   */
  getRemainingRequests(userId: string, maxRequests: number, windowMs: number): number {
    const now = Date.now();
    const userRequests = this.requests.get(userId) ?? [];
    const recentRequests = userRequests.filter((timestamp) => now - timestamp < windowMs);

    return Math.max(0, maxRequests - recentRequests.length);
  }
}

/**
 * Data Service Handler with Comprehensive Error Handling
 */
class DataServiceHandler {
  private rateLimiter = new RateLimiter();
  private readonly MAX_REQUESTS_PER_MINUTE = 10;
  private readonly RATE_LIMIT_WINDOW_MS = 60000; // 1 minute
  private readonly MAX_QUERY_LENGTH = 1000;
  private readonly MAX_LIMIT = 100;

  /**
   * Unary RPC with error handling
   */
  async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
    console.log(`[Unary] ${request.methodName} called`);

    try {
      // 1. Validate authentication
      this.validateAuth(request);

      // Deserialize request
      const payload = JSON.parse(request.payload.toString('utf-8')) as DataRequest;

      // 2. Validate input
      this.validateInput(payload);

      // 3. Check rate limit
      this.checkRateLimit(payload.userId);

      // 4. Check permissions
      this.checkPermissions(payload.userId, payload.query);

      // 5. Process request (simulate work)
      await this.delay(100);

      // Success response
      const response: DataResponse = {
        result: `Processed query: ${payload.query}`,
        processedAt: Date.now(),
      };

      return {
        payload: Buffer.from(JSON.stringify(response), 'utf-8'),
        metadata: {
          'x-rate-limit-remaining': String(
            this.rateLimiter.getRemainingRequests(
              payload.userId,
              this.MAX_REQUESTS_PER_MINUTE,
              this.RATE_LIMIT_WINDOW_MS
            )
          ),
        },
      };
    } catch (error) {
      // Convert errors to gRPC errors
      throw this.toGrpcError(error);
    }
  }

  /**
   * Server Streaming RPC with mid-stream error handling
   */
  async handleServerStream(request: GrpcRequest): Promise<GrpcResponse> {
    console.log(`[Server Streaming] ${request.methodName} called`);

    const responses: DataResponse[] = [];

    try {
      // Initial validation
      this.validateAuth(request);

      const payload = JSON.parse(request.payload.toString('utf-8')) as DataRequest;
      this.validateInput(payload);
      this.checkRateLimit(payload.userId);

      const limit = payload.limit ?? 10;

      // Process stream with potential mid-stream errors
      for (let i = 0; i < limit; i++) {
        // Simulate mid-stream failure at halfway point
        if (i === Math.floor(limit / 2)) {
          console.log('  ⚠️  Simulating mid-stream error');
          throw new Error('UNAVAILABLE: Service temporarily unavailable');
        }

        const response: DataResponse = {
          result: `Result ${i + 1} for query: ${payload.query}`,
          processedAt: Date.now() + i * 100,
        };

        responses.push(response);
        console.log(`  Generated response ${i + 1}/${limit}`);

        await this.delay(50);
      }

      return {
        payload: Buffer.from(JSON.stringify({ responses }), 'utf-8'),
        metadata: {},
      };
    } catch (error) {
      // Mid-stream errors need special handling
      console.error('  ✗ Stream error:', error);
      throw this.toGrpcError(error);
    }
  }

  /**
   * Client Streaming RPC with batch validation
   */
  async handleClientStream(
    requestStream: AsyncIterator<GrpcRequest>
  ): Promise<GrpcResponse> {
    console.log('[Client Streaming] ProcessBatch called');

    const results: string[] = [];
    let requestCount = 0;
    const MAX_BATCH_SIZE = 50;

    try {
      for await (const request of requestStream) {
        requestCount++;

        // Enforce batch size limit
        if (requestCount > MAX_BATCH_SIZE) {
          throw new Error(
            `RESOURCE_EXHAUSTED: Batch size exceeded (max ${MAX_BATCH_SIZE})`
          );
        }

        // Validate each item
        const payload = JSON.parse(request.payload.toString('utf-8')) as DataRequest;
        this.validateInput(payload);

        results.push(`Processed: ${payload.query}`);
        console.log(`  Processed item ${requestCount}`);

        await this.delay(30);
      }

      const response = {
        totalProcessed: requestCount,
        results: results.slice(0, 10), // Return first 10 as sample
      };

      return {
        payload: Buffer.from(JSON.stringify(response), 'utf-8'),
        metadata: {
          'x-batch-size': String(requestCount),
        },
      };
    } catch (error) {
      console.error('  ✗ Batch processing error:', error);
      throw this.toGrpcError(error);
    }
  }

  /**
   * Validate authentication metadata
   */
  private validateAuth(request: GrpcRequest): void {
    const authToken = request.metadata['authorization'];

    if (!authToken) {
      throw new Error('UNAUTHENTICATED: Missing authorization token');
    }

    if (!authToken.startsWith('Bearer ')) {
      throw new Error('UNAUTHENTICATED: Invalid authorization format');
    }

    // Simulate token validation
    const token = authToken.substring(7);
    if (token.length < 10) {
      throw new Error('UNAUTHENTICATED: Invalid token');
    }
  }

  /**
   * Validate request input
   */
  private validateInput(payload: DataRequest): void {
    if (!payload.userId || payload.userId.trim() === '') {
      throw new Error('INVALID_ARGUMENT: userId is required');
    }

    if (!payload.query || payload.query.trim() === '') {
      throw new Error('INVALID_ARGUMENT: query is required');
    }

    if (payload.query.length > this.MAX_QUERY_LENGTH) {
      throw new Error(
        `INVALID_ARGUMENT: query exceeds maximum length of ${this.MAX_QUERY_LENGTH}`
      );
    }

    if (payload.limit !== undefined && (payload.limit < 1 || payload.limit > this.MAX_LIMIT)) {
      throw new Error(`INVALID_ARGUMENT: limit must be between 1 and ${this.MAX_LIMIT}`);
    }

    // Validate query format (simple check)
    if (payload.query.includes('<script>')) {
      throw new Error('INVALID_ARGUMENT: query contains invalid characters');
    }
  }

  /**
   * Check rate limit
   */
  private checkRateLimit(userId: string): void {
    const allowed = this.rateLimiter.checkLimit(
      userId,
      this.MAX_REQUESTS_PER_MINUTE,
      this.RATE_LIMIT_WINDOW_MS
    );

    if (!allowed) {
      const remaining = this.rateLimiter.getRemainingRequests(
        userId,
        this.MAX_REQUESTS_PER_MINUTE,
        this.RATE_LIMIT_WINDOW_MS
      );

      throw new Error(
        `RESOURCE_EXHAUSTED: Rate limit exceeded (${this.MAX_REQUESTS_PER_MINUTE} requests per minute). Try again later.`
      );
    }
  }

  /**
   * Check user permissions
   */
  private checkPermissions(userId: string, query: string): void {
    // Simulate permission check
    if (query.toLowerCase().includes('admin') && !userId.startsWith('admin-')) {
      throw new Error('PERMISSION_DENIED: Insufficient permissions for admin queries');
    }

    // Simulate sensitive data access
    if (query.toLowerCase().includes('sensitive') && userId.startsWith('guest-')) {
      throw new Error('PERMISSION_DENIED: Guest users cannot access sensitive data');
    }
  }

  /**
   * Convert application errors to gRPC errors with proper status codes
   */
  private toGrpcError(error: unknown): Error {
    if (error instanceof Error) {
      const message = error.message;

      // Extract status code from message prefix
      const match = message.match(/^([A-Z_]+):\s*(.+)$/);
      if (match) {
        const [, code, details] = match;
        return new Error(`gRPC ${code}: ${details}`);
      }

      // Default to INTERNAL for unstructured errors
      return new Error(`gRPC INTERNAL: ${message}`);
    }

    return new Error('gRPC INTERNAL: Unknown error');
  }

  /**
   * Utility: Delay for ms milliseconds
   */
  private delay(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
}

/**
 * Client: Test error handling scenarios
 */
async function runClient() {
  console.log('=== Error Handling Examples ===\n');

  const service = new DataServiceHandler();

  // Test 1: Missing authentication
  console.log('Test 1: Missing Authentication');
  console.log('--------------------------------');
  try {
    await service.handleRequest({
      serviceName: 'data.v1.DataService',
      methodName: 'GetData',
      payload: Buffer.from(JSON.stringify({ userId: 'user1', query: 'test' }), 'utf-8'),
      metadata: {}, // No auth token
    });
    console.log('✗ Should have failed\n');
  } catch (error) {
    console.log(`✓ Expected error: ${error}\n`);
  }

  // Test 2: Invalid input
  console.log('Test 2: Invalid Input');
  console.log('---------------------');
  try {
    await service.handleRequest({
      serviceName: 'data.v1.DataService',
      methodName: 'GetData',
      payload: Buffer.from(JSON.stringify({ userId: '', query: 'test' }), 'utf-8'),
      metadata: { authorization: 'Bearer valid-token-12345' },
    });
    console.log('✗ Should have failed\n');
  } catch (error) {
    console.log(`✓ Expected error: ${error}\n`);
  }

  // Test 3: Permission denied
  console.log('Test 3: Permission Denied');
  console.log('-------------------------');
  try {
    await service.handleRequest({
      serviceName: 'data.v1.DataService',
      methodName: 'GetData',
      payload: Buffer.from(
        JSON.stringify({ userId: 'guest-user1', query: 'sensitive data' }),
        'utf-8'
      ),
      metadata: { authorization: 'Bearer valid-token-12345' },
    });
    console.log('✗ Should have failed\n');
  } catch (error) {
    console.log(`✓ Expected error: ${error}\n`);
  }

  // Test 4: Rate limiting
  console.log('Test 4: Rate Limiting');
  console.log('---------------------');
  try {
    const userId = 'user-rate-test';
    // Make 11 rapid requests (limit is 10 per minute)
    for (let i = 0; i < 11; i++) {
      await service.handleRequest({
        serviceName: 'data.v1.DataService',
        methodName: 'GetData',
        payload: Buffer.from(JSON.stringify({ userId, query: `test ${i}` }), 'utf-8'),
        metadata: { authorization: 'Bearer valid-token-12345' },
      });
      console.log(`  Request ${i + 1} succeeded`);
    }
    console.log('✗ Should have been rate limited\n');
  } catch (error) {
    console.log(`✓ Rate limit enforced: ${error}\n`);
  }

  // Test 5: Mid-stream error
  console.log('Test 5: Mid-stream Error');
  console.log('------------------------');
  try {
    await service.handleServerStream({
      serviceName: 'data.v1.DataService',
      methodName: 'StreamData',
      payload: Buffer.from(
        JSON.stringify({ userId: 'user2', query: 'stream test', limit: 10 }),
        'utf-8'
      ),
      metadata: { authorization: 'Bearer valid-token-12345' },
    });
    console.log('✗ Should have failed mid-stream\n');
  } catch (error) {
    console.log(`✓ Mid-stream error handled: ${error}\n`);
  }

  // Test 6: Successful request
  console.log('Test 6: Successful Request');
  console.log('--------------------------');
  try {
    const response = await service.handleRequest({
      serviceName: 'data.v1.DataService',
      methodName: 'GetData',
      payload: Buffer.from(
        JSON.stringify({ userId: 'user3', query: 'valid query' }),
        'utf-8'
      ),
      metadata: { authorization: 'Bearer valid-token-12345' },
    });

    const data = JSON.parse(response.payload.toString('utf-8')) as DataResponse;
    console.log(`✓ Success: ${data.result}`);
    console.log(`  Remaining requests: ${response.metadata?.['x-rate-limit-remaining']}\n`);
  } catch (error) {
    console.log(`✗ Unexpected error: ${error}\n`);
  }

  console.log('=== Demo Complete ===');
}

/**
 * Main entry point
 */
async function main() {
  await runClient();
}

// Run if executed directly
if (require.main === module) {
  main().catch((error) => {
    console.error('Error:', error);
    process.exit(1);
  });
}

export { DataServiceHandler, RateLimiter, GrpcStatus };
