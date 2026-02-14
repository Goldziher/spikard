/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcFullAuthorizationContextWithRoleBasedAccessControl } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Full authorization context with role-based access control", async () => {
		// Tests complete authorization context including user roles, permissions, and resource-level access control.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
			"x-user-id": "user-admin-001",
			authorization: "Bearer token123",
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
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(JSON.stringify({ authorized: true, message: "Access granted with admin privileges" })),
		);
		expect(response.metadata).toBeDefined();
	});
});
