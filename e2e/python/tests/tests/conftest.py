"""Pytest configuration for e2e tests.

Each test creates its own isolated app and client from per-fixture app factories.
This ensures complete test isolation and allows multiple tests for the same route.
"""
