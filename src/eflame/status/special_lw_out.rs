use super::*;
use crate::element::status::special_lw_out::*;

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_OUT, element_special_lw_out_main);
}