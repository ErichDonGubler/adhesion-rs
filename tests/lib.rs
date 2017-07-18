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

#[test]
#[allow(unused)]
fn mutability() {
    contract! {
        #[allow(unused)]
        fn mutable(mut i_am_mutable: String) -> String {
            body {
                "wut".to_string()
            }
        }
    }

    contract! {
        #[allow(unused)]
        fn mutable_multiple1(mut stuff: u32, mut things: u64) {}
    }

    contract! {
        #[allow(unused)]
        fn mutable_mixed_first(mut stuff: String, things: bool) {}
    }

    contract! {
        #[allow(unused)]
        fn mutable_mixed_second(stuff: f64, things: ()) {}
    }
}

#[test]
fn structs() {
    struct TestStruct {
        var: u32
    }

    impl TestStruct {
        contract! {
            fn eat(self) -> u32 {
                body {
                    self.var
                }
            }
        }

        contract! {
            fn change(&mut self, value: u32) {
                body {
                    self.var = value;
                }
            }
        }

        contract! {
            fn check_out(&self) -> u32 {
                body {
                    self.var
                }
            }
        }
    }

    assert!(TestStruct { var: 4 }.eat() == 4, "var not properly extracted from TestStruct");
    {
        let mut t = TestStruct { var: 4 };
        t.change(26);
        assert!(t.var == 26, "var not mutated in TestStruct");
    }
    {
        let t = TestStruct { var: 3 };
        assert!(t.check_out() == 3, "var not properly read from TestStruct");
        assert!(t.eat() == 3, "var not properly extracted from TestStruct");
    }
}
