#[status_script(agent = "demon", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    WorkModule::on_flag(module_accessor,*FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    WorkModule::on_flag(module_accessor,*FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_hi_start")),L2CValue::new_int(hash40("special_air_hi_start")),L2CValue::new_bool(false));
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_hi")));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if MotionModule::is_end(module_accessor) == false {
        if WorkModule::is_flag(module_accessor,*FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_REVERSE_LR) {
            WorkModule::off_flag(module_accessor,*FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
            let stick = fighter.global_table[0x1a].get_f32().abs();
            let lr_stick_x = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("lr_stick_x"));
            if stick >= lr_stick_x {
                PostureModule::set_stick_lr(module_accessor,0.0);
                PostureModule::update_rot_y_lr(module_accessor);
            }
        }
        fighter.sub_set_special_start_inherit_common_kinetic_setting(L2CValue::new_int(hash40("param_special_hi")));
        let unk = fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            let start_stop_y_frame_air = WorkModule::get_param_int(module_accessor,hash40("param_special_hi"),hash40("start_stop_y_frame_air")) as f32;
            if unk {
                fighter.clear_lua_stack();
                lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
                sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                if fighter.global_table[0xe].get_f32() + 1.0 < start_stop_y_frame_air {
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.0);
                    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                }
            }
            if fighter.global_table[0xe].get_f32() + 1.0 <= start_stop_y_frame_air {
                let fall_speed_y = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("fall_speed_y")) * -1.0;
                fighter.clear_lua_stack();
                lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,fall_speed_y);
                sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE.into(),false.into());
    }
    return L2CValue::I32(0)
}
