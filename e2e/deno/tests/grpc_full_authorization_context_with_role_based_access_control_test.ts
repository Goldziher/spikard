/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcFullAuthorizationContextWithRoleBasedAccessControl, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Full authorization context with role-based access control", async () => {
  // Tests complete authorization context including user roles, permissions, and resource-level access control.

  const metadata: Record<string, string> = {
    "x-user-id": "user-admin-001",
    "content-type": "application/grpc",
    "authorization": "Bearer token123",
    "x-user-roles": "admin,editor",
    "x-user-permissions": "read,write,delete",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.AuthzService",
    methodName: "CheckAccess",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcFullAuthorizationContextWithRoleBasedAccessControl(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ authorized: true, message: "Access granted with admin privileges" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});