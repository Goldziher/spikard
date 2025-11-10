import asyncio
from testing_data.content_types._16_text_plain_not_accepted import create_app_content_types_16_text_plain_not_accepted
from packages.python.spikard.testing import TestClient


async def main():
    app = create_app_content_types_16_text_plain_not_accepted()
    client = TestClient(app)

    # Test with Content-Type: text/plain
    headers = {"Content-Type": "text/plain"}
    raw_body = '{"data": "value"}'
    response = await client.post("/data", headers=headers, data=raw_body)

    print(f"Status: {response.status_code}")
    print(f"Body: {response.text()}")


if __name__ == "__main__":
    asyncio.run(main())
