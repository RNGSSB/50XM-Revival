// mario
mod acmd;
mod frame;
mod common;

mod pumpwater;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    frame::install(agent);
    common::install(agent);
    pumpwater::install();
    agent.install();
}
