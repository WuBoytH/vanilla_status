mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("template");
    status::install(agent);
    agent.install();
}