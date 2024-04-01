// ness
mod acmd;
mod frame;
mod status;
mod common;

mod ness_yoyohead;

pub fn install() {
    let agent = &mut smashline::Agent::new("ness");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    common::install(agent);
    ness_yoyohead::install();
    agent.install();
}
