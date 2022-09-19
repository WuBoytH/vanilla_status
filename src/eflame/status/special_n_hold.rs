#[status_script(agent = "eflame", status = FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n_hold")),L2CValue::new_int(hash40("special_air_n_hold")),L2CValue::new_bool(false));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_CHANGED);
    WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_REQUEST);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_hold_main_loop as *const () as _))
}

unsafe fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let keep_frame_max = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("keep_frame_max")) as f32;
    let mut unk = false;
    if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == false {
        unk = true;
    }
    if fighter.global_table[0xe].get_f32() >= keep_frame_max {
        unk = true;
    }
    if unk == false {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n_hold")),L2CValue::new_int(hash40("special_air_n_hold")),L2CValue::new_bool(true));
        fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
        fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_n")));
        if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_CHANGED) == false {
            if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_REQUEST) {
                if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                    let hold_end_accel_y_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("hold_end_accel_y_mul"));
                    let hold_end_max_speed_y_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("hold_end_max_speed_y_mul"));
                    let mut air_accel_y = WorkModule::get_param_float(module_accessor,hash40("air_accel_y"),0) * -1.0;
                    if hold_end_accel_y_mul != 0.0 {
                        air_accel_y *= hold_end_accel_y_mul;
                    }
                    let mut air_speed_y_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_y_stable"),0);
                    if hold_end_max_speed_y_mul != 0.0 {
                        air_speed_y_stable *= hold_end_max_speed_y_mul;
                    }
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_accel_y);
                    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_speed_y_stable);
                    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
                    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
                    WorkModule::on_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_CHANGED);
                    WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_REQUEST);

                }
            }
        }
    }
    else {
        let keep_frame_2rot = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("keep_frame_2rot")) as f32;
        let keep_frame_3rot = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("keep_frame_3rot")) as f32;
        if fighter.global_table[0xe].get_f32() < keep_frame_3rot {
            if fighter.global_table[0xe].get_f32() < keep_frame_2rot {
                WorkModule::set_int(module_accessor,1,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
            }
            else {
                WorkModule::set_int(module_accessor,2,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
            }
        }
        else {
            WorkModule::set_int(module_accessor,3,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
        }
        let ratio = fighter.global_table[0xe].get_f32()/keep_frame_max;
        let finish_attack_power_rate = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("finish_attack_power_rate"));
        let lerp_num = fighter.lerp(L2CValue::new_num(1.0),L2CValue::new_num(finish_attack_power_rate),L2CValue::new_num(ratio)).get_f32() + 0.0;
        WorkModule::set_float(module_accessor,lerp_num,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLOAT_ATTACK_MUL);
        fighter.change_status(FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_ATTACK.into(),false.into());
    }
    return L2CValue::I32(0)
}
