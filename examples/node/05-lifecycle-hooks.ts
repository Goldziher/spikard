/**
 * Lifecycle Hooks Example
 *
 * Demonstrates registering and using lifecycle hooks for request logging,
 * authentication, error handling, and response transformation.
 */

import {
	get,
	type LifecycleHooks,
	post,
	type Request,
	Spikard,
} from "@spikard/node";

const app = new Spikard({
	port: 8000,
});

// Simple user authentication store
const authenticatedUsers = new Set(["alice", "bob"]);

/**
 * Lifecycle Hooks Configuration
 *
 * Hooks are executed in this order:
 * 1. on_request - Initial processing, logging
 * 2. pre_validation - Before parameter validation
 * 3. pre_handler - Before route handler execution
 * 4. on_response - After successful handler response
 * 5. on_error - On handler error or validation failure
 */
const hooks: LifecycleHooks = {
	/**
	 * Called at the start of request processing
	 * Useful for logging, metrics, request tracing
	 */
	on_request: async (req) => {
		console.log(`[REQUEST] ${req.method} ${req.path} from ${req.ip}`);

		// Add request ID to context
		const requestId =
			req.headers?.["x-request-id"] ||
			Math.random().toString(36).substring(7);

		// Store in request context for later use (implementation-specific)
		return {
			...req,
			_request_id: requestId,
		};
	},

	/**
	 * Called before parameter validation
	 * Can validate headers, cookies, etc.
	 */
	pre_validation: async (req) => {
		// Check for required headers
		if (req.method === "POST" || req.method === "PUT") {
			if (!req.headers?.["content-type"]) {
				return {
					status: 400,
					body: {
						error: "Missing Content-Type header",
						code: "missing_header",
					},
				};
			}
		}

		return req;
	},

	/**
	 * Called before the route handler is executed
	 * Useful for authentication, authorization, request transformation
	 */
	pre_handler: async (req) => {
		// Extract token from Authorization header
		const authHeader = req.headers?.["authorization"];
		if (authHeader) {
			const [scheme, token] = authHeader.split(" ");

			if (scheme === "Bearer") {
				// Simulate token validation (extract username from token)
				const username = token.split(":")[0];

				if (!authenticatedUsers.has(username)) {
					return {
						status: 401,
						body: {
							error: "Unauthorized",
							code: "invalid_token",
						},
					};
				}

				// Store user info in request
				return {
					...req,
					_user: { username },
				};
			}
		}

		return req;
	},

	/**
	 * Called after a successful response from the handler
	 * Useful for response transformation, adding headers, metrics
	 */
	on_response: async (req, res) => {
		// Log successful response
		const status = res.status || 200;
		console.log(
			`[RESPONSE] ${status} for ${req.method} ${req.path} (${req._request_id || "no-id"})`
		);

		// Add custom headers
		const headers = res.headers || {};
		headers["X-Request-ID"] = req._request_id || "unknown";
		headers["X-Response-Time"] = `${Math.random() * 100}ms`;

		return {
			...res,
			headers,
		};
	},

	/**
	 * Called when an error occurs during request processing
	 * Useful for error logging, custom error responses
	 */
	on_error: async (req, error) => {
		console.error(
			`[ERROR] ${error.code} on ${req.method} ${req.path}: ${error.error}`
		);

		// Could transform error response here
		return {
			status: error.code === "unauthorized" ? 401 : 500,
			body: {
				error: error.error,
				code: error.code,
				request_id: req._request_id || "unknown",
				timestamp: new Date().toISOString(),
			},
		};
	},
};

// Register lifecycle hooks
app.registerHooks(hooks);

/**
 * Public endpoint (no authentication required)
 */
@get("/public")
async function publicEndpoint(req: Request) {
	return {
		message: "This is a public endpoint",
		timestamp: new Date().toISOString(),
	};
}

/**
 * Protected endpoint (requires Bearer token)
 * Expected format: Authorization: Bearer alice:secret
 */
@get("/protected")
async function protectedEndpoint(req: Request) {
	const user = req._user;

	if (!user) {
		return {
			status: 401,
			body: {
				error: "Unauthorized",
				code: "no_token",
			},
		};
	}

	return {
		message: `Hello, ${user.username}!`,
		user,
		timestamp: new Date().toISOString(),
	};
}

/**
 * POST endpoint with request body transformation
 */
@post("/echo")
async function echoEndpoint(req: Request) {
	const body = req.body;

	if (!body || typeof body !== "object") {
		return {
			status: 400,
			body: {
				error: "Request body must be a JSON object",
				code: "invalid_body",
			},
		};
	}

	return {
		echo: body,
		received_at: new Date().toISOString(),
		user: req._user?.username || "anonymous",
	};
}

/**
 * Admin endpoint with extra authorization
 */
