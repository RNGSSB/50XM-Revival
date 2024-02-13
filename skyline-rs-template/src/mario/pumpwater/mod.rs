// fludd projectile
mod acmd;


pub fn install() {
    let agent = &mut smashline::Agent::new("mario_pumpwater");
    acmd::install(agent);
    agent.install();
}
