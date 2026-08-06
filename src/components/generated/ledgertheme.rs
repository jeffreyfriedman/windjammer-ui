//! LedgerTheme — LedgerKit SPA layout stylesheet (fonts + component CSS).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.
//! `.wj` source: `components_wj/ledgertheme.wj` (structural marker).
//! Token `:root` vars stay in finance-ui `FinanceTokens` (desktop egui maps the same RGB).

/// Google Fonts import for LedgerKit shell.
pub fn ledger_font_import_css() -> &'static str {
    r#"@import url("https://fonts.googleapis.com/css2?family=Fraunces:opsz,wght@9..144,500;9..144,650&family=IBM+Plex+Mono:wght@400;500&family=Sora:wght@400;500;600&display=swap");"#
}

/// Layout + component CSS (expects `--lk-*` variables already defined on `:root`).
pub fn ledger_layout_stylesheet() -> &'static str {
    r##"* { box-sizing: border-box; }
html { scroll-behavior: smooth; }
body {
  margin: 0;
  min-height: 100vh;
  color: var(--lk-ink);
  font-family: var(--lk-font-sans);
  font-size: var(--lk-font-size, 15px);
  line-height: 1.45;
  background:
    radial-gradient(1200px 600px at 12% -10%, var(--lk-bg-wash) 0%, transparent 55%),
    radial-gradient(900px 500px at 100% 0%, rgba(26, 107, 87, 0.10) 0%, transparent 50%),
    linear-gradient(165deg, var(--lk-bg) 0%, #eef3f7 48%, var(--lk-bg) 100%);
  background-attachment: fixed;
}
body::before {
  content: "";
  position: fixed;
  inset: 0;
  pointer-events: none;
  opacity: 0.35;
  background-image:
    linear-gradient(rgba(20, 34, 46, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(20, 34, 46, 0.03) 1px, transparent 1px);
  background-size: 28px 28px;
  mask-image: linear-gradient(180deg, rgba(0,0,0,0.55), transparent 70%);
  z-index: 0;
}
#app { position: relative; z-index: 1; }

.shell-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.95rem 1.5rem 0.75rem;
  background: color-mix(in srgb, var(--lk-surface) 88%, transparent);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid var(--lk-line);
  animation: lk-fade-down 420ms ease-out both;
}
.shell-brand {
  font-family: var(--lk-font-display);
  font-weight: 650;
  font-size: 1.55rem;
  letter-spacing: -0.02em;
  color: var(--lk-accent-deep);
}
.shell-brand strong { font-weight: 650; }
.shell-title {
  color: var(--lk-muted);
  font-size: 0.85rem;
  font-weight: 500;
  padding-left: 0.65rem;
  border-left: 1px solid var(--lk-line);
}

.shell-nav {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  padding: 0.45rem 1.35rem;
  background: color-mix(in srgb, var(--lk-surface) 92%, transparent);
  border-bottom: 1px solid var(--lk-line);
  animation: lk-fade-down 480ms ease-out both;
  animation-delay: 40ms;
}
.shell-nav a {
  position: relative;
  color: var(--lk-muted);
  text-decoration: none;
  font-size: 0.88rem;
  font-weight: 500;
  padding: 0.4rem 0.75rem;
  border-radius: 999px;
  transition: color 160ms ease, background 160ms ease;
}
.shell-nav a:hover {
  color: var(--lk-accent-deep);
  background: rgba(26, 107, 87, 0.08);
}
.shell-nav a:focus-visible {
  outline: 2px solid var(--lk-accent);
  outline-offset: 2px;
}

.shell-nav a.is-active {
  color: var(--lk-accent-deep);
  background: rgba(26, 107, 87, 0.14);
  font-weight: 600;
}
.shell-main {
  padding: calc(var(--lk-space, 1rem) * 1.35) 1.35rem 2.75rem;
  max-width: 1040px;
  margin: 0 auto;
  animation: lk-fade-up 500ms ease-out both;
}

.panel {
  background: var(--lk-surface);
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius);
  box-shadow: var(--lk-shadow);
  padding: 1.25rem 1.4rem 1.35rem;
  margin-bottom: 1rem;
}
.panel-quiet {
  background: transparent;
  border: none;
  box-shadow: none;
  padding: 0.35rem 0.15rem 0;
}
.panel-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  margin-bottom: 0.85rem;
}
.panel-head h2 { margin-bottom: 0.15rem; }
.panel-filters { margin: 0 0 0.85rem; }
.panel h2 {
  font-family: var(--lk-font-display);
  font-weight: 650;
  font-size: 1.4rem;
  letter-spacing: -0.02em;
  margin: 0 0 0.4rem;
  color: var(--lk-ink);
}
.lede {
  margin: 0;
  color: var(--lk-muted);
  font-size: 0.95rem;
  max-width: 36rem;
  line-height: 1.4;
}
.hub-kicker {
  margin: 0 0 0.3rem;
  font-size: 0.7rem;
  font-weight: 600;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: var(--lk-accent);
}
.hub-list {
  list-style: none;
  padding: 0;
  margin: 0.65rem 0 0;
  display: grid;
  gap: 0.35rem;
}
.hub-list-compact {
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 0.35rem 0.75rem;
}
.hub-link {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.35rem 0;
  border: none;
  border-bottom: 1px solid transparent;
  border-radius: 0;
  background: transparent;
  color: var(--lk-accent-deep) !important;
  font-weight: 600 !important;
  font-size: 0.92rem;
  text-decoration: none !important;
  transition: color 140ms ease, border-color 140ms ease;
}
.hub-link::after {
  content: "→";
  font-size: 0.85em;
  opacity: 0.55;
  transition: transform 140ms ease, opacity 140ms ease;
}
.hub-link:hover {
  border-bottom-color: color-mix(in srgb, var(--lk-accent) 45%, transparent);
  text-decoration: none !important;
}
.hub-link:hover::after {
  opacity: 1;
  transform: translateX(2px);
}

