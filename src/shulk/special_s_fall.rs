#[status_script(agent = "shulk", status = FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    AttackModule::set_accept_no_lr(module_accessor,0,true);
    let speed_x = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,speed_x,0.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,speed_y);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    let cut_fall_accel_y = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("cut_fall_accel_y")) * -1.0;
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,cut_fall_accel_y);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    let cut_fall_speed_y_max = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("cut_fall_speed_y_max"));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,cut_fall_speed_y_max);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,*ENERGY_MOTION_RESET_TYPE_AIR_TRANS,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    MotionModule::change_motion(module_accessor,Hash40::new("special_air_s_fall"),0.0,1.0,false,0.0,false,0.0,0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_fall_loop as *const () as _))
}

unsafe fn special_s_fall_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        let cut_fall_accel_y = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("cut_fall_accel_y")) * -1.0;
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,cut_fall_accel_y);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        let cut_fall_speed_y_max = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("cut_fall_speed_y_max"));
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,cut_fall_speed_y_max);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        let control_limit_speed = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("control_limit_speed"));
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,control_limit_speed,0.0);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING.into(),false.into());
        }
        return L2CValue::I32(0)
    }
    else {
        return L2CValue::I32(1)
    }
}
