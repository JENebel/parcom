use parcom::lex_rule;
#[derive(Debug, PartialEq)]
enum Token {
    A, B, C, Eof
}
use Token::*;

#[test]
fn wildcard_matches_any_char() {
    lex_rule!{wildcard() -> Token {
        _ => |_| Token::A,
    }};

    let toks = wildcard("abc").into_token_vec();
    assert_eq!(toks, vec![A, A, A]);
}

#[test]
fn can_use_buf_in_sub_rule() {
    lex_rule!{main -> Token {
        "a" => |_| A,
        "b" => |_, buf| {
            let sub = sub(buf);
            let a = sub.once_token();
            a
        }
    }}

    lex_rule!{sub -> Token {
        "c" => |_| C, 
    }}

    let lexed = main("aabcbc").into_token_vec();
    assert_eq!(lexed, vec![A, A, C, C])
}

#[test]
fn subrule_mutates_same_buf() {
    lex_rule!{main -> Token {
        "a" => |_| A,
        "b" => |_, buf| {
            sub(buf).empty();
            continue
        },
        "c" => |_| C
    }}

    lex_rule!{sub -> () {
        "c" => |_| break,
        _ => |_| continue, 
    }}
    
    let tokens = main("aabaabbacc").into_token_vec();
    assert_eq!(tokens, vec![A, A, C])
}

#[test]
fn can_concatenate_rules() {
    lex_rule!{lex -> Token {
        "a" "b" => |_| A,
        "b" "a" => |_| B,
        _ => |_| C
    }}

    let tokens = lex("abbac").into_token_vec();
    assert_eq!(tokens, vec![A, B, C])
}

#[test]
fn can_use_const_and_static_regex_rules() {
    const A_RULE: &str = "a";
    static B_RULE: &str = "b";

    lex_rule!{lex -> Token {
        A_RULE => |_| A,
        B_RULE => |_| B,
        _ => |_| C
    }}

    let tokens = lex("abc").into_token_vec();
    assert_eq!(tokens, vec![A, B, C])
}

#[test]
fn can_concatenate_string_and_const_rules() {
    const A_RULE: &str = "a";
    const B_RULE: &str = "b";

    lex_rule!{lex -> Token {
        A_RULE "a" "b" => |_| A,
        B_RULE "b" "a" => |_| B,
        _ => |_| C
    }}

    let tokens = lex("aabbbac").into_token_vec();
    assert_eq!(tokens, vec![A, B, C])
}

#[test]
fn lex_params_work_as_expected() {
    lex_rule!{lex -> Token {
        "a" => |_, _, _| A,
        "b" => |_, _| B,
        "c" => |_| C,
        "d" => |a, b, c| {
            let _a: &str = a;
            let _b: parcom::LexBuf = b;
            let _c: parcom::SrcLoc = c;
            break;
        }
    }}

    let tokens = lex("abcd").into_token_vec();
    assert_eq!(tokens, vec![A, B, C])
}

#[test]
fn break_stops_the_lexer() {
    lex_rule!{break_and_continue -> Token {
        "a" => |_| A,
        "b" => |_| break,
    }};

    let toks = break_and_continue("abaaaa").into_token_vec();
    assert_eq!(toks, vec![A]);
}

#[test]
fn continue_skips_the_character() {
    lex_rule!{break_and_continue -> Token {
        "a" => |_| A,
        "b" => |_| continue,
    }};

    let toks = break_and_continue("aba").into_token_vec();
    assert_eq!(toks, vec![A, A]);
}

#[test]
fn eof_detected_in_empty_string() {
    lex_rule!{eof() -> Token {
        eof => |_| Token::Eof
    }};

    let toks = eof("").into_token_vec();
    assert_eq!(toks, vec![Eof]);
}

#[test]
fn stops_at_eof() {
    lex_rule!{eof() -> Token {
        "a" => |_| continue,
        eof => |_| Token::Eof
    }};

    let toks = eof("aaa").into_token_vec();
    assert_eq!(toks, vec![Eof]);
}

#[test]
fn terminate_without_explicit_eof_rule() {
    lex_rule!{eof() -> Token {}};

    let toks = eof("").into_vec();
    assert_eq!(toks, vec![]);
}

#[test]
fn wildcard_does_not_match_eof() {
    lex_rule!{wildcard() -> Token {
        _ => |_| Token::A,
        eof => |_| Token::Eof
    }};

    let toks = wildcard("").into_token_vec();
    assert_eq!(toks, vec![Eof]);
}

#[test]
fn only_single_eof_with_wildcard_rule() {
    lex_rule!{wildcard() -> Token {
        eof => |_| Token::Eof,
        _ => |_| continue,
    }};

    let toks = wildcard("aaaa").into_token_vec();
    assert_eq!(toks, vec![Eof]);
}