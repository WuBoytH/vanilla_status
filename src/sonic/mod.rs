mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("sonic");
    status::install(agent);
    agent.install();
}