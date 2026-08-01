#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::badge::{Badge, BadgeSize, BadgeVariant};
use super::traits::Renderable;
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
        ApprovalCard {
            workflow_id,
            state,
            title: "".to_string(),
            summary: "".to_string(),
            resource_type: "".to_string(),
            resource_id: "".to_string(),
        }
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
        let badge = Badge::new(state.clone())
            .variant(state_badge_variant(&state))
            .size(BadgeSize::Small)
            .render();
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
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "<article class='wj-approval-card' data-wj-approval",
            " data-wj-workflow-id='",
            workflow_id,
            "'",
            " data-wj-approval-state='",
            state,
            "'",
            " data-wj-resource-type='",
            resource_type,
            "'",
            " data-wj-resource-id='",
            resource_id,
            "'>",
            "<header class='wj-approval-header'><h3 class='wj-approval-title'>",
            heading,
            "</h3>",
            badge,
            "</header>",
            summary_html,
            actions,
            "</article>"
        )
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

/// Framework runtime: Approve button → POST /api/v1/workflow/{id}/approve (finance P3.6).
#[inline]
pub fn approval_card_runtime_js() -> &'static str {
    "\n(function () {\n  if (window.__wjApprovalCardBound) return;\n  window.__wjApprovalCardBound = true;\n  document.addEventListener('click', async function (ev) {\n    var t = ev.target;\n    if (!t || !t.closest) return;\n    var approveBtn = t.closest('[data-wj-approval-approve]');\n    if (!approveBtn) return;\n    ev.preventDefault();\n    var card = approveBtn.closest('[data-wj-approval]');\n    if (!card) return;\n    var id = card.getAttribute('data-wj-workflow-id') || '';\n    if (!id) return;\n    var token = localStorage.getItem('ledgerkit_token') || '';\n    if (!token) {\n      card.insertAdjacentHTML('beforeend', '<p class=\"err\">Sign in first.</p>');\n      return;\n    }\n    approveBtn.disabled = true;\n    try {\n      var res = await fetch((window.LEDGERKIT_API || '') + '/api/v1/workflow/' + encodeURIComponent(id) + '/approve', {\n        method: 'POST',\n        headers: { Authorization: 'Bearer ' + token }\n      });\n      var data = {};\n      try { data = await res.json(); } catch (e) {}\n      if (!res.ok) {\n        card.insertAdjacentHTML('beforeend', '<p class=\"err\">Approve failed.</p>');\n        approveBtn.disabled = false;\n        return;\n      }\n      card.setAttribute('data-wj-approval-state', (data && data.state) || 'completed');\n      var actions = card.querySelector('.wj-approval-actions');\n      if (actions) actions.remove();\n      var badge = card.querySelector('.wj-badge');\n      if (badge) {\n        badge.textContent = (data && data.state) || 'completed';\n        badge.className = 'wj-badge wj-badge-success wj-badge-sm';\n      }\n      if (typeof window.lkSetSyncStatus === 'function') window.lkSetSyncStatus('synced');\n    } catch (e) {\n      card.insertAdjacentHTML('beforeend', '<p class=\"err\">Approve offline.</p>');\n      approveBtn.disabled = false;\n      if (typeof window.lkSetSyncStatus === 'function') window.lkSetSyncStatus('offline');\n    }\n  });\n})();\n"
}
