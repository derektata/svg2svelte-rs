use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref RE_BIND_ID: Regex = Regex::new(r#"id="bind:(.*?)""#).unwrap();
    static ref RE_BIND_CLASS: Regex = Regex::new(r#"class="bind:(.*?)""#).unwrap();
}

/// Replace id="bind:..." with class="bind:..."
pub fn replace_bind_ids(content: &str) -> String {
    RE_BIND_ID
        .replace_all(content, r#"class="bind:$1""#)
        .to_string()
}

/// Extract variable names from class="bind:..."
pub fn extract_bind_variables(content: &str) -> Vec<String> {
    RE_BIND_CLASS
        .captures_iter(content)
        .map(|c| c[1].to_string())
        .collect()
}

/// Transform class="bind:..." into class="..." bind:this={...}
pub fn bind_svg_variables(content: &str) -> String {
    RE_BIND_CLASS
        .replace_all(content, |caps: &Captures| {
            let var = &caps[1];
            format!(r#"class="{}" bind:this={{{}}}"#, var, var)
        })
        .to_string()
}
