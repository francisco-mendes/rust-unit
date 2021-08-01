use std::{
    borrow::Cow,
    collections::HashSet,
};

use crate::{
    Result,
    Test,
    TestName,
};

pub struct SimpleTest {
    name: TestName<()>,
    tags: HashSet<&'static str>,
    func: fn() -> Result,
}

impl SimpleTest {
    pub fn new(name: TestName<()>, tags: &[&'static str], func: fn() -> Result) -> Box<dyn Test> {
        Box::new(Self {
            name,
            tags: tags.iter().copied().collect(),
            func,
        })
    }
}

impl Test for SimpleTest {
    fn contains(&self, tag: &'static str) -> bool {
        self.tags.contains(tag)
    }

    fn name(&self) -> Cow<'static, str> {
        match &self.name {
            TestName::Static(s) => Cow::from(*s),
            TestName::Dynamic(f) => Cow::from(f(&())),
        }
    }

    fn exec(self: Box<Self>) -> Result {
        (self.func)()
    }
}
