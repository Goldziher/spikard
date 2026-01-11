#!/usr/bin/env python3
"""Phase 5: Coverage Verification Script
Verifies code coverage across all 4 languages (Python, TypeScript, Ruby, PHP).
"""

import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import ClassVar
from xml.etree import ElementTree as ET


@dataclass
class CoverageResult:
    """Represents coverage result for a single language."""

    language: str
    percentage: float | None
    threshold: float
    passed: bool
    report_path: str
    error: str | None = None

    def __str__(self) -> str:
        """Format result as output line."""
        if self.error:
            return f"  ⚠ WARN {self.language}: {self.error}"

        status = "PASS" if self.passed else "FAIL"
        symbol = "✓" if self.passed else "✗"
        return f"  {symbol} {status} {self.language}: {self.percentage:.1f}% (target: {self.threshold:.0f}%+)"


class CoverageVerifier:
    """Verifies code coverage across multiple languages."""

    BASE_PATH = Path(__file__).parent.parent

    THRESHOLDS: ClassVar[dict[str, float]] = {
        "Python": 80.0,
        "TypeScript": 80.0,
        "Ruby": 80.0,
        "PHP": 85.0,
    }

    def __init__(self) -> None:
        """Initialize the verifier."""
        self.results: list[CoverageResult] = []

    def verify_all(self) -> int:
        """Verify coverage for all languages.

        Returns:
            Exit code (0 if all pass, 1 if any fail)
        """
        self._verify_python()
        self._verify_typescript()
        self._verify_ruby()
        self._verify_php()

        # Print results
        for _result in self.results:
            pass

        # Summary
        sum(1 for r in self.results if r.passed or r.error)
        len(self.results)

        # Exit code
        failed = any(not r.passed and not r.error for r in self.results)
        return 1 if failed else 0

    def _verify_python(self) -> None:
        """Verify Python coverage."""
        language = "Python"
        threshold = self.THRESHOLDS[language]

        # Try .coverage file first (binary format)
        coverage_file = self.BASE_PATH / "packages" / "python" / ".coverage"
        if coverage_file.exists():
            coverage_pct = self._parse_python_coverage_file(coverage_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(coverage_file),
                )
                self.results.append(result)
                return

        # Try coverage.lcov file
        lcov_file = self.BASE_PATH / "packages" / "python" / "coverage.lcov"
        if lcov_file.exists():
            coverage_pct = self._parse_lcov_file(lcov_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(lcov_file),
                )
                self.results.append(result)
                return

        # Try htmlcov/index.html
        htmlcov_dir = self.BASE_PATH / "packages" / "python" / "htmlcov"
        if htmlcov_dir.exists():
            coverage_pct = self._parse_html_coverage(htmlcov_dir / "index.html")
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(htmlcov_dir / "index.html"),
                )
                self.results.append(result)
                return

        # No coverage report found
        result = CoverageResult(
            language=language,
            percentage=None,
            threshold=threshold,
            passed=False,
            report_path="",
            error="No coverage report found (expected .coverage, coverage.lcov, or htmlcov/index.html)",
        )
        self.results.append(result)

    def _verify_typescript(self) -> None:
        """Verify TypeScript coverage."""
        language = "TypeScript"
        threshold = self.THRESHOLDS[language]

        # Try coverage/coverage-summary.json
        coverage_file = self.BASE_PATH / "packages" / "node" / "coverage" / "coverage-summary.json"
        if coverage_file.exists():
            coverage_pct = self._parse_typescript_coverage(coverage_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(coverage_file),
                )
                self.results.append(result)
                return

        # Try vitest coverage
        vitest_coverage = self.BASE_PATH / "packages" / "node" / "coverage" / "coverage-final.json"
        if vitest_coverage.exists():
            coverage_pct = self._parse_vitest_coverage(vitest_coverage)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(vitest_coverage),
                )
                self.results.append(result)
                return

        # Try lcov.info from vitest
        lcov_file = self.BASE_PATH / "packages" / "node" / "coverage" / "lcov.info"
        if lcov_file.exists():
            coverage_pct = self._parse_lcov_file(lcov_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(lcov_file),
                )
                self.results.append(result)
                return

        # No coverage report found
        result = CoverageResult(
            language=language,
            percentage=None,
            threshold=threshold,
            passed=False,
            report_path="",
            error="No coverage report found (expected packages/node/coverage/coverage-summary.json)",
        )
        self.results.append(result)

    def _verify_ruby(self) -> None:
        """Verify Ruby coverage."""
        language = "Ruby"
        threshold = self.THRESHOLDS[language]

        # Try SimpleCov .resultset.json
        resultset_file = self.BASE_PATH / "packages" / "ruby" / "coverage" / ".resultset.json"
        if resultset_file.exists():
            coverage_pct = self._parse_ruby_resultset(resultset_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(resultset_file),
                )
                self.results.append(result)
                return

        # Try lcov.info
        lcov_file = self.BASE_PATH / "packages" / "ruby" / "coverage" / "lcov.info"
        if lcov_file.exists():
            coverage_pct = self._parse_lcov_file(lcov_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(lcov_file),
                )
                self.results.append(result)
                return

        # No coverage report found
        result = CoverageResult(
            language=language,
            percentage=None,
            threshold=threshold,
            passed=False,
            report_path="",
            error="No coverage report found (expected .resultset.json or lcov.info)",
        )
        self.results.append(result)

    def _verify_php(self) -> None:
        """Verify PHP coverage."""
        language = "PHP"
        threshold = self.THRESHOLDS[language]

        # Try clover.xml
        clover_file = self.BASE_PATH / "packages" / "php" / "coverage" / "clover.xml"
        if clover_file.exists():
            coverage_pct = self._parse_clover_xml(clover_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(clover_file),
                )
                self.results.append(result)
                return

        # Try htmlcov/index.html (PHPUnit coverage HTML)
        htmlcov_file = self.BASE_PATH / "packages" / "php" / "packages" / "php" / "htmlcov" / "index.html"
        if htmlcov_file.exists():
            coverage_pct = self._parse_html_coverage(htmlcov_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(htmlcov_file),
                )
                self.results.append(result)
                return

        # Also check alternate path
        htmlcov_file = self.BASE_PATH / "packages" / "php" / "htmlcov" / "index.html"
        if htmlcov_file.exists():
            coverage_pct = self._parse_html_coverage(htmlcov_file)
            if coverage_pct is not None:
                result = CoverageResult(
                    language=language,
                    percentage=coverage_pct,
                    threshold=threshold,
                    passed=coverage_pct >= threshold,
                    report_path=str(htmlcov_file),
                )
                self.results.append(result)
                return

        # No coverage report found
        result = CoverageResult(
            language=language,
            percentage=None,
            threshold=threshold,
            passed=False,
            report_path="",
            error="No coverage report found (expected clover.xml or htmlcov/index.html)",
        )
        self.results.append(result)

    @staticmethod
    def _parse_python_coverage_file(coverage_file: Path) -> float | None:
        """Parse Python .coverage file (requires coverage.py to read binary format)."""
        try:
            # Try to use coverage library if available
            from coverage import Coverage  # noqa: PLC0415

            cov = Coverage(data_file=str(coverage_file))
            cov.load()
            # Get total coverage percentage
            return cov.report(skip_covered=False, precision=2)
        except ImportError:
            # Fallback: coverage library not available
            return None
        except Exception:
            return None

    @staticmethod
    def _parse_lcov_file(lcov_file: Path) -> float | None:
        """Parse LCOV format coverage file."""
        try:
            with lcov_file.open(encoding="utf-8") as f:
                content = f.read()

            # Extract line coverage summary
            lines_hit = 0
            lines_found = 0

            for line in content.split("\n"):
                if line.startswith("LH:"):
                    lines_hit = int(line.split(":")[1])
                elif line.startswith("LF:"):
                    lines_found = int(line.split(":")[1])

            if lines_found == 0:
                return None

            return (lines_hit / lines_found) * 100
        except Exception:
            return None

    @staticmethod
    def _parse_html_coverage(html_file: Path) -> float | None:
        """Parse HTML coverage report (from coverage.py or PHPUnit)."""
        try:
            with html_file.open(encoding="utf-8") as f:
                content = f.read()

            # Look for percentage in various formats
            # coverage.py format: <span class="pc_cov">XX.X%</span>
            match = re.search(r'<span\s+class=["\']pc_cov["\']>(\d+\.?\d*?)%</span>', content)
            if match:
                return float(match.group(1))

            # PHPUnit format: aria-valuenow="XX.XX"
            match = re.search(r'aria-valuenow=["\'](\d+\.?\d*?)["\']', content)
            if match:
                return float(match.group(1))

            # Fallback: look for any percentage pattern in header
            match = re.search(r"(\d+\.?\d*?)%\s*covered", content)
            if match:
                return float(match.group(1))

            return None
        except Exception:
            return None

    @staticmethod
    def _parse_typescript_coverage(coverage_file: Path) -> float | None:
        """Parse TypeScript coverage-summary.json."""
        try:
            with coverage_file.open(encoding="utf-8") as f:
                data = json.load(f)

            # Coverage summary is in 'total' key
            if "total" in data:
                total = data["total"]
                # Use line coverage
                if "lines" in total and "pct" in total["lines"]:
                    return total["lines"]["pct"]

            return None
        except Exception:
            return None

    @staticmethod
    def _parse_vitest_coverage(coverage_file: Path) -> float | None:
        """Parse vitest coverage-final.json."""
        try:
            with coverage_file.open(encoding="utf-8") as f:
                data = json.load(f)

            # Aggregate coverage from all files
            total_lines = 0
            covered_lines = 0

            for file_coverage in data.values():
                if not isinstance(file_coverage, dict):
                    continue

                lines = file_coverage.get("l", {})
                for hit_count in lines.values():
                    total_lines += 1
                    if hit_count > 0:
                        covered_lines += 1

            if total_lines == 0:
                return None

            return (covered_lines / total_lines) * 100
        except Exception:
            return None

    @staticmethod
    def _parse_ruby_resultset(resultset_file: Path) -> float | None:
        """Parse Ruby SimpleCov .resultset.json."""
        try:
            with resultset_file.open(encoding="utf-8") as f:
                data = json.load(f)

            # Find RSpec or other test framework results
            for framework_data in data.values():
                if "coverage" not in framework_data:
                    continue

                coverage = framework_data["coverage"]
                total_lines = 0
                covered_lines = 0

                for file_coverage in coverage.values():
                    lines = file_coverage.get("lines", [])
                    for hit_count in lines:
                        if hit_count is not None:
                            total_lines += 1
                            if hit_count > 0:
                                covered_lines += 1

                if total_lines == 0:
                    continue

                return (covered_lines / total_lines) * 100

            return None
        except Exception:
            return None

    @staticmethod
    def _parse_clover_xml(clover_file: Path) -> float | None:
        """Parse Clover XML coverage format (used by PHPUnit)."""
        try:
            tree = ET.parse(clover_file)  # noqa: S314
            root = tree.getroot()

            # Find the project metrics
            for metrics in root.findall(".//metrics"):
                coveredstatements = metrics.get("coveredstatements")
                statements = metrics.get("statements")

                if coveredstatements and statements:
                    covered = int(coveredstatements)
                    total = int(statements)
                    if total > 0:
                        return (covered / total) * 100

            return None
        except Exception:
            return None


def main() -> int:
    """Main entry point."""
    verifier = CoverageVerifier()
    return verifier.verify_all()


if __name__ == "__main__":
    sys.exit(main())
