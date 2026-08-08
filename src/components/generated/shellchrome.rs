//! ShellChrome — persona / density / shell-nav host helpers.
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/shellchrome.wj` (structural marker).
//! Product host wires this next to WJ-UI runtimes in finance-ui shell.

/// Framework runtime: persona chip, density CSS vars, shell nav swap, post-mount.
pub fn shell_chrome_runtime_js() -> &'static str {
    r##"
function lkPersona() {
  return (localStorage.getItem('ledgerkit_persona') || 'owner').toLowerCase(); // lockstep: finance_screens::persona_storage_key / persona_default_wire
}
function lkApplyPersonaNav() {
  const p = lkPersona();
  document.querySelectorAll('#shellNav a[data-personas]').forEach((a) => {
    const allowed = (a.getAttribute('data-personas') || '').split(/\s+/);
    a.hidden = allowed.indexOf(p) < 0;
  });
  const chip = document.getElementById('personaChip');
  if (chip) chip.textContent = p.charAt(0).toUpperCase() + p.slice(1);
}
function lkDensity() {
  const stored = localStorage.getItem('ledgerkit_density'); // lockstep: finance_screens::density_storage_key
  if (stored) return stored.toLowerCase();
  if (lkPersona() === 'bookkeeper') return 'compact';
  return 'comfortable';
}
function lkSetDensity(mode) {
  const d = (mode === 'compact' || mode === 'dense') ? 'compact' : 'comfortable';
  localStorage.setItem('ledgerkit_density', d); // lockstep: finance_screens::density_storage_key
  lkApplyDensity();
}
function lkApplyDensity() {
  const d = lkDensity();
  document.documentElement.classList.toggle('lk-density-compact', d === 'compact');
  const root = document.documentElement.style;
  if (d === 'compact') {
    root.setProperty('--lk-space', '0.65rem');
    root.setProperty('--lk-space-sm', '0.35rem');
    root.setProperty('--lk-font-size', '13px');
    root.setProperty('--lk-row-pad', '0.32rem');
    root.setProperty('--lk-table-font', '0.8rem');
  } else {
    root.setProperty('--lk-space', '1rem');
    root.setProperty('--lk-space-sm', '0.55rem');
    root.setProperty('--lk-font-size', '15px');
    root.setProperty('--lk-row-pad', '0.55rem');
    root.setProperty('--lk-table-font', '0.9rem');
  }
}
function lkApplyShellNav(hash) {
  if (typeof LK_NAVS === 'undefined') return;
  const html = LK_NAVS[hash] || LK_NAVS['#/'];
  const cur = document.getElementById('shellNav');
  if (!html || !cur) return;
  const wrap = document.createElement('div');
  wrap.innerHTML = html.trim();
  const next = wrap.firstElementChild;
  if (next) cur.replaceWith(next);
}
function lkReexecScripts(root) {
  const el = root || document.getElementById('main');
  if (!el) return;
  el.querySelectorAll('script').forEach((old) => {
    const s = document.createElement('script');
    s.textContent = old.textContent;
    old.replaceWith(s);
  });
}
function lkPostMount() {
  lkReexecScripts(document.getElementById('main'));
  if (typeof lkApplyShellNav === 'function') {
    lkApplyShellNav(location.hash || '#/');
  }
  if (typeof window.wjMemorizedRefresh === 'function') {
    try { window.wjMemorizedRefresh(); } catch (e) {}
  }
  lkApplyPersonaNav();
  lkApplyDensity();
}
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_chrome_runtime_covers_persona_density_nav() {
        let js = shell_chrome_runtime_js();
        assert!(js.contains("function lkPersona("));
        assert!(js.contains("function lkApplyPersonaNav("));
        assert!(js.contains("function lkApplyDensity("));
        assert!(js.contains("function lkSetDensity("));
        assert!(js.contains("function lkApplyShellNav("));
        assert!(js.contains("function lkPostMount("));
        assert!(js.contains("ledgerkit_persona"));
        assert!(js.contains("ledgerkit_density"));
        assert!(js.contains("lk-density-compact"));
        assert!(js.contains("LK_NAVS"));
    }
}
