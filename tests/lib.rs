#[macro_use]
extern crate adhesion;

macro_rules! assert_panic {
    ($e: expr) => {
        let result = ::std::panic::catch_unwind(|| $e);
        assert!(result.is_err());
    }
}

#[test]
fn most_basic() {
    contract! {
        fn asdf() {
            body {
                ()
            }
        }
    }

    asdf();
}

#[test]
fn happy_path() {
    contract! {
        fn asdf(asda: bool, mut stuff: u64) -> bool {
            pre {
                assert!(stuff < 30, "pre-condition violation");
            }
            body {
                stuff += 0;
                asda
            }
            post(return_value) {
                assert!(return_value == (stuff % 3 == 0), "post-condition violation");
            }
            invariant {
                assert!(stuff > 5, "invariant violation");
            }
        }
    }

    assert_panic!(asdf(true, 7)); // post failure
    assert_panic!(asdf(true, 64)); // pre failure
    assert_panic!(asdf(false, 3)); // invariant failure
    asdf(true, 6);
    asdf(false, 7);
    asdf(false, 11);
    asdf(true, 24);
}

#[test]
fn ordering_doesnt_matter() {
    contract! {
        fn sqrt(x: f64) -> f64 {
            body {
                x.sqrt()
            }
            post(y) {
                assert!(x == y * y, "wat!");
            }
        }
    }

    sqrt(25_f64);
}

#[test]
fn attributes() {
    contract! {
        #[cold]
        fn before() {
            body {}
        }
    }

    before();

    contract! {
        fn after() {
            #![cold]
            body {}
        }
    }

    after();
}
