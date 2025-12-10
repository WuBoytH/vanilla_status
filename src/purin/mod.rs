use super::*;

mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("purin");
    status::install(agent);
    agent.install();
}