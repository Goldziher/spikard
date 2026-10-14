"""Install the exact published CLI fixture with its GitHub release digest verified."""

import hashlib
import json
import platform
import shutil
import subprocess
import sys
import tarfile
import urllib.request
from pathlib import Path
from urllib.parse import urlparse

from sync_plugin_version import core_version


def main() -> None:
    """Download and extract a single digest-verified CLI for native plugin tests."""
    destination = Path(sys.argv[1]).resolve()
    destination.mkdir(parents=True, exist_ok=True)
    targets = {("Linux", "x86_64"): "x86_64-unknown-linux-gnu", ("Darwin", "arm64"): "aarch64-apple-darwin"}
    target = targets[(platform.system(), platform.machine())]
    filename = f"spikard-cli-{core_version()}-{target}.tar.gz"
    release = json.loads(
        subprocess.check_output(
            [
                shutil.which("gh") or sys.exit("Install gh to verify published CLI metadata"),
                "release",
                "view",
                "v" + core_version(),
                "--repo",
                "Goldziher/spikard",
                "--json",
                "assets",
            ],
            text=True,
        )
    )
    asset = next(item for item in release["assets"] if item["name"] == filename)
    expected = asset["digest"]
    if not expected.startswith("sha256:"):
        raise ValueError("Published CLI must carry a SHA-256 release digest")
    archive = destination / filename
    location = urlparse(asset["url"])
    if location.scheme != "https" or location.hostname != "github.com":
        raise ValueError("CLI assets must use GitHub HTTPS release URLs")
    urllib.request.urlretrieve(asset["url"], archive)  # noqa: S310 ~keep HTTPS allowlist and digest checked above.
    if hashlib.sha256(archive.read_bytes()).hexdigest() != expected.split(":", 1)[1]:
        raise ValueError("Published CLI digest mismatch")
    with tarfile.open(archive) as bundle:
        bundle.extractall(destination, filter="data")
    binaries = list(destination.rglob("spikard"))
    if len(binaries) != 1 or not binaries[0].is_file():
        raise ValueError("Expected exactly one extracted Kreuzberg executable")
    sys.stdout.write(str(binaries[0]) + "\n")


if __name__ == "__main__":
    main()
