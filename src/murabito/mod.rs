use super::*;

mod status;

mod clayrocket;

pub fn install() {
    let agent =  &mut Agent::new("murabito");
    status::install(agent);
    agent.install();

    clayrocket::install();
}