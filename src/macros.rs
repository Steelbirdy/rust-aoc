macro_rules! samples {
    ($($inp:literal => $oup:literal),* $(,)?) => {
        &[$(crate::Sample { input: $inp, expected_output: $oup }),*]
    };
}

macro_rules! solution {
    ($T:ident, $day:literal, $generator:ident, $part1:ident, $part1_checks:ident, $part2:ident, $part2_checks:ident) => {
        mod __solution {
            use std::any::Any;

            pub fn generator(input: &str) -> Box<dyn Any> {
                Box::new((super::$generator)(input))
            }

            pub fn part1(input: &dyn Any) -> String {
                let input = input.downcast_ref::<super::$T>().unwrap();
                ToString::to_string(&(super::$part1)(input))
            }

            pub fn part1_checks() {
                for &crate::Sample { input: raw_inp, expected_output: expected_oup } in super::$part1_checks {
                    let inp = (super::$generator)(raw_inp);
                    let oup = ToString::to_string(&(super::$part1)(&inp));
                    if oup != ToString::to_string(&expected_oup) {
                        panic!("Check for Day {} Part 1 failed on input:\n{raw_inp}\nGot: {oup}\nExpected: {expected_oup}", $day);
                    }
                }
            }

            pub fn part2(input: &dyn Any) -> String {
                let input = input.downcast_ref::<super::$T>().unwrap();
                ToString::to_string(&(super::$part2)(input))
            }

            pub fn part2_checks() {
                for &crate::Sample { input: raw_inp, expected_output: expected_oup } in super::$part2_checks {
                    let inp = (super::$generator)(raw_inp);
                    let oup = ToString::to_string(&(super::$part2)(&inp));
                    if oup != ToString::to_string(&expected_oup) {
                        panic!("Check for Day {} Part 2 failed on input:\n{raw_inp}\n  Got: {oup}\n  Expected: {expected_oup}", $day);
                    }
                }
            }
        }

        inventory::submit!($crate::Solution {
            day: $day,
            generator: __solution::generator,
            part1: __solution::part1,
            part1_checks: __solution::part1_checks,
            part2: __solution::part2,
            part2_checks: __solution::part2_checks,
        });
    };
}

pub(crate) use {samples, solution};
