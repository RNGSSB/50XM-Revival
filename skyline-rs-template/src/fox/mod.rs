// fox
mod acmd;
mod frame;

pub fn install() {
    let agent = &mut smashline::Agent::new("fox");
    acmd::install(agent);
    frame::install(agent);
    agent.install();
}
