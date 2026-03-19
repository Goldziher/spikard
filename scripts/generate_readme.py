"""Generate README files for spikard root, crates, and packages from Jinja2 templates.

Uses scripts/readme_config.yaml for configuration and scripts/readme_templates/
for Jinja2 templates. Section content is loaded from scripts/readme_content/.

Usage:
    python scripts/generate_readme.py                     # Generate all READMEs
    python scripts/generate_readme.py --target pkg-python  # Generate one target
    python scripts/generate_readme.py --dry-run            # Preview without writing
    python scripts/generate_readme.py --validate           # Check existing match generated
    python scripts/generate_readme.py --verbose            # Verbose output
"""

import argparse
import logging
import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    raise SystemExit("Error: PyYAML is required. Install with: pip install pyyaml jinja2") from None

try:
    from jinja2 import Environment, FileSystemLoader, TemplateNotFound, select_autoescape
except ImportError:
    raise SystemExit("Error: Jinja2 is required. Install with: pip install pyyaml jinja2") from None


logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")
logger = logging.getLogger(__name__)

PROJECT_ROOT = Path(__file__).parent.parent
SCRIPTS_DIR = PROJECT_ROOT / "scripts"
TEMPLATES_DIR = SCRIPTS_DIR / "readme_templates"
CONTENT_DIR = SCRIPTS_DIR / "readme_content"
CONFIG_PATH = SCRIPTS_DIR / "readme_config.yaml"


def get_workspace_version() -> str:
    """Read workspace version from root Cargo.toml."""
    cargo_toml = PROJECT_ROOT / "Cargo.toml"
    content = cargo_toml.read_text(encoding="utf-8")
    match = re.search(r'^\s*version\s*=\s*"([^"]+)"', content, re.MULTILINE)
    if not match:
        raise ValueError("Could not find version in Cargo.toml")
    return match.group(1)


def load_config() -> dict:
    """Load and parse README configuration from YAML."""
    if not CONFIG_PATH.exists():
        raise FileNotFoundError(f"Configuration not found: {CONFIG_PATH}")
    with CONFIG_PATH.open(encoding="utf-8") as f:
        config = yaml.safe_load(f)
    if not config or "targets" not in config:
        raise ValueError("Configuration file is empty or missing 'targets'")
    logger.info("Loaded %d targets from config", len(config["targets"]))
    return config


def load_content_file(filename: str) -> str:
    """Load a content fragment from scripts/readme_content/."""
    path = CONTENT_DIR / filename
    if not path.exists():
        logger.warning("Content file not found: %s", path)
        return ""
    return path.read_text(encoding="utf-8").rstrip()


def setup_jinja_env() -> Environment:
    """Configure Jinja2 environment."""
    if not TEMPLATES_DIR.exists():
        raise FileNotFoundError(f"Templates directory not found: {TEMPLATES_DIR}")
    return Environment(
        loader=FileSystemLoader(str(TEMPLATES_DIR)),
        autoescape=select_autoescape(
            enabled_extensions=("html", "xml"),
            default=False,
            default_for_string=False,
        ),
        keep_trailing_newline=True,
        trim_blocks=True,
        lstrip_blocks=True,
    )


def resolve_content_files(target: dict) -> dict:
    """Resolve all *_file keys in target config to their content."""
    resolved = dict(target)
    for key in list(resolved.keys()):
        if key.endswith("_file") and isinstance(resolved[key], str):
            content_key = key[: -len("_file")] + "_content"
            if content_key.endswith("_content_content"):
                content_key = key[: -len("_file")]
            resolved[content_key] = load_content_file(resolved[key])
    return resolved


def generate_readme(
    target: dict,
    config: dict,
    env: Environment,
    version: str,
    dry_run: bool = False,
) -> str:
    """Render a single README from template."""
    template_name = target.get("template", "crate_readme.md.jinja")
    try:
        template = env.get_template(template_name)
    except TemplateNotFound as err:
        raise TemplateNotFound(
            f"Template not found: {template_name}\nExpected at: {TEMPLATES_DIR / template_name}"
        ) from err

    # Resolve content files
    resolved = resolve_content_files(target)

    context = {
        "version": version,
        "project": config.get("project", {}),
        **resolved,
    }

    content = template.render(**context)

    # Ensure single trailing newline
    content = content.rstrip() + "\n"

    output_path = PROJECT_ROOT / target["output_path"]
    if not dry_run:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(content, encoding="utf-8")
        logger.info("Generated: %s", output_path.relative_to(PROJECT_ROOT))
    else:
        logger.info("[DRY-RUN] Would generate: %s", output_path.relative_to(PROJECT_ROOT))

    return content


def validate_readme(
    target: dict,
    config: dict,
    env: Environment,
    version: str,
) -> bool:
    """Check if existing README matches generated output."""
    output_path = PROJECT_ROOT / target["output_path"]
    if not output_path.exists():
        logger.warning("README not found: %s", output_path.relative_to(PROJECT_ROOT))
        return False

    generated = generate_readme(target, config, env, version, dry_run=True)
    existing = output_path.read_text(encoding="utf-8")

    if generated == existing:
        logger.info("Valid: %s", output_path.relative_to(PROJECT_ROOT))
        return True

    logger.warning("Out of date: %s", output_path.relative_to(PROJECT_ROOT))
    return False


def process_targets(
    config: dict,
    env: Environment,
    version: str,
    target_filter: str | None = None,
    dry_run: bool = False,
    validate_only: bool = False,
) -> bool:
    """Process all configured README targets."""
    targets = config.get("targets", [])
    if target_filter:
        targets = [t for t in targets if t["id"] == target_filter]
        if not targets:
            available = [t["id"] for t in config.get("targets", [])]
            logger.error("Unknown target: %s", target_filter)
            logger.info("Available: %s", ", ".join(available))
            return False

    all_ok = True
    for target in targets:
        try:
            if validate_only:
                if not validate_readme(target, config, env, version):
                    all_ok = False
            else:
                generate_readme(target, config, env, version, dry_run)
        except Exception as e:
            logger.error("Failed to process %s: %s", target["id"], e)
            all_ok = False

    return all_ok


def parse_args() -> argparse.Namespace:
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser(
        description="Generate README files from Jinja2 templates",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python scripts/generate_readme.py
  python scripts/generate_readme.py --target pkg-python
  python scripts/generate_readme.py --dry-run
  python scripts/generate_readme.py --validate
        """,
    )
    parser.add_argument("--target", help="Generate only this target ID", metavar="ID")
    parser.add_argument("--dry-run", action="store_true", help="Preview without writing")
    parser.add_argument("--validate", action="store_true", help="Check existing READMEs match templates")
    parser.add_argument("-v", "--verbose", action="store_true", help="Verbose output")
    return parser.parse_args()


def main() -> int:
    """Main entry point."""
    args = parse_args()
    if args.verbose:
        logger.setLevel(logging.DEBUG)

    try:
        config = load_config()
        version = get_workspace_version()
        logger.info("Workspace version: %s", version)

        env = setup_jinja_env()

        success = process_targets(
            config,
            env,
            version,
            target_filter=args.target,
            dry_run=args.dry_run,
            validate_only=args.validate,
        )

        if args.validate:
            if success:
                logger.info("All READMEs are up-to-date")
            else:
                logger.error("Some READMEs are out of date. Run: python scripts/generate_readme.py")
        elif success:
            logger.info("README generation completed successfully")
        else:
            logger.error("README generation completed with errors")

        return 0 if success else 1

    except Exception as e:
        logger.error("Fatal error: %s", e)
        return 1


if __name__ == "__main__":
    sys.exit(main())
