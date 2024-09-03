use super::*;

mod template;

pub fn install(agent: &mut smashline::Agent) {
    template::install(agent);
}