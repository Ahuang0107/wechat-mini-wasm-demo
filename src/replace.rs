use std::fs::{read_to_string, File};
use std::io::Write;

fn main() {
    let mut content = read_to_string("./pkg/wechat_mini_wasm_demo.js").unwrap();
    let encoding_js = read_to_string("encoding_utf8.min.js").unwrap();
    content = encoding_js + "\n\r" + &content;
    content = content.replace("WebAssembly", "WXWebAssembly");
    content = content.replace(
        "input = new URL('wechat_mini_wasm_demo_bg.wasm', import.meta.url);",
        "",
    );
    content = content.replace("input = fetch(input);", "");
    File::create("./pkg/wechat_mini_wasm_demo.js")
        .unwrap()
        .write(content.as_bytes())
        .unwrap();
}
