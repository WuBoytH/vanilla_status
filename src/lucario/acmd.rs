mod aura;

pub fn install(agent: &mut smashline::Agent) {
    aura::install(agent);
}