mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("edge");
    status::install(agent);
    agent.install();
}