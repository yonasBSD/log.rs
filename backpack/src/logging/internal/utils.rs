#[must_use]
pub fn format_duration(d: std::time::Duration) -> String {
    if d.as_secs() > 0 {
        format!("{:.1}s", d.as_secs_f64())
    } else {
        format!("{}ms", d.as_millis())
    }
}
