#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    let held_frames = WorkModule::get_int(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_HOLD_FRAME) as f32;
    let charge_frame = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("charge_frame")) as f32;
    let attack_charge_max_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("attack_charge_max_mul"));
    let attack_ratio = (held_frames/charge_frame).clamp(0.0,1.0) + 0.0;
    WorkModule::set_float(module_accessor,attack_ratio,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_ATTACK_RATIO);
    let power_mul = ((attack_charge_max_mul - 1.0) * attack_ratio) + 1.0;
    AttackModule::set_power_mul_status(module_accessor,power_mul);
    let mut motion = hash40("special_n");
    let mut motion_air = hash40("special_air_n");
    if attack_ratio >= 1.0 {
        motion = hash40("special_n2");
        motion_air = hash40("special_air_n2");
    }
    fighter.sub_change_motion_by_situation(L2CValue::new_int(motion),L2CValue::new_int(motion_air),L2CValue::new_bool(false));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_end_main_loop as *const () as _))
}

unsafe fn special_n_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if CancelModule::is_enable_cancel(module_accessor) == false
    || (fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() == false) {
        if MotionModule::is_end(module_accessor) == false
        || (fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND
        &&  fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR) {
            let attack_ratio = WorkModule::get_float(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_ATTACK_RATIO);
            let mut motion = hash40("special_n");
            let mut motion_air = hash40("special_air_n");
            if attack_ratio >= 1.0 {
                motion = hash40("special_n2");
                motion_air = hash40("special_air_n2");
            }
            fighter.sub_change_motion_by_situation(L2CValue::new_int(motion),L2CValue::new_int(motion_air),L2CValue::new_bool(true));
            special_n_kinetics_setup(fighter);
            if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET) {
                WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET);
                let attack_offset_id_base = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("attack_offset_id_base"));
                let attack_offset_id_num = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("attack_offset_id_num"));
                let offset_charge_max_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("offset_charge_max_mul"));
                let offset_mul = (offset_charge_max_mul - 1.0) * attack_ratio + 1.0;
                let attack_data_0 = AttackModule::attack_data(module_accessor,0,false);
                let store_0 = smash::app::lua_bind::AttackData::store_l2c_table(attack_data_0);
                if store_0.get_num() == L2CValue::new_void().get_num() {
                    let mut id = attack_offset_id_base + 1;
                    let id_max = attack_offset_id_num + attack_offset_id_base - 1;
                    if id <= id_max {
                        id -= 1;
                        while id <= id_max {
                            let attack_data = AttackModule::attack_data(module_accessor,id,false);
                            let store = smash::app::lua_bind::AttackData::store_l2c_table(attack_data);
                            if store.get_num() == L2CValue::new_void().get_num() {
                                let base_offset = Vector3f{x: store_0["offset_"]["x"].get_f32(), y: store_0["offset_"]["y"].get_f32(), z: store_0["offset_"]["z"].get_f32()};
                                let curr_offset = Vector3f{x: store["offset_"]["x"].get_f32(), y: store["offset_"]["y"].get_f32(), z: store["offset_"]["z"].get_f32()};
                                let offset = Vector3f{x: (curr_offset.x - base_offset.x) * offset_mul,
                                                      y: (curr_offset.y - base_offset.y) * offset_mul,
                                                      z: (curr_offset.z - base_offset.z) * offset_mul};
                                AttackModule::set_offset(module_accessor,id + 1,&offset);
                            }
                            id += 1;
                        }
                    }
                }
            }
            let pos_x_min = WorkModule::get_float(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MIN);
            let pos_x_max = WorkModule::get_float(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_POS_X_MAX);
            let pos_x = attack_ratio * (pos_x_max - pos_x_min) + pos_x_min + 0.0;
            WorkModule::set_float(module_accessor,pos_x,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT1_X);
            let pos_x_min = WorkModule::get_float(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MIN);
            let pos_x_max = WorkModule::get_float(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_POS_X_MAX);
            let pos_x = attack_ratio * (pos_x_max - pos_x_min) + pos_x_min + 0.0;
            WorkModule::set_float(module_accessor,pos_x,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLOAT_EFFECT2_X);
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL) {
                    WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL);
                    let end_control_accel_x_mul_air = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("end_control_accel_x_mul_air"));
                    let end_control_max_speed_x_mul_air = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("end_control_max_speed_x_mul_air"));
                    let air_speed_x_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_x_stable"),0) * end_control_max_speed_x_mul_air;
                    let air_accel_x_mul = WorkModule::get_param_float(module_accessor,hash40("air_accel_x_mul"),0) * end_control_accel_x_mul_air;
                    let air_accel_x_add = WorkModule::get_param_float(module_accessor,hash40("air_accel_x_add"),0) * end_control_accel_x_mul_air;
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,air_speed_x_stable);
                    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,air_accel_x_mul);
                    sv_kinetic_energy::controller_set_accel_x_mul(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,air_accel_x_add);
                    sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    sv_kinetic_energy::enable(fighter.lua_state_agent);
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
            }
            if WorkModule::count_down_int(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_VOICE_PLAY_FRAME,0) {
                let vc_kind = WorkModule::get_int(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_N_INT_VOICE_KIND_NUM);
                if fighter.global_table[0x2].get_i32() != *FIGHTER_KIND_KIRBY {
                    match vc_kind {
                        0 => SoundModule::play_se(module_accessor,Hash40::new("vc_elight_special_n01"),true,false,false,false,enSEType(0)),
                        1 => SoundModule::play_se(module_accessor,Hash40::new("vc_elight_special_n02_02"),true,false,false,false,enSEType(0)),
                        _ => SoundModule::play_se(module_accessor,Hash40::new("vc_elight_special_n03"),true,false,false,false,enSEType(0)),
                    };
                }
                else {
                    match vc_kind {
                        0 => SoundModule::play_se(module_accessor,Hash40::new("vc_kirby_copy_elight_01"),true,false,false,false,enSEType(0)),
                        1 => SoundModule::play_se(module_accessor,Hash40::new("vc_kirby_copy_elight_02_02"),true,false,false,false,enSEType(0)),
                        _ => SoundModule::play_se(module_accessor,Hash40::new("vc_kirby_copy_elight_03"),true,false,false,false,enSEType(0)),
                    };
                }
            }
            return L2CValue::I32(0)
        }
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}
