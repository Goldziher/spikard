import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createApp } from './app.js';

const __dirname = dirname(fileURLToPath(import.meta.url));

describe('Spikard WASM Test App', () => {
  let server;
  let baseUrl;

  beforeAll(async () => {
    server = await createApp();
    await server.start();
    const address = server.address();
    baseUrl = `http://${address.host}:${address.port}`;
  });

  afterAll(async () => {
    await server.stop();
  });

  it('should use the correct package version', () => {
    const pkg = JSON.parse(
      readFileSync(join(__dirname, 'package.json'), 'utf-8')
    );
    expect(pkg.dependencies['@spikard/wasm']).toBe('0.6.1');
  });

  it('should respond to health check', async () => {
    const res = await fetch(`${baseUrl}/health`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data).toEqual({ status: 'ok' });
  });

  it('should handle query parameters', async () => {
    const res = await fetch(`${baseUrl}/query?name=Alice&age=30`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data).toEqual({ name: 'Alice', age: 30 });
  });

  it('should echo JSON requests', async () => {
    const payload = { message: 'Hello from WASM!' };
    const res = await fetch(`${baseUrl}/echo`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.received).toEqual(payload);
    expect(data.method).toBe('POST');
  });

  it('should extract path parameters', async () => {
    const res = await fetch(`${baseUrl}/users/42`);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.userId).toBe('42');
    expect(data.type).toBe('string');
  });
});
