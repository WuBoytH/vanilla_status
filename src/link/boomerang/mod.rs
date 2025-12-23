use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("link_boomerang");
    status::install(agent);
    agent.install();
}