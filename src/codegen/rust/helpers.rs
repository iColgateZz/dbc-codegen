pub fn format_float(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{:.1}", v)
    } else {
        v.to_string()
    }
}