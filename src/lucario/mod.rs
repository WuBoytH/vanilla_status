use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("ike");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}