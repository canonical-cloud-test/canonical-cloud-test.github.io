#!/usr/bin/env bash
set -euo pipefail

out=${1:-${TMPDIR:-/tmp}/canonical-production-training-canary-$$}
rm -rf "$out"
mkdir -p "$out"
trap 'rm -rf "$out"' EXIT HUP INT TERM

WEB_REPO=canonical-cloud/canonical-web-server.rs
WEB_SHA=fb132c98722b4e4a59dda155091e2e6858c12290
MARKETING_REPO=canonical-cloud/canonical-marketing-site.web
MARKETING_SHA=c3509c36802e4e4e77225466a13a08efab158a20

clone_exact() {
  repo=$1
  sha=$2
  dest=$3
  git init -q "$dest"
  git -C "$dest" remote add origin "https://github.com/$repo.git"
  git -C "$dest" fetch -q --depth=1 origin "$sha"
  git -C "$dest" checkout -q --detach FETCH_HEAD
  actual=$(git -C "$dest" rev-parse HEAD)
  if [ "$actual" != "$sha" ]; then
    echo "exact-SHA checkout mismatch for $repo: expected $sha got $actual" >&2
    exit 1
  fi
}

require_text() {
  file=$1
  text=$2
  grep -F "$text" "$file" >/dev/null || {
    echo "missing required training contract text in $file: $text" >&2
    exit 1
  }
}

forbid_text() {
  file=$1
  text=$2
  if grep -Fi "$text" "$file" >/dev/null; then
    echo "forbidden/unsupported training claim present in $file: $text" >&2
    exit 1
  fi
}

clone_exact "$WEB_REPO" "$WEB_SHA" "$out/web"
clone_exact "$MARKETING_REPO" "$MARKETING_SHA" "$out/marketing"

web_training="$out/web/src/routes/training.rs"
web_routes="$out/web/src/routes/mod.rs"
marketing_training="$out/marketing/src/pages/training.astro"

test -f "$web_training"
test -f "$web_routes"
test -f "$marketing_training"

# Rust/Maud product authority.
require_text "$web_training" 'Turn compliance into a competitive advantage'
require_text "$web_training" 'SOC 2 Foundations'
require_text "$web_training" 'Control Owner Workshop'
require_text "$web_training" 'Audit Readiness Bootcamp'
require_text "$web_training" 'Readiness, not assurance.'
require_text "$web_training" 'Start your readiness assessment'
require_text "$web_training" 'Talk to an expert'
require_text "$web_training" '.route("/training", get(page))'
require_text "$web_training" '.route("/training/", get(page))'
require_text "$web_routes" '.merge(training::router())'
forbid_text "$web_training" '85% faster'
forbid_text "$web_training" 'guaranteed clean'

# Public marketing discovery/interactive surface points into the Rust authority.
require_text "$marketing_training" "const appTrainingHref = 'https://app.canonical.plus/training';"
require_text "$marketing_training" 'Turn compliance into a competitive advantage'
require_text "$marketing_training" 'SOC 2 Foundations'
require_text "$marketing_training" 'Control Owner Workshop'
require_text "$marketing_training" 'Audit Readiness Bootcamp'
require_text "$marketing_training" 'Training is not an audit opinion'
require_text "$marketing_training" 'no account required'
require_text "$marketing_training" 'no evidence or personal data is submitted'
forbid_text "$marketing_training" '85% faster'
forbid_text "$marketing_training" 'guaranteed clean'

# Keep the public/product split explicit and immutable in this canary receipt.
printf 'canonical production training canary passed: %s@%s -> %s@%s\n' \
  "$MARKETING_REPO" "$MARKETING_SHA" "$WEB_REPO" "$WEB_SHA"
