/**
 * WebSocket and Advanced SSE Example
 *
 * Demonstrates WebSocket bidirectional communication and
 * advanced Server-Sent Events patterns.
 */

import { get, post, type Request, Spikard, StreamingResponse } from "@spikard/node";

const app = new Spikard();

interface Message {
	id: number;
	user: string;
	text: string;
	timestamp: string;
}

const messages: Message[] = [];
let messageId = 0;

/**
 * WebSocket endpoint for real-time chat
 */
app.websocket(
	"/ws/chat",
	async (message: unknown) => {
		if (typeof message === "string") {
			try {
				const data = JSON.parse(message);
				const userName = data.user || "Anonymous";
				const text = data.message || "";

				if (!text) {
					return {
						type: "error",
						error: "Empty message",
					};
				}

				const msg: Message = {
					id: ++messageId,
					user: userName,
					text,
					timestamp: new Date().toISOString(),
				};
				messages.push(msg);

				return {
					type: "message",
					data: msg,
					totalMessages: messages.length,
				};
			} catch {
				return {
					type: "error",
					error: "Invalid message format",
				};
			}
		}

		return null;
	},
	{
		onConnect: async () => {
			console.log("Client connected to chat");
		},
		onDisconnect: async () => {
			console.log("Client disconnected from chat");
		},
	},
);

/**
 * WebSocket endpoint for real-time notifications
 */
app.websocket(
	"/ws/notifications",
	async (message: unknown) => {
		return {
			type: "echo",
			received: message,
			timestamp: new Date().toISOString(),
		};
	},
	{
		onConnect: async () => {
			console.log("Client connected to notifications");
		},
		onDisconnect: async () => {
			console.log("Client disconnected from notifications");
		},
	},
);

/**
 * POST endpoint to retrieve chat history via SSE
 */
post("/sse/chat-history")(async function chatHistorySse(req: Request) {
	const startId = (req.query?.since as string | undefined) ? parseInt(req.query.since as string, 10) : 0;

	async function* generateHistory() {
		yield {
			event: "history_start",
			data: JSON.stringify({
				total: messages.length,
				since: startId,
				timestamp: new Date().toISOString(),
			}),
		};

		for (const msg of messages.filter((m) => m.id > startId)) {
			yield {
				event: "message",
				data: JSON.stringify(msg),
			};

			await new Promise((resolve) => setTimeout(resolve, 10));
		}

		yield {
			event: "history_complete",
			data: JSON.stringify({
				total: messages.length,
				timestamp: new Date().toISOString(),
			}),
		};
	}

	return new StreamingResponse(generateHistory(), {
		statusCode: 200,
		headers: {
			"Content-Type": "text/event-stream",
			"Cache-Control": "no-cache",
			Connection: "keep-alive",
			"Access-Control-Allow-Origin": "*",
		},
	});
});

/**
 * GET endpoint for metrics stream via SSE
 */
get("/sse/metrics")(async function metricsStream(req: Request) {
	const interval = (req.query?.interval as string | undefined) ? parseInt(req.query.interval as string, 10) : 1000;

	async function* generateMetrics() {
		let iteration = 0;

		while (iteration < 60) {
			const cpuUsage = Math.random() * 100;
			const memoryUsage = Math.random() * 100;
			const requestCount = Math.floor(Math.random() * 1000);

			yield {
				event: "metrics",
				data: JSON.stringify({
					timestamp: new Date().toISOString(),
					cpu: cpuUsage.toFixed(2),
					memory: memoryUsage.toFixed(2),
					requests: requestCount,
					iteration,
				}),
			};

			iteration++;
			await new Promise((resolve) => setTimeout(resolve, interval));
		}

		yield {
			event: "complete",
			data: JSON.stringify({
				message: "Metrics collection complete",
				iterations: iteration,
			}),
		};
	}

	return new StreamingResponse(generateMetrics(), {
		statusCode: 200,
		headers: {
			"Content-Type": "text/event-stream",
			"Cache-Control": "no-cache",
			Connection: "keep-alive",
		},
	});
});

