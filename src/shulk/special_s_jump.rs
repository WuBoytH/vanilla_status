#[status_script(agent = "shulk", status = FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    AttackModule::set_accept_no_lr(module_accessor,0,true);
    ItemModule::set_have_item_visibility(module_accessor,false,0);
    let mut speed_x = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("speed_x"));
    let mut speed_y = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("speed_y"));
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        speed_x = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("air_speed_x")) * PostureModule::lr(module_accessor);
        speed_y = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("air_speed_y"));
    }
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,speed_y,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let jump_accel_y = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("jump_accel_y")) * -1.0;
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,jump_accel_y);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    let jump_speed_y_max = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("jump_speed_y_max"));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,jump_speed_y_max);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,speed_x,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    let control_limit_speed = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("control_limit_speed"));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,control_limit_speed,0.0);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,*ENERGY_MOTION_RESET_TYPE_AIR_TRANS,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_jump_loop as *const () as _))
}

unsafe fn special_s_jump_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}
