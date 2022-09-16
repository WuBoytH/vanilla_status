#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_start")),L2CValue::new_int(hash40("special_air_s_start")),L2CValue::new_bool(false));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
    WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF);
    WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) == false {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_start")),L2CValue::new_int(hash40("special_air_s_start")),L2CValue::new_bool(true));
        fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
        fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
        if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF) {
            WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
            WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
            WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
            WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
            WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF);
        }
    }
    else {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD.into(),false.into());
    }
    return L2CValue::I32(0)
}
