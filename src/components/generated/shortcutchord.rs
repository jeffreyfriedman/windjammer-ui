//! ShortcutChord — g-then-letter / single-key hash navigation + "/" opens palette.
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/shortcutchord.wj` (structural marker).
//! Host must emit `const LK_SHORTCUTS = { ... }` before this runtime.

/// Framework runtime: chord keydown listener (expects global `LK_SHORTCUTS`).
pub fn shortcut_chord_runtime_js() -> &'static str {
    r##"
let lkChord = [];
let lkChordTimer = null;
function lkTypingTarget(el) {
  if (!el) return false;
  const tag = (el.tagName || '').toLowerCase();
  return tag === 'input' || tag === 'textarea' || tag === 'select' || el.isContentEditable;
}
document.addEventListener('keydown', (e) => {
  if (lkTypingTarget(e.target)) return;
  if (e.key === '/' && !e.metaKey && !e.ctrlKey) {
    e.preventDefault();
    if (window.wjCmdOpen) window.wjCmdOpen();
    return;
  }
  if (e.metaKey || e.ctrlKey || e.altKey) return;
  const k = e.key.length === 1 ? e.key.toLowerCase() : '';
  if (!k) return;
  clearTimeout(lkChordTimer);
  lkChord.push(k);
  const chord = lkChord.join(' ');
  if (typeof LK_SHORTCUTS !== 'undefined' && LK_SHORTCUTS[chord]) {
    e.preventDefault();
    location.hash = LK_SHORTCUTS[chord];
    lkChord = [];
    return;
  }
  if (lkChord.length >= 2) lkChord = [];
  lkChordTimer = setTimeout(() => { lkChord = []; }, 800);
});
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcut_chord_runtime_covers_slash_and_chords() {
        let js = shortcut_chord_runtime_js();
        assert!(js.contains("function lkTypingTarget("));
        assert!(js.contains("lkChord"));
        assert!(js.contains("LK_SHORTCUTS"));
        assert!(js.contains("wjCmdOpen"));
        assert!(js.contains("keydown"));
        assert!(js.contains("location.hash"));
        assert!(!js.contains("const LK_SHORTCUTS"));
    }
}
