#[status_script(agent = "demon", status = FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let stable_speed_x = sv_kinetic_energy::get_stable_speed_x(fighter.lua_state_agent);
    let limit_speed_x = sv_kinetic_energy::get_limit_speed_x(fighter.lua_state_agent);
    let fall_speed_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("fall_speed_x_mul"));
    let air_accel_x_mul = WorkModule::get_param_float(module_accessor,hash40("air_accel_x_mul"),0);
    fighter.clear_lua_stack();
    lua_args!(fighter,fall_speed_x_mul * air_accel_x_mul);
    sv_kinetic_energy::controller_set_accel_x_mul(fighter.lua_state_agent);
    let fall_max_speed_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("fall_max_speed_x_mul"));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,fall_max_speed_x_mul * stable_speed_x);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,fall_max_speed_x_mul * limit_speed_x);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    fighter.status_Fall()
}