@get("/admin/stats")
async function adminStats(req: Request) {
	const user = req._user;

	if (!user || user.username !== "alice") {
		return {
			status: 403,
			body: {
				error: "Admin access required",
				code: "forbidden",
			},
		};
	}

	return {
		admin: true,
		stats: {
			uptime: Math.floor(process.uptime()),
			memory: Math.round(process.memoryUsage().heapUsed / 1024 / 1024),
		},
		timestamp: new Date().toISOString(),
	};
}

/**
 * Serve demo page
 */
@get("/")
async function servePage() {
	return {
		status: 200,
		body: `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Spikard Lifecycle Hooks Example</title>
  <style>
    body { font-family: monospace; margin: 20px; }
    .section { margin: 20px 0; padding: 15px; border: 1px solid #ccc; background: #f9f9f9; }
    input { width: 100%; padding: 8px; margin: 5px 0; border: 1px solid #ddd; }
    button { padding: 8px 15px; margin: 5px 0; background: #007bff; color: white; border: none; cursor: pointer; }
    button:hover { background: #0056b3; }
    pre { background: #f0f0f0; padding: 10px; overflow-x: auto; }
    .success { color: green; }
    .error { color: red; }
  </style>
</head>
<body>
  <h1>Spikard Lifecycle Hooks Example</h1>

  <div class="section">
    <h2>Public Endpoint (No Auth Required)</h2>
    <button onclick="testPublic()">GET /public</button>
    <pre id="public-result"></pre>
  </div>

  <div class="section">
    <h2>Protected Endpoint (Requires Bearer Token)</h2>
    <p>Use format: <code>alice:secret</code> or <code>bob:secret</code></p>
    <input type="text" id="token" placeholder="Enter token (e.g., alice:secret)" value="alice:secret">
    <button onclick="testProtected()">GET /protected</button>
    <pre id="protected-result"></pre>
  </div>

  <div class="section">
    <h2>Echo Endpoint (POST)</h2>
    <textarea id="body" style="width: 100%; height: 100px;" placeholder='{"message":"hello"}'></textarea>
    <button onclick="testEcho()">POST /echo</button>
    <pre id="echo-result"></pre>
  </div>

  <div class="section">
    <h2>Admin Stats (alice only)</h2>
    <button onclick="testAdmin()">GET /admin/stats</button>
    <pre id="admin-result"></pre>
  </div>

  <script>
    async function testPublic() {
      try {
        const res = await fetch('/public');
        const data = await res.json();
        document.getElementById('public-result').textContent =
          \`<span class="success">✓ \${res.status}</span>\\n\${JSON.stringify(data, null, 2)}\`;
      } catch (e) {
        document.getElementById('public-result').textContent =
          \`<span class="error">✗ Error: \${e.message}</span>\`;
      }
    }

    async function testProtected() {
      try {
        const token = document.getElementById('token').value;
        const res = await fetch('/protected', {
          headers: { 'Authorization': \`Bearer \${token}\` }
        });
        const data = await res.json();
        document.getElementById('protected-result').textContent =
          \`<span class="\${res.status === 200 ? 'success' : 'error'}">\${res.status === 200 ? '✓' : '✗'} \${res.status}</span>\\n\${JSON.stringify(data, null, 2)}\`;
      } catch (e) {
        document.getElementById('protected-result').textContent =
          \`<span class="error">✗ Error: \${e.message}</span>\`;
      }
    }

    async function testEcho() {
      try {
        const body = document.getElementById('body').value || '{}';
        const res = await fetch('/echo', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body
        });
        const data = await res.json();
        document.getElementById('echo-result').textContent =
          \`<span class="success">✓ \${res.status}</span>\\n\${JSON.stringify(data, null, 2)}\`;
      } catch (e) {
        document.getElementById('echo-result').textContent =
          \`<span class="error">✗ Error: \${e.message}</span>\`;
      }
    }

    async function testAdmin() {
      try {
        const res = await fetch('/admin/stats', {
          headers: { 'Authorization': 'Bearer alice:secret' }
        });
        const data = await res.json();
        document.getElementById('admin-result').textContent =
          \`<span class="\${res.status === 200 ? 'success' : 'error'}">\${res.status === 200 ? '✓' : '✗'} \${res.status}</span>\\n\${JSON.stringify(data, null, 2)}\`;
      } catch (e) {
        document.getElementById('admin-result').textContent =
          \`<span class="error">✗ Error: \${e.message}</span>\`;
      }
    }
  </script>
</body>
</html>
	`,
		headers: {
			"Content-Type": "text/html; charset=utf-8",
		},
	};
}

// Register handlers
app.registerHandler(servePage);
app.registerHandler(publicEndpoint);
app.registerHandler(protectedEndpoint);
app.registerHandler(echoEndpoint);
app.registerHandler(adminStats);

console.log("Starting Lifecycle Hooks Example on http://127.0.0.1:8000");
console.log("Open http://127.0.0.1:8000 in your browser");
console.log("");
console.log("Example tokens:");
console.log("  alice:secret  (admin)");
console.log("  bob:secret    (regular user)");
console.log("");

app.listen().catch((error) => {
	console.error("Server error:", error);
	process.exit(1);
});
