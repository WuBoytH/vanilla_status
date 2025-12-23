use super::*;

unsafe extern "C" fn turn_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_BOOMERANG_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_BOOMERANG_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_BOOMERANG_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK,
    );

    0.into()
}

unsafe extern "C" fn turn_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.0
    );

    let angle = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let new_angle = if angle <= 0.0 {
        angle + std::f32::consts::PI
    } else {
        angle - std::f32::consts::PI
    };
    WorkModule::set_float(weapon.module_accessor, new_angle, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);

    let follow_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_boomerang"), hash40("follow_frame"));
    WorkModule::set_int(weapon.module_accessor, follow_frame, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);

    let rot_x = PostureModule::rot_x(weapon.module_accessor, *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);
    let rot_y = PostureModule::rot_y(weapon.module_accessor, *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);
    let rot_z = PostureModule::rot_z(weapon.module_accessor, *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);

    let angle_x_turn = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("angle_x_turn"));

    PostureModule::set_rot(weapon.module_accessor, &Vector3f{ x: rot_x, y: angle_x_turn, z: rot_z }, *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN);

    WorkModule::set_float(weapon.module_accessor, 0.0, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);

    let param = if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FLICK) {
        hash40("flick_rot_speed")
    }
    else {
        hash40("rot_speed")
    };
    let rot_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), param);
    let angle_x_back = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("angle_x_back"));
    let speed = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_SPEED);
    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("accel"));
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_min"));
    let speed_diff = speed - speed_min;
    let accel_diff = speed_diff / accel;
    let floor = accel_diff.floor();
    let idkman = angle_x_back - rot_y;
    let huh = idkman / floor;
    WorkModule::set_int(weapon.module_accessor, floor as i32, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        rot_speed,
        huh,
        0.0
    );

    0.into()
}

unsafe extern "C" fn turn_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("turn"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    if !StopModule::is_stop(weapon.module_accessor) {
        if false {
            turn_substatus_inner(weapon);
        }
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(turn_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(turn_fastshift as *const () as _))
}

unsafe extern "C" fn turn_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        turn_substatus_inner(weapon);
    }

    0.into()
}

unsafe extern "C" fn turn_substatus_inner(weapon: &mut L2CWeaponCommon) {
    if WorkModule::count_down_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE, 0) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn turn_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !StopModule::is_stop(weapon.module_accessor) {
        if turn_fastshift_inner(weapon).get_bool() {
            return 1.into();
        }
    }

    0.into()
}

unsafe extern "C" fn turn_fastshift_inner(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let correct = GroundModule::get_correct(weapon.module_accessor);
    if LinkModule::is_link(weapon.module_accessor, *LINK_NO_ARTICLE) {
        let parent_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_ARTICLE, true);
        if !WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REFLECT) {
            let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor);
            if parent_id == team_owner_id {
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let x = weapon.pop_lua_stack(1).get_f32();
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let y = weapon.pop_lua_stack(1).get_f32();
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Z, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let z = weapon.pop_lua_stack(1).get_f32();

                let pos_x = PostureModule::pos_x(weapon.module_accessor);
                let pos_y = PostureModule::pos_y(weapon.module_accessor);
                let pos_z = PostureModule::pos_z(weapon.module_accessor);

                let length = sv_math::vec3_length(x - pos_x, y - pos_y, z - pos_z);
                if length <= 9.0 {
                    weapon.clear_lua_stack();
                    lua_args!(
                        weapon,
                        MA_MSC_LINK_SEND_EVENT_PARENTS,
                        LINK_NO_ARTICLE,
                        Hash40::new_raw(0x170db96f9c),
                        WN_LINK_BOOMERANG_TURN_WORK_INT_LINK_EVENT_RESULT_01,
                        WN_LINK_BOOMERANG_TURN_WORK_FLOAT_LINK_EVENT_RESULT_01,
                        WN_LINK_BOOMERANG_TURN_WORK_FLAG_LINK_EVENT_RESULT_01
                    );
                    sv_module_access::link(weapon.lua_state_agent);
                    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_LINK_EVENT_RESULT_01) {
                        PostureModule::set_rot(
                            weapon.module_accessor,
                            &Vector3f{ x: 0.0, y: 0.0, z: 0.0 },
                            *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN
                        );
                        PostureModule::set_rot(
                            weapon.module_accessor,
                            &Vector3f{ x: 0.0, y: 0.0, z: 0.0 },
                            *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_ROTN
                        );
                        WorkModule::set_float(weapon.module_accessor, 0.0, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
                        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_HAVED.into(), false.into());
                        return 1.into();
                    }
                    else {
                        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
                        return 1.into();
                    }
                }
            }
        }

        if correct != *GROUND_CORRECT_KIND_NONE {
            let parent_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_ARTICLE, true);
            let parent_module_accessor = sv_battle_object::module_accessor(parent_id as u32);
            let parent_pos_y = PostureModule::pos_y(parent_module_accessor);
            let pos_y = PostureModule::pos_y(weapon.module_accessor);
            let diff = parent_pos_y - pos_y;
            let dist = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), 0x1cb25d4dc6);
            if diff.abs() >= dist * 10.0 {
                GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
            }
        }
    }

    let correct = GroundModule::get_correct(weapon.module_accessor);

    let param = if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FLICK) {
        hash40("turn_dist_flick")
    }
    else {
        hash40("turn_dist")
    };
    let turn_dist = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), param);
    let dist = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    if turn_dist * 10.0 <= dist {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }

    if correct == *GROUND_CORRECT_KIND_NONE {
        if !StatusModule::is_changing(weapon.module_accessor) {
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
                GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
                return 0.into();
            }
        }
        if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_DOWN) as u32) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        }
    }

    0.into()
}

