#[macro_export]
macro_rules! test_day {
    ($day:ident) => {
        mod $day {
            $crate::test_version!(
                stringify!($day).trim_start_matches("day").parse().unwrap();
                version01: sample, real;
                version02: sample, real
            );
        }
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! test_version {
    ($day:expr ; $($version:ident: $($input:ident),*);*) => {
        $(
            mod $version {
                use crate::*;
                $(
                    #[test]
                    fn $input() -> Result<(), String> {
                        let runner = create_runner();
                        runner.verify_solution(
                            $day,
                            stringify!($version).trim_start_matches("version").parse().unwrap(),
                            stringify!($input) == "sample"
                        )
                    }
                )*
            }
        )*
    };
}