.err { color: var(--lk-danger); }
.muted { color: var(--lk-muted); font-size: 0.9rem; }
code {
  font-family: var(--lk-font-mono);
  font-size: 0.84em;
  background: rgba(20, 34, 46, 0.05);
  padding: 0.1em 0.35em;
  border-radius: 4px;
}
textarea {
  width: 100%;
  font-family: var(--lk-font-mono);
  font-size: 0.85rem;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  padding: 0.65rem 0.75rem;
  background: var(--lk-surface-elevated);
  color: var(--lk-ink);
}
.row { margin-top: 0.85rem; display: flex; flex-wrap: wrap; gap: 0.55rem; }

button, .wj-button {
  appearance: none;
  border: 1px solid var(--lk-accent-deep);
  background: linear-gradient(180deg, var(--lk-accent) 0%, var(--lk-accent-deep) 100%);
  color: #fff;
  font-family: var(--lk-font-sans);
  font-weight: 600;
  font-size: 0.88rem;
  padding: 0.5rem 1rem;
  border-radius: 8px;
  cursor: pointer;
  box-shadow: 0 1px 0 rgba(255,255,255,0.18) inset;
  transition: transform 140ms ease, box-shadow 140ms ease, filter 140ms ease;
}
button:hover:not(:disabled), .wj-button:hover {
  transform: translateY(-1px);
  filter: brightness(1.04);
  box-shadow: 0 8px 18px rgba(14, 74, 60, 0.22);
}
button:active:not(:disabled) { transform: translateY(0); }
button:disabled { opacity: 0.55; cursor: not-allowed; box-shadow: none; }
button.btn-secondary, .btn-secondary {
  background: var(--lk-surface-elevated) !important;
  color: var(--lk-accent-deep) !important;
  border: 1px solid var(--lk-line) !important;
  box-shadow: none !important;
  font-weight: 600 !important;
}
button.btn-secondary:hover, .btn-secondary:hover {
  background: rgba(26, 107, 87, 0.08) !important;
  border-color: color-mix(in srgb, var(--lk-accent) 40%, var(--lk-line)) !important;
  filter: none !important;
  box-shadow: none !important;
  transform: none !important;
}

.kpi-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(156px, 1fr));
  gap: 0.65rem;
  margin: 0.35rem 0 1rem;
}
.kpi {
  background: var(--lk-surface-elevated);
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  padding: 0.9rem 0.95rem 0.95rem;
  box-shadow: 0 1px 0 rgba(20, 34, 46, 0.03);
  animation: lk-fade-up 520ms ease-out both;
}
.kpi:nth-child(2) { animation-delay: 40ms; }
.kpi:nth-child(3) { animation-delay: 80ms; }
.kpi:nth-child(4) { animation-delay: 120ms; }
.kpi:nth-child(5) { animation-delay: 160ms; }
.kpi-label {
  display: block;
  font-size: 0.68rem;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: var(--lk-muted);
  margin-bottom: 0.45rem;
  font-weight: 600;
}
.kpi-value {
  min-height: 1.4rem;
  display: flex;
  align-items: center;
}
.kpi strong, .kpi .wj-money, .kpi-value .wj-money, .kpi-value strong {
  font-family: var(--lk-font-mono);
  font-size: 1.12rem;
  font-weight: 500;
  color: var(--lk-ink);
  letter-spacing: -0.01em;
}
.wj-kpi-tile.kpi { /* alias for WJ KpiTile */ }
.wj-kpi-grid.kpi-grid { /* alias for WJ KpiGrid */ }

