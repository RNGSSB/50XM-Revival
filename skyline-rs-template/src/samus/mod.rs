// samus
mod acmd;
mod frame;
mod status;
mod common;

mod cshot;

pub fn install() {
    let agent = &mut smashline::Agent::new("samus");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    common::install(agent);
    cshot::install();
    agent.install();
}
