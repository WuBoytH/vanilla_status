unsafe fn kinetic_setup(fighter: &mut L2CFighterCommon, unk: bool) {
    let module_accessor = fighter.module_accessor;
    let unk2 = fighter.sub_change_kinetic_type_by_situation(L2CValue::I32(*FIGHTER_KINETIC_TYPE_MOTION_AIR),L2CValue::I32(*FIGHTER_KINETIC_TYPE_MOTION_AIR));
    if unk == false {
        if unk2 == false {
            return;
        }
    }
    else {
        let mut speed_y_mul_height = 0.0;
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
            speed_y_mul_height = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("speed_y_mul_height_g"));
        }
        else {
            speed_y_mul_height = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("speed_y_mul_height_a"));
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.0,0.0);
            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,0.0,0.0);
            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,speed_y_mul_height);
        sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
        let rise_speed_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("rise_speed_x_mul"));
        let air_accel_x_mul = WorkModule::get_param_float(module_accessor,hash40("air_accel_x_mul"),0);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,rise_speed_x_mul * air_accel_x_mul);
        sv_kinetic_energy::controller_set_accel_x_mul(fighter.lua_state_agent);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        StatusModule::set_keep_situation_air(module_accessor,true);
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        let rise_max_speed_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("rise_max_speed_x_mul"));
        let air_speed_x_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_x_stable"),0);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,rise_max_speed_x_mul * air_speed_x_stable,0.0);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[status_script(agent = "demon", status = FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_rise_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    WorkModule::off_flag(module_accessor,*FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_AIR);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::on_flag(module_accessor,*FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_AIR);
    }
    MotionModule::change_motion(module_accessor,Hash40::new("special_hi"),0.0,1.0,false,0.0,false,false);
    GroundModule::set_passable_check(module_accessor,false);
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    kinetic_setup(fighter,true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_rise_main_loop as *const () as _))
}

unsafe fn special_hi_rise_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if CancelModule::is_enable_cancel(module_accessor) == false
    || fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool() == false {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return L2CValue::I32(0)
        }
        let landing_frame = WorkModule::get_param_int(module_accessor,hash40("param_special_hi"),hash40("landing_frame")) as f32;
        if landing_frame > fighter.global_table[0xe].get_f32()
        && fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
            StatusModule::set_keep_situation_air(module_accessor,true);
            if MotionModule::is_end(module_accessor) == false {
                fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
                kinetic_setup(fighter,false);
                return L2CValue::I32(0)
            }
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_FALL.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}
