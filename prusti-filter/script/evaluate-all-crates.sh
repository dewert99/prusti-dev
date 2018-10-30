#!/bin/bash

set -uo pipefail

# Get the directory in which this script is contained
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
SCRIPT_NAME="$(basename -s '.sh' "$0")"

# Get the folder in which all the crates has been downloaded
CRATE_DOWNLOAD_DIR="$(cd "${1:-.}" && pwd)"
cd "$CRATE_DOWNLOAD_DIR"

rustup override set nightly-2018-06-27

if [[ ! -d "$CRATE_DOWNLOAD_DIR/000_libc" ]]; then
	echo "It looks like CRATE_DOWNLOAD_DIR is wrong: '$CRATE_DOWNLOAD_DIR'"
	exit 1
fi

# Force exit on Ctrl-c
function ctrl_c() {
        echo "Force exit"
        exit 1
}
trap ctrl_c INT

for crate_dir in "$CRATE_DOWNLOAD_DIR"/*; do
	# Timeout of 1 hour (+5 min)
	timeout -k 300 3600 "$DIR/evaluate-crate.sh" "$crate_dir"
	sleep 1
done

# Print nice summary of summaries

summaries="$(grep -h "Summary" "$CRATE_DOWNLOAD_DIR"/*/evaluate-crate.log)"
prusti_ok="$(echo "$summaries" | grep "exit status 0")"
prusti_error="$(echo "$summaries" | grep -v "exit status 42" | grep -v "exit status 0")"
compilation_error="$(echo "$summaries" | grep "exit status 42")"

summaries_num="$(echo "$summaries" | wc -l)"
prusti_ok_num="$(echo "$prusti_ok" | wc -l)"
prusti_error_num="$(echo "$prusti_error" | wc -l)"
compilation_error_num="$(echo "$compilation_error" | wc -l)"

echo "Collected $summaries_num summaries"
echo " - Prusti ok: $prusti_ok_num crates"
echo " - Prusti error: $prusti_error_num crates"
echo " - Invalid: $compilation_error_num crates"
