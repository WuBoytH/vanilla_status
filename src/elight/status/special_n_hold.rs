#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n_hold")),L2CValue::new_int(hash40("special_air_n_hold")),L2CValue::new_bool(false));
    WorkModule::set_int(module_accessor,0,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_HOLD_FRAME);
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_hold_main_loop as *const () as _))
}

unsafe fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) == false {
        if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n_hold")),L2CValue::new_int(hash40("special_air_n_hold")),L2CValue::new_bool(true));
            special_n_kinetics_setup(fighter);
            WorkModule::count_down_int(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_VOICE_PLAY_FRAME,0);
            return L2CValue::I32(0)
        }
    }
    fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END.into(),false.into());
    return L2CValue::I32(0)
}
