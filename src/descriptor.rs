pub mod data;
pub mod simple;

pub enum TestName<Data: 'static> {
    Static(&'static str),
    Dynamic(Box<dyn Fn(&Data) -> String>),
}
