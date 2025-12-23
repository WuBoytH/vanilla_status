use super::*;

mod turn;

pub fn install(agent: &mut Agent) {
    turn::install(agent);
}
