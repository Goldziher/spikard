"""Example app for testing DEBUG mode error handling."""

from spikard import Spikard

app = Spikard()


@app.get("/ok")
async def ok_endpoint():
    """This works fine."""
    return {"status": "ok"}


@app.get("/error")
async def error_endpoint():
    """This raises an error."""
    # Intentional error - accessing undefined variable
    return {"result": undefined_variable}


@app.get("/type_error")
async def type_error_endpoint():
    """This raises a TypeError."""
    # Intentional TypeError
    result = "hello" + 123
    return {"result": result}


@app.get("/deep_error/{user_id}")
async def deep_error(user_id: int, debug: bool = False):
    """Error deep in call stack."""
    def level1():
        def level2():
            def level3():
                # Error at level 3
                return 1 / 0
            return level3()
        return level2()

    result = level1()
    return {"user_id": user_id, "result": result}


if __name__ == "__main__":
    app.run(host="127.0.0.1", port=8000)
