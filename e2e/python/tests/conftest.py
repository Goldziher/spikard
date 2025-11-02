"""Pytest configuration and fixtures for e2e tests."""

import pytest
from spikard.testing import TestClient

from app.main import app


@pytest.fixture(scope="session")
def client():
    """Provide a test client."""
    return TestClient(app)
