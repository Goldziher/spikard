/**
 * Lifecycle Hooks Example
 *
 * Demonstrates registering and using lifecycle hooks for request logging,
 * authentication, error handling, and response transformation.
 */

import { get, post, type Request, Spikard } from "@spikard/node";

const app = new Spikard();

const authenticatedUsers = new Set(["alice", "bob"]);

/**
 * Lifecycle Hooks Configuration
 *
 * Hooks are executed in this order:
 * 1. onRequest - Initial processing, logging
 * 2. preValidation - Before parameter validation
 * 3. preHandler - Before route handler execution
 * 4. onResponse - After successful handler response
 * 5. onError - On handler error or validation failure
 */

/**
 * Called at the start of request processing
 * Useful for logging, metrics, request tracing
 */
app.onRequest(async (req) => {
	if (typeof req !== "object" || req === null || !("method" in req)) {
		return req;
	}
	const request = req as Request & { _request_id?: string };

	console.log(`[REQUEST] ${request.method} ${request.path}`);

	const requestId = request.headers?.["x-request-id"] || Math.random().toString(36).substring(7);

	return {
		...request,
		_request_id: requestId,
	};
});

/**
 * Called before parameter validation
 * Can validate headers, cookies, etc.
 */
app.preValidation(async (req) => {
	if (typeof req !== "object" || req === null || !("method" in req)) {
		return req;
	}
	const request = req as Request;

	if (request.method === "POST" || request.method === "PUT") {
		if (!request.headers?.["content-type"]) {
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
});

/**
 * Called before the route handler is executed
 * Useful for authentication, authorization, request transformation
 */
app.preHandler(async (req) => {
	if (typeof req !== "object" || req === null || !("method" in req)) {
		return req;
	}
	const request = req as Request & { _user?: { username: string } };

	const authHeader = request.headers?.authorization;
	if (authHeader) {
		const parts = authHeader.split(" ");
		const scheme = parts[0];
		const token = parts[1];

		if (scheme === "Bearer" && token) {
			const username = token.split(":")[0] || "";

			if (!authenticatedUsers.has(username)) {
				return {
					status: 401,
					body: {
						error: "Unauthorized",
						code: "invalid_token",
					},
				};
			}

			return {
				...request,
				_user: { username },
			};
		}
	}

	return req;
});

/**
 * Called after a successful response from the handler
 * Useful for response transformation, adding headers, metrics
 */
app.onResponse(async (payload) => {
	if (typeof payload !== "object" || payload === null || !("status" in payload)) {
		return payload;
	}

	const res = payload as Record<string, unknown> & { headers?: Record<string, string> };

	const status = (res.status as number | undefined) || 200;
	console.log(`[RESPONSE] ${status}`);

	const headers = (res.headers as Record<string, string> | undefined) || {};
	headers["X-Request-ID"] = "unknown";
	headers["X-Response-Time"] = `${Math.random() * 100}ms`;

	return {
		...res,
		headers,
	};
});

/**
 * Called when an error occurs during request processing
 * Useful for error logging, custom error responses
 */
app.onError(async (payload) => {
	if (typeof payload !== "object" || payload === null) {
		return payload;
	}

	return {
		...payload,
		headers: {
			...((payload as Record<string, unknown>).headers as Record<string, string> | undefined),
		},
	};
});

/**
 * Public endpoint (no authentication required)
 */
get("/public")(async function publicEndpoint(_req: Request) {
	return {
		message: "This is a public endpoint",
		timestamp: new Date().toISOString(),
	};
});

/**
 * Protected endpoint (requires Bearer token)
 * Expected format: Authorization: Bearer alice:secret
 */
get("/protected")(async function protectedEndpoint(req: Request & { _user?: { username: string } }) {
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
});

/**
 * POST endpoint with request body transformation
 */
post("/echo")(async function echoEndpoint(req: Request & { _user?: { username: string } }) {
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
});

/**
 * Admin endpoint with extra authorization
 */
get("/admin/stats")(async function adminStats(req: Request & { _user?: { username: string } }) {
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
});

/**
 * Serve demo page
 */
get("/")(async function servePage() {
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
});

console.log("Starting Lifecycle Hooks Example on http://127.0.0.1:8000");
console.log("Open http://127.0.0.1:8000 in your browser");
console.log("");
console.log("Example tokens:");
console.log("  alice:secret  (admin)");
console.log("  bob:secret    (regular user)");
console.log("");

app.run({ port: 8000, host: "0.0.0.0" });
