pub mod parser;
pub mod data;

pub type Result<T> = anyhow::Result<T>;

extern crate pest;
#[macro_use]
extern crate pest_derive;

