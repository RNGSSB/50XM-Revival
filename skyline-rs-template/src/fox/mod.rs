// fox
mod acmd;
mod frame;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("fox");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent.install();
}
