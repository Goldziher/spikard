/**
 * AsyncAPI WebSocket tests
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "jsr:@std/assert@1";
import { join, resolve } from "jsr:@std/path@1";
import {
	createAppWebsocketChat,
	ChatMessageMessageSchema,
	UserJoinedMessageSchema,
	UserLeftMessageSchema,
} from "../app/main.ts";

const ROOT_DIR = resolve(new URL(".", import.meta.url).pathname, "../../..");
const WEBSOCKET_FIXTURE_ROOT = join(ROOT_DIR, "testing_data", "websockets");

function loadFixtureExamples(name: string): string[] {
	const fixturePath = join(WEBSOCKET_FIXTURE_ROOT, `${name}.json`);
	const data = JSON.parse(Deno.readTextFileSync(fixturePath));
	const examples = Array.isArray(data.examples) ? data.examples : [];
	if (examples.length === 0) {
		return [JSON.stringify({})];
	}
	return examples.map((example) => JSON.stringify(example));
}

Deno.test("asyncapi_websocket: WebSocket /chat", async () => {
		const app = createAppWebsocketChat();
		const client = new TestClient(app);
		const ws = await client.websocketConnect("/chat");
		const fixtures = [
			{ name: "chatMessage", schema: ChatMessageMessageSchema },
			{ name: "userLeft", schema: UserLeftMessageSchema },
			{ name: "userJoined", schema: UserJoinedMessageSchema },
		];
		for (const { name, schema } of fixtures) {
			const payload = schema.parse(JSON.parse(loadFixtureExamples(name)[0] ?? "{}"));
			await ws.sendJson(payload);
			const response = await ws.receiveJson();
			assertEquals(response.validated, true);
			for (const [key, value] of Object.entries(payload)) {
				assertEquals(response[key], value);
			}
		}
		await ws.close();
	});