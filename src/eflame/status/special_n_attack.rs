unsafe fn special_n_kinetics_setup(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.module_accessor;
    let speed_x = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    let mut unk1 = 0.0;
    let mut unk2 = 0.0;
    let mut unk3 = 0.0;
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
        unk1 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x186d9e2da0u64);
        unk2 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x22f517d47cu64);
        unk3 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x16c9d2bc49u64);
        let unk4 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x16e109c926u64) * -1.0;
        let unk5 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x1ae9a361a9u64);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,unk4);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,unk5);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    }
    else {
        unk1 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x1884fd8895u64);
        unk2 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x221c747149u64);
        unk3 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x1620b1197cu64);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    }
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    if unk2 > speed_x {
        unk2 = speed_x;
    }
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,unk2,0.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,unk3,0.0);
    sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,speed_x,0.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,unk1,0.0);
    sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);
}

#[status_script(agent = "eflame", status = FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    match WorkModule::get_int(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM) {
        2 => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n2")),L2CValue::new_int(hash40("special_air_n2")),L2CValue::new_bool(false)),
        3 => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n3")),L2CValue::new_int(hash40("special_air_n3")),L2CValue::new_bool(false)),
        _ => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n1")),L2CValue::new_int(hash40("special_air_n1")),L2CValue::new_bool(false)),
    };
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    special_n_kinetics_setup(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_attack_main_loop as *const () as _))
}

unsafe fn special_n_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_END.into(),false.into());
    }
    let unk1 = match WorkModule::get_int(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM) {
        2 => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n2")),L2CValue::new_int(hash40("special_air_n2")),L2CValue::new_bool(true)),
        3 => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n3")),L2CValue::new_int(hash40("special_air_n3")),L2CValue::new_bool(true)),
        _ => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n1")),L2CValue::new_int(hash40("special_air_n1")),L2CValue::new_bool(true)),
    };
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    let mut unk = 0.0;
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        unk = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x22f517d47cu64);
    }
    else {
        unk = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x221c747149u64);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,unk,0.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    if unk1.get_bool() {
        special_n_kinetics_setup(fighter);
    }
    return L2CValue::I32(0)
}
