use {
    crate::imports::status_imports::*,
    crate::element::status::special_lw_out::*
};

#[status_script(agent = "elight", status = FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_OUT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn elight_special_lw_out_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_out_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        elight_special_lw_out_main
    );
}
