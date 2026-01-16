use super::*;

mod sunbullet;

pub fn install() {
    let agent = &mut smashline::Agent::new("trail");
    agent.install();

    sunbullet::install();
}