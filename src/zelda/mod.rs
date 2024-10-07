use super::*;

mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("zelda");
    status::install(agent);
    agent.install();
}