use super::*;

mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("buddy");
    status::install(agent);
    agent.install();
}