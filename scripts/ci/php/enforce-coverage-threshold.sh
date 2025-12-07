#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

# Parse arguments
COVERAGE_FILE="${1:-target/clover.xml}"
THRESHOLD="${2:-85}"

# Validate that coverage file exists
if [[ ! -f "${REPO_ROOT}/${COVERAGE_FILE}" ]]; then
	echo "Error: Coverage file not found at ${REPO_ROOT}/${COVERAGE_FILE}"
	exit 1
fi

# Extract coverage percentage from Clover XML
echo "Extracting coverage from ${COVERAGE_FILE}..."

COVERAGE=$(php -r "
	if (!file_exists('${COVERAGE_FILE}')) {
		fprintf(STDERR, \"Error: Coverage file ${COVERAGE_FILE} not found\n\");
		exit(1);
	}

	\$xml = simplexml_load_file('${COVERAGE_FILE}');
	if (\$xml === false) {
		fprintf(STDERR, \"Error: Failed to parse XML from ${COVERAGE_FILE}\n\");
		exit(1);
	}

	\$metrics = \$xml->project->metrics;
	if (\$metrics === null) {
		fprintf(STDERR, \"Error: Could not find metrics in coverage file\n\");
		exit(1);
	}

	\$lines = (int)\$metrics['coveredstatements'];
	\$total = (int)\$metrics['statements'];

	if (\$total === 0) {
		fprintf(STDERR, \"Error: No statements found in coverage file\n\");
		exit(1);
	}

	\$percentage = round((\$lines / \$total) * 100, 2);
	echo \$percentage;
")

if [[ -z "${COVERAGE}" ]]; then
	echo "Error: Failed to extract coverage percentage"
	exit 1
fi

echo "Code coverage: ${COVERAGE}%"

# Validate coverage against threshold using bc
BELOW_THRESHOLD=$(echo "${COVERAGE} < ${THRESHOLD}" | bc -l)

if [[ "${BELOW_THRESHOLD}" == "1" ]]; then
	echo "Error: Code coverage ${COVERAGE}% is below the minimum threshold of ${THRESHOLD}%"
	exit 1
else
	echo "Success: Code coverage ${COVERAGE}% meets the minimum threshold of ${THRESHOLD}%"
	exit 0
fi
