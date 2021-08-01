use std::borrow::Cow;

pub mod descriptor;
pub mod error;

pub use error::Result;
pub use rust_unit_macros::test;

pub trait Test {
    fn contains(&self, tag: &'static str) -> bool;
    fn name(&self) -> Cow<'static, str>;
    fn exec(self: Box<Self>) -> Result;
}

impl Test for Box<dyn Test> {
    fn contains(&self, tag: &'static str) -> bool {
        (&**self).contains(tag)
    }

    fn name(&self) -> Cow<'static, str> {
        (&**self).name()
    }

    fn exec(self: Box<Self>) -> Result {
        (*self).exec()
    }
}

pub fn my_runner(tests: &[&dyn Fn() -> Box<dyn Iterator<Item = Box<dyn Test>>>]) {
    let tests = tests
        .iter()
        .flat_map(|&iter| iter())
        .collect::<Vec<Box<dyn Test>>>();

    for test in tests {
        println!("Test: {}", test.name());
    }
}
