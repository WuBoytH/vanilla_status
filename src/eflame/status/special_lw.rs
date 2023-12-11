use {
    crate::imports::status_imports::*,
    crate::element::status::special_lw::*
};

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, element_special_lw_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, element_special_lw_end_main);
}