use {
    smash::{
        lua2cpp::{L2CFighterCommon},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::element::status::special_lw::*
};

#[status_script(agent = "eflame", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn eflame_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_main(fighter)
}

#[status_script(agent = "eflame", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn eflame_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_end_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        eflame_special_lw_main, eflame_special_lw_end
    );
}
