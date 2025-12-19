"""FastAPI benchmark server with DTO validation (Pydantic)."""

import sys
from typing import Any

import uvicorn
from fastapi import FastAPI, Request
from pydantic import BaseModel

app = FastAPI()


class SmallPayload(BaseModel):
    name: str
    description: str
    price: float
    tax: float | None = None


class Image(BaseModel):
    url: str
    name: str


class MediumPayload(BaseModel):
    name: str
    price: float
    image: Image


class Country(BaseModel):
    name: str
    code: str


class Address(BaseModel):
    street: str
    city: str
    country: Country


class SellerWithAddress(BaseModel):
    name: str
    address: Address


class LargePayload(BaseModel):
    name: str
    price: float
    seller: SellerWithAddress


class VeryLargePayload(BaseModel):
    name: str
    tags: list[str]
    images: list[Image]


@app.post("/json/small")
async def post_json_small(body: SmallPayload) -> SmallPayload:
    return body


@app.post("/json/medium")
async def post_json_medium(body: MediumPayload) -> MediumPayload:
    return body


@app.post("/json/large")
async def post_json_large(body: LargePayload) -> LargePayload:
    return body


@app.post("/json/very-large")
async def post_json_very_large(body: VeryLargePayload) -> VeryLargePayload:
    return body


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
    form = await request.form()
    return dict(form)


@app.post("/urlencoded/complex")
async def post_urlencoded_complex(request: Request) -> dict[str, Any]:
    form = await request.form()
    return dict(form)


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
