extern crate proc_macro;

use proc_macro2::{Group, Ident, TokenStream, TokenTree};

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
            'a'...'z' => rot(c, 'a'),
            'A'...'Z' => rot(c, 'A'),
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
