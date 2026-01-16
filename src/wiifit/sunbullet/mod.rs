use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("wiifit_sunbullet");
    status::install(agent);
    agent.install();
}