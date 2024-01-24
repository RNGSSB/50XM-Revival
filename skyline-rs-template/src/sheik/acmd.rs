mod aerials;
mod throws;
mod ground;
mod specials;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    throws::install(agent);
    ground::install(agent);
    specials::install(agent);
}