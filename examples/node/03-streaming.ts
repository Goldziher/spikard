/**
 * Streaming Responses Example
 *
 * Demonstrates streaming large responses and Server-Sent Events (SSE)
 * for real-time server-to-client communication.
 */

import { get, type Request, Spikard, StreamingResponse } from "@spikard/node";

const app = new Spikard({
	port: 8000,
});

/**
 * GET endpoint returning a large dataset as a streamed JSON array
 */
get("/stream/numbers")(async function streamNumbers(req: Request) {
	const count = (req.query?.count as string | undefined) ? parseInt(req.query.count as string, 10) : 100;

	// Create a generator that yields numbers
	async function* generateNumbers() {
		for (let i = 1; i <= count; i++) {
			yield {
				number: i,
				squared: i * i,
				timestamp: new Date().toISOString(),
			};
			// Simulate processing delay
			await new Promise((resolve) => setTimeout(resolve, 10));
		}
	}

	return new StreamingResponse(generateNumbers(), {
		contentType: "application/x-ndjson", // Newline-delimited JSON
	});
});

/**
 * GET endpoint for Server-Sent Events (SSE)
 * Use in browser: new EventSource('/stream/events')
 */
get("/stream/events")(async function streamEvents(req: Request) {
	const durationSeconds = (req.query?.duration as string | undefined) ? parseInt(req.query.duration as string, 10) : 30;

	async function* generateEvents() {
		const startTime = Date.now();
		let eventId = 0;

		while (Date.now() - startTime < durationSeconds * 1000) {
			eventId++;
			const elapsedSeconds = Math.floor((Date.now() - startTime) / 1000);

			yield {
				event: "tick",
				data: JSON.stringify({
					id: eventId,
					elapsed: elapsedSeconds,
					message: `Event ${eventId} after ${elapsedSeconds}s`,
					timestamp: new Date().toISOString(),
				}),
			};

			// Send event every second
			await new Promise((resolve) => setTimeout(resolve, 1000));
		}

		// Send completion event
		yield {
			event: "complete",
			data: JSON.stringify({
				message: "Stream complete",
				totalEvents: eventId,
			}),
		};
	}

	return new StreamingResponse(generateEvents(), {
		contentType: "text/event-stream",
		headers: {
			"Cache-Control": "no-cache",
			Connection: "keep-alive",
			"Access-Control-Allow-Origin": "*",
		},
	});
});

/**
 * GET endpoint for CSV streaming
 */
get("/stream/csv")(async function streamCsv(_req: Request) {
	async function* generateCsv() {
		// Yield CSV header
		yield "id,name,email,created_at\n";

		// Generate 1000 rows
		for (let i = 1; i <= 1000; i++) {
			const row = `${i},user_${i},user_${i}@example.com,${new Date().toISOString()}\n`;
			yield row;

			// Simulate processing
			if (i % 100 === 0) {
				await new Promise((resolve) => setTimeout(resolve, 10));
			}
		}
	}

	return new StreamingResponse(generateCsv(), {
		contentType: "text/csv",
		headers: {
			"Content-Disposition": 'attachment; filename="users.csv"',
		},
	});
});

/**
 * Browser-friendly HTML page for SSE demonstration
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
  <title>Spikard Streaming Example</title>
  <style>
    body { font-family: monospace; margin: 20px; }
    .section { margin: 20px 0; padding: 10px; border: 1px solid #ccc; }
    #events { border: 1px solid #999; padding: 10px; height: 200px; overflow-y: auto; background: #f5f5f5; }
    button { padding: 5px 10px; margin: 5px; cursor: pointer; }
  </style>
</head>
<body>
  <h1>Spikard Streaming Example</h1>

  <div class="section">
    <h2>Server-Sent Events (SSE)</h2>
    <button onclick="startSSE()">Start Event Stream</button>
    <button onclick="stopSSE()">Stop Stream</button>
    <div id="events">Events will appear here...</div>
  </div>

  <div class="section">
    <h2>Other Examples</h2>
    <p><a href="/stream/numbers?count=50" target="_blank">Stream 50 Numbers (NDJSON)</a></p>
    <p><a href="/stream/csv" target="_blank">Stream CSV</a></p>
  </div>

  <script>
    let eventSource = null;

    function startSSE() {
      if (eventSource) return;

      const eventLog = document.getElementById('events');
      eventLog.innerHTML = 'Connecting...\\n';

      eventSource = new EventSource('/stream/events?duration=10');

      eventSource.addEventListener('tick', (e) => {
        const data = JSON.parse(e.data);
        eventLog.innerHTML += 'TICK: ' + data.message + '\\n';
        eventLog.scrollTop = eventLog.scrollHeight;
      });

      eventSource.addEventListener('complete', (e) => {
        const data = JSON.parse(e.data);
        eventLog.innerHTML += 'COMPLETE: ' + data.message + '\\n';
        stopSSE();
      });

      eventSource.addEventListener('error', () => {
        eventLog.innerHTML += 'ERROR: Connection closed\\n';
        stopSSE();
      });
    }

    function stopSSE() {
      if (eventSource) {
        eventSource.close();
        eventSource = null;
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

console.log("Starting Streaming Example on http://127.0.0.1:8000");
console.log("Open http://127.0.0.1:8000 in your browser to test SSE");
console.log("Or try:");
console.log("  curl http://127.0.0.1:8000/stream/numbers?count=10");
console.log("  curl http://127.0.0.1:8000/stream/events?duration=5");
console.log("  curl http://127.0.0.1:8000/stream/csv");
console.log("");

app.listen().catch((error) => {
	console.error("Server error:", error);
	process.exit(1);
});
