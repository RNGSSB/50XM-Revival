#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

#![allow(warnings)] 

mod mario;
mod fox;
mod falcon;
mod sheik;
mod marth;
mod roy;
mod wolf;
mod custom;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    mario::install();
    fox::install();
    falcon::install();
    marth::install();
    roy::install();
    sheik::install();
    wolf::install();
    custom::install();
}