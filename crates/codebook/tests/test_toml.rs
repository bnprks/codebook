use codebook::{
    parser::{TextRange, WordLocation},
    queries::LanguageType,
};

mod utils;

#[test]
fn test_toml_location() {
    let sample_toml = r#"
        name = "testx"
        [dependencies]
        toml = "0.5.8"
        testz = "0.1.0"
"#;
    let expected = vec![WordLocation::new(
        "testx".to_string(),
        vec![TextRange {
            start_char: 16,
            end_char: 21,
            line: 1,
        }],
    )];
    let not_expected = ["testz"];
    let processor = utils::get_processor();
    let misspelled = processor
        .spell_check(sample_toml, Some(LanguageType::TOML), None)
        .to_vec();
    println!("Misspelled words: {misspelled:?}");
    assert_eq!(misspelled, expected);
    assert!(misspelled[0].locations.len() == 1);
    for result in misspelled {
        assert!(!not_expected.contains(&result.word.as_str()));
    }
}
