#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: bash scripts/package-release.sh vVERSION" >&2
  exit 2
fi

tag="$1"
node scripts/release-check.cjs tag "$tag"

exe="preagent"
if [[ "$RUNNER_OS" == 'Windows' ]]; then
  exe="preagent.exe"
fi
if [[ ! -f "target/release/$exe" ]]; then
  echo "release binary missing: $exe" >&2
  exit 1
fi

mkdir -p dist
archive="preagent-${tag}-${RUNNER_OS}-${RUNNER_ARCH}.tar.gz"
tar -czf "dist/$archive" -C target/release "$exe"

(
  cd dist
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$archive" > "$archive.sha256"
    sha256sum -c "$archive.sha256"
  else
    shasum -a 256 "$archive" > "$archive.sha256"
    shasum -a 256 -c "$archive.sha256"
  fi
)

entry="$(tar -tzf "dist/$archive")"
if [[ "$entry" != "$exe" && "$entry" != "./$exe" ]]; then
  echo "release archive must contain only $exe: $entry" >&2
  exit 1
fi

echo "Verified dist/$archive"
