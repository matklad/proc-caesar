# proc-caesar

A proc-macro to apply Caesar cipher to Rust source code.

Example:

```rust
caesar! {

sa pnrfne_qrpbqr(f: &fge) -> Fgevat {
    erghea f.punef().znc(qrpbqr_pune).pbyyrpg();

    sa qrpbqr_pune(p: pune) -> pune {
        zngpu p {
            'a'...'z' => ebg(p, 'a'),
            'A'...'Z' => ebg(p, 'A'),
            _ => p,
        }
    }

    sa ebg(p: pune, onfr: pune) -> pune {
        fgq::pune::sebz_h32(((p nf h32 - onfr nf h32) + 13) % 26 + onfr nf h32).hajenc()
    }
}

}

#[test]
fn decoding_works() {
    assert_eq!(
        &caesar_decode("Ornhgvshy vf orggre guna htyl."),
        "Beautiful is better than ugly."
    )
}
```

# Why?

To break code completion. Seriously, the single reason this crate exists is to
break code completion in IDEs.

More specifically, this is a simple demonstration that it is in general
impossible to provide IDE features for code *inside* macro invocations.

Consider this snippet of code:

```rust
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

fn main() {
    let entry = HASHMAP.get(&0).unwrap();
    println!("The entry for `0` is \"{}\".", entry);
}
```

"IDE support for macros" here means two things:

* inside `main`, IDE should complete `HASHMAP`
* inside `lazy_static!`, IDE should complete `HashMap`

The first case is "easy": IDE needs to "just" expand the macro and process the
resulting code as usual.

The second case is, as proven by proc-caesar macro, is impossible to handle in
general: connection between the tokens after the bang and the expanded code
might be arbitrary complex. To provide correct completions for code inside
`caesar!` macro, IDE would have to figure out the inverse transformation of
identifiers!


# Future Work

Currently the crate uses a particularly weak cipher, so a sufficiently smart
AI-powered IDE could in theory figure out the inverse transformation required
for completions. To make this macro AI-proof, we need to apply public-key
cryptography, to make sure that computing the inverse transformation is
computationally infeasible.
