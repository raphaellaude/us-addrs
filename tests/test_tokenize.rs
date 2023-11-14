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
