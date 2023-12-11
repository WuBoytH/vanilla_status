mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("ike");
    status::install(agent);
    agent.install();
}