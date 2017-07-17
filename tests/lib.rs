#[macro_use]
extern crate adhesion;

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
fn can_omit_post_param() {
    contract! {
        fn omitted() {
            post {

            }
        }
    }
    omitted();

    contract! {
        fn not_omitted() {
            post(stuff) {

            }
        }
    }
    not_omitted();

    contract! {
        fn no_macro_clash() {
            post(def) { // default name of post param in macro

            }
        }
    }
    no_macro_clash();
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
