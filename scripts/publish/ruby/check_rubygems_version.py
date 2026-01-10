#!/usr/bin/env python3

"""Check if a RubyGem version exists and emit output for GitHub Actions.

Exit codes:
  0: Gem version exists on RubyGems
  1: Gem version does NOT exist on RubyGems (404 or not in version list)
  2: Check failed due to network/API error (should retry)
"""

from __future__ import annotations

import json
import logging
import os
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

# Configure logging for debugging
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s",
    stream=sys.stderr,
)
logger = logging.getLogger(__name__)

# Configuration
MAX_RETRIES = 3
BASE_BACKOFF_SECONDS = 1.0
RUBYGEMS_API_URL = "https://rubygems.org/api/v1/versions/spikard.json"
REQUEST_TIMEOUT = 10


def write_output(line: str) -> None:
    """Append a single line to GITHUB_OUTPUT if available."""
    output_path = os.environ.get("GITHUB_OUTPUT")
    if not output_path:
        return
    output_file = Path(output_path)
    with output_file.open("a", encoding="utf-8") as handle:
        handle.write(f"{line}\n")


def check_version_exists(version: str) -> bool:
    """Check if a specific version exists in the RubyGems API response.

    Args:
        version: Version string to check (e.g., "0.6.0")

    Returns:
        True if version exists in the API response, False otherwise.

    Raises:
        urllib.error.HTTPError: On HTTP errors
        urllib.error.URLError: On network/connection errors
    """
    logger.info("Checking RubyGems API for version: %s", version)
    request = urllib.request.Request(RUBYGEMS_API_URL, method="GET")  # noqa: S310
    request.add_header("User-Agent", "spikard-release-check/1.0")

    with urllib.request.urlopen(request, timeout=REQUEST_TIMEOUT) as resp:  # noqa: S310
        data = json.load(resp)

    exists = any(entry.get("number") == version for entry in data)
    logger.info("Version %s exists on RubyGems: %s", version, exists)
    return exists


def _handle_retry_error(
    e: Exception,
    error_type: str,
    attempt: int,
) -> tuple[bool | None, int] | None:
    """Handle retry errors and return result if final attempt, else None.

    Returns (exists, exit_code) tuple on final error, None to continue retry loop.
    """
    if isinstance(e, urllib.error.HTTPError):
        if e.code == 404:
            logger.error("RubyGems gem not found (404): %s", e.reason)
            return False, 1
        if e.code == 403:
            logger.error("RubyGems access forbidden (403): %s", e.reason)
        else:
            logger.error("RubyGems HTTP error %s: %s", e.code, e.reason)
    elif isinstance(e, urllib.error.URLError):
        logger.warning("Network/connection error: %s", e.reason)
    elif isinstance(e, (json.JSONDecodeError, KeyError, ValueError)):
        logger.error("Failed to parse RubyGems response: %s", e)
    elif isinstance(e, TimeoutError):
        logger.warning("Request timeout: %s", e)
    else:
        logger.error("Unexpected error during check: %s: %s", type(e).__name__, e)

    if attempt >= MAX_RETRIES:
        logger.error("Max retries exceeded for %s", error_type)
        return None, 2
    return None


def _apply_backoff_and_retry(attempt: int) -> None:
    """Apply exponential backoff before retry."""
    backoff = BASE_BACKOFF_SECONDS * (2 ** (attempt - 1))
    logger.warning("Retrying in %.1fs (attempt %s/%s)", backoff, attempt, MAX_RETRIES)
    time.sleep(backoff)


def check_with_retry(version: str) -> tuple[bool | None, int]:
    """Check if version exists with retry logic and exponential backoff.

    Args:
        version: Version string to check

    Returns:
        Tuple of (exists, exit_code) where:
        - exists is True/False if check succeeded, None if it failed
        - exit_code is 0 (found), 1 (not found), or 2 (check failed)
    """
    for attempt in range(1, MAX_RETRIES + 1):
        try:
            logger.info("Attempt %s/%s to check RubyGems", attempt, MAX_RETRIES)
            exists = check_version_exists(version)
            logger.info("Successfully checked RubyGems: version exists=%s", exists)
            return exists, 0 if exists else 1

        except (
            urllib.error.HTTPError,
            urllib.error.URLError,
            json.JSONDecodeError,
            KeyError,
            ValueError,
            TimeoutError,
            Exception,
        ) as e:
            error_type = type(e).__name__
            result = _handle_retry_error(e, error_type, attempt)
            if result is not None:
                return result
            if attempt < MAX_RETRIES:
                _apply_backoff_and_retry(attempt)
                continue
            return None, 2

    # Should not reach here
    logger.error("Unexpected: exited retry loop without result")
    return None, 2


def main() -> int:
    """Main entrypoint."""
    if len(sys.argv) < 2:
        logger.error("Missing required argument: version")
        sys.stderr.write("Usage: check_rubygems_version.py <version>\n")
        return 2

    version = sys.argv[1]
    logger.info("Starting RubyGems version check for: %s", version)

    exists, exit_code = check_with_retry(version)

    if exists is None:
        logger.error("Check failed - could not determine if version exists")
        write_output("exists=unknown")
        return exit_code

    exists_str = "true" if exists else "false"
    write_output(f"exists={exists_str}")
    logger.info("Check complete: version exists=%s, exit code=%s", exists, exit_code)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
