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
mod mythra;
mod pyra;
mod cloud;
mod incineroar;
mod sephiroth;
mod samus;
mod pikachu;
mod greninja;
mod mewtwo;
mod byleth;
mod bowser;
mod ness;
mod falco;
mod lucario;
mod luigi;
mod sora;
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
    mythra::install();
    pyra::install();
    cloud::install();
    incineroar::install();
    sephiroth::install();
    samus::install();
    pikachu::install();
    greninja::install();
    mewtwo::install();
    byleth::install();
    bowser::install();
    ness::install();
    falco::install();
    lucario::install();
    luigi::install();
    custom::install();
    sora::install();
}