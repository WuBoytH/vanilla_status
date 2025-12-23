use super::*;

unsafe extern "C" fn special_s_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_s_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_attack_main_inner(fighter)
}

unsafe extern "C" fn special_s_attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let touch_down = GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    let to_search = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH);
    WorkModule::set_flag(fighter.module_accessor, to_search, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH_PREV);
    WorkModule::set_flag(fighter.module_accessor, touch_down, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TOUCH_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH);

    let attack_button = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH_PREV)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_STICK);
    WorkModule::set_flag(fighter.module_accessor, attack_button, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_ATTACK_BUTTON);

    if 0 < attack_count {
        // hell
        special_s_attack_set_motion_multiple(fighter, attack_count.into(), false.into(), touch_down.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::set_passable_check(fighter.module_accessor, true);
        let target_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_TARGET_ID) as u32;
        let mut speed = special_s_attack_get_speed(fighter).get_f32();
        let mut speed_vec = fighter.Vector2__create(speed.into(), 0.0_f32.into());
        let lr = PostureModule::lr(fighter.module_accessor);
        let mut new_lr = lr;
        if target_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            // hell 2
            let pos = FighterSpecializer_Trail::get_special_s_target_pos(target_id);
            let target_pos_vec = &mut fighter.Vector3__create(pos.x.into(), pos.y.into(), pos.z.into());
            let joint = WorkModule::get_param_int64(fighter.module_accessor, 0x187367db00, 0);
            let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
            ModelModule::joint_global_position(fighter.module_accessor, Hash40::new_raw(joint), pos, true);
            let pos_vec = &mut fighter.Vector3__create(pos.x.into(), pos.y.into(), pos.z.into());
            let diff_x = target_pos_vec["x"].get_f32() - pos_vec["x"].get_f32();
            let diff_y = target_pos_vec["y"].get_f32() - pos_vec["y"].get_f32();
            let diff_vec = fighter.Vector2__create(diff_x.into(), diff_y.into());
            // let some_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), 0x1719bf7eb5);
            // let attack_num = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_num"));
            // Unfinished
            // if attack_count == attack_num - 1 {
            //     let attack_check_down_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_check_down_y"));
            //     let attack_check_down_y = attack_check_down_y * 10.0;
            //     let grav = special_s_attack_check_gravity_world(fighter, pos_vec["x"].clone(), pos_vec["y"].clone(), attack_check_down_y.into());
            //     let line = GroundModule::ray_check_get_line(
            //         fighter.module_accessor,
            //         &Vector2f{x: pos_vec["x"].get_f32(), y: pos_vec["y"].get_f32()},
            //         &Vector2f{x: grav["x"].get_f32(), y: grav["y"].get_f32()},
            //         true
            //     );

            //     if line != 0 {
            //         if sv_ground_collision_line::is_floor(line as *mut GroundCollisionLine) {
            //             let dead_range = sv_camera_manager::dead_range(fighter.lua_state_agent);

            //             let some_x_thing = if lr == -1.0 {
            //                 pos_vec["x"].get_f32() - dead_range.x
            //             }
            //             else {
            //                 dead_range.y - pos_vec["x"].get_f32()
            //             };

            //             let attack_check_dead_range_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_check_dead_range_x"));
            //             if some_x_thing <= attack_check_dead_range_x * 10.0 {

            //             }
            //         }
            //     }
            // }

            let diff_x = diff_vec["x"].get_f32();
            let diff_y = diff_vec["y"].get_f32();
            if sv_math::vec2_is_zero(diff_x, diff_y) {
                todo!()
            }
            else {
                todo!()
            }
        }
        else {
            let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
            let attack_up_angle_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_up_angle_min"));
            let attack_up_angle_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_up_angle_max"));
            if angle >= attack_up_angle_min && angle <= attack_up_angle_max {
                let attack_up_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_up_speed_mul"));
                speed *= attack_up_speed_mul;
            }

            let rad = angle.to_radians();
            let cos = rad.cos();
            speed_vec["x"].assign(&L2CValue::F32(cos * speed));
            let sin = rad.sin();
            speed_vec["y"].assign(&L2CValue::F32(sin * speed));

            if !special_s_attack_check_angle_multiple(fighter, angle.into()).get_bool() {
                let speed_x = speed_vec["x"].get_f32();
                if speed_x < 0.0 {
                    new_lr = -1.0;
                }
                else if speed_x > 0.0 {
                    new_lr = 1.0;
                }
            }
        }

        if lr != new_lr {
            PostureModule::set_lr(fighter.module_accessor, new_lr);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }

        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            speed_vec["x"].get_f32(),
            speed_vec["y"].get_f32()
        );
        sv_kinetic_energy!(
            set_brake,
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
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_1").into(), Hash40::new("special_air_s_1").into(), false.into());
        special_s_attack_set_kinetic(fighter);
        special_s_attack_set_speed(fighter);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }

    let attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_frame"));
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    let rate = (end_frame / attack_frame as f32) + 0.01;
    MotionModule::set_rate(fighter.module_accessor, rate);

    special_s_set_cursor_on_posture(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_attack_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_attack_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let correct = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_CLIFF_STOP) {
            *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
        }
        else {
            *GROUND_CORRECT_KIND_GROUND
        };
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
}

