#![allow(improper_ctypes)]

use super::*;

unsafe extern "C" fn zelda_special_hi_2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR |
            *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn zelda_special_hi_2_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let mut length = sv_math::vec2_length(stick_x, stick_y).min(1.0);

    let wrap_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_stick"));
    let mut follow_stick = false;
    if length <= wrap_stick {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
            follow_stick = true;
        }
        else {
            if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
                if GroundModule::is_passable_ground(fighter.module_accessor) {
                    follow_stick = true;
                }
                else {
                    let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32);
                    let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32);
                    let angle = sv_math::vec2_angle(normal_x, normal_y, stick_x, stick_y);
                    if angle < 90.0_f32.to_radians() {
                        follow_stick = true;
                    }
                }
            }
        }
    }
    else {
        follow_stick = true;
    }

    if 0.00001 < stick_x {
        PostureModule::set_lr(fighter.module_accessor, 1.0);
    }
    else if stick_x < -0.00001 {
        PostureModule::set_lr(fighter.module_accessor, -1.0);
    }
    PostureModule::update_rot_y_lr(fighter.module_accessor);

    let lr = PostureModule::lr(fighter.module_accessor);

    let wrap_speed_multi = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_speed_multi"));
    let wrap_speed_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_speed_add"));

    let mut speed_x;
    let mut speed_y = 0.0;
    if !follow_stick {
        let atan = stick_y.atan2(stick_x * lr);
        let length_mul = wrap_speed_multi * length;

        let speed = length_mul + wrap_speed_add;
        let cos = atan.cos();
        speed_x = speed * cos;
        speed_x *= lr;
    }
    else {
        let angle = if length < wrap_stick {
            length = 1.0;
            90.0_f32.to_radians()
        }
        else {
            stick_y.atan2(stick_x * lr)
        };

        let length_mul = wrap_speed_multi * length;

        let speed = length_mul + wrap_speed_add;
        let cos = angle.cos();
        speed_x = speed * cos;
        speed_x *= lr;

        let sin = angle.sin();
        speed_y = speed * sin;

        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }

    KineticModule::unable_energy_all(fighter.module_accessor);

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_FREE,
        speed_x,
        speed_y,
        0.0,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        -1.0,
        -1.0
    );

    sv_kinetic_energy!(
        enable,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    GroundModule::clear_cliff_point(fighter.module_accessor);

    0.into()
}

unsafe extern "C" fn zelda_special_hi_2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    VisibilityModule::set_whole(fighter.module_accessor, false);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);

    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);

    GroundModule::set_passable_check(fighter.module_accessor, true);

    let cliff_check = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
    fighter.sub_fighter_cliff_check(cliff_check.into());

    if !StopModule::is_stop(fighter.module_accessor) {
        zelda_special_hi_2_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(zelda_special_hi_2_substatus as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(zelda_special_hi_2_main_loop as *const () as _))
}

unsafe extern "C" fn zelda_special_hi_2_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        let special_hi_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let move_xlu = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_xlu"));
        if special_hi_frame == move_xlu {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
        let move_cliff_check = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_cliff_check"));
        if special_hi_frame == move_cliff_check {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let special_hi_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        if 2 <= special_hi_frame {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND);
        }
    }
    0.into()
}

unsafe extern "C" fn zelda_special_hi_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let special_hi_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    let move_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_time"));
    if move_time <= special_hi_frame {
        fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), false.into());
        return 0.into();
    }

    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }

    zelda_special_hi_2_check_ground(fighter);

    0.into()
}

extern "C" {
    #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11get_speed3fEP9lua_State"]
    pub fn get_speed3f(arg1: u64) -> smash_rs::cpp::simd::Vector3;
}

