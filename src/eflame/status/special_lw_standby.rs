use {
    crate::imports::status_imports::*,
    crate::element::status::special_lw_standby::*
};

#[status_script(agent = "eflame", status = FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn eflame_special_lw_standby_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_standby_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        eflame_special_lw_standby_main
    );
}
