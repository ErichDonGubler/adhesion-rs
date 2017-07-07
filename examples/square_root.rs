//! This example is derived from material found [here](https://tour.dlang.org/tour/en/gems/contract-programming).

#[macro_use]
extern crate adhesion;

contract! {
    fn square_root(x: i64) -> i64 {
        pre {
            assert!(x >= 0);
        }
        post(result) {
            assert!((result * result) <= x && (result + 1) * (result + 1) > x);
        }
        body {
            (x as f64).sqrt() as i64
        }
    }
}

fn main() {
    assert!(square_root(0) == 0);
    assert!(square_root(1) == 1);
    assert!(square_root(25) == 5);
}
