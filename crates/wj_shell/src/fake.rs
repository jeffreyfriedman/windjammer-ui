use std::collections::VecDeque;

use accesskit::TreeUpdate;
use wj_paint::DisplayList;

use crate::shell::{Shell, ShellEvent};

/// A scripted, recording shell. `queue` events before a frame; read back what was presented.
pub struct FakeShell {
    pub size: (u32, u32),
    pub scale: f32,
    pending: VecDeque<ShellEvent>,
    /// Every event ever delivered, in order: the replayable event log.
    pub log: Vec<ShellEvent>,
    pub presented: Vec<DisplayList>,
    pub a11y: Vec<TreeUpdate>,
}

impl FakeShell {
    pub fn new(width: u32, height: u32) -> Self {
        FakeShell { size: (width, height), scale: 1.0, pending: VecDeque::new(), log: vec![], presented: vec![], a11y: vec![] }
    }
    pub fn push(&mut self, e: ShellEvent) {
        self.pending.push_back(e);
    }
}

impl Shell for FakeShell {
    fn size(&self) -> (u32, u32) {
        self.size
    }
    fn scale(&self) -> f32 {
        self.scale
    }
    fn poll_events(&mut self) -> Vec<ShellEvent> {
        let evs: Vec<ShellEvent> = self.pending.drain(..).collect();
        for e in &evs {
            if let ShellEvent::Resize { width, height, scale } = e {
                self.size = (*width, *height);
                self.scale = *scale;
            }
        }
        self.log.extend(evs.iter().cloned());
        evs
    }
    fn present(&mut self, list: &DisplayList, _w: u32, _h: u32) {
        self.presented.push(list.clone());
    }
    fn a11y_update(&mut self, update: TreeUpdate) {
        self.a11y.push(update);
    }
}