.lk-table, .wj-datatable table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--lk-table-font, 0.9rem);
}
.lk-table caption, .wj-datatable-caption {
  display: none;
}
.lk-table th, .lk-table td,
.wj-datatable th, .wj-datatable td {
  text-align: left;
  padding: var(--lk-row-pad, 0.55rem) 0.65rem;
  border-bottom: 1px solid var(--lk-line);
}
.lk-table th, .wj-datatable th {
  color: var(--lk-muted);
  font-weight: 600;
  font-size: 0.72rem;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  background: color-mix(in srgb, var(--lk-bg) 45%, var(--lk-surface));
}
.lk-table tbody tr:nth-child(even) td,
.wj-datatable tbody tr:nth-child(even) td {
  background: color-mix(in srgb, var(--lk-bg) 28%, var(--lk-surface));
}
.lk-table tbody tr:hover td, .wj-datatable tbody tr:hover td {
  background: rgba(26, 107, 87, 0.05);
}
.lk-num, .lk-table .lk-num, .wj-datatable .lk-num {
  text-align: right !important;
  font-family: var(--lk-font-mono);
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}
.lk-table-scroll {
  max-height: min(60vh, 28rem);
  overflow: auto;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  background: var(--lk-surface-elevated);
}
.wj-checkbook-table {
  min-width: 0;
  table-layout: fixed;
  width: 100%;
}
.wj-checkbook-table th:nth-child(1),
.wj-checkbook-table td:nth-child(1) { width: 5.75rem; }
.wj-checkbook-table th:nth-child(2),
.wj-checkbook-table td:nth-child(2) { width: 2.75rem; }
.wj-checkbook-table th:nth-child(4),
.wj-checkbook-table td:nth-child(4),
.wj-checkbook-table th:nth-child(6),
.wj-checkbook-table td:nth-child(6) {
  width: 6.25rem;
  max-width: 6.25rem;
  box-sizing: border-box;
  overflow: hidden;
  padding-left: 0.45rem;
  padding-right: 0.45rem;
}
.wj-checkbook-table th:nth-child(5),
.wj-checkbook-table td:nth-child(5) {
  width: 2.25rem;
  max-width: 2.25rem;
  text-align: center;
  box-sizing: border-box;
  padding-left: 0.25rem;
  padding-right: 0.25rem;
}
.wj-checkbook-table th:nth-child(6),
.wj-checkbook-table td:nth-child(6) {
  border-left: 1px solid var(--lk-line);
}
.wj-checkbook-table td.lk-clr {
  text-align: center;
  vertical-align: middle;
}
.wj-checkbook-table td.lk-num {
  white-space: nowrap;
  overflow: hidden;
  vertical-align: middle;
  text-align: right;
}
.wj-checkbook-table td.lk-num .lk-num,
.wj-checkbook-table td.lk-num .wj-money {
  display: inline;
}
.wj-checkbook-table .btn-link {
  border: 0;
  background: transparent;
  color: var(--lk-accent-deep);
  font: inherit;
  font-size: 0.78rem;
  padding: 0 0.2rem;
  cursor: pointer;
  text-decoration: underline;
  text-underline-offset: 0.12em;
}
.wj-checkbook-table .btn-link:hover {
  color: var(--lk-accent);
}
.wj-write-check-form select[data-wj-write-check-expense] {
  width: 100%;
  margin-bottom: 0.55rem;
}
.lk-table-scroll .lk-table {
  margin: 0;
}
.lk-table-scroll thead th {
  position: sticky;
  top: 0;
  z-index: 1;
  box-shadow: 0 1px 0 var(--lk-line);
}
html.lk-density-compact .lk-table th,
html.lk-density-compact .wj-datatable th {
  font-size: 0.68rem;
  letter-spacing: 0.02em;
}
html.lk-density-compact .lk-table-scroll {
  max-height: min(70vh, 34rem);
}

.wj-table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--lk-table-font, 0.9rem);
}
.wj-table-bordered {
  border: 1px solid var(--lk-line);
}
.wj-table thead {
  background: color-mix(in srgb, var(--lk-bg) 45%, var(--lk-surface));
  border-bottom: 1px solid var(--lk-line);
}
.wj-table th, .wj-table td {
  text-align: left;
  padding: var(--lk-row-pad, 0.55rem) 0.65rem;
  border-bottom: 1px solid var(--lk-line);
  color: var(--lk-ink);
}
.wj-table th {
  color: var(--lk-muted);
  font-weight: 600;
  font-size: 0.72rem;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.wj-table-striped tbody tr:nth-child(even) td {
  background: color-mix(in srgb, var(--lk-bg) 28%, var(--lk-surface));
}
.wj-table-hover tbody tr:hover td {
  background: rgba(26, 107, 87, 0.05);
}

.wj-money {
  font-family: var(--lk-font-mono);
  font-variant-numeric: tabular-nums;
  color: var(--lk-money-pos);
}
.wj-money-negative { color: var(--lk-money-neg); }

.wj-form-field { margin-bottom: 0.85rem !important; }
.wj-form-field-label {
  color: var(--lk-muted) !important;
  font-size: 0.8rem !important;
  font-weight: 600 !important;
  letter-spacing: 0.02em;
}
input[type="email"], input[type="password"], input[type="text"], input:not([type]) {
  width: 100%;
  max-width: 420px;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  padding: 0.55rem 0.7rem;
  font-family: var(--lk-font-sans);
  background: var(--lk-surface-elevated);
  color: var(--lk-ink);
}
input:focus {
  outline: 2px solid color-mix(in srgb, var(--lk-accent) 55%, transparent);
  border-color: var(--lk-accent);
}

.panel ul { padding-left: 1.1rem; }
.panel li { margin: 0.35rem 0; }
.panel a { color: var(--lk-accent); font-weight: 500; text-decoration: none; }
.panel a:hover { text-decoration: underline; text-underline-offset: 3px; }

.lk-status {
  margin: 0.85rem 0 0;
  padding: 0;
  border: none;
  background: transparent;
  font-family: var(--lk-font-sans);
  font-size: 0.8rem;
  color: var(--lk-muted);
  min-height: 1.2em;
}
.lk-status.is-error { color: var(--lk-danger); }
.lk-status[hidden] { display: none !important; }
.lk-banner {
  margin: 0 0 0.75rem;
  padding: 0.55rem 0.75rem;
  border-radius: var(--lk-radius-sm);
  font-size: 0.88rem;
  font-weight: 500;
}
.lk-banner-ok {
  background: rgba(31, 122, 77, 0.1);
  color: var(--lk-success);
  border: 1px solid rgba(31, 122, 77, 0.22);
}
.lk-banner-err {
  background: rgba(180, 42, 34, 0.08);
  color: var(--lk-danger);
  border: 1px solid rgba(180, 42, 34, 0.2);
}

pre#out, pre.muted {
  margin-top: 0.85rem;
  padding: 0.65rem 0.75rem;
  border-radius: var(--lk-radius-sm);
  background: rgba(20, 34, 46, 0.04);
  border: 1px solid var(--lk-line);
  font-family: var(--lk-font-mono);
  font-size: 0.78rem;
  white-space: pre-wrap;
  word-break: break-word;
}

