"""Ergonomic-layer smoke server (Python / msgspec).

Exercises the ergonomic typed-handler + DTO API end-to-end: a typed handler
whose body is a msgspec.Struct, hydrated by the ergonomic layer, with request
validation delegated to the Rust core (invalid bodies -> 422 ProblemDetails).
"""

import msgspec

from spikard import App, Body


class CreateUser(msgspec.Struct):
    name: str
    email: str
    age: int


app = App()


@app.post("/users")
async def create_user(user: Body[CreateUser]) -> CreateUser:
    # The ergonomic layer must hand us a real CreateUser instance.
    assert isinstance(user, CreateUser), f"expected CreateUser, got {type(user)!r}"
    return user


# NOTE: the current binding's app_run(registrations) ignores ServerConfig, so
# the server always binds 127.0.0.1:8000 (a known low-level limitation).
app.run()
