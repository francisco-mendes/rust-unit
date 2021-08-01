#![feature(try_blocks)]
#![feature(custom_test_frameworks)]
#![test_runner(rust_unit::my_runner)]

#[cfg(test)]
mod tests {
    use rust_unit::test;

    fn data_source() -> impl Iterator<Item = (i32, i32, bool)> {
        vec![
            (1, 1, true),
            (1, 2, false),
            (0, 0, true),
            (-1, -1, true),
            (-1, -2, false),
        ]
        .into_iter()
    }

    #[test("uwu")]
    #[source(data_source)]
    fn it_works(a: i32, b: i32, res: bool) -> rust_unit::Result {
        try {
            assert_eq!(a == b, res);
        }
    }

    #[test("{} equals {} is {}", a, b, res)]
    #[tags("slow", "fast", "uwu")]
    #[source(data_source)]
    fn it_works2(a: i32, b: i32, res: bool) -> rust_unit::Result {
        try {
            assert_eq!(a == b, res);
        }
    }
}
