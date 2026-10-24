use nu_ansi_term::Style;
use nu_protocol::ValueHighlight;
use nu_test_support::{fs::Stub::FileWithContent, prelude::*};
use rstest::rstest;

#[rstest]
#[case::highlighted(
    "moe",
    Value::test_string_with_highlights(
        "moe",
        vec![ValueHighlight::new(0..3, Style::default().bold())],
    ),
)]
#[case::plain("--no-highlight moe", "moe")]
fn find_with_list_search_with_string(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[moe larry curly] | find {find_args} | get 0");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "l",
    [
        Value::test_string_with_highlights(
            "larry",
            vec![ValueHighlight::new(0..4, Style::default().bold())],
        ),
        Value::test_string_with_highlights(
            "curly",
            vec![ValueHighlight::new(0..5, Style::default().bold())],
        ),
    ]
)]
#[case::plain("--no-highlight l", ["larry", "curly"])]
fn find_with_list_search_with_char(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[moe larry curly] | find {find_args}");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "-i abc",
    Value::test_string_with_highlights(
        "ABC",
        vec![ValueHighlight::new(0..3, Style::default().bold())],
    ),
)]
#[case::plain("-i --no-highlight abc", "ABC")]
fn find_with_bytestream_search_with_char(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    Playground::setup("find_with_bytestream_search_with_char", |dirs, sandbox| {
        sandbox.with_files(&[FileWithContent("foo.txt", "ABC")]);
        let code = format!("open foo.txt | find {find_args} | get 0");
        test().cwd(dirs.test()).run(code).expect_value_eq(expected)
    })
}

