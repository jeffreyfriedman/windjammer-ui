//! HashRouter — static SPA hash mount (expects host `LK_PAGES` / `LK_TITLES`).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/hashrouter.wj` (structural marker).
//! WASM boot does not embed this; it remounts `#main` from Rust.

/// Framework runtime: normalize hash, mount page body, wire hashchange + SW.
pub fn hash_router_runtime_js() -> &'static str {
    r##"
function lkNormalizeHash(h) {
  if (!h || h === '#' || h === '#/') return '#/';
  return h.endsWith('/') && h !== '#/' ? h.slice(0, -1) : h;
}
function lkMount() {
  const hash = lkNormalizeHash(location.hash || '#/');
  const body = (typeof LK_PAGES !== 'undefined' && (LK_PAGES[hash] || LK_PAGES['#/404']))
    || '<div class="panel"><p class="err">Unknown route</p></div>';
  const title = (typeof LK_TITLES !== 'undefined' && LK_TITLES[hash]) || 'Not found';
  const main = document.getElementById('main');
  if (main) main.innerHTML = body;
  const t = document.querySelector('.shell-title');
  if (t) t.textContent = title;
  document.title = 'LedgerKit — ' + title;
  if (typeof lkApplyShellNav === 'function') lkApplyShellNav(hash);
  if (typeof lkPostMount === 'function') lkPostMount();
}
window.addEventListener('hashchange', lkMount);
if (!location.hash) location.hash = '#/';
else lkMount();
if (typeof lkApplyPersonaNav === 'function') lkApplyPersonaNav();
if (typeof lkApplyDensity === 'function') lkApplyDensity();
if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('sw.js').catch(() => {});
}
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_router_runtime_covers_mount_and_hashchange() {
        let js = hash_router_runtime_js();
        assert!(js.contains("function lkNormalizeHash("));
        assert!(js.contains("function lkMount("));
        assert!(js.contains("LK_PAGES"));
        assert!(js.contains("LK_TITLES"));
        assert!(js.contains("hashchange"));
        assert!(js.contains("lkApplyShellNav"));
        assert!(js.contains("lkPostMount"));
        assert!(js.contains("serviceWorker"));
        assert!(!js.contains("const LK_PAGES"));
    }
}