unsafe extern "C" fn turn_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    turn_exec_inner(weapon)
}

unsafe extern "C" fn turn_exec_inner(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let mut angle = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    if LinkModule::is_link(weapon.module_accessor, *LINK_NO_ARTICLE) {
        if !WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REFLECT)
        && WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME) > 0 {
            weapon.clear_lua_stack();
            lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("waist"), true);
            FL_sv_module_access::link(weapon.lua_state_agent);
            let x = weapon.pop_lua_stack(1).get_f32();
            weapon.clear_lua_stack();
            lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("waist"), true);
            FL_sv_module_access::link(weapon.lua_state_agent);
            let y = weapon.pop_lua_stack(1).get_f32();
            weapon.clear_lua_stack();

            let pos_x = PostureModule::pos_x(weapon.module_accessor);
            let pos_y = PostureModule::pos_y(weapon.module_accessor);
            
            let diff_x = x - pos_x;
            let diff_y = y - pos_y;
            
            let atan = diff_y.atan2(diff_x);
            
            let atan = if atan < -std::f32::consts::PI {
                atan + std::f32::consts::PI * 2.0
            }
            else {
                if std::f32::consts::PI < atan {
                    atan - std::f32::consts::PI * 2.0
                }
                else {
                    atan
                }
            };

            let atan = atan - angle;
            
            let atan = if atan < -std::f32::consts::PI {
                atan + std::f32::consts::PI * 2.0
            }
            else {
                if std::f32::consts::PI < atan {
                    atan - std::f32::consts::PI * 2.0
                }
                else {
                    atan
                }
            };

            let turn_angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("turn_angle")).to_radians();
            let atan = if turn_angle < atan {
                turn_angle
            }
            else {
                if atan < -turn_angle {
                    -turn_angle
                }
                else {
                    atan
                }
            };

            angle = atan;

            WorkModule::set_float(weapon.module_accessor, atan, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
            WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);
        }
    }

    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let mut length = sv_kinetic_energy::get_speed_length(weapon.lua_state_agent);
    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("accel"));
    length += accel;
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_max"));
    let speed_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_mul"));
    let speed_max = speed_max * speed_mul;
    if speed_max < length {
        length = speed_max;
    }
    let cos = angle.cos();
    let sin = angle.sin();
    let vel_x = cos * length;
    let vel_y = sin * length;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, vel_x, vel_y);

    let turn_dist = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    WorkModule::set_float(weapon.module_accessor, turn_dist + length, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);

    let turn_follow_dist = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("turn_follow_dist"));
    if turn_follow_dist * 10.0 <= turn_dist + length {
        WorkModule::set_int(weapon.module_accessor, 0, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);
    }

    let back_rot_frame = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME);
    if back_rot_frame > 0 {
        if back_rot_frame - 1 == 0 {
            let rot_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("rot_speed"));
            sv_kinetic_energy!(
                set_speed,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
                rot_speed,
                0.0,
                0.0
            );
        }
        WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WN_LINK_BOOMERANG_STATUS_KIND_TURN, turn_pre);
    agent.status(Init, *WN_LINK_BOOMERANG_STATUS_KIND_TURN, turn_init);
    agent.status(Main, *WN_LINK_BOOMERANG_STATUS_KIND_TURN, turn_main);
    agent.status(Exec, *WN_LINK_BOOMERANG_STATUS_KIND_TURN, turn_exec);
}