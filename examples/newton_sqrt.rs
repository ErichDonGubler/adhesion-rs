//! This example demonstrates a great place to use contract programming: math
//! libraries! Let's say that we want to use our possibly-less-accurate, but
//! faster, algorithm for generating square roots using the Newton-Raphson
//! method (see [here](https://en.wikipedia.org/wiki/Newton%27s_method#Square_root_of_a_number)
//! for reference)

#[macro_use]
extern crate adhesion;

trait WithinPrecisionOf {
    fn is_within_precision_of(self, target: Self, tolerance: Self) -> bool;
}

impl WithinPrecisionOf for f64 {
    fn is_within_precision_of(self, target: Self, tolerance: Self) -> bool {
        return (self - target).abs() < tolerance.abs();
    }
}

contract! {
    fn newton_sqrt(x: f64, initial_guess: f64, precision: f64) -> f64 {
        // We bind the return result of this function here, so we we can use
        // it in the postcondition.
        post(y) {
            // It's not trivial to FIND the square root, but it's easy to
            // verify...so let's write our postcondition first.
            assert!(x.is_within_precision_of(y * y, precision), "cannot square y to get x");
        }
        // Note how it doesn't matter what order we write these blocks in.
        pre {
            // While we're thinking about it, there are some inputs we should
            // NEVER accept in this method...for instance, trying to get the
            // square root of a negative number requires more imagination than
            // we're willing to implement here, so let's disallow floats as
            // part of our contract.
            assert!(!(x < 0_f64), "cannot compute root of negative number x");
        }
        body {
            if x == 0_f64 {
                return 0_f64;
            }

            // Now, we can just focus on the algorithm here.
            println!("Finding square root of {} at precision of {}; initial guess {}", x, precision, initial_guess);
            let mut current;
            let mut previous = initial_guess;

            let mut i = 1;
            loop {
                current = previous - (previous * previous - x) / (2_f64 * previous);
                println!("  Iteration {}: {}", i, current);

                if current.is_nan() || current.is_within_precision_of(previous, precision) {
                    break;
                }

                previous = current;

                i += 1;
            }
            current
        }
    }
}

fn main() {
    let tolerance = 0.000_000_2_f64;
    let assert_root_close_to = |square, initial_guess, root| assert!(newton_sqrt(square, initial_guess, tolerance).is_within_precision_of(root, tolerance));

    assert_root_close_to(0_f64, 0_f64, 0_f64);
    assert_root_close_to(4_f64, 3.5_f64, 2_f64);
    assert_root_close_to(9_f64, 2_f64, 3_f64);
    assert_root_close_to(25_f64, 8_f64, 5_f64);
    assert_root_close_to(612_f64, 10_f64, 24.738_633_753_f64);
}
