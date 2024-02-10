// samus charge shot
mod acmd;


pub fn install() {
    let agent = &mut smashline::Agent::new("samus_cshot");
    acmd::install(agent);
    agent.install();
}
