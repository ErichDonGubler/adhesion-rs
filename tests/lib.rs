#![feature(fn_traits)]

#[macro_use]
extern crate adhesion;

contract! {
    fn asdf(asda: bool, stuff: u64) -> bool {
        pre {
            // println!("Running pre-condition check");
            assert!(stuff < 30, "pre-condition violation");
        }
        body {
            // println!("Running body");
            asda
        }
        post(return_value) {
            // println!("Running post-condition check");
            assert!(return_value == (stuff % 3 == 0), "post-condition violation");
        }
        invariant {
            // println!("Running invariant check");
            assert!(stuff > 5, "invariant violation");
        }
    }
}

#[test]
#[should_panic]
fn pre_failure() {
    asdf(true, 64);
}

#[test]
#[should_panic]
fn invariant_failure() {
    asdf(false, 3);
}


#[test]
#[should_panic]
fn post_failure() {
    asdf(true, 7);
}

#[test]
fn no_failures() {
    asdf(true, 6);
    asdf(false, 7);
    asdf(false, 11);
    asdf(true, 24);
}

