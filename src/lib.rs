extern crate proc_macro;

use std::mem;

use proc_macro2::{Group, Ident, Punct, Spacing, TokenStream, TokenTree};

/// Applies Caesar chipher to the input source code.
#[proc_macro]
pub fn caesar(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let output: TokenStream = decode_stream(input);

    proc_macro::TokenStream::from(output)
}

fn decode_stream(ts: TokenStream) -> TokenStream {
    ts.into_iter().map(decode_tree).collect()
}

fn decode_tree(tt: TokenTree) -> TokenTree {
    match tt {
        TokenTree::Group(g) => {
            let mut dg = Group::new(g.delimiter(), decode_stream(g.stream()));
            dg.set_span(g.span());
            TokenTree::Group(dg)
        }
        TokenTree::Ident(i) => {
            let gi = Ident::new(&caesar_decode(&i.to_string()), i.span());
            TokenTree::Ident(gi)
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => tt,
    }
}

fn caesar_decode(s: &str) -> String {
    return s.chars().map(decode_char).collect();

    fn decode_char(c: char) -> char {
        match c {
            'a'..='z' => rot(c, 'a'),
            'A'..='Z' => rot(c, 'A'),
            _ => c,
        }
    }

    fn rot(c: char, base: char) -> char {
        std::char::from_u32(((c as u32 - base as u32) + 13) % 26 + base as u32).unwrap()
    }
}

#[test]
fn decoding_works() {
    assert_eq!(
        &caesar_decode("Ornhgvshy vf orggre guna htyl."),
        "Beautiful is better than ugly."
    )
}

#[proc_macro]
pub fn mirror(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let output: TokenStream = mirror_stream(input);
    proc_macro::TokenStream::from(output)
}

fn mirror_stream(ts: TokenStream) -> TokenStream {
    let mut spacing = Spacing::Alone;
    ts.into_iter()
        .map(|tt| match tt {
            TokenTree::Group(g) => {
                let mut mg = Group::new(g.delimiter(), mirror_stream(g.stream()));
                mg.set_span(g.span());
                TokenTree::Group(mg)
            }
            TokenTree::Punct(p) => {
                let c = mirror_char(p.as_char());
                let spacing = mem::replace(&mut spacing, p.spacing());
                let mut mp = Punct::new(c, spacing);
                mp.set_span(p.span());
                TokenTree::Punct(mp)
            }
            TokenTree::Ident(..) | TokenTree::Literal(..) => tt,
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn mirror_char(c: char) -> char {
    match c {
        '<' => '>',
        '>' => '<',
        _ => c,
    }
}
