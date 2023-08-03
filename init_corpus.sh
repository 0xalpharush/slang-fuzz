#!/usr/bin/env bash

set -e

repos=(
    "ethereum/solidity"
)
mkdir -p sol
for repo in "${repos[@]}"; do
  base=$(basename "${repo}")
  if ! [[ -d "${base}" ]]; then
    git clone --jobs 4 --depth 1 "https://github.com/${repo}"
  fi
  for f in $(find "${base}/test/libsolidity/semanticTests/" -type f -name "*.sol"); do
    echo "${f}"
    cp "${f}" fuzz/corpus/roundtrip_parse/"${base}-$(sha256sum "${f}" | head -c 5)-$(basename "${f}")"
  done
done
