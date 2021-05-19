// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

fn generate(n: usize) -> String {
    let mut result: String = "{\"version\":1}\n[\n[".to_string();
    let mut items: Vec<String> = Vec::new();
    for _i in 0..=n {
        let mut item = String::new();
        item.push_str("{");
        item.push_str(&format!("\"name\": \"name{n}\",\"instance\": \"instance{n}\",\"markup\": \"markup{n}\",\"full_text\": \"full_text{n}\"",n=n));
        item.push_str("}");
        items.push(item);
    }
    result.push_str(&items.join(","));
    result.push_str("]");
    return result;
}

#[test]
fn test_position() {
    generate(30);
    let input = generate(10);
    let mut reader = input.as_bytes();
    let output = std::io::stdout();
    let mut writer = LineWriter::new(output.lock());
    begin(&mut reader, &mut writer).unwrap();
    update(&mut reader, &mut writer, "new", 0, false, "new text").unwrap();
}
