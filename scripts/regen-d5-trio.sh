#!/usr/bin/env bash
# Selective regen for D5 SOT trio: CurrencyInput, WriteCheckForm, AuthFetch.
# Full-tree regen remains unsafe (TECH_DEBT #1–#7) — never drop SKIP_WJ_REGEN for
# whole-crate builds until that is green.
#
# Usage:
#   WJ=~/src/wj/windjammer/target/release/wj ./scripts/regen-d5-trio.sh
#   # then reviews diffs under src/components/generated/{currencyinput,writecheckform,authfetch}.rs
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WJ="${WJ:-$(command -v wj || true)}"
if [[ -z "${WJ}" || ! -x "${WJ}" ]]; then
  echo "error: set WJ to a release windjammer binary" >&2
  exit 1
fi

TMP="$(mktemp -d "${TMPDIR:-/tmp}/wjui-d5-regen.XXXXXX")"
cleanup() { rm -rf "${TMP}"; }
trap cleanup EXIT

SRC="${TMP}/src"
OUT="${TMP}/out"
mkdir -p "${SRC}" "${OUT}"

# Minimal traits stub (no vnode) so library compose cargo-checks.
cat >"${SRC}/traits.wj" <<'EOF'
pub trait Renderable {
    fn render(self) -> string
}
EOF

cp "${ROOT}/src/components_wj/moneydisplay.wj" "${SRC}/" 2>/dev/null || true
cp "${ROOT}/src/components_wj/currencyinput.wj" "${SRC}/"
cp "${ROOT}/src/components_wj/writecheckform.wj" "${SRC}/"
cp "${ROOT}/src/components_wj/authfetch.wj" "${SRC}/"

MODS=$'pub mod traits\n'
[[ -f "${SRC}/moneydisplay.wj" ]] && MODS+=$'pub mod moneydisplay\n'
MODS+=$'pub mod currencyinput\npub mod writecheckform\npub mod authfetch\n'
printf '%s' "${MODS}" >"${SRC}/mod.wj"

echo "→ wj library build (${WJ})"
"${WJ}" build "${SRC}/mod.wj" --module-file --library -o "${OUT}" --no-cargo

# Strip accidental main() if present; keep only component modules.
for f in currencyinput writecheckform authfetch; do
  rs="${OUT}/${f}.rs"
  [[ -f "${rs}" ]] || { echo "missing ${rs}" >&2; exit 1; }
done

echo "→ cargo check probe"
cat >"${OUT}/Cargo.toml" <<'EOF'
[package]
name = "wjui_d5_regen_probe"
version = "0.1.0"
edition = "2021"
[workspace]
[lib]
path = "lib.rs"
EOF
{
  echo "pub mod traits;"
  [[ -f "${OUT}/moneydisplay.rs" ]] && echo "pub mod moneydisplay;"
  echo "pub mod currencyinput;"
  echo "pub mod writecheckform;"
  echo "pub mod authfetch;"
} >"${OUT}/lib.rs"

(cd "${OUT}" && cargo check --quiet)

DEST="${ROOT}/src/components/generated"
# Prefer a short SOT header if missing; strip `use super::*` (wasm ambiguous globs).
for f in currencyinput writecheckform authfetch; do
  rs="${OUT}/${f}.rs"
  # Drop blanket super glob — pulls ambiguous `mount` from app/renderer.
  perl -i -0pe 's/#\[allow\(unused_imports\)\]\s*\nuse super::\*;\n\n//g' "${rs}"
  if ! grep -q 'source of truth' "${rs}" 2>/dev/null; then
    {
      echo '#![allow(clippy::all)]'
      echo '#![allow(noop_method_call)]'
      echo "//! Regenerated from \`components_wj/${f}.wj\` — Windjammer is source of truth."
      echo '//! Note: avoid `use super::*` (ambiguous glob imports under wasm deny).'
      echo
      cat "${rs}"
    } >"${DEST}/${f}.rs"
  else
    cp "${rs}" "${DEST}/${f}.rs"
  fi
  echo "installed ${DEST}/${f}.rs"
done

echo "✓ D5 trio regenerated. Keep SKIP_WJ_REGEN=1 for full-crate builds."
echo "  Verify: SKIP_WJ_REGEN=1 cargo test --test finance_p0_components_test"
