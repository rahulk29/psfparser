pub mod data;
pub mod parser;

pub type Result<T> = anyhow::Result<T>;

extern crate pest;
#[macro_use]
extern crate pest_derive;
