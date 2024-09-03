use super::*;

mod special_n;
mod special_n_hold;
mod special_n_shoot;
pub mod helper;

pub fn install(agent: &mut smashline::Agent) {
    special_n::install(agent);
    special_n_hold::install(agent);
    special_n_shoot::install(agent);
}