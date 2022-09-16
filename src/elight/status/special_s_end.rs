#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_end")),L2CValue::new_int(hash40("special_air_s_end")),L2CValue::new_bool(false));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_end_main_loop as *const () as _))
}

unsafe fn special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if (CancelModule::is_enable_cancel(module_accessor) == false
    || (fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool()))
    && fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if MotionModule::is_end(module_accessor) == false {
            if fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND
            && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                let frame = MotionModule::frame(module_accessor);
                let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(module_accessor,Hash40::new("special_s_end"),true);
                if frame >=  cancel_frame {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
                }
                SoundModule::play_landing_se(module_accessor,Hash40::new("se_elight_landing01"));
            }
            fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_end")),L2CValue::new_int(hash40("special_air_s_end")),L2CValue::new_bool(true));
            fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
            fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
        }
        else {
            if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
            }
        }
    }
    return L2CValue::I32(0)
}
