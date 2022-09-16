use {
    smash::{
        lua2cpp::{L2CFighterCommon},
        hash40,
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*
};

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn elight_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF);
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF);
        }
    }
    else {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        elight_special_s_main
    );
}
