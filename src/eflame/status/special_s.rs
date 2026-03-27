#[status_script(agent = "eflame", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
    if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FLICK) == false {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s")),L2CValue::new_int(hash40("special_air_s")),L2CValue::new_bool(false));
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_flick")),L2CValue::new_int(hash40("special_air_s_flick")),L2CValue::new_bool(false));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if CancelModule::is_enable_cancel(module_accessor) == false
    || (fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() == false) {
        fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
        if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FLICK) == false {
            fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s")),L2CValue::new_int(hash40("special_air_s")),L2CValue::new_bool(true));
        }
        else {
            fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_flick")),L2CValue::new_int(hash40("special_air_s_flick")),L2CValue::new_bool(true));
        }
        if MotionModule::is_end(module_accessor) {
            if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
            }
        }
    }
    else {
        if fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool()
        && fighter.sub_air_check_fall_common().get_bool() {
            return L2CValue::I32(1)
        }
    }
    return L2CValue::I32(0)
}
