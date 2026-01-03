use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TimelineEvent {
    pub title: String,
    pub description: String,
    pub timestamp: String,
    pub icon: String,
    pub color: String,
}

impl TimelineEvent {
#[inline]
pub fn new(title: String, timestamp: String) -> TimelineEvent {
        TimelineEvent { title, description: String::new(), timestamp, icon: "●".to_string(), color: "#3b82f6".to_string() }
}
#[inline]
pub fn description(mut self, desc: String) -> TimelineEvent {
        self.description = desc;
        self
}
#[inline]
pub fn icon(mut self, icon: String) -> TimelineEvent {
        self.icon = icon;
        self
}
#[inline]
pub fn color(mut self, color: String) -> TimelineEvent {
        self.color = color;
        self
}
}

#[derive(Debug, Clone, Default)]
pub struct Timeline {
    pub events: Vec<TimelineEvent>,
}

impl Timeline {
#[inline]
pub fn new() -> Timeline {
        Timeline { events: Vec::new() }
}
#[inline]
pub fn event(mut self, event: TimelineEvent) -> Timeline {
        self.events.push(event);
        self
}
}

impl Renderable for Timeline {
#[inline]
fn render(self) -> String {
        let mut html = String::new();
        html.push_str("<div style='position: relative; padding-left: 32px;'>");
        html.push_str("<div style='position: absolute; left: 8px; top: 0; bottom: 0; width: 2px; background: #e2e8f0;'></div>");
        for (_event_index, event) in self.events.iter().enumerate() {
            html.push_str("<div style='position: relative; padding-bottom: 32px;'>");
            html.push_str("<div style='position: absolute; left: -24px; width: 16px; height: 16px; border-radius: 50%; background: ");
            html.push_str(&event.color);
            html.push_str("; border: 2px solid white; display: flex; align-items: center; justify-content: center; color: white; font-size: 10px;'>");
            html.push_str(&event.icon);
            html.push_str("</div>");
            html.push_str("<div style='background: white; border: 1px solid #e2e8f0; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);'>");
            html.push_str("<div style='font-size: 12px; color: #718096; margin-bottom: 4px;'>");
            html.push_str(&event.timestamp);
            html.push_str("</div>");
            html.push_str("<div style='font-weight: 600; font-size: 16px; color: #1a202c; margin-bottom: 8px;'>");
            html.push_str(&event.title);
            html.push_str("</div>");
            if event.description.len() > (0 as usize) {
                html.push_str("<div style='font-size: 14px; color: #4a5568;'>");
                html.push_str(&event.description);
                html.push_str("</div>")
            }
            html.push_str("</div>");
            html.push_str("</div>");
        }
        html.push_str("</div>");
        html
}
}

