// fludd projectile
mod acmd;


pub fn install() {
    let agent = &mut smashline::Agent::new("ness_yoyohead");
    acmd::install(agent);
    agent.install();
}
