//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

#[cfg(test)]
mod test {

    use wasm_bindgen_test::*;
    use rustwasm_markdown_parser::parse;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn output_as_expected() {
        let expected_html: &str = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
        assert_eq!(expected_html, &parse());
    }
}
