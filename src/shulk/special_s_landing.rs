#[status_script(agent = "shulk", status = FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let speed_x = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,*ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    MotionModule::change_motion(module_accessor,Hash40::new("special_s_landing"),0.0,1.0,false,0.0,false,0.0,0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_landing_loop as *const () as _))
}

unsafe fn special_s_landing_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if CancelModule::is_enable_cancel(module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    else {
        if MotionModule::is_end(module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
}
