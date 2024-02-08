// sephiroth
mod acmd;
mod frame;
mod status;
mod common;

pub fn install() {
    let agent = &mut smashline::Agent::new("edge");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    common::install(agent);
    agent.install();
}
