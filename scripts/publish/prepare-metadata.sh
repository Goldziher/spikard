#!/usr/bin/env bash
set -euo pipefail

event="${GH_EVENT_NAME:-${GITHUB_EVENT_NAME:-}}"
input_tag="${INPUT_TAG:-}"
input_dry_run="${INPUT_DRY_RUN:-}"
input_ref="${INPUT_REF:-}"
input_targets="${INPUT_TARGETS:-}"
input_force_republish="${INPUT_FORCE_REPUBLISH:-}"
release_tag="${RELEASE_TAG_NAME:-}"
dispatch_tag="${DISPATCH_TAG:-}"
dispatch_dry_run="${DISPATCH_DRY_RUN:-}"
dispatch_ref="${DISPATCH_REF:-}"
dispatch_targets="${DISPATCH_TARGETS:-}"
dispatch_force_republish="${DISPATCH_FORCE_REPUBLISH:-}"
ref_name="${GITHUB_REF_NAME:-}"

case "${event}" in
workflow_dispatch)
	tag="${input_tag}"
	dry_run_input="${input_dry_run:-false}"
	ref_input="${input_ref}"
	targets_input="${input_targets}"
	force_republish_input="${input_force_republish:-false}"
	;;
release)
	tag="${release_tag}"
	dry_run_input="false"
	ref_input="refs/tags/${tag}"
	targets_input=""
	force_republish_input="false"
	;;
repository_dispatch)
	tag="${dispatch_tag}"
	dry_run_input="${dispatch_dry_run}"
	ref_input="${dispatch_ref}"
	targets_input="${dispatch_targets}"
	force_republish_input="${dispatch_force_republish:-false}"
	;;
*)
	tag="${ref_name}"
	dry_run_input="false"
	ref_input=""
	targets_input=""
	force_republish_input="false"
	if [[ "${tag}" == *-pre* || "${tag}" == *-rc* ]]; then
		dry_run_input="true"
	fi
	;;
esac

if [[ -z "${tag}" ]]; then
	echo "Release tag could not be determined" >&2
	exit 1
fi
if [[ "${tag}" != v* ]]; then
	echo "Tag must start with 'v' (e.g., v0.5.0)" >&2
	exit 1
fi

version="${tag#v}"

if [[ -n "${ref_input}" ]]; then
	ref="${ref_input}"
else
	ref="refs/tags/${tag}"
fi

if [[ "${ref}" =~ ^[0-9a-f]{40}$ ]]; then
	checkout_ref="refs/heads/main"
	target_sha="${ref}"
elif [[ "${ref}" =~ ^refs/ ]]; then
	checkout_ref="${ref}"
	target_sha=""
else
	checkout_ref="refs/heads/${ref}"
	target_sha=""
fi

if [[ "${ref}" =~ ^[0-9a-f]{40}$ ]]; then
	matrix_ref="main"
elif [[ "${ref}" =~ ^refs/heads/(.+)$ ]]; then
	matrix_ref="${BASH_REMATCH[1]}"
elif [[ "${ref}" =~ ^refs/tags/(.+)$ ]]; then
	matrix_ref="${BASH_REMATCH[1]}"
else
	matrix_ref="${ref}"
fi

dry_run="${dry_run_input}"
if [[ "${ref}" =~ ^refs/tags/ ]]; then
	is_tag="true"
else
	is_tag="false"
fi

normalize_target_list() {
	local raw="$1"
	raw="${raw:-all}"
	if [[ -z "${raw}" ]]; then
		echo "all"
	else
		echo "${raw}"
	fi
}

targets_value="$(normalize_target_list "${targets_input}")"
release_python=false
release_node=false
release_ruby=false
release_php=false
release_cli=false
release_crates=false
release_wasm=false
release_homebrew=false

set_all_targets() {
	release_python=true
	release_node=true
	release_ruby=true
	release_php=true
	release_cli=true
	release_crates=true
	release_wasm=true
	release_homebrew=true
}

mapfile -t requested_targets < <(echo "${targets_value}" | tr ',' '\n')

