// samus
mod acmd;
mod frame;
mod status;
mod common;

pub fn install() {
    let agent = &mut smashline::Agent::new("samus");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    common::install(agent);
    agent.install();
}
