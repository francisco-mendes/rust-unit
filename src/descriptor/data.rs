use std::{
    borrow::Cow,
    collections::HashSet,
    sync::Arc,
};

use crate::{
    Result,
    Test,
    TestName,
};

pub struct DataTestCommon<Data, F>
where
    Data: 'static,
    F: Fn(Data) -> Result + 'static,
{
    name: TestName<Data>,
    tags: HashSet<&'static str>,
    func: F,
}

pub struct DataTest<Data, F>
where
    Data: 'static,
    F: Fn(Data) -> Result + 'static,
{
    common: Arc<DataTestCommon<Data, F>>,
    args: Data,
}

impl<Data, F> DataTestCommon<Data, F>
where
    Data: 'static,
    F: Fn(Data) -> Result + 'static,
{
    pub fn new(name: TestName<Data>, tags: &[&'static str], func: F) -> Arc<Self> {
        Arc::new(Self {
            name,
            tags: tags.iter().copied().collect(),
            func,
        })
    }
}

impl<Data, F> DataTest<Data, F>
where
    Data: 'static,
    F: Fn(Data) -> Result + 'static,
{
    pub fn new(common: Arc<DataTestCommon<Data, F>>, args: Data) -> Box<dyn Test> {
        Box::new(Self { common, args })
    }
}

impl<Data, F> Test for DataTest<Data, F>
where
    Data: 'static,
    F: Fn(Data) -> Result + 'static,
{
    fn contains(&self, tag: &'static str) -> bool {
        self.common.tags.contains(tag)
    }

    fn name(&self) -> Cow<'static, str> {
        match &self.common.name {
            TestName::Static(s) => Cow::from(*s),
            TestName::Dynamic(f) => Cow::from(f(&self.args)),
        }
    }

    fn exec(self: Box<Self>) -> Result {
        (self.common.func)(self.args)
    }
}
