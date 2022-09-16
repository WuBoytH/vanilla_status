use {
    smash::{
        lua2cpp::{L2CFighterCommon},
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::table_const::*
};

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn elight_special_s_forward_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_CHECK_CLIFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            let pos = Vector2f{
                x: GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32),
                y: GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
            };
            let length = sv_math::vec2_length(pos.x, pos.y);
            if length > 0.00001 {
                let angle = ((pos.y / pos.x).atan()).to_degrees().abs();
                if WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x11e7fad1adu64) < angle {
                    if PostureModule::lr(fighter.module_accessor) * pos.x < 0.0 {
                        fighter.set_situation(SITUATION_KIND_AIR.into());
                        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
                    }
                }
            }
        }
    }
    fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    let speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("speed_x_mul"));
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed,
        0.0
    );
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,speed_x_mul);
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        speed_x_mul
    );
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_forward_main_loop as *const () as _))
}

unsafe fn special_s_forward_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        let ground_cliff_stop_frame = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_s"),hash40("ground_cliff_stop_frame")) as f32;
        if fighter.global_table[MOTION_FRAME].get_f32() >= ground_cliff_stop_frame {
            let is_near_cliff_threshold = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_s"),hash40("is_near_cliff_threshold"));
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_CHECK_CLIFF)
            && !WorkModule::is_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF) {
                let num = PostureModule::scale(fighter.module_accessor) * is_near_cliff_threshold;
                if GroundModule::is_ottotto(fighter.module_accessor,num) {
                    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF);
                }
            }
        }
        else {
            let mut unk = true;
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT) {
                unk = false;
                let air_fix_frame = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_s"),hash40("air_fix_frame")) as f32;
                if fighter.global_table[0xe].get_f32() >= air_fix_frame {
                    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
                }
            }
            if unk {
                let motion = MotionModule::motion_kind(fighter.module_accessor);
                let frame = MotionModule::frame(fighter.module_accessor);
                let rate = MotionModule::rate(fighter.module_accessor);
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
                || motion == hash40("special_s") {
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
                    && motion != hash40("special_air_s") {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), frame, rate, 0.0, true, false);
                    }
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), frame, rate, 0.0, true, false);
                }
                fighter.sub_set_ground_correct_by_situation(true.into());
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        elight_special_s_forward_main
    );
}
