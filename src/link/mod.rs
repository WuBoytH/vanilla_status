use super::*;

// mod status;

mod boomerang;

pub fn install() {
    let agent = &mut smashline::Agent::new("link");
    // status::install(agent);
    agent.install();

    boomerang::install();
}