processed_any=false
for raw_target in "${requested_targets[@]}"; do
	trimmed="$(echo "${raw_target}" | tr '[:upper:]' '[:lower:]' | xargs)"
	if [[ -z "${trimmed}" ]]; then
		continue
	fi
	processed_any=true
	case "${trimmed}" in
	all | "*" | "default")
		set_all_targets
		break
		;;
	python) release_python=true ;;
	node) release_node=true ;;
	ruby) release_ruby=true ;;
	php) release_php=true ;;
	cli) release_cli=true ;;
	crates) release_crates=true ;;
	wasm) release_wasm=true ;;
	homebrew) release_homebrew=true ;;
	none)
		release_python=false
		release_node=false
		release_ruby=false
		release_php=false
		release_cli=false
		release_crates=false
		release_wasm=false
		release_homebrew=false
		;;
	*)
		echo "Unknown release target '${trimmed}'. Allowed: all, python, node, ruby, php, cli, crates, wasm, homebrew." >&2
		exit 1
		;;
	esac
done

if [[ "${processed_any}" == "false" ]]; then
	set_all_targets
	requested_targets=("all")
fi

enabled_targets=()
if [[ "${release_python}" == "true" ]]; then enabled_targets+=("python"); fi
if [[ "${release_node}" == "true" ]]; then enabled_targets+=("node"); fi
if [[ "${release_ruby}" == "true" ]]; then enabled_targets+=("ruby"); fi
if [[ "${release_php}" == "true" ]]; then enabled_targets+=("php"); fi
if [[ "${release_cli}" == "true" ]]; then enabled_targets+=("cli"); fi
if [[ "${release_crates}" == "true" ]]; then enabled_targets+=("crates"); fi
if [[ "${release_wasm}" == "true" ]]; then enabled_targets+=("wasm"); fi

if [[ ${#enabled_targets[@]} -eq 7 ]]; then
	release_targets_summary="all"
elif [[ ${#enabled_targets[@]} -eq 0 ]]; then
	release_targets_summary="none"
else
	release_targets_summary="$(
		IFS=','
		echo "${enabled_targets[*]}"
	)"
fi

release_any="false"
if [[ ${#enabled_targets[@]} -gt 0 ]]; then
	release_any="true"
fi

cat <<JSON >release-metadata.json
{
  "tag": "${tag}",
  "version": "${version}",
  "ref": "${ref}",
  "checkout_ref": "${checkout_ref}",
  "target_sha": "${target_sha}",
  "matrix_ref": "${matrix_ref}",
  "dry_run": ${dry_run:-false},
  "force_republish": ${force_republish_input:-false},
  "is_tag": ${is_tag},
  "release_targets": "${release_targets_summary}",
  "release_any": ${release_any},
  "release_python": ${release_python},
  "release_node": ${release_node},
  "release_ruby": ${release_ruby},
  "release_php": ${release_php},
  "release_cli": ${release_cli},
  "release_crates": ${release_crates},
  "release_wasm": ${release_wasm}
}
JSON

append_output() {
	local key="$1"
	local value="$2"
	if [[ -z "${GITHUB_OUTPUT:-}" ]]; then
		return
	fi
	echo "${key}=${value}" >>"${GITHUB_OUTPUT}"
}

append_output "tag" "${tag}"
append_output "version" "${version}"
append_output "ref" "${ref}"
append_output "dry_run" "${dry_run:-false}"
append_output "force_republish" "${force_republish_input:-false}"
append_output "checkout_ref" "${checkout_ref}"
append_output "target_sha" "${target_sha}"
append_output "matrix_ref" "${matrix_ref}"
append_output "is_tag" "${is_tag}"
append_output "release_targets" "${release_targets_summary}"
append_output "release_any" "${release_any}"
append_output "release_python" "${release_python}"
append_output "release_node" "${release_node}"
append_output "release_ruby" "${release_ruby}"
append_output "release_php" "${release_php}"
append_output "release_cli" "${release_cli}"
append_output "release_crates" "${release_crates}"
append_output "release_wasm" "${release_wasm}"
append_output "release_homebrew" "${release_homebrew}"
