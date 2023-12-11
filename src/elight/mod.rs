mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("elight");
    status::install(agent);
    agent.install();
}