@keyframes lk-fade-up {
  from { opacity: 0; transform: translateY(8px); }
  to { opacity: 1; transform: translateY(0); }
}
@keyframes lk-fade-down {
  from { opacity: 0; transform: translateY(-6px); }
  to { opacity: 1; transform: translateY(0); }
}

@media (max-width: 640px) {
  .shell-header { flex-direction: column; align-items: flex-start; }
  .shell-brand { font-size: 1.35rem; }
  .shell-title { border-left: none; padding-left: 0; }
  .shell-main { padding: 1.1rem 0.85rem 2rem; }
  .panel-head { flex-direction: column; align-items: stretch; }
}

.shell-header-meta { display: flex; align-items: center; gap: 0.55rem; flex-wrap: wrap; }
.cmd-trigger {
  background: transparent !important;
  border: 1px solid var(--lk-line) !important;
  color: var(--lk-muted) !important;
  box-shadow: none !important;
  font-weight: 500 !important;
  padding: 0.35rem 0.7rem !important;
}
.cmd-trigger:hover { background: rgba(26,107,87,0.08) !important; color: var(--lk-accent-deep) !important; transform: none !important; box-shadow: none !important; }
.persona-chip {
  font-size: 0.7rem;
  font-weight: 600;
  letter-spacing: 0.05em;
  text-transform: uppercase;
  color: var(--lk-accent-deep);
  background: rgba(26,107,87,0.1);
  border: 1px solid color-mix(in srgb, var(--lk-accent) 35%, var(--lk-line));
  border-radius: 999px;
  padding: 0.28rem 0.6rem;
}
.lk-sync-badge {
  font-size: 0.7rem;
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  border-radius: 999px;
  padding: 0.28rem 0.6rem;
  border: 1px solid var(--lk-line);
}
.lk-sync-synced {
  color: var(--lk-accent-deep);
  background: rgba(26,107,87,0.08);
  border-color: color-mix(in srgb, var(--lk-accent) 30%, var(--lk-line));
}
.lk-sync-syncing {
  color: var(--lk-muted);
  background: color-mix(in srgb, var(--lk-bg) 50%, var(--lk-surface));
}
.lk-sync-offline {
  color: #8a4b2e;
  background: rgba(180, 90, 40, 0.08);
  border-color: rgba(180, 90, 40, 0.35);
}
.wj-period-badge {
  display: inline-flex;
  align-items: center;
}
.wj-approval-card {
  display: flex;
  flex-direction: column;
  gap: 0.65rem;
  margin: 0.75rem 0;
  padding: 1rem 1.1rem;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  background: var(--lk-surface);
}
.wj-approval-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
}
.wj-approval-title {
  margin: 0;
  font-size: 1.05rem;
  font-weight: 600;
}
.wj-approval-summary {
  margin: 0;
  color: var(--lk-muted);
  font-size: 0.92rem;
}
.wj-approval-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}
.wj-compliance-score {
  display: inline-flex;
  align-items: center;
}
.lk-compliance-score-wrap {
  margin: 0.75rem 0 1rem;
}
.lk-compliance-controls {
  margin: 0 0 1rem;
  padding-left: 1.25rem;
}
.lk-approval-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
.lk-empty {
  margin: 0.75rem 0;
  padding: 1.1rem;
  border: 1px dashed var(--lk-line);
  border-radius: var(--lk-radius-sm);
  color: var(--lk-muted);
  background: color-mix(in srgb, var(--lk-bg) 40%, var(--lk-surface));
  text-align: center;
  font-size: 0.9rem;
}
.wj-badge {
  display: inline-flex;
  align-items: center;
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  border-radius: 999px;
  padding: 0.15rem 0.5rem;
  border: 1px solid var(--lk-line);
  background: var(--lk-bg);
  color: var(--lk-muted);
}
.wj-badge-sm { font-size: 0.68rem; padding: 0.12rem 0.45rem; }
.wj-badge-success { background: rgba(31,122,77,0.12); color: var(--lk-success); border-color: rgba(31,122,77,0.28); }
.wj-badge-warning { background: rgba(180,122,18,0.12); color: var(--lk-warning); border-color: rgba(180,122,18,0.28); }
.wj-badge-danger { background: rgba(180,42,34,0.1); color: var(--lk-danger); border-color: rgba(180,42,34,0.25); }
.wj-badge-info { background: rgba(59,110,165,0.12); color: #2f5f8f; border-color: rgba(59,110,165,0.28); }
.wj-badge-primary { background: rgba(26,107,87,0.12); color: var(--lk-accent-deep); border-color: rgba(26,107,87,0.28); }
.wj-icon { display: inline-block; vertical-align: -0.2em; color: currentColor; }

.cmd-palette { position: fixed; inset: 0; z-index: 40; }
.cmd-backdrop { position: absolute; inset: 0; background: rgba(20,34,46,0.35); backdrop-filter: blur(2px); }
.cmd-panel {
  position: relative;
  z-index: 1;
  width: min(520px, calc(100vw - 2rem));
  margin: 12vh auto 0;
  background: var(--lk-surface-elevated);
  border: 1px solid var(--lk-line);
  border-radius: 12px;
  box-shadow: 0 24px 48px rgba(20,34,46,0.18);
  padding: 0.75rem;
  animation: lk-fade-up 180ms ease-out both;
}
.cmd-panel input {
  width: 100%;
  max-width: none;
  border: 1px solid var(--lk-line);
  border-radius: 8px;
  padding: 0.7rem 0.8rem;
  font-size: 1rem;
}
.cmd-list { margin-top: 0.55rem; max-height: 320px; overflow: auto; display: grid; gap: 0.25rem; }
.cmd-item {
  text-align: left;
  background: transparent !important;
  border: 1px solid transparent !important;
  color: var(--lk-ink) !important;
  box-shadow: none !important;
  font-weight: 500 !important;
  padding: 0.55rem 0.7rem !important;
}
.cmd-item:hover { background: rgba(26,107,87,0.08) !important; border-color: var(--lk-line) !important; transform: none !important; }
.cmd-hint { margin: 0.55rem 0 0; font-size: 0.75rem; }
.shell-nav a[hidden] { display: none !important; }

.lk-breadcrumb {
  display: flex; flex-wrap: wrap; align-items: center; gap: 0.35rem;
  margin: 0 0 0.75rem; font-size: 0.82rem;
}
.bc-link { color: var(--lk-muted) !important; font-weight: 500 !important; text-decoration: none !important; }
.bc-link:hover { color: var(--lk-accent-deep) !important; }
.bc-sep { color: var(--lk-line); }

.wj-split-panel {
  gap: 0;
  min-height: 320px;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  overflow: hidden;
  background: var(--lk-surface-elevated);
}
.wj-split-pane { min-width: 0; padding: 0.85rem; }
.wj-split-pane:first-child {
  background: color-mix(in srgb, var(--lk-bg) 55%, var(--lk-surface));
  border-right: 1px solid var(--lk-line);
}
.wj-split-divider { display: none; }
.lk-rail .hub-kicker, .lk-detail .hub-kicker { margin-top: 0; }
.rail-list { list-style: none; padding: 0; margin: 0.5rem 0; display: grid; gap: 0.3rem; }
.rail-item {
  width: 100%;
  text-align: left !important;
  background: transparent !important;
  border: 1px solid transparent !important;
  color: var(--lk-ink) !important;
  box-shadow: none !important;
  font-weight: 500 !important;
  padding: 0.45rem 0.55rem !important;
  border-radius: 6px !important;
}
.rail-item:hover { background: rgba(26,107,87,0.08) !important; transform: none !important; }
.rail-item.is-active {
  background: rgba(26,107,87,0.12) !important;
  border-color: color-mix(in srgb, var(--lk-accent) 40%, var(--lk-line)) !important;
  color: var(--lk-accent-deep) !important;
}
.wj-account-rail-item {
  display: flex !important;
  justify-content: space-between;
  align-items: center;
  gap: 0.5rem;
  width: 100%;
}
.wj-account-rail-label { flex: 1; text-align: left; }
.wj-account-rail-bal {
  font-family: var(--lk-font-mono);
  font-size: 0.8rem;
  color: var(--lk-muted);
  white-space: nowrap;
}
.wj-account-rail-item.is-active .wj-account-rail-bal { color: var(--lk-accent-deep); }
.wj-account-rail-hint { margin: 0.45rem 0 0; font-size: 0.85rem; }
.lk-skeleton { display: grid; gap: 0.45rem; padding: 0.5rem 0; }
.lk-skel-row, .lk-skeleton .wj-skeleton {
  background: linear-gradient(90deg, #e8eef2 25%, #dce6ec 50%, #e8eef2 75%) !important;
}
.register-panel h2 { margin-top: 0.15rem; }

/* R1 Business workspace — single pane of glass */
.workspace-panel { max-width: none; }
.wj-business-workspace[data-layout="pane"] {
  display: grid;
  grid-template-columns: minmax(160px, 200px) minmax(0, 1fr) minmax(240px, 320px);
  grid-template-rows: auto minmax(280px, 1fr) minmax(160px, 220px);
  gap: 1px;
  background: var(--lk-line);
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  overflow: hidden;
  min-height: 520px;
}
.wj-business-workspace[data-layout="pane"] .wj-bw-layout-toggle {
  grid-column: 1 / -1;
  display: flex;
  gap: 0.4rem;
  padding: 0.55rem 0.75rem;
  background: var(--lk-surface-elevated);
}
.wj-business-workspace[data-layout="pane"] .wj-bw-rail {
  grid-row: 2 / 4;
  background: color-mix(in srgb, var(--lk-bg) 55%, var(--lk-surface));
  padding: 0.75rem;
  overflow: auto;
}
.wj-business-workspace[data-layout="pane"] .wj-bw-register {
  background: var(--lk-surface-elevated);
  padding: 0.75rem;
  overflow: auto;
  min-width: 0;
}
.wj-business-workspace[data-layout="pane"] .wj-bw-write {
  background: var(--lk-surface-elevated);
  padding: 0.75rem;
  overflow: auto;
}
.wj-business-workspace[data-layout="pane"] .wj-bw-memorized {
  grid-column: 2;
  background: var(--lk-surface-elevated);
  padding: 0.75rem;
  overflow: auto;
  border-top: 1px solid var(--lk-line);
}
.wj-business-workspace[data-layout="pane"] .wj-bw-bank {
  grid-column: 3;
  background: var(--lk-surface-elevated);
  padding: 0.75rem;
  overflow: auto;
  border-top: 1px solid var(--lk-line);
  min-height: 12rem;
}
.wj-bw-bank .lk-table-scroll {
  max-height: 18rem;
}
.wj-bw-bank table,
.wj-bank-match table {
  min-width: 0;
  width: 100%;
  table-layout: auto;
  font-size: 0.78rem;
}
.wj-bw-bank th:nth-child(1),
.wj-bw-bank td:nth-child(1),
.wj-bank-match th:nth-child(1),
.wj-bank-match td:nth-child(1) { width: 5rem; white-space: nowrap; }
.wj-bw-bank th:nth-child(2),
.wj-bw-bank td:nth-child(2),
.wj-bank-match th:nth-child(2),
.wj-bank-match td:nth-child(2) { width: 9rem; white-space: nowrap; }
/* R27/E10: cap Description width + ellipsis so Amount stays fully visible. */
.wj-bw-bank th:nth-child(3),
.wj-bw-bank td:nth-child(3),
.wj-bank-match th:nth-child(3),
.wj-bank-match td:nth-child(3) {
  max-width: 14rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.wj-bw-bank th:nth-child(4),
.wj-bw-bank td:nth-child(4),
.wj-bank-match th:nth-child(4),
.wj-bank-match td:nth-child(4) {
  width: 6.5rem;
  white-space: nowrap;
  text-align: right;
}
.wj-bank-match-action {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 0.25rem;
  align-items: center;
}
.wj-reconcile-strip {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-end;
  gap: 1rem 1.5rem;
  margin: 0.65rem 0 0.85rem;
  padding: 0.75rem 0.9rem;
  background: color-mix(in srgb, var(--lk-bg) 55%, var(--lk-surface));
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
}
.wj-reconcile-metric {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  min-width: 7rem;
}
.wj-reconcile-label {
  font-size: 0.72rem;
  letter-spacing: 0.03em;
  text-transform: uppercase;
  color: var(--lk-muted);
  font-weight: 600;
}
.wj-reconcile-value {
  font-family: var(--lk-font-mono);
  font-variant-numeric: tabular-nums;
  font-size: 0.95rem;
}
.wj-reconcile-statement-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.35rem;
}
.wj-reconcile-statement-input {
  width: 7.5rem;
  font-family: var(--lk-font-mono);
  font-variant-numeric: tabular-nums;
  font-size: 0.95rem;
  padding: 0.28rem 0.45rem;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  background: var(--lk-surface);
  color: inherit;
}
.wj-reconcile-apply,
.wj-reconcile-clear-all {
  align-self: center;
  font-size: 0.82rem;
  padding: 0.35rem 0.75rem;
}
.wj-reconcile-clear-all:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}
.wj-reconcile-diff.is-balanced {
  color: var(--lk-accent, #1a6b57);
}
.wj-reconcile-diff.is-open {
  color: #8a4b12;
}
.wj-reconcile-badge {
  margin-left: auto;
  align-self: center;
  font-size: 0.75rem;
  font-weight: 650;
  letter-spacing: 0.02em;
  padding: 0.28rem 0.55rem;
  border-radius: var(--lk-radius-sm);
  border: 1px solid var(--lk-line);
}
.wj-reconcile-badge.is-balanced {
  color: var(--lk-accent, #1a6b57);
  background: color-mix(in srgb, var(--lk-accent, #1a6b57) 12%, var(--lk-surface));
}
.wj-reconcile-badge.is-open {
  color: #8a4b12;
  background: color-mix(in srgb, #c47a2c 14%, var(--lk-surface));
}
.wj-reconcile-finish {
  align-self: center;
  margin-left: 0.5rem;
  font-size: 0.82rem;
  padding: 0.35rem 0.75rem;
}
.wj-reconcile-finish:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}
.wj-reconcile-last {
  margin: 0 0 0.75rem;
  font-size: 0.82rem;
}
.wj-reconcile-queue {
  margin: 0 0 0.75rem;
  padding: 0.65rem 0.85rem;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  background: color-mix(in srgb, var(--lk-bg) 40%, var(--lk-surface));
}
.wj-reconcile-queue-head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem 1rem;
  margin-bottom: 0.35rem;
}
.wj-reconcile-queue-title {
  margin: 0;
  font-size: 0.8rem;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}
.wj-reconcile-start-next {
  font-size: 0.82rem;
}
.wj-reconcile-queue-list {
  margin: 0;
  padding-left: 0;
  list-style: none;
  font-size: 0.85rem;
}
.wj-reconcile-queue-row {
  margin: 0.25rem 0;
}
.wj-reconcile-queue-row-needs {
  font-weight: 600;
}
.wj-reconcile-queue-select {
  border: none;
  background: none;
  padding: 0;
  font: inherit;
  color: var(--lk-accent);
  cursor: pointer;
  text-decoration: underline;
  text-underline-offset: 2px;
}
.wj-reconcile-queue-status.is-open {
  color: var(--lk-danger);
}
.wj-reconcile-queue-status.is-warn {
  color: var(--lk-warn, #b45309);
}
.wj-reconcile-queue-status.is-balanced {
  color: var(--lk-ok);
}
.wj-account-rail-recon.is-open {
  color: var(--lk-danger);
}
.wj-account-rail-recon.is-warn {
  color: var(--lk-warn, #b45309);
}
.wj-reconcile-handoff {
  margin: 0 0 0.75rem;
  padding: 0.65rem 0.85rem;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  background: color-mix(in srgb, var(--lk-ok, #15803d) 8%, var(--lk-surface));
}
.wj-reconcile-handoff-msg {
  margin: 0 0 0.45rem;
  font-size: 0.9rem;
}
.wj-reconcile-handoff-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem 0.75rem;
  align-items: center;
}
.wj-reconcile-history {
  margin: 0 0 0.75rem;
}
.wj-reconcile-history-title {
  margin: 0 0 0.25rem;
  font-size: 0.8rem;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}
.wj-reconcile-history-list {
  margin: 0;
  padding-left: 1.1rem;
  font-size: 0.85rem;
}
.wj-reconcile-history-date {
  font-variant-numeric: tabular-nums;
}
.wj-reconcile-report {
  margin: 0 0 0.75rem;
  padding: 0.65rem 0.85rem;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  background: color-mix(in srgb, var(--lk-bg) 40%, var(--lk-surface));
}
.wj-reconcile-report-head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem 1rem;
  margin-bottom: 0.2rem;
}
.wj-reconcile-report-title {
  margin: 0;
  font-size: 0.8rem;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}
.wj-reconcile-exports {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  align-items: center;
}
.wj-reconcile-csv,
.wj-reconcile-pdf {
  font-size: 0.78rem;
  padding: 0.25rem 0.55rem;
}
.wj-reconcile-report-account {
  margin: 0 0 0.55rem;
  font-size: 0.9rem;
  font-weight: 600;
}
.wj-reconcile-report-metrics {
  display: flex;
  flex-wrap: wrap;
  gap: 0.85rem 1.25rem;
  margin-bottom: 0.55rem;
}
.wj-reconcile-report-uncleared-label {
  margin: 0 0 0.2rem;
  font-size: 0.78rem;
}
.wj-reconcile-report-lines {
  margin: 0;
  padding-left: 1.1rem;
  font-size: 0.85rem;
}
.wj-reconcile-report-date {
  font-variant-numeric: tabular-nums;
}
.wj-reconcile-report-empty {
  margin: 0;
  font-size: 0.85rem;
}
.wj-account-rail-status {
  display: block;
  margin-top: 0.2rem;
}
.wj-bank-match-action.is-focus {
  outline: 2px solid var(--lk-accent);
  outline-offset: 2px;
  border-radius: var(--lk-radius-sm);
  background: color-mix(in srgb, var(--lk-accent) 10%, var(--lk-surface-elevated));
  padding: 0.15rem;
}
.wj-bank-match-action [data-wj-bank-match-je] {
  font-size: 0.72rem;
  max-width: 7.5rem;
  padding: 0.1rem 0.2rem;
}
.wj-bw-bank [data-wj-bank-match] {
  font-size: 0.72rem;
  padding: 0.12rem 0.4rem;
  white-space: nowrap;
}
.wj-business-workspace[data-layout="tabs"] {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--lk-line);
  border-radius: var(--lk-radius-sm);
  background: var(--lk-surface-elevated);
  min-height: 480px;
}
.wj-bw-layout-toggle {
  display: flex;
  gap: 0.4rem;
  padding: 0.55rem 0.75rem;
  border-bottom: 1px solid var(--lk-line);
}
.wj-bw-layout-toggle .btn-secondary.is-active {
  border-color: var(--lk-accent);
  color: var(--lk-accent-deep);
  background: color-mix(in srgb, var(--lk-accent) 12%, var(--lk-surface));
}
/* R2.1: auto-triggered AuthFetch stays available to click() but off the chrome. */
[data-wj-auth-fetch][data-auto="1"] {
  position: absolute !important;
  width: 1px !important;
  height: 1px !important;
  padding: 0 !important;
  margin: -1px !important;
  overflow: hidden !important;
  clip: rect(0, 0, 0, 0) !important;
  white-space: nowrap !important;
  border: 0 !important;
}
.row:has(> [data-wj-auth-fetch][data-auto="1"]:only-child) {
  margin: 0;
  min-height: 0;
}
.wj-bw-tabbar {
  display: flex;
  gap: 0.2rem;
  padding: 0.45rem 0.75rem 0;
  border-bottom: 1px solid var(--lk-line);
}
.wj-bw-tab {
  border: 1px solid transparent;
  border-bottom: none;
  background: transparent;
  padding: 0.45rem 0.85rem;
  border-radius: 6px 6px 0 0;
  color: var(--lk-muted);
  cursor: pointer;
  font: inherit;
}
.wj-bw-tab.is-active {
  background: var(--lk-surface);
  border-color: var(--lk-line);
  color: var(--lk-ink);
  margin-bottom: -1px;
}
.wj-bw-tab-panes { padding: 0.85rem; flex: 1; min-height: 0; overflow: auto; }
.bw-advanced { margin: 0.65rem 0; }
.bw-advanced summary { cursor: pointer; color: var(--lk-muted); font-size: 0.9rem; }

.wj-alert {
  display: flex; flex-wrap: wrap; align-items: baseline; gap: 0.45rem 0.65rem;
  padding: 0.7rem 0.85rem; margin: 0.75rem 0;
  border-radius: var(--lk-radius-sm); border: 1px solid var(--lk-line);
  background: color-mix(in srgb, var(--lk-bg) 40%, var(--lk-surface));
  font-size: 0.9rem;
}
.wj-alert-mark {
  font-size: 0.68rem; font-weight: 700; letter-spacing: 0.06em; text-transform: uppercase;
  color: var(--lk-muted);
}
.wj-alert-title { margin-right: 0.25rem; color: var(--lk-ink); }
.wj-alert-error { border-color: rgba(180,42,34,0.35); background: rgba(180,42,34,0.06); }
.wj-alert-error .wj-alert-mark { color: var(--lk-danger); }
.wj-alert-warning { border-color: rgba(180,122,18,0.35); background: rgba(180,122,18,0.07); }
.wj-alert-warning .wj-alert-mark { color: var(--lk-warning); }
.wj-alert-info { border-color: rgba(59,110,165,0.3); background: rgba(59,110,165,0.06); }
.wj-alert-success { border-color: rgba(31,122,77,0.3); background: rgba(31,122,77,0.07); }
.wj-alert-success .wj-alert-mark { color: var(--lk-success); }

.task-queue { list-style: none; padding: 0; margin: 0.85rem 0 0; display: grid; gap: 0.15rem; }
.task-item a {
  display: flex; align-items: center; gap: 0.65rem;
  padding: 0.55rem 0.15rem 0.55rem 0.7rem;
  border: none; border-bottom: 1px solid color-mix(in srgb, var(--lk-line) 70%, transparent);
  border-radius: 0; background: transparent;
  color: var(--lk-ink) !important; font-weight: 500 !important; text-decoration: none !important;
}
.task-item a::before {
  content: ""; width: 3px; align-self: stretch; border-radius: 2px; background: var(--lk-line);
  flex: 0 0 auto;
}
.task-item a:hover { color: var(--lk-accent-deep) !important; }
.task-high a::before { background: var(--lk-danger); }
.task-med a::before { background: var(--lk-warning); }
.task-low a::before { background: var(--lk-line); }

html.lk-density-compact .panel { padding: 0.75rem 0.9rem; }
html.lk-density-compact .hub-link { padding: 0.25rem 0; }
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ledger_layout_covers_shell_and_recon_tokens() {
        let css = ledger_layout_stylesheet();
        assert!(css.contains(".lk-num"));
        assert!(css.contains(".lk-table-scroll"));
        assert!(css.contains(".wj-table") || css.contains(".wj-reconcile"));
        assert!(css.contains(".wj-reconcile-handoff"));
        assert!(css.contains(".wj-reconcile-queue"));
        assert!(css.contains("lk-fade") || css.contains("@keyframes"));
        assert!(!css.contains(":root"));
        assert!(!css.contains("@import"));
    }

    #[test]
    fn ledger_font_import_loads_sora_fraunces() {
        let imp = ledger_font_import_css();
        assert!(imp.contains("fonts.googleapis.com"));
        assert!(imp.contains("Sora"));
        assert!(imp.contains("Fraunces"));
    }
}
