use std::error::Error;
use regex::Regex;

pub fn get_tldr_text(text: String) -> Result<(String), Box<dyn Error>> {
    let mut cleaned_string = "".to_string();
    let mut chars = text.chars();
    loop {
        if let Some(c) = chars.next() {
            if c.is_alphanumeric() || c.is_whitespace() {
                cleaned_string = format!("{}{}", cleaned_string, c);
            }
        } else {
            break;
        }
    }

    cleaned_string = cleaned_string.replace("\n", ".");

    let re = Regex::new(r"tldr [A-Z][^\\.;]*[^\\.;]*").unwrap();

    let mut tldr_result = "".to_string();
    for cap in re.captures_iter(&cleaned_string) {
        if cap.len() > 0 {
            tldr_result = cap[0].to_string();
        }
    }

    Ok(tldr_result)
}
