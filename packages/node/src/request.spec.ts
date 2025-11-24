import { describe, expect, it } from "vitest";
import { isNativeHandler, wrapHandler } from "./handler-wrapper";
import type { NativeRequestData } from "./request";

const basePayload = (): NativeRequestData => ({
	method: "GET",
	path: "/users/42",
	params: { id: "42" },
	query: { verbose: "true" },
	headers: { authorization: "Bearer token", "x-custom": "Value" },
	cookies: { session_id: "abc123" },
	body: Array.from(Buffer.from(JSON.stringify({ hello: "world" }))),
});

describe("Request wrapper", () => {
	it("exposes parsed params, query, headers, cookies, and JSON body", async () => {
		let captured: {
			id?: string;
			verbose?: string;
			auth?: string;
			cookie?: string;
			body?: { hello: string };
		} = {};

		const handler = wrapHandler(async (req) => {
			captured = {
				id: req.params.id,
				verbose: req.query.verbose,
				auth: req.headers.authorization,
				cookie: req.cookies.session_id,
				body: req.json<{ hello: string }>(),
			};
			return { ok: true };
		});

		const payload = basePayload();
		const responseJson = await handler(JSON.stringify(payload));
		expect(JSON.parse(responseJson)).toEqual({ ok: true });

		expect(captured).toEqual({
			id: "42",
			verbose: "true",
			auth: "Bearer token",
			cookie: "abc123",
			body: { hello: "world" },
		});
	});

	it("parses urlencoded form bodies", async () => {
		const handler = wrapHandler(async (req) => {
			return { form: req.form() };
		});

		const payload = basePayload();
		payload.body = Array.from(Buffer.from("foo=bar&baz=qux"));

		const responseJson = await handler(JSON.stringify(payload));
		expect(JSON.parse(responseJson)).toEqual({ form: { foo: "bar", baz: "qux" } });
	});

	it("marks wrapped handlers as native handlers", () => {
		const handler = wrapHandler(async () => undefined);
		expect(isNativeHandler(handler)).toBe(true);
	});
});
