use pdf_extract::extract_text;

// Shorthand for creating ExpectedText
// example: expected!("atomic.pdf", "Atomic Data");
macro_rules! expected {
    ($filename:expr, $text:expr) => {
        ExpectedText {
            filename: $filename,
            text: $text,
        }
    };
}

// Use the macro to create a list of ExpectedText
// and then check if the text is correctly extracted
#[test]
fn extract_expected_text() {
    let docs = vec![
        expected!("documents_stack.pdf", "mouse button until"),
        expected!("complex.pdf", "communicate state changes"),
        expected!("simple.pdf", "And more text"),
        expected!("version1_2.pdf", "HERE IS ALL CAPS"),
        expected!("version1_3.pdf", "HERE IS ALL CAPS"),
        expected!("from_macos_pages.pdf", "hope this works"),
        expected!("alternate-color-space.pdf", ""),
    ];

    for doc in docs {
        doc.test();
    }
}

// data structure to make it easy to check if certain files are correctly parsed
// e.g. ExpectedText { filename: "atomic.pdf", text: "Atomic Data" }
#[derive(Debug, PartialEq)]
struct ExpectedText<'a> {
    filename: &'a str,
    text: &'a str,
}

impl ExpectedText<'_> {
    /// Opens the `filename` from `tests/docs`, extracts the text and checks if it contains `text`
    /// If the file ends with `_link`, it will download the file from the url in the file to the `tests/docs_cache` directory
    fn test(self) {
        let ExpectedText { filename, text } = self;
        let file_path = format!("tests/docs/{}", filename);
        let out = extract_text(file_path)
            .unwrap_or_else(|e| panic!("Failed to extract text from {}, {}", filename, e));
        assert!(
            out.contains(text),
            "Text {} does not contain '{}'",
            filename,
            text
        );
    }
}