/**
 * Serve demo HTML page
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
  <title>Spikard WebSocket & SSE Example</title>
  <style>
    * { box-sizing: border-box; }
    body { font-family: monospace; margin: 0; padding: 20px; background: #f0f0f0; }
    .container { max-width: 1200px; margin: 0 auto; }
    .section { background: white; margin: 20px 0; padding: 15px; border-radius: 5px; }
    h2 { margin-top: 0; }
    .chat-box, .notification-box { border: 1px solid #ccc; height: 200px; overflow-y: auto; background: #f9f9f9; padding: 10px; margin: 10px 0; }
    .message { margin: 5px 0; padding: 5px; border-left: 3px solid #007bff; background: #f0f8ff; }
    .notification { margin: 5px 0; padding: 5px; border-left: 3px solid #17a2b8; background: #f0f7ff; }
    input { width: 100%; padding: 8px; margin: 5px 0; border: 1px solid #ddd; border-radius: 3px; }
    button { padding: 8px 15px; margin: 5px 5px 5px 0; background: #007bff; color: white; border: none; border-radius: 3px; cursor: pointer; }
    button:hover { background: #0056b3; }
    button:disabled { background: #ccc; cursor: not-allowed; }
    .metrics { font-size: 12px; }
    .metric-row { display: grid; grid-template-columns: 100px 100px 100px; gap: 10px; margin: 5px 0; }
  </style>
</head>
<body>
  <div class="container">
    <h1>Spikard WebSocket & SSE Example</h1>

    <div class="section">
      <h2>WebSocket Chat</h2>
      <div class="chat-box" id="chat"></div>
      <input type="text" id="userName" placeholder="Your name" value="User">
      <input type="text" id="messageInput" placeholder="Type message..." onkeypress="if (event.key === 'Enter') sendMessage()">
      <button onclick="connectChat()">Connect</button>
      <button onclick="disconnectChat()">Disconnect</button>
      <button onclick="sendMessage()">Send</button>
    </div>

    <div class="section">
      <h2>WebSocket Notifications</h2>
      <div class="notification-box" id="notifications"></div>
      <button onclick="connectNotifications()">Start Notifications</button>
      <button onclick="disconnectNotifications()">Stop</button>
    </div>

    <div class="section">
      <h2>Metrics Stream (SSE)</h2>
      <div class="metrics">
        <div class="metric-row">
          <div style="font-weight: bold;">CPU %</div>
          <div style="font-weight: bold;">Memory %</div>
          <div style="font-weight: bold;">Requests</div>
        </div>
      </div>
      <div class="metrics" id="metrics"></div>
      <button onclick="streamMetrics()">Start Metrics</button>
    </div>
  </div>

  <script>
    let chatWs = null;
    let notifWs = null;
    let metricsEventSource = null;

    // Chat WebSocket functions
    function connectChat() {
      if (chatWs) return;
      chatWs = new WebSocket('ws://localhost:8000/ws/chat');
      chatWs.onmessage = (e) => {
        const msg = JSON.parse(e.data);
        const chatDiv = document.getElementById('chat');
        const html = \`<div class="message">\${msg.type}: \${JSON.stringify(msg).substring(0, 80)}...</div>\`;
        chatDiv.innerHTML += html;
        chatDiv.scrollTop = chatDiv.scrollHeight;
      };
      chatWs.onerror = () => alert('Chat connection error');
    }

    function disconnectChat() {
      if (chatWs) chatWs.close();
      chatWs = null;
    }

    function sendMessage() {
      if (!chatWs || chatWs.readyState !== WebSocket.OPEN) {
        alert('Not connected');
        return;
      }
      const name = document.getElementById('userName').value || 'User';
      const text = document.getElementById('messageInput').value;
      if (!text) return;
      chatWs.send(JSON.stringify({ user: name, message: text }));
      document.getElementById('messageInput').value = '';
    }

    // Notifications WebSocket
    function connectNotifications() {
      if (notifWs) return;
      notifWs = new WebSocket('ws://localhost:8000/ws/notifications');
      notifWs.onmessage = (e) => {
        const msg = JSON.parse(e.data);
        const notifDiv = document.getElementById('notifications');
        const html = \`<div class="notification">\${msg.type}: \${msg.message || JSON.stringify(msg).substring(0, 60)}</div>\`;
        notifDiv.innerHTML += html;
        notifDiv.scrollTop = notifDiv.scrollHeight;
      };
    }

    function disconnectNotifications() {
      if (notifWs) notifWs.close();
      notifWs = null;
    }

    // Metrics SSE
    function streamMetrics() {
      if (metricsEventSource) return;
      const metricsDiv = document.getElementById('metrics');
      metricsDiv.innerHTML = '';

      metricsEventSource = new EventSource('/sse/metrics?interval=500');
      metricsEventSource.addEventListener('metrics', (e) => {
        const data = JSON.parse(e.data);
        const html = \`<div class="metric-row">
          <div>\${data.cpu}%</div>
          <div>\${data.memory}%</div>
          <div>\${data.requests}</div>
        </div>\`;
        metricsDiv.innerHTML += html;
        if (metricsDiv.children.length > 20) {
          metricsDiv.removeChild(metricsDiv.firstChild);
        }
      });
      metricsEventSource.addEventListener('complete', () => {
        metricsEventSource.close();
        metricsEventSource = null;
      });
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

console.log("Starting WebSocket & SSE Example on http://127.0.0.1:8000");
console.log("Open http://127.0.0.1:8000 in your browser");
console.log("");

app.run({ port: 8000, host: "0.0.0.0" });
