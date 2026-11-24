//! Differential test against Chrome's flexbox: fixtures recorded by `tools/gen_chrome_fixtures.py`
//! (headless Chrome `getBoundingClientRect`), an implementation independent of Taffy.
use serde_json::Value;
use wj_layout::*;
use wj_scene::*;

fn dim(v: &Value) -> Dim {
    match v {
        Value::Null => Dim::Auto,
        Value::Array(a) => {
            let n = a[1].as_f64().unwrap() as f32;
            if a[0] == "px" { Dim::Px(n) } else { Dim::Pct(n) }
        }
        _ => unreachable!(),
    }
}

fn style(n: &Value, root: bool, vp: (f32, f32)) -> Style {
    let m = n["margin"].as_array().unwrap();
    let mg = |i: usize| if root { 0.0 } else { m[i].as_f64().unwrap() as f32 };
    Style {
        direction: if n["direction"] == "row" { Direction::Row } else { Direction::Column },
        width: if root { Dim::Px(vp.0) } else { dim(&n["width"]) },
        height: if root { Dim::Px(vp.1) } else { dim(&n["height"]) },
        padding: Edges::all(n["padding"].as_f64().unwrap() as f32),
        margin: Edges { left: mg(0), top: mg(1), right: mg(2), bottom: mg(3) },
        gap: n["gap"].as_f64().unwrap() as f32,
        flex_grow: if root { 0.0 } else { n["grow"].as_f64().unwrap() as f32 },
        flex_shrink: n["shrink"].as_f64().unwrap() as f32,
        align_items: match n["align"].as_str().unwrap() {
            "start" => Align::Start,
            "center" => Align::Center,
            "end" => Align::End,
            _ => Align::Stretch,
        },
        justify: match n["justify"].as_str().unwrap() {
            "start" => Justify::Start,
            "center" => Justify::Center,
            "end" => Justify::End,
            "space_between" => Justify::SpaceBetween,
            _ => Justify::SpaceAround,
        },
        ..Default::default()
    }
}

fn build(s: Scene, parent: Option<NodeId>, n: &Value, vp: (f32, f32), out: &mut Vec<NodeId>) {
    let id = match parent {
        None => s.root(),
        Some(p) => s.create(p),
    };
    s.set_style(id, style(n, parent.is_none(), vp));
    out.push(id);
    for c in n["children"].as_array().unwrap() {
        build(s, Some(id), c, vp, out);
    }
}

#[test]
fn matches_chrome_flexbox_on_recorded_fixtures() {
    let raw = include_str!("fixtures/chrome_flex.json");
    let fixtures: Vec<Value> = serde_json::from_str(raw).unwrap();
    assert!(fixtures.len() >= 150, "fixture set too small: {}", fixtures.len());
    let mut failures = vec![];
    for (i, f) in fixtures.iter().enumerate() {
        wj_reactive::reset();
        let s = Scene::new();
        let vp = (f["viewport"][0].as_f64().unwrap() as f32, f["viewport"][1].as_f64().unwrap() as f32);
        let mut ids = vec![];
        build(s, None, &f["spec"], vp, &mut ids);
        let mut e = LayoutEngine::new();
        let d = s.drain_dirty();
        e.update(s, &d, vp, &FixedAdvanceMeasure);
        let rects = f["rects"].as_array().unwrap();
        assert_eq!(rects.len(), ids.len());
        for (k, id) in ids.iter().enumerate() {
            let want: Vec<f32> = rects[k].as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
            let got = s.rect(*id);
            let got = [got.x, got.y, got.w, got.h];
            if got.iter().zip(&want).any(|(a, b)| (a - b).abs() > 0.05) {
                failures.push(format!("fixture {i} node {k}: ours {got:?} chrome {want:?}"));
                break;
            }
        }
    }
    let allowed = std::env::var("WJ_CHROME_ALLOWED_MISMATCHES").ok().and_then(|v| v.parse().ok()).unwrap_or(0usize);
    assert!(failures.len() <= allowed, "{} of {} fixtures differ from Chrome:\n{}", failures.len(), fixtures.len(), failures.iter().take(12).cloned().collect::<Vec<_>>().join("\n"));
}
