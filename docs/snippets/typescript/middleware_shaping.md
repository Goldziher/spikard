```typescript
import { Spikard, type Request, type Response, HTTPError } from "spikard";
import * as zlib from "zlib";

const app = new Spikard();

// Simple in-memory rate limiter (use Redis in production)
const rateLimits = new Map<string, number[]>();

app.onRequest(async (request: Request): Promise<Request> => {
  // 1. Rate limiting: 100 requests per minute per IP
  const clientIp = request.clientIp || "unknown";
  const now = Date.now();

  // Clean old entries
  const timestamps = (rateLimits.get(clientIp) || [])
    .filter(ts => now - ts < 60000);

  if (timestamps.length >= 100) {
    throw new HTTPError(429, "Rate limit exceeded");
  }

  timestamps.push(now);
  rateLimits.set(clientIp, timestamps);

  // 2. Normalize headers (lowercase keys)
  if (request.headers) {
    const normalized: Record<string, string> = {};
    for (const [key, value] of Object.entries(request.headers)) {
      normalized[key.toLowerCase()] = value;
    }
    request.headers = normalized;
  }

  // 3. Inject tenant from subdomain
  const host = request.headers?.host || "";
  const tenant = host.includes(".") ? host.split(".")[0] : "default";
  request.context = request.context || {};
  request.context.tenant = tenant;

  // 4. Feature flags from query params or headers
  const featureStr = request.query?.features || "";
  request.context.features = new Set(
    featureStr.split(",").filter(f => f)
  );

  return request;
});

app.onResponse(async (response: Response): Promise<Response> => {
  // Response compression for large payloads
  const body = response.body || "";
  if (body.length > 1024) {  // Compress if > 1KB
    response.body = zlib.gzipSync(Buffer.from(body));
    response.headers = response.headers || {};
    response.headers["content-encoding"] = "gzip";
  }

  return response;
});
```