unsafe extern "C" fn zelda_special_hi_2_check_ground(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND) {
        return;
    }

    if GroundModule::is_attach_cliff(fighter.module_accessor) {
        return;
    }

    let mut touch_id = *GROUND_TOUCH_ID_NONE;
    let mut touch_flag = *GROUND_TOUCH_FLAG_NONE;

    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        touch_id = *GROUND_TOUCH_ID_RIGHT;
        touch_flag = *GROUND_TOUCH_FLAG_RIGHT;
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
        touch_id = *GROUND_TOUCH_ID_LEFT;
        touch_flag = *GROUND_TOUCH_FLAG_LEFT;
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
        touch_id = *GROUND_TOUCH_ID_UP;
        touch_flag = *GROUND_TOUCH_FLAG_UP;
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        touch_id = *GROUND_TOUCH_ID_DOWN;
        touch_flag = *GROUND_TOUCH_FLAG_DOWN;
    }

    if touch_flag == *GROUND_TOUCH_FLAG_NONE {
        return;
    }

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed = get_speed3f(fighter.lua_state_agent);

    let mut length = sv_math::vec3_length(speed.x, speed.y, speed.z);
    if 0.0 < length {
        let touch_x = GroundModule::get_touch_normal_x(fighter.module_accessor, touch_flag as u32);
        let touch_y = GroundModule::get_touch_normal_y(fighter.module_accessor, touch_flag as u32);

        let touch = fighter.Vector3__create(touch_x.into(), touch_y.into(), 0.0_f32.into());
        let something = fighter.Vector3__create(0.0_f32.into(), 0.0_f32.into(), 1.0_f32.into());
        let mut cross = fighter.Vector3__cross(touch.clone(), something);

        let math = 1.0 / length;
        let speed_mul = Vector3f{x: speed.x * math, y: speed.y * math, z: speed.z * math};
        let mut final_dot = 0.0;
        if touch_flag != *GROUND_TOUCH_FLAG_DOWN
        && 0.0 < speed_mul.y {
            if cross["y"].get_f32() < 0.0 {
                final_dot = -1.0;
            }
            let x = touch["x"].get_f32();
            let y = touch["y"].get_f32();
            let deg = x.atan2(y).to_degrees().abs();
            let deg = 180.0 - deg;
            let deg = deg.abs();
            let something = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x158bb5418d);
            if deg <= something {
                length = speed_mul.x.abs();
                cross["x"].assign(&L2CValue::F32(speed_mul.x.signum()));
            }
        }
        else {
            final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), speed_mul.x, speed_mul.y, speed_mul.z);
            if -0.00001 <= final_dot
            && final_dot <= 0.00001 {
                if touch_flag == *GROUND_TOUCH_FLAG_RIGHT
                || touch_flag == *GROUND_TOUCH_FLAG_LEFT {
                    final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), 0.0, 1.0, 0.0);
                }
                else {
                    let lr = PostureModule::lr(fighter.module_accessor);
                    final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), lr, 0.0, 0.0);
                }
            }
        }

        if final_dot < 0.0 {
            let x = cross["x"].get_f32();
            let y = cross["y"].get_f32();
            let z = cross["z"].get_f32();
            cross["x"].assign(&L2CValue::F32(x * -1.0));
            cross["y"].assign(&L2CValue::F32(y * -1.0));
            cross["z"].assign(&L2CValue::F32(z * -1.0));
        }
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            cross["x"].get_f32() * length,
            cross["y"].get_f32() * length,
            cross["z"].get_f32() * length
        );
        let situation = StatusModule::situation_kind(fighter.module_accessor);
        if situation == *SITUATION_KIND_GROUND {
            let line = GroundModule::get_touch_line_raw(fighter.module_accessor, GroundTouchID(touch_id)) as *mut GroundCollisionLine;
            let is_floor = sv_ground_collision_line::is_floor(line);
            GroundModule::set_attach_ground(fighter.module_accessor, is_floor);
        }
    }
}

unsafe extern "C" fn zelda_special_hi_2_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3 {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, zelda_special_hi_2_pre);
    agent.status(Init, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, zelda_special_hi_2_init);
    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, zelda_special_hi_2_main);
    agent.status(Exit, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, zelda_special_hi_2_exit);
}