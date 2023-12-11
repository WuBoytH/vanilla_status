use {
    crate::imports::status_imports::*,
    crate::element::status::special_lw_standby::*
};

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY, element_special_lw_standby_main);
}