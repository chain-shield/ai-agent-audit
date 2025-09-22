use once_cell::sync::Lazy;
use regex::Regex;

pub fn sanitize_for_claude(raw: &str) -> String {
    static MD_IMAGE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"!\[(?P<alt>[^\]]*)\]\([^)]+\)").unwrap());

    static HTML_MEDIA: Lazy<Regex> = Lazy::new(|| {
        // matches <img …>, <video …>, <audio …>, <iframe …>, <object …>
        Regex::new(r#"(?is)<\s*(img|video|audio|iframe|object)\b[^>]*>"#).unwrap()
    });

    static NESTED_IMG_LINK: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[\s*\[IMAGE[^\]]*\]\s*\]\([^)]+\)").unwrap());

    static IMAGE_LIKE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[IMAGE:[^\]]+\]").unwrap());

    // ── 1.  Markdown images → [IMAGE] or [IMAGE: alt]
    let step1 = MD_IMAGE.replace_all(raw, |caps: &regex::Captures| {
        let alt = caps.name("alt").map_or("", |m| m.as_str()).trim();
        if alt.is_empty() {
            "[IMAGE]".into()
        } else {
            format!("[IMAGE: {alt}]")
        }
    });

    // ── 2.  HTML media tags → [IMAGE] / [VIDEO] …
    let step2 = HTML_MEDIA.replace_all(&step1, |caps: &regex::Captures| {
        let tag = caps.get(1).unwrap().as_str().to_uppercase();
        format!("[{tag}]")
    });

    // ── 3.  Nested [IMAGE] inside a Markdown link
    let step3 = NESTED_IMG_LINK.replace_all(&step2, |caps: &regex::Captures| {
        let s = caps.get(0).unwrap().as_str();
        let url = s.rsplit("](").next().unwrap_or("").trim_end_matches(')');
        format!("[LINK] → {url}")
    });

    // ── 4.  Remove any remaining “[IMAGE: …]” with an extension-looking alt
    let step4 = IMAGE_LIKE.replace_all(&step3, "[IMAGE]");

    // ── 5.  Strip control characters
    step4
        .chars()
        .filter(|c| matches!(*c, '\n' | '\r' | '\t') || *c >= '\u{20}')
        .collect::<String>()
}
