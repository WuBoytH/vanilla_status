use crate::imports::status_imports::*;

pub unsafe extern "C" fn elight_special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let jump_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("jump_stick"));
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    if stick_x.abs() + stick_y.abs() >= jump_stick {
        let angle = sv_math::vec2_angle(1.0, stick_x, stick_y, 0.0).to_degrees();
        let mut new_angle = angle * stick_x.signum() * -1.0;
        let jump_angle_limit_front = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("jump_angle_limit_front"));
        let jump_angle_limit_back = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("jump_angle_limit_back"));
        if PostureModule::lr(fighter.module_accessor) > 0.0 {
            new_angle = new_angle.clamp(jump_angle_limit_front * -1.0, jump_angle_limit_back);
        }
        else {
            new_angle = new_angle.clamp(jump_angle_limit_back * -1.0, jump_angle_limit_front);
        }
        sv_kinetic_energy!(
            set_angle,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            new_angle.to_radians()
        );
    }
    if !WorkModule::is_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_GROUND_START) {
        let jump_speed_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("jump_speed_mul_air"));
        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            jump_speed_mul_air
        );
    }
    else {
        let jump_speed_mul_ground = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_hi"),hash40("jump_speed_mul_ground"));
        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            jump_speed_mul_ground
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_jump_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let frame = WorkModule::get_int(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_INT_FRAME_FROM_START);
    if frame > 0 {
        if WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("attack_input_frame")) > frame {
            if ControlModule::get_trigger_count(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL as u8) & 0xff != 0
            && ControlModule::get_trigger_count(fighter.module_accessor,*CONTROL_PAD_BUTTON_ATTACK as u8) & 0xff != 0 {
                return 0.into();
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET);
        }
        if !WorkModule::is_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET) {
            fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2.into(), false.into());
        }
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, elight_special_hi_jump_main);
}