#[rstest]
#[case::highlighted("3", 3)]
#[case::plain("--no-highlight 3", 3)]
fn find_with_list_search_with_number(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[1 2 3 4 5] | find {find_args} | get 0");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "toml",
    Value::test_string_with_highlights(
        "Cargo.toml",
        vec![ValueHighlight::new(6..10, Style::default().bold())],
    ),
)]
#[case::plain("--no-highlight toml", "Cargo.toml")]
fn find_with_string_search_with_string(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("'Cargo.toml' | find {find_args}");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted("shemp", true)]
#[case::plain("--no-highlight shemp", true)]
fn find_with_string_search_with_string_not_found(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[moe larry curly] | find {find_args} | is-empty");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "arep",
    Value::test_string_with_highlights(
        "arepas.clu",
        vec![ValueHighlight::new(0..4, Style::default().bold())],
    ),
)]
#[case::plain("--no-highlight arep", ["arepas.clu"])]
fn find_with_filepath_search_with_string(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!(r#"["amigos.txt", "arepas.clu", "los.txt", "tres.txt"] | find {find_args}"#);
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "arep ami",
    [
        Value::test_string_with_highlights(
            "amigos.txt",
            vec![ValueHighlight::new(0..3, Style::default().bold())],
        ),
        Value::test_string_with_highlights(
            "arepas.clu",
            vec![ValueHighlight::new(0..4, Style::default().bold())],
        ),
    ]
)]
#[case::plain("--no-highlight arep ami", ["amigos.txt", "arepas.clu"])]
fn find_with_filepath_search_with_multiple_patterns(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!(r#"["amigos.txt", "arepas.clu", "los.txt", "tres.txt"] | find {find_args}"#);
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted("a", 2)]
#[case::plain("--no-highlight a", 2)]
fn find_takes_into_account_linebreaks_in_string(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!(r#""atest\nanothertest\nnohit\n" | find {find_args} | length"#);
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "--regex ce",
    [
        Value::test_string_with_highlights(
            "Maurice",
            vec![ValueHighlight::new(5..7, Style::default().bold())],
        ),
        Value::test_string_with_highlights(
            "Laurence",
            vec![ValueHighlight::new(6..8, Style::default().bold())],
        ),
    ]
)]
#[case::plain(
    "--no-highlight --regex ce",
    ["Maurice", "Laurence"]
)]
fn find_with_regex_in_table_keeps_row_if_one_column_matches(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code =
        format!("[[name nickname]; [Maurice moe] [Laurence larry]] | find {find_args} | get name");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "--regex moe --invert",
    ["Laurence"]
)]
#[case::plain(
    "--no-highlight --regex moe --invert",
    ["Laurence"]
)]
fn inverted_find_with_regex_in_table_keeps_row_if_none_of_the_columns_matches(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code =
        format!("[[name nickname]; [Maurice moe] [Laurence larry]] | find {find_args} | get name");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "r --columns [nickname]",
    ["Laurence"]
)]
#[case::plain(
    "r --no-highlight --columns [nickname]",
    ["Laurence"]
)]
fn find_in_table_only_keep_rows_with_matches_on_selected_columns(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code =
        format!("[[name nickname]; [Maurice moe] [Laurence larry]] | find {find_args} | get name");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "r --columns [nickname] --invert",
    ["Maurice"]
)]
#[case::plain(
    "r --no-highlight --columns [nickname] --invert",
    ["Maurice"]
)]
fn inverted_find_in_table_keeps_row_if_none_of_the_selected_columns_matches(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code =
        format!("[[name nickname]; [Maurice moe] [Laurence larry]] | find {find_args} | get name");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "Maurice",
    test_table![
        ["name", "nickname", "Age"];
        [
            Value::test_string_with_highlights(
                "Maurice",
                vec![ValueHighlight::new(0..7, Style::default().bold())],
            ),
            "moe",
            23,
        ],
    ]
)]
#[case::plain(
    "--no-highlight Maurice",
    test_table![
        ["name", "nickname", "Age"];
        ["Maurice", "moe", 23],
    ]
)]
fn find_in_table_keeps_row_with_single_matched_and_keeps_other_columns(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!(
        "[[name nickname Age]; [Maurice moe 23] [Laurence larry 67] [William will 18]] | find {find_args}"
    );
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "moe William",
    test_table![
        ["name", "nickname", "Age"];
        [
            "Maurice",
            Value::test_string_with_highlights(
                "moe",
                vec![ValueHighlight::new(0..3, Style::default().bold())],
            ),
            23,
        ],
        [
            Value::test_string_with_highlights(
                "William",
                vec![ValueHighlight::new(0..7, Style::default().bold())],
            ),
            "will",
            18,
        ],
        [
            Value::test_string_with_highlights(
                "William",
                vec![ValueHighlight::new(0..7, Style::default().bold())],
            ),
            "bill",
            60,
        ],
    ]
)]
#[case::plain(
    "--no-highlight moe William",
    test_table![
        ["name", "nickname", "Age"];
        ["Maurice", "moe", 23],
        ["William", "will", 18],
        ["William", "bill", 60],
    ]
)]
fn find_in_table_keeps_row_with_multiple_matched_and_keeps_other_columns(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!(
        "[[name nickname Age]; [Maurice moe 23] [Laurence larry 67] [William will 18] [William bill 60]] | find {find_args}"
    );
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "'?'",
    test_table![
        ["d"];
        [Value::test_string_with_highlights(
            "?",
            vec![ValueHighlight::new(0..1, Style::default().bold())],
        )],
    ]
)]
#[case::plain(
    "--no-highlight '?'",
    test_table![["d"]; ["a?b"]]
)]
fn find_with_string_search_with_special_char_1(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[[d]; [a?b] [a*b] [a{{1}}b] ] | find {find_args}");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "'*'",
    test_table![
        ["d"];
        [Value::test_string_with_highlights(
            "a*b",
            vec![ValueHighlight::new(1..2, Style::default().bold())],
        )],
    ]
)]
#[case::plain(
    "--no-highlight '*'",
    test_table![["d"]; ["a*b"]]
)]
fn find_with_string_search_with_special_char_2(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[[d]; [a?b] [a*b] [a{{1}}b]] | find {find_args}");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "'{1}'",
    test_table![
        ["d"];
        ["\u{1b}[39ma\u{1b}[0m\u{1b}[41;39m{1}\u{1b}[0m\u{1b}[39mb\u{1b}[0m"],
        [Value::test_string_with_highlights(
            "a{1}b",
            vec![ValueHighlight::new(1..4, Style::default().bold())],
        )],
    ]
)]
#[case::plain(
    "--no-highlight '{1}'",
    test_table![["d"]; ["a{1}b"]]
)]
fn find_with_string_search_with_special_char_3(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[[d]; [a?b] [a*b] [a{{1}}b] ] | find {find_args}");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "'['",
    test_table![
        ["d"];
        [Value::test_string_with_highlights(
            "a[]b",
            vec![ValueHighlight::new(1..2, Style::default().bold())],
        )],
    ]
)]
#[case::plain(
    "--no-highlight '['",
    test_table![["d"]; ["a[]b"]]
)]
fn find_with_string_search_with_special_char_4(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[{{d: a?b}} {{d: a*b}} {{d: a{{1}}b}} {{d: a[]b}}] | find {find_args}");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "']'",
    test_table![
        ["d"];
        [Value::test_string_with_highlights(
            "a[]b",
            vec![ValueHighlight::new(2..3, Style::default().bold())],
        )],
    ]
)]
#[case::plain(
    "--no-highlight ']'",
    test_table![["d"]; ["a[]b"]]
)]
fn find_with_string_search_with_special_char_5(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[{{d: a?b}} {{d: a*b}} {{d: a{{1}}b}} {{d: a[]b}}] | find {find_args}");
    test().run(code).expect_value_eq(expected)
}

#[rstest]
#[case::highlighted(
    "'[]'",
    test_table![
        ["d"];
        [Value::test_string_with_highlights(
            "a[]b",
            vec![ValueHighlight::new(1..3, Style::default().bold())],
        )],
    ]
)]
#[case::plain(
    "--no-highlight '[]'",
    test_table![["d"]; ["a[]b"]]
)]
fn find_with_string_search_with_special_char_6(
    #[case] find_args: &str,
    #[case] expected: impl IntoValue,
) -> Result {
    let code = format!("[{{d: a?b}} {{d: a*b}} {{d: a{{1}}b}} {{d: a[]b}}] | find {find_args}");
    test().run(code).expect_value_eq(expected)
}

#[test]
fn find_in_nested_list_dont_match_bracket() -> Result {
    test()
        .run(r#"[ [foo bar] [foo baz] ] | find "[""#)
        .expect_value_eq(Vec::<Value>::new())
}

#[test]
fn find_and_highlight_in_nested_list() -> Result {
    test()
        .run(r#"[ [foo bar] [foo baz] ] | find "foo""#)
        .expect_value_eq(test_value!([
            [
                Value::test_string_with_highlights(
                    "foo",
                    vec![ValueHighlight::new(0..3, Style::default().bold())],
                ),
                Value::test_string("bar"),
            ],
            [
                Value::test_string_with_highlights(
                    "foo",
                    vec![ValueHighlight::new(0..3, Style::default().bold())],
                ),
                Value::test_string("baz"),
            ],
        ]))
}
