#!/usr/bin/env bash
# shellcheck disable=SC2207
set -euo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"/..

# shellcheck disable=SC2154
trap 's=$?; echo >&2 "$0: Error on line "${LINENO}": ${BASH_COMMAND}"; exit ${s}' ERR

# Update the list of targets that do not support atomic/CAS operations.
#
# USAGE:
#    ./tools/no_atomic.sh

bail() {
    echo >&2 "error: $*"
    exit 1
}

file="no_atomic.rs"

# We don't refer to NO_ATOMIC_CAS and NO_ATOMIC_64 in nightly-2022-02-11+
# because feature(cfg_target_has_atomic) stabilized. So, we get the list
# as of nightly-2022-02-10.
rustup toolchain add nightly-2022-02-10 --profile minimal --no-self-update &>/dev/null

no_atomic_cas=()
no_atomic_64=()
no_atomic=()
for target in $(rustc --print target-list); do
    target_spec=$(rustc --print target-spec-json -Z unstable-options --target "${target}")
    max_atomic_width=$(jq <<<"${target_spec}" -r '."max-atomic-width"')
    min_atomic_width=$(jq <<<"${target_spec}" -r '."min-atomic-width"')
    case "${max_atomic_width}" in
        # `"max-atomic-width" == 0` means that atomic is not supported at all.
        0) no_atomic+=("${target}") ;;
        32 | 64 | 128 | null) ;;
        # There is no `"max-atomic-width" == 16` or `"max-atomic-width" == 8` targets.
        *) bail "'${target}' has max-atomic-width == ${max_atomic_width}" ;;
    esac
    case "${min_atomic_width}" in
        8 | null) ;;
        *)
            case "${target}" in
                bpfeb-unknown-none | bpfel-unknown-none) ;;
                *) bail "'${target}' has min-atomic-width == ${min_atomic_width}" ;;
            esac
            no_atomic+=("${target}")
            ;;
    esac
done
for target in $(rustc +nightly-2022-02-10 --print target-list); do
    target_spec=$(rustc +nightly-2022-02-10 --print target-spec-json -Z unstable-options --target "${target}")
    [[ -z "$(jq <<<"${target_spec}" -r 'select(."atomic-cas" == false)')" ]] || no_atomic_cas+=("${target}")
    max_atomic_width=$(jq <<<"${target_spec}" -r '."max-atomic-width"')
    case "${max_atomic_width}" in
        # It is not clear exactly what `"max-atomic-width" == null` means, but they
        # actually seem to have the same max-atomic-width as the target-pointer-width.
        # The targets currently included in this group are "mipsel-sony-psp",
        # "thumbv4t-none-eabi", "thumbv6m-none-eabi", all of which are
        # `"target-pointer-width" == "32"`, so assuming them `"max-atomic-width" == 32`
        # for now.
        32 | null) no_atomic_64+=("${target}") ;;
        # `"max-atomic-width" == 0` means that atomic is not supported at all.
        0) no_atomic_64+=("${target}") ;;
        64 | 128) ;;
        # There is no `"max-atomic-width" == 16` or `"max-atomic-width" == 8` targets.
        *) bail "${target}" ;;
    esac
done

# sort and dedup
IFS=$'\n'
no_atomic_cas=($(LC_ALL=C sort -u <<<"${no_atomic_cas[*]}"))
no_atomic_64=($(LC_ALL=C sort -u <<<"${no_atomic_64[*]}"))
no_atomic=($(LC_ALL=C sort -u <<<"${no_atomic[*]}"))
IFS=$'\n\t'

cat >"${file}" <<EOF
// This file is @generated by $(basename "$0").
// It is not intended for manual editing.

// Note: This is the list as of nightly-2022-02-10. We don't refer to this in
// nightly-2022-02-11+ because feature(cfg_target_has_atomic) stabilized.
static NO_ATOMIC_CAS: &[&str] = &[
EOF
for target in "${no_atomic_cas[@]}"; do
    echo "    \"${target}\"," >>"${file}"
done
cat >>"${file}" <<EOF
];

// Note: This is the list as of nightly-2022-02-10. We don't refer to this in
// nightly-2022-02-11+ because feature(cfg_target_has_atomic) stabilized.
static NO_ATOMIC_64: &[&str] = &[
EOF
for target in "${no_atomic_64[@]}"; do
    echo "    \"${target}\"," >>"${file}"
done
cat >>"${file}" <<EOF
];

static NO_ATOMIC: &[&str] = &[
EOF
for target in "${no_atomic[@]}"; do
    echo "    \"${target}\"," >>"${file}"
done
cat >>"${file}" <<EOF
];
EOF
