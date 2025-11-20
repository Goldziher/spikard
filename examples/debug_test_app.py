"""Example app for testing DEBUG mode error handling."""

from typing import Any, NoReturn

from spikard import Spikard

app = Spikard()


def _raise_intentional_error(message: str) -> NoReturn:
    """Raise a runtime error with a well-defined message."""
    raise RuntimeError(message)


@app.get("/ok")
async def ok_endpoint() -> dict[str, str]:
    """This works fine."""
    return {"status": "ok"}


@app.get("/error")
async def error_endpoint() -> NoReturn:
    """This endpoint intentionally raises an error."""
    _raise_intentional_error("Intentional error triggered by /error endpoint")


@app.get("/type_error")
async def type_error_endpoint() -> NoReturn:
    """This endpoint intentionally raises a TypeError."""
    raise TypeError("Intentional TypeError triggered by /type_error endpoint")


@app.get("/deep_error/{user_id}")
async def deep_error(user_id: int, debug: bool = False) -> NoReturn:
    """Error deep in call stack."""
    del debug

    def level1() -> Any:
        def level2() -> Any:
            def level3() -> float:
                raise ZeroDivisionError("Intentional division by zero in level3")

            return level3()

        return level2()

    level1()
    _raise_intentional_error(f"Intentional error triggered for user {user_id}")


if __name__ == "__main__":
    app.run(host="127.0.0.1", port=8000)
