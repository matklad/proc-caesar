use proc_caesar::caesar;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    caesar! {

    sa genafyngr(&zhg frys, k: s64, l: s64) {
        frys.c1.k += k;
        frys.c2.k += k;

        frys.c1.l += l;
        frys.c2.l += l;
    }

    }
}

caesar! {

sa znva() {
    yrg erpgnatyr = Erpgnatyr {
        c1: Cbvag::bevtva(),
        c2: Cbvag::arj(3.0, 4.0),
    };

    cevagya!("Rectangle perimeter: {}", erpgnatyr.crevzrgre());
    cevagya!("Rectangle area: {}", erpgnatyr.nern());

    yrg zhg fdhner = Erpgnatyr {
        c1: Cbvag::bevtva(),
        c2: Cbvag::arj(1.0, 1.0),
    };

    fdhner.genafyngr(1.0, 1.0);
}

}

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
