// mario
mod acmd;
mod frame;
mod common;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    frame::install(agent);
    common::install(agent);
    agent.install();
}
