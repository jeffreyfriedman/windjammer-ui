/// Index range `[start, end)` of rows to materialize for a fixed-row-height list.
/// `overscan` rows are added on both sides. Pure function; the scene's list binding uses it.
pub fn visible_range(scroll: f32, viewport: f32, row_height: f32, count: usize, overscan: usize) -> (usize, usize) {
    if count == 0 || row_height <= 0.0 || viewport <= 0.0 {
        return (0, 0);
    }
    let first = (scroll.max(0.0) / row_height).floor() as usize;
    let last = ((scroll.max(0.0) + viewport) / row_height).ceil() as usize;
    let start = first.saturating_sub(overscan).min(count);
    let end = (last + overscan).min(count);
    (start, end)
}
