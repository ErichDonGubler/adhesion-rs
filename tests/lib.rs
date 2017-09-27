#[macro_use]
extern crate adhesion;
#[macro_use]
extern crate galvanic_assert;

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

    struct Counter {
        count: u64
    }

    impl Counter {
        contract! {
            pub fn new() -> Counter {
                body {
                    Counter {
                        count: 0
                    }
                }
            }

            pub fn increment(&mut self) {
                pre {
                    assert!(self.count != u64::max_value(), "cannot increment counter with max value");
                }
                body {
                    self.count += 1;
                }
            }

            pub fn decrement(&mut self) {
                pre {
                    assert!(self.count != 0, "cannot decrement counter with count of 0");
                }
                body {
                    self.count -= 1;
                }
            }

            pub fn borrow_count(&self) -> &u64 {
                body {
                    &self.count
                }
            }

            pub fn consume(self) -> u64 {
                body {
                    self.count
                }
            }
        }
    }

    assert_that!(Counter{ count: u64::max_value() }.increment(), panics);
    assert_that!(Counter{ count: 0 }.decrement(), panics);
    assert!(Counter::new().borrow_count() == &0);
    assert!(Counter::new().consume() == 0);
}

#[test]
fn generics() {
    contract! {
        fn add_together1<T: ::std::ops::Add>(left: T, right: T) -> <T as std::ops::Add>::Output {
            pre {}
            body {
                left + right
            }
            post (def) {}
            double_check {}
        }
    }

    assert!(add_together1(2, 4) == 6, "add impl broken (!?)");

    contract! {
        fn add_together2<T>(left: T, right: T) -> T::Output where T: ::std::ops::Add {
            pre {}
            body {
                left + right
            }
            post (def) {}
            double_check {}
        }
    }

    assert!(add_together2(2, 4) == 6, "add impl broken (!?)");
}

#[test]
fn visibility() {
    contract! {
        fn no_pub() {}
        pub fn just_pub() {}
        pub(crate) fn pub_crate() {}
    }

    no_pub();
    just_pub();
    pub_crate();
}