unsafe extern "C" fn special_s_attack_set_speed(fighter: &mut L2CFighterCommon) {
    let speed = special_s_attack_get_speed(fighter).get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed * lr,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
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
        0.0
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
    );
}

unsafe extern "C" fn special_s_attack_get_speed(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut attack_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_speed_x"));
    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    if 0 < attack_count {
        let attack_reduction_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x15530d2d10);
        for _ in 0..attack_count {
            attack_speed_x *= attack_reduction_mul;
        }
    }
    let hit_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_HIT_NUM);
    if 0 < hit_num {
        let hit_reduction_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x17dd304b6f);
        for _ in 0..hit_num {
            attack_speed_x *= hit_reduction_mul;
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_ATTACK_BUTTON) {
        let button_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x1a41a10288);
        attack_speed_x *= button_speed_mul;
    }

    attack_speed_x.into()
}

unsafe extern "C" fn special_s_attack_set_motion_multiple(
    fighter: &mut L2CFighterCommon,
    attack_count: L2CValue,
    is_inherit: L2CValue,
    touch_ground: L2CValue
) {
    let motion = if attack_count.get_i32() == 1 {
        if touch_ground.get_bool() {
            "special_s_2"
        }
        else {
            "special_air_s_2"
        }
    }
    else {
        if touch_ground.get_bool() {
            "special_s_3"
        }
        else {
            "special_air_s_3"
        }
    };

    if is_inherit.get_bool() {
        let frame = MotionModule::frame(fighter.module_accessor);
        let rate = MotionModule::rate(fighter.module_accessor);
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new(motion),
            frame,
            rate,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new(motion),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
}

unsafe extern "C" fn special_s_attack_check_angle_multiple(fighter: &mut L2CFighterCommon, angle: L2CValue) -> L2CValue {
    let some_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0xdeb5675e2);
    let angle_adjust_1 = angle.get_f32() - 90.0;
    let angle_adjust_2 = angle.get_f32() - 270.0;
    if angle_adjust_1.abs() > some_angle
    && angle_adjust_2.abs() > some_angle {
        return false.into();
    }

    true.into()
}

unsafe extern "C" fn special_s_attack_check_gravity_world(
    fighter: &mut L2CFighterCommon,
    pos_x: L2CValue,
    pos_y: L2CValue,
    check_down_y: L2CValue
) -> L2CValue {
    let ret = &mut fighter.Vector2__create(0.0_f32.into(), check_down_y.clone());
    if !lua_bind::BattleObjectWorld::is_gravity_normal(singletons::BattleObjectWorld()) {
        let gravity_pos = lua_bind::BattleObjectWorld::gravity_pos(singletons::BattleObjectWorld());
        let diff_x = pos_x.get_f32() - gravity_pos.x;
        let diff_y = pos_y.get_f32() - gravity_pos.y;
        let atan = -diff_y.atan2(diff_x);
        let rot = sv_math::vec2_rot(0.0, check_down_y.get_f32(), atan);
        ret["x"].assign(&L2CValue::F32(rot.x));
        ret["y"].assign(&L2CValue::F32(rot.y));
    }
    fighter.Vector2__create(ret["x"].clone(), ret["y"].clone())
}

