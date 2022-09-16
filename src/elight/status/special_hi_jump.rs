#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_jump"),0.0,1.0,false,0.0,false,false);
    let jump_stick = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("jump_stick"));
    let stick_x = fighter.global_table[0x1a].get_f32();
    let stick_y = fighter.global_table[0x1b].get_f32();
    if stick_x.abs() + stick_y.abs() >= jump_stick {
        let angle = sv_math::vec2_angle(1.0,stick_x,stick_y,0.0).to_degrees();
        let sign = fighter.sign(L2CValue::new_num(stick_x)).get_f32();
        let new_angle = angle * sign * -1.0;
        let jump_angle_limit_front = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("jump_angle_limit_front"));
        let jump_angle_limit_back = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("jump_angle_limit_back"));
        if PostureModule::lr(module_accessor) > 0.0 {
            fighter.clamp(L2CValue::new_num(new_angle),L2CValue::new_num(jump_angle_limit_front * -1.0),L2CValue::new_num(jump_angle_limit_back));
        }
        else {
            fighter.clamp(L2CValue::new_num(new_angle),L2CValue::new_num(jump_angle_limit_back * -1.0),L2CValue::new_num(jump_angle_limit_front));
        }
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,new_angle.to_radians());
        sv_kinetic_energy::set_angle(fighter.lua_state_agent);
    }
    if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_GROUND_START) == false {
        let jump_speed_mul_air = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("jump_speed_mul_air"));
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,jump_speed_mul_air);
        sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    }
    else {
        let jump_speed_mul_ground = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("jump_speed_mul_ground"));
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,jump_speed_mul_ground);
        sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_jump_main_loop as *const () as _))
}

unsafe fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return L2CValue::I32(1)
    }
    let frame = WorkModule::get_int(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_INT_FRAME_FROM_START);
    if frame > 0 {
        if WorkModule::get_param_int(module_accessor,hash40("param_special_hi"),hash40("attack_input_frame")) > frame {
            if ControlModule::get_trigger_count(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL as u8) & 0xff != 0 {
                if ControlModule::get_trigger_count(module_accessor,*CONTROL_PAD_BUTTON_ATTACK as u8) & 0xff != 0 {
                    return L2CValue::I32(0)
                }
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET);
        }
    }
    if MotionModule::is_end(module_accessor) {
        if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::on_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET);
        }
        if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET) == false {
            fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}
