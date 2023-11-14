use std::vec;

use us_addrs::tokenize;

#[test]
fn test_tokenizing() {
    let mut tokens = tokenize("# 1 abc st");
    assert_eq!(tokens, vec!["#", "1", "abc", "st"]);

    tokens = tokenize("#1 abc st");
    assert_eq!(tokens, vec!["#", "1", "abc", "st"]);

    tokens = tokenize("box # 1 abc st");
    assert_eq!(tokens, vec!["box", "#", "1", "abc", "st"]);

    tokens = tokenize("box #1 abc st");
    assert_eq!(tokens, vec!["box", "#", "1", "abc", "st"]);

    tokens = tokenize("box# 1 abc st");
    assert_eq!(tokens, vec!["box", "#", "1", "abc", "st"]);

    tokens = tokenize("box#1 abc st");
    assert_eq!(tokens, vec!["box", "#", "1", "abc", "st"]);
}

#[test]
fn test_split_on_punc() {
    // let mut tokens = tokenize("1 abc st,suite 1");
    // assert_eq!(tokens, vec!["1", "abc", "st,", "suite", "1"]);

    // tokens = tokenize("1 abc st;suite 1");
    // assert_eq!(tokens, vec!["1", "abc", "st;", "suite", "1"]);

    let tokens = tokenize("1-5 abc road");
    assert_eq!(tokens, vec!["1-5", "abc", "road"]);
}

#[test]
fn test_spaces() {
    let result = vec!["1", "abc", "st"];

    let mut tokens = tokenize("1 abc st");
    assert_eq!(tokens, result);

    tokens = tokenize("1 abc  st");
    assert_eq!(tokens, result);

    tokens = tokenize("1 abc st ");
    assert_eq!(tokens, result);

    tokens = tokenize(" 1 abc st");
    assert_eq!(tokens, result);
}
