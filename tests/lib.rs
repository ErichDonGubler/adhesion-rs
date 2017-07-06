#![feature(fn_traits)]

#[macro_use]
extern crate adhesion;

#[test]
#[should_panic]
fn smoke() {
    contract! {
        fn asdf(asda: bool, stuff: u64) -> bool {
            pre {
                println!("Running pre-condition");
                assert!(stuff < 30);
            }
            body {
                asda
            }
            post(return_value) {
                println!("Running post-condition");
                assert!(return_value == asda);
            }
        }
    }

    let two = 2;
    let huge_number = 64_000;

    asdf(true, huge_number);
}

