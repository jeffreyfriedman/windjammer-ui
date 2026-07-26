//! MemorizedList — Business-mode memorized transactions (ADR-002 / R0+).
//! Hand-maintained. Always SKIP_WJ_REGEN=1.

use super::traits::Renderable;

#[derive(Clone, Debug)]
pub struct MemorizedItem {
    pub id: String,
    pub title: String,
    pub body: String,
}

impl MemorizedItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self { id: id.into(), title: title.into(), body: body.into() }
    }
}

#[derive(Clone, Debug)]
pub struct MemorizedList {
    pub items: Vec<MemorizedItem>,
    pub empty_message: String,
}

impl MemorizedList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            empty_message: "No memorized transactions yet — save one below.".to_string(),
        }
    }

    pub fn empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = message.into();
        self
    }

    pub fn item(mut self, item: MemorizedItem) -> Self {
        self.items.push(item);
        self
    }
}

impl Default for MemorizedList {
    fn default() -> Self { Self::new() }
}

impl Renderable for MemorizedList {
    fn render(&self) -> String {
        let list = if self.items.is_empty() {
            format!(r#"<li class="muted lk-empty">{}</li>"#, self.empty_message)
        } else {
            self.items.iter().map(|i| format!(
                r#"<li class="wj-memorized-item" data-wj-memorized-id="{id}"><span class="wj-memorized-title">{title}</span>
<button type="button" class="btn-secondary" data-wj-memorized-run="{id}">Run</button>
<button type="button" class="btn-secondary" data-wj-memorized-delete="{id}">Remove</button></li>"#,
                id = i.id, title = i.title
            )).collect::<Vec<_>>().join("")
        };
        format!(
            r##"<div class="wj-memorized-list" data-wj-memorized-list>
  <ul class="hub-list" id="memorizedList">{list}</ul>
  <div class="panel-filters">
    <label for="memTitle">Title</label>
    <input id="memTitle" type="text" placeholder="e.g. Monthly rent"/>
    <label for="memBody">Journal JSON</label>
    <textarea id="memBody" rows="4" placeholder="Paste balanced journal body"></textarea>
    <div class="row"><button type="button" class="btn-secondary" data-wj-memorized-save>Save memorized</button></div>
  </div>
</div>"##,
            list = list
        )
    }
}

pub fn memorized_list_runtime_js() -> &'static str {
    r##"
(function () {
  if (window.__wjMemorizedBound) return;
  window.__wjMemorizedBound = true;
  const KEY = 'ledgerkit_memorized';
  const DEMO = [
    {
      id: 'mem-rent',
      title: 'Office rent',
      body: '{"reference":"MEM-RENT","transaction_date":"2026-07-01","memo":"Office rent","lines":[{"account_code":"5000","amount_cents":250000,"description":"rent"},{"account_code":"1000","amount_cents":-250000,"description":"cash"}]}'
    },
    {
      id: 'mem-payroll',
      title: 'Biweekly payroll',
      body: '{"reference":"MEM-PAY","transaction_date":"2026-07-15","memo":"Biweekly payroll","lines":[{"account_code":"5000","amount_cents":320000,"description":"payroll"},{"account_code":"1000","amount_cents":-320000,"description":"cash"}]}'
    }
  ];
  function load() {
    try { return JSON.parse(localStorage.getItem(KEY) || '[]'); } catch (_) { return []; }
  }
  function save(items) { localStorage.setItem(KEY, JSON.stringify(items)); }
  function ensureSeed() {
    if (localStorage.getItem(KEY + '_seeded') === '1') return;
    if (!load().length) save(DEMO.slice());
    localStorage.setItem(KEY + '_seeded', '1');
  }
  function renderList() {
    const ul = document.getElementById('memorizedList');
    if (!ul) return;
    ensureSeed();
    const items = load();
    if (!items.length) {
      ul.innerHTML = '<li class="muted lk-empty">No memorized transactions yet — save one below.</li>';
      return;
    }
    ul.innerHTML = items.map((i) =>
      '<li class="wj-memorized-item" data-wj-memorized-id="' + i.id + '"><span class="wj-memorized-title">' +
      (i.title || 'Untitled') +
      '</span> <button type="button" class="btn-secondary" data-wj-memorized-run="' + i.id +
      '">Run</button> <button type="button" class="btn-secondary" data-wj-memorized-delete="' + i.id +
      '">Remove</button></li>'
    ).join('');
  }
  window.wjMemorizedRefresh = renderList;
  document.addEventListener('click', async (e) => {
    const t = e.target;
    if (!(t instanceof Element)) return;
    if (t.closest('[data-wj-memorized-save]')) {
      const title = (document.getElementById('memTitle') || {}).value || 'Untitled';
      const body = (document.getElementById('memBody') || {}).value || '';
      if (!body.trim()) return;
      const items = load();
      items.push({ id: 'm' + Date.now(), title: title, body: body });
      save(items);
      renderList();
      return;
    }
    const del = t.closest('[data-wj-memorized-delete]');
    if (del) {
      const id = del.getAttribute('data-wj-memorized-delete');
      save(load().filter((i) => i.id !== id));
      renderList();
      return;
    }
    const run = t.closest('[data-wj-memorized-run]');
    if (run) {
      const id = run.getAttribute('data-wj-memorized-run');
      const item = load().find((i) => i.id === id);
      const out = document.getElementById('out');
      if (!item) return;
      const token = localStorage.getItem('ledgerkit_token') || '';
      if (!token) {
        if (out) { out.hidden = false; out.classList.add('is-error'); out.textContent = 'Sign in first.'; }
        return;
      }
      if (out) { out.hidden = false; out.classList.remove('is-error'); out.textContent = 'Posting memorized…'; }
      try {
        const res = await fetch((window.LEDGERKIT_API || '') + '/api/v1/journal-entries', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', Authorization: 'Bearer ' + token },
          body: item.body
        });
        if (out) {
          if (!res.ok) { out.classList.add('is-error'); out.textContent = 'Could not post (' + res.status + ')'; }
          else {
            out.textContent = 'Memorized transaction posted — register refreshing…';
            var load = document.getElementById('loadCheckbook')
              || document.querySelector('[data-wj-render-kind="checkbook"]');
            if (load) { try { load.click(); } catch (err) {} }
          }
        }
      } catch (err) {
        if (out) { out.classList.add('is-error'); out.textContent = String(err); }
      }
    }
  });
  renderList();
})();
"##
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memorized_list_empty_shell() {
        let html = MemorizedList::new().render();
        assert!(html.contains("wj-memorized-list"));
        assert!(html.contains("data-wj-memorized-save"));
    }

    #[test]
    fn memorized_list_renders_seed_items() {
        let html = MemorizedList::new()
            .item(MemorizedItem::new("mem-rent", "Office rent", "{}"))
            .render();
        assert!(html.contains("data-wj-memorized-id=\"mem-rent\""));
        assert!(html.contains("Office rent"));
    }

    #[test]
    fn memorized_runtime_uses_storage_key() {
        let js = memorized_list_runtime_js();
        assert!(js.contains("ledgerkit_memorized"));
        assert!(js.contains("data-wj-memorized-run"));
        assert!(js.contains("ensureSeed") || js.contains("mem-rent"));
    }
}
