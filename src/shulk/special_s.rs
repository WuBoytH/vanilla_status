#[status_script(agent = "shulk", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_s"),0.0,1.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("special_s"),0.0,1.0,false,0.0,false,false);
    }
    let speed_x = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,speed_y,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,speed_x,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,*ENERGY_MOTION_RESET_TYPE_AIR_TRANS,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP.into(),false.into());
    }
    else {
        if MotionModule::frame(module_accessor) >= 7.9 {
            return L2CValue::I32(0)
        }
    }
    return L2CValue::I32(0)
}
