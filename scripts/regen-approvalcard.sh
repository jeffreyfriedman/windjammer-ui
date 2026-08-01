#!/usr/bin/env bash
# Selective regen for ApprovalCard (Phase 3 / dogfood P1).
# Full-tree regen remains unsafe — keep SKIP_WJ_REGEN=1 for crate builds.
#
# Usage:
#   WJ=~/src/wj/windjammer/target/release/wj ./scripts/regen-approvalcard.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WJ="${WJ:-$(command -v wj || true)}"
if [[ -z "${WJ}" || ! -x "${WJ}" ]]; then
  echo "error: set WJ to a release windjammer binary" >&2
  exit 1
fi

TMP="$(mktemp -d "${TMPDIR:-/tmp}/wjui-approval-regen.XXXXXX")"
cleanup() { rm -rf "${TMP}"; }
trap cleanup EXIT

SRC="${TMP}/src"
OUT="${TMP}/out"
mkdir -p "${SRC}" "${OUT}"

cat >"${SRC}/traits.wj" <<'EOF'
pub trait Renderable {
    fn render(self) -> string
}
EOF

cp "${ROOT}/src/components_wj/badge.wj" "${SRC}/"
cp "${ROOT}/src/components_wj/approvalcard.wj" "${SRC}/"

printf '%s' $'pub mod traits\npub mod badge\npub mod approvalcard\n' >"${SRC}/mod.wj"

echo "→ wj library build (${WJ})"
"${WJ}" build "${SRC}/mod.wj" --module-file --library -o "${OUT}" --no-cargo

rs="${OUT}/approvalcard.rs"
[[ -f "${rs}" ]] || { echo "missing ${rs}" >&2; exit 1; }

perl -i -0pe 's/#\[allow\(unused_imports\)\]\s*\nuse super::\*;\n\n//' "${rs}"

DEST="${ROOT}/src/components/generated/approvalcard.rs"
{
  echo '#![allow(clippy::all)]'
  echo '#![allow(noop_method_call)]'
  echo '//! Regenerated from `components_wj/approvalcard.wj` — Windjammer is source of truth.'
  echo '//! Note: avoid `use super::*` (ambiguous glob imports under wasm deny).'
  echo
  cat "${rs}"
} >"${DEST}"

echo "installed ${DEST}"
echo "✓ ApprovalCard regenerated. Verify: SKIP_WJ_REGEN=1 cargo test --test finance_p0_components_test approval_card"
