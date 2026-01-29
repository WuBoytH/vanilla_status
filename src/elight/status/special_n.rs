unsafe fn special_n_kinetics_setup(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.module_accessor;
    fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_n")));
    if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_ATTACK) == false {
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND
        || fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND {
            return;
        }
        if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_N_LANDING) {
            let air_accel_y = WorkModule::get_param_float(module_accessor,hash40("air_accel_y"),0);
            let air_speed_y_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_y_stable"),0);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_accel_y * -1.0);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_speed_y_stable);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        }
        else {
            WorkModule::on_flag(module_accessor,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_N_LANDING);
        }
    }
    else {
        let mut unk = false;
        if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_ATTACK_INIT) == false {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                unk = true;
            }
        }
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            if fighter.global_table[0x17].get_i32() != *SITUATION_KIND_AIR {
                unk = true;
            }
        }
        if unk {
            let speed_y = KineticModule::get_sum_speed_y(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let mut air_speed_y_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_y_stable"),0);
            let mut air_accel_y = WorkModule::get_param_float(module_accessor,hash40("air_accel_y"),0) * -1.0;
            let hit_max_speed_y_mul_air = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("hit_max_speed_y_mul_air"));
            let hit_accel_y_mul_air = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("hit_accel_y_mul_air"));
            if hit_max_speed_y_mul_air != 0.0 {
                air_speed_y_stable *= hit_max_speed_y_mul_air;
            }
            if hit_accel_y_mul_air != 0.0 {
                air_accel_y *= hit_accel_y_mul_air;
            }
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_speed_y_stable);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_accel_y);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,speed_y);
            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            WorkModule::on_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_ATTACK_INIT);
        }
    }
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_KIRBY {
        if WorkModule::get_int(module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_ELIGHT {
            if ArticleModule::is_exist(module_accessor,*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
                ArticleModule::set_visibility_whole(module_accessor,*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD,true,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
            else {
                ArticleModule::generate_article(module_accessor,*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD,false,-1);
                let fboma: *mut FighterModuleAccessor = std::mem::transmute(fighter.global_table[0x5].get_ptr());
                FighterSpecializer_Elight__kirby_sword_update_lr(fboma);
            }
        }
    }
    let mut seed = sv_math::rand(hash40("fighter"),*FIGHTER_ELIGHT_N_VOICE_NUM);
    if WorkModule::get_int(module_accessor,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_N_VOICE_KIND_NUM_PREV) == seed {
        seed += 1;
        seed %= *FIGHTER_ELIGHT_N_VOICE_NUM;
    }
    WorkModule::set_int(module_accessor,seed,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_N_VOICE_KIND_NUM_PREV);
    WorkModule::set_int(module_accessor,seed,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_VOICE_KIND_NUM);
    if seed == 1 {
        if fighter.global_table[0x2].get_i32() != *FIGHTER_KIND_KIRBY {
            SoundModule::play_se(module_accessor,Hash40::new("vc_elight_special_n02_01"),true,false,false,false,enSEType(0));
        }
        else {
            SoundModule::play_se(module_accessor,Hash40::new("vc_kirby_copy_elight_02_01"),true,false,false,false,enSEType(0));
        }
        seed = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),0x14eb311c43u64);
    }
    WorkModule::set_int(module_accessor,seed,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_VOICE_PLAY_FRAME);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n_start")),L2CValue::new_int(hash40("special_air_n_start")),L2CValue::new_bool(false));
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_n")));
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_N_LANDING) {
            let air_accel_y = WorkModule::get_param_float(module_accessor,hash40("air_accel_y"),0);
            let air_speed_y_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_y_stable"),0);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_accel_y * -1.0);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_speed_y_stable);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        }
        else {
            WorkModule::on_flag(module_accessor,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_N_LANDING);
        }
    }
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if MotionModule::is_end(module_accessor) == false {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n_start")),L2CValue::new_int(hash40("special_air_n_start")),L2CValue::new_bool(true));
        special_n_kinetics_setup(fighter);
    }
    else {
        if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == false {
            fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_HOLD.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}
