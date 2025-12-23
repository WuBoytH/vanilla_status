use super::*;

mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("trail");
    status::install(agent);
    agent.install();
}