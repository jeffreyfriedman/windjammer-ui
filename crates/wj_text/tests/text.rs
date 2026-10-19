use wj_layout::TextMeasure;
use wj_paint::TextLayouter;
use wj_text::{SharedText, TextEngine, DEFAULT_FONT};

fn gid(ch: char) -> u32 {
    let face = ttf_parser::Face::parse(DEFAULT_FONT, 0).unwrap();
    face.glyph_index(ch).unwrap().0 as u32
}

#[test]
fn glyph_ids_match_the_font_cmap_oracle() {
    let mut e = TextEngine::new();
    let l = e.layout("Hello", 20.0, None);
    let got: Vec<u32> = l.runs.iter().flat_map(|r| r.glyphs.iter().map(|g| g.id)).collect();
    let want: Vec<u32> = "Hello".chars().map(gid).collect();
    assert_eq!(got, want, "shaped glyph ids equal the cmap lookup (no ligatures in this string)");
}

#[test]
fn advances_scale_with_size_and_differ_by_glyph() {
    let mut e = TextEngine::new();
    let a = e.layout("iiii", 20.0, None);
    let b = e.layout("WWWW", 20.0, None);
    let c = e.layout("WWWW", 40.0, None);
    assert!(a.width < b.width, "narrow glyphs are narrower: {} vs {}", a.width, b.width);
    assert!((c.width / b.width - 2.0).abs() < 0.01, "linear in font size");
    assert!(b.height > 20.0 && b.height < 30.0, "line height from font metrics: {}", b.height);
}

#[test]
fn wrapping_respects_max_width_and_adds_lines() {
    let mut e = TextEngine::new();
    let one = e.layout("the quick brown fox jumps over the lazy dog", 16.0, None);
    let wrapped = e.layout("the quick brown fox jumps over the lazy dog", 16.0, Some(120.0));
    assert!(wrapped.width <= 120.5, "{}", wrapped.width);
    assert!(wrapped.height > one.height * 2.0, "wrapped into several lines: {} vs {}", wrapped.height, one.height);
}

#[test]
fn layouts_are_cached_and_measure_shares_the_cache() {
    let m = SharedText(std::cell::RefCell::new(TextEngine::new()));
    let a = m.measure("cache me", 14.0, None);
    let b = m.measure("cache me", 14.0, None);
    assert_eq!(a, b);
    assert_eq!(m.0.borrow().cached(), 1);
    m.measure("cache me", 15.0, None);
    assert_eq!(m.0.borrow().cached(), 2);
}

#[test]
fn startup_does_not_scan_system_fonts() {
    let t = std::time::Instant::now();
    let mut e = TextEngine::new();
    e.layout("warm", 14.0, None);
    let ms = t.elapsed().as_secs_f64() * 1e3;
    println!("embedded-font engine: first layout {ms:.1} ms");
    // The spike measured 165-735 ms for the first layout with system font discovery.
    assert!(ms < 150.0, "first layout took {ms:.1} ms");
}

#[test]
fn empty_and_whitespace_text_do_not_panic() {
    let mut e = TextEngine::new();
    for s in ["", " ", "\n", "a\nb"] {
        let l = e.layout(s, 14.0, Some(50.0));
        assert!(l.width.is_finite() && l.height.is_finite());
    }
}
