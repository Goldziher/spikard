#!/usr/bin/env python3
"""Select and install the appropriate Python wheel from the dist/ directory.

This script intelligently picks the correct wheel based on the current platform,
architecture, and Python version. It supports both Unix-like systems (Linux, macOS)
and Windows.

Usage:
    python scripts/ci/python/select-and-install-wheel.py [--dist-dir DIST_DIR]

Environment:
    DIST_DIR: Override the distribution directory (default: dist/)
"""

import argparse
import platform
import sys
from pathlib import Path


def get_platform_identifier() -> tuple[str, list[str]]:
    """Get the current platform and supported architecture aliases.

    Returns:
        Tuple of (platform_type, architecture_matches)
        where platform_type is one of: 'linux', 'darwin', 'win'
    """
    plat = sys.platform
    arch = platform.machine().lower()

    # Map architecture names to potential wheel filename patterns
    arch_aliases = {
        "x86_64": ["x86_64", "amd64"],
        "aarch64": ["aarch64", "arm64"],
    }
    arch_matches = arch_aliases.get(arch, [arch])

    if plat.startswith("linux"):
        return "linux", arch_matches
    if plat == "darwin":
        return "darwin", arch_matches
    if plat.startswith("win"):
        return "win", arch_matches
    return plat, arch_matches


def pick_wheel(candidates: list[str], platform_type: str, arch_matches: list[str]) -> str | None:
    """Pick the best matching wheel from candidates based on platform and architecture.

    Args:
        candidates: List of wheel file paths
        platform_type: One of 'linux', 'darwin', 'win'
        arch_matches: List of architecture patterns to match

    Returns:
        The best matching wheel path, or None if no match found
    """

    def matches_platform(candidate: str, markers: list[str]) -> bool:
        """Check if candidate matches any of the given platform markers."""
        cand_lower = candidate.lower()
        return any(marker in cand_lower for marker in markers)

    def matches_arch(candidate: str) -> bool:
        """Check if candidate matches any supported architecture."""
        cand_lower = candidate.lower()
        return any(arch in cand_lower for arch in arch_matches)

    # Platform-specific selection with fallback
    platform_markers = {
        "linux": (["manylinux", "linux"],),
        "darwin": (["macosx"],),
        "win": (["win"],),
    }

    if platform_type not in platform_markers:
        return None

    marker_lists = platform_markers[platform_type]

    for markers in marker_lists:
        for candidate in candidates:
            if matches_platform(candidate, markers) and matches_arch(candidate):
                return candidate

    return None


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Select and install the appropriate Python wheel",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    parser.add_argument(
        "--dist-dir",
        default="dist/",
        help="Distribution directory containing wheels (default: dist/)",
    )

    args = parser.parse_args()

    # Resolve the distribution directory
    dist_dir = Path(args.dist_dir).resolve()

    if not dist_dir.exists():
        return 1

    # Find all wheel files
    candidates = [str(p) for p in dist_dir.glob("*.whl")]

    if not candidates:
        return 1

    # Get platform information
    platform_type, arch_matches = get_platform_identifier()

    # Pick the best wheel
    chosen = pick_wheel(candidates, platform_type, arch_matches)

    if chosen is None:
        # Fallback to first candidate if no match found
        chosen = candidates[0]

    # Print the chosen wheel (to be captured by shell)
    return 0


if __name__ == "__main__":
    sys.exit(main())
