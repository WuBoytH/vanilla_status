// use super::*;

// mod sunbullet;

pub fn install() {
    let agent = &mut smashline::Agent::new("wiifit");
    agent.install();

    // sunbullet::install();
}