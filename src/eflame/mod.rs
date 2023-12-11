mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("eflame");
    status::install(agent);
    agent.install();
}