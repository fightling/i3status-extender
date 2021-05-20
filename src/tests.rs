// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

fn generate() -> String {
    let mut result: String = "{\"version\":1}\n[\n[".to_string();
    result.push_str("{");
    result.push_str(&format!("\"name\": \"My Name\",\"instance\": \"My Instance\",\"markup\": \"My Markup\",\"full_text\": \"My Full Text\""));
    result.push_str("}\n");
    return result;
}

#[test]
fn test_position() {
    // open stdin and stdout
    let input = generate();
    let mut io = begin_str(&input).unwrap();
    update(&mut io, "new", 0, false, "new text").unwrap();
}
