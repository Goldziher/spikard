"""FastAPI benchmark server (raw/no validation)."""

import sys
import urllib.parse
from typing import Any

import uvicorn
from fastapi import FastAPI, Request

app = FastAPI()

async def _parse_urlencoded(request: Request) -> dict[str, Any]:
    raw = await request.body()
    text = raw.decode("utf-8", errors="replace")
    parsed = urllib.parse.parse_qs(text, keep_blank_values=True, strict_parsing=False)
    return {k: (v[0] if len(v) == 1 else v) for k, v in parsed.items()}


@app.post("/json/small")
async def post_json_small(request: Request) -> dict[str, Any]:
    return await request.json()


@app.post("/json/medium")
async def post_json_medium(request: Request) -> dict[str, Any]:
    return await request.json()


@app.post("/json/large")
async def post_json_large(request: Request) -> dict[str, Any]:
    return await request.json()


@app.post("/json/very-large")
async def post_json_very_large(request: Request) -> dict[str, Any]:
    return await request.json()


@app.post("/multipart/small")
async def post_multipart_small() -> dict[str, int]:
    return {"files_received": 1, "total_bytes": 1024}


@app.post("/multipart/medium")
async def post_multipart_medium() -> dict[str, int]:
    return {"files_received": 2, "total_bytes": 10240}


@app.post("/multipart/large")
async def post_multipart_large() -> dict[str, int]:
    return {"files_received": 5, "total_bytes": 102400}


@app.post("/urlencoded/simple")
async def post_urlencoded_simple(request: Request) -> dict[str, Any]:
    return await _parse_urlencoded(request)


@app.post("/urlencoded/complex")
async def post_urlencoded_complex(request: Request) -> dict[str, Any]:
    return await _parse_urlencoded(request)


@app.get("/path/simple/{id}")
async def get_path_simple(id: str) -> dict[str, str]:
    return {"id": id}


@app.get("/path/multiple/{user_id}/{post_id}")
async def get_path_multiple(user_id: str, post_id: str) -> dict[str, str]:
    return {"user_id": user_id, "post_id": post_id}


@app.get("/path/deep/{org}/{team}/{project}/{resource}/{id}")
async def get_path_deep(org: str, team: str, project: str, resource: str, id: str) -> dict[str, str]:
    return {"org": org, "team": team, "project": project, "resource": resource, "id": id}


@app.get("/path/int/{id}")
async def get_path_int(id: int) -> dict[str, int]:
    return {"id": id}


@app.get("/path/uuid/{id}")
async def get_path_uuid(id: str) -> dict[str, str]:
    return {"id": id}


@app.get("/path/date/{date}")
async def get_path_date(date: str) -> dict[str, str]:
    return {"date": date}


@app.get("/query/few")
async def get_query_few(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


@app.get("/query/medium")
async def get_query_medium(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


@app.get("/query/many")
async def get_query_many(request: Request) -> dict[str, Any]:
    return dict(request.query_params)


@app.get("/health")
async def health() -> dict[str, str]:
    return {"status": "ok"}


@app.get("/")
async def root() -> dict[str, str]:
    return {"status": "ok"}


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    uvicorn.run(app, host="127.0.0.1", port=port, log_level="error", access_log=False)
