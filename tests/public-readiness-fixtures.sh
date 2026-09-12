#!/usr/bin/env bash
set -euo pipefail

out=${1:?usage: public-readiness-fixtures.sh <output-dir>}
rm -rf "$out"
mkdir -p "$out"

seed_repo() {
  local root=$1
  mkdir -p "$root/.github" "$root/src/pages" "$root/docs/claims"
  printf '# Fixture repository\n' > "$root/README.md"
  printf 'Test-only fixture license.\n' > "$root/LICENSE"
  printf '# Test fixture instructions\nDo not publish fixture claims.\n' > "$root/AGENTS.md"
  printf '# GitHub policy fixture\n' > "$root/.github/README.md"
}

pass="$out/pass-astro"
seed_repo "$pass"
cat > "$pass/src/pages/training.astro" <<'EOF'
---
const title = 'SOC 2 readiness training';
---
<!-- ores-claim-substantiation: docs/claims/training-speed.md -->
<h1>{title}</h1>
<p>Readiness, not assurance. An independent audit remains a separate engagement.</p>
<p>In the measured fixture population, evidence collection was 60% faster.</p>
EOF
cat > "$pass/docs/claims/training-speed.md" <<'EOF'
# Test substantiation

Synthetic fixture evidence with population, sample size, timing method, caveats, and source notes.
EOF

missing_boundary="$out/fail-missing-boundary"
seed_repo "$missing_boundary"
cat > "$missing_boundary/src/pages/training.astro" <<'EOF'
<h1>SOC 2 compliance training</h1>
<p>Operate controls continuously and prepare evidence before review.</p>
EOF

forbidden="$out/fail-forbidden-outcome"
seed_repo "$forbidden"
cat > "$forbidden/src/pages/readiness.astro" <<'EOF'
<h1>Compliance readiness</h1>
<p>An independent audit happens later.</p>
<p>Guaranteed compliance and SOC 2 certified in one sprint.</p>
EOF

metric="$out/fail-unsubstantiated-metric"
seed_repo "$metric"
cat > "$metric/src/pages/training.astro" <<'EOF'
<h1>SOC 2 readiness training</h1>
<p>Readiness, not assurance. Independent review remains separate.</p>
<p>Collect evidence 85% faster with this program.</p>
EOF

printf 'pass-astro\nfail-missing-boundary\nfail-forbidden-outcome\nfail-unsubstantiated-metric\n'
