#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Regenerated from `components_wj/approvalcard.wj` — Windjammer is source of truth.
//! Note: avoid `use super::*` (ambiguous glob imports under wasm deny).

use super::traits::Renderable;
use super::badge::{Badge, BadgeVariant, BadgeSize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ApprovalCard {
    pub workflow_id: String,
    pub state: String,
    pub title: String,
    pub summary: String,
    pub resource_type: String,
    pub resource_id: String,
}

impl ApprovalCard {
#[inline]
pub fn new(workflow_id: String, state: String) -> ApprovalCard {
        ApprovalCard { workflow_id, state, title: "".to_string(), summary: "".to_string(), resource_type: "".to_string(), resource_id: "".to_string() }
}
#[inline]
pub fn title(mut self, title: String) -> ApprovalCard {
        self.title = title;
        self
}
#[inline]
pub fn summary(mut self, summary: String) -> ApprovalCard {
        self.summary = summary;
        self
}
#[inline]
pub fn resource_type(mut self, resource_type: String) -> ApprovalCard {
        self.resource_type = resource_type;
        self
}
#[inline]
pub fn resource_id(mut self, resource_id: String) -> ApprovalCard {
        self.resource_id = resource_id;
        self
}
}

impl Renderable for ApprovalCard {
#[inline]
fn render(&self) -> String {
        let workflow_id = self.workflow_id.clone();
        let state = self.state.clone();
        let title = self.title.clone();
        let summary = self.summary.clone();
        let resource_type = self.resource_type.clone();
        let resource_id = self.resource_id.clone();
        let heading = heading_text(title, resource_id.clone());
        let badge = Badge::new(state.clone()).variant(state_badge_variant(&state)).size(BadgeSize::Small).render();
        let summary_html = {
            if summary.len() == 0 {
                "".to_string()
            } else {
                format!("{}{}{}", "<p class='wj-approval-summary'>", summary, "</p>")
            }
        };
        let actions = {
            if state.to_lowercase() == "pending" {
                format!("{}{}{}{}", "<div class='wj-approval-actions'>", "<button type='button' class='wj-btn wj-btn-success' data-wj-approval-approve>Approve</button>", "<button type='button' class='wj-btn wj-btn-danger' data-wj-approval-reject>Reject</button>", "</div>")
            } else {
                "".to_string()
            }
        };
        format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", "<article class='wj-approval-card' data-wj-approval", " data-wj-workflow-id='", workflow_id, "'", " data-wj-approval-state='", state, "'", " data-wj-resource-type='", resource_type, "'", " data-wj-resource-id='", resource_id, "'>", "<header class='wj-approval-header'><h3 class='wj-approval-title'>", heading, "</h3>", badge, "</header>", summary_html, actions, "</article>")
}
}

#[inline]
pub fn state_badge_variant(state: &str) -> BadgeVariant {
    let s = state.to_lowercase();
    if s == "pending" {
        BadgeVariant::Warning
    } else {
        if s == "completed" || s == "approved" {
            BadgeVariant::Success
        } else {
            if s == "rejected" {
                BadgeVariant::Danger
            } else {
                BadgeVariant::Info
            }
        }
    }
}

#[inline]
pub fn heading_text(title: String, resource_id: String) -> String {
    if title.is_empty() {
        if resource_id.is_empty() {
            "Approval".to_string()
        } else {
            format!("{}{}", "Approve ", resource_id)
        }
    } else {
        title
    }
}

/// Framework runtime: Approve/Reject → POST /api/v1/workflow/{id}/{approve|reject} (P3.6/P3.7).
#[inline]
pub fn approval_card_runtime_js() -> &'static str {
    "\n(function () {\n  if (window.__wjApprovalCardBound) return;\n  window.__wjApprovalCardBound = true;\n  async function postWorkflowAction(card, action, btn) {\n    var id = card.getAttribute('data-wj-workflow-id') || '';\n    if (!id) return;\n    var token = localStorage.getItem('ledgerkit_token') || '';\n    if (!token) {\n      card.insertAdjacentHTML('beforeend', '<p class=\"err\">Sign in first.</p>');\n      return;\n    }\n    if (btn) btn.disabled = true;\n    try {\n      var res = await fetch((window.LEDGERKIT_API || '') + '/api/v1/workflow/' + encodeURIComponent(id) + '/' + action, {\n        method: 'POST',\n        headers: { Authorization: 'Bearer ' + token }\n      });\n      var data = {};\n      try { data = await res.json(); } catch (e) {}\n      if (!res.ok) {\n        card.insertAdjacentHTML('beforeend', '<p class=\"err\">' + (action === 'reject' ? 'Reject' : 'Approve') + ' failed.</p>');\n        if (btn) btn.disabled = false;\n        return;\n      }\n      var state = (data && data.state) || (action === 'reject' ? 'rejected' : 'completed');\n      card.setAttribute('data-wj-approval-state', state);\n      var actions = card.querySelector('.wj-approval-actions');\n      if (actions) actions.remove();\n      var badge = card.querySelector('.wj-badge');\n      if (badge) {\n        badge.textContent = state;\n        badge.className = 'wj-badge ' + (state === 'rejected' ? 'wj-badge-danger' : 'wj-badge-success') + ' wj-badge-sm';\n      }\n      if (typeof window.lkSetSyncStatus === 'function') window.lkSetSyncStatus('synced');\n    } catch (e) {\n      card.insertAdjacentHTML('beforeend', '<p class=\"err\">' + (action === 'reject' ? 'Reject' : 'Approve') + ' offline.</p>');\n      if (btn) btn.disabled = false;\n      if (typeof window.lkSetSyncStatus === 'function') window.lkSetSyncStatus('offline');\n    }\n  }\n  document.addEventListener('click', function (ev) {\n    var t = ev.target;\n    if (!t || !t.closest) return;\n    var approveBtn = t.closest('[data-wj-approval-approve]');\n    if (approveBtn) {\n      ev.preventDefault();\n      var card = approveBtn.closest('[data-wj-approval]');\n      if (card) postWorkflowAction(card, 'approve', approveBtn);\n      return;\n    }\n    var rejectBtn = t.closest('[data-wj-approval-reject]');\n    if (rejectBtn) {\n      ev.preventDefault();\n      var rejectCard = rejectBtn.closest('[data-wj-approval]');\n      if (rejectCard) postWorkflowAction(rejectCard, 'reject', rejectBtn);\n    }\n  });\n})();\n"
}