unsafe extern "C" fn special_s_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    let to_search = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH);
    if !to_search {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH);
            }
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let attack_num = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_num"));
        if attack_count < attack_num - 1 {
            let mut status = FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
            let mut clear_buffer = false;
            let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
            let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
            let stick_vec = fighter.Vector2__create(stick_x.into(), stick_y.into());
            let length = fighter.Vector2__length(stick_vec);
            let search_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("search_stick"));
            if length.get_f32() < search_stick {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH) {
                    status = FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH;
                    clear_buffer = true;
                    if attack_count == 0 {
                        special_s_reset_angle(fighter);
                    }
                }
            }
            else {
                if attack_count == 0 {
                    special_s_reset_angle(fighter);
                }
                status = FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH;
                clear_buffer = true;
            }
            fighter.change_status(status.into(), clear_buffer.into());
        }
        else {
            fighter.change_status(FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        let flags = GroundModule::get_touch_flag(fighter.module_accessor) as u32;
        if flags & *GROUND_TOUCH_FLAG_DOWN as u32 != 0 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TOUCH_GROUND) {

            }
            else {
                if flags & (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_RIGHT) as u32 != 0 {
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_TOUCH_GROUND_FRAME);
                }
                else {
                    if !fighter.global_table[IS_STOP].get_bool() {
                        let ground_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_TOUCH_GROUND_FRAME);
                        let ground_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), 0x1b949b05bc);
                        if ground_frame < ground_frame_max {
                            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_TOUCH_GROUND_FRAME);
                        }
                        else {
                            fighter.change_status(FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END.into(), false.into());
                            return 0.into();
                        }
                    }
                }
            }
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_TOUCH_GROUND_FRAME);
        }

        if attack_count > 0 {
            let touch_ground = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TOUCH_GROUND);
            if !touch_ground {
                special_s_attack_set_motion_multiple(fighter, attack_count.into(), true.into(), touch_ground.into());
                // Why does it do this?
                WorkModule::set_flag(fighter.module_accessor, touch_ground, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TOUCH_GROUND);
            }
        }
        else {
            if StatusModule::is_situation_changed(fighter.module_accessor) {
                fighter.sub_change_motion_by_situation(
                    Hash40::new("special_s_1").into(),
                    Hash40::new("special_air_s_1").into(),
                    true.into()
                );
                special_s_attack_set_kinetic(fighter);
                special_s_attack_set_speed(fighter);
            }
        }
    }

    if 0 < attack_count {
        let attack_num = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_num"));
        if attack_count < attack_num - 1 {
            let handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
            if fighter.global_table[IS_STOP].get_bool() {
                if handle != 0 {
                    let guide_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_GUIDE_EFFECT_ANGLE_ATTACK);
                    let guide_pos = special_s_get_guide_pos(fighter, guide_angle.into());
                    EffectModule::set_pos(fighter.module_accessor, handle as u32, &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0});
                }
            }
            else {
                special_s_handle_guide(fighter, handle.into(), FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_GUIDE_EFFECT_ANGLE_ATTACK.into());
            }
        }
    }

    if 0 < attack_count {
        let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
        special_s_set_joint_rotate(fighter, angle.into(), 1.0_f32.into());
    }

    0.into()
}

unsafe extern "C" fn special_s_attack_calc_param(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    if 0 < attack_count {
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            -1.0,
            -1.0
        );
    }

    0.into()
}

unsafe extern "C" fn special_s_attack_fix_pos_slow(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    if 0 < attack_count {
        let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
        special_s_set_joint_rotate(fighter, angle.into(), 1.0_f32.into());
    }
    0.into()
}

unsafe extern "C" fn special_s_attack_on_change_lr(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, _param_3: &L2CValue) -> L2CValue {
    let mut angle = 180.0 - WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
    if angle < 0.0 {
        angle += 360.0;
    }
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
    true.into()
}

unsafe extern "C" fn special_s_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();

    if status != *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH {
        let handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE) as u32;
        if handle != 0 {
            EffectModule::kill(fighter.module_accessor, handle, true, true);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
        }
    }

    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);

    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_LOCKONCURSOR, ArticleOperationTarget(0));

    GroundModule::set_passable_check(fighter.module_accessor, false);

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack_pre);
    agent.status(Main, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack_main);
    agent.status(CalcParam, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack_calc_param);
    agent.status(FixPosSlow, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack_fix_pos_slow);
    agent.status(OnChangeLr, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack_on_change_lr);
    agent.status(End, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack_end);
}