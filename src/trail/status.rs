use super::*;

unsafe extern "C" fn special_s_get_guide_pos(fighter: &mut L2CFighterCommon, angle: L2CValue) -> Vector2f {
    let pos = PostureModule::pos(fighter.module_accessor);
    let rad = angle.get_f32().to_radians();
    let scale = PostureModule::scale(fighter.module_accessor);
    let dist = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("search_cursor_dist"));
    let dist_scaled = dist * scale;
    let x_pos = rad.cos() * dist_scaled + (*pos).x;
    let y_pos = rad.sin() * dist_scaled + (*pos).y;
    let y_offset = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("search_cursor_offset_y"));
    let y_pos = y_offset * scale + y_pos;
    Vector2f{x: x_pos, y: y_pos}
}

unsafe extern "C" fn special_s_handle_guide(fighter: &mut L2CFighterCommon, eff_handle: L2CValue, angle_const: L2CValue) {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_vec = fighter.Vector2__create(stick_x.into(), stick_y.into());
    let length = fighter.Vector2__length(stick_vec);
    let search_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("search_stick"));
    let angle = if search_stick <= length.get_f32() {
        let atan = stick_y.atan2(stick_x);
        let mut deg = atan.to_degrees();
        if deg < 0.0 {
            deg += 360.0;
        }
        WorkModule::set_float(fighter.module_accessor, deg, angle_const.get_i32());
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_STICK);
        deg
    }
    else {
        if eff_handle.get_u32() == 0 {
            return;
        }
        WorkModule::get_float(fighter.module_accessor, angle_const.get_i32())
    };

    let guide_pos = special_s_get_guide_pos(fighter, angle.into());

    let handle = eff_handle.get_u32();
    if handle != 0 {
        EffectModule::set_pos(fighter.module_accessor, handle, &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0});
        EffectModule::set_rot(fighter.module_accessor, handle, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});
    }
    else {
        let handle = EffectModule::req(
            fighter.module_accessor,
            Hash40::new("sys_direction2"),
            &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            1.0,
            0,
            -1,
            false,
            0
        ) as u32;
        EffectModule::set_rot(fighter.module_accessor, handle, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});
        let team_color = FighterUtil::get_team_color(fighter.module_accessor);
        let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
        EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
        WorkModule::set_int(fighter.module_accessor, handle as i32, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
    }
}

pub unsafe extern "C" fn special_s_set_joint_rotate(fighter: &mut L2CFighterCommon, angle: L2CValue, param_3: L2CValue) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut angle = angle.get_f32();

    if lr == -1.0 {
        angle = 180.0 - angle;
    }

    if angle > 180.0 {
        angle = 360.0 - angle;
    }

    if param_3.get_f32() < 1.0 {
        angle = fighter.lerp(0.0_f32.into(), angle.into(), param_3).get_f32();
    }

    ModelModule::set_joint_rotate(
        fighter.module_accessor,
        Hash40::new("rot"),
        &Vector3f{x: -angle, y: 0.0, z: 0.0},
        MotionNodeRotateCompose { _address: 0 },
        MotionNodeRotateOrder { _address: 0 }
    );
}

pub unsafe extern "C" fn special_s_set_cursor_on_posture(fighter: &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR_ON_POSTURE);
    let cursor_offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("cursor_offset_y"));
    let scale = PostureModule::scale(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, cursor_offset_y * scale, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CURSOR_OFFSET_Y);
}

pub unsafe extern "C" fn special_s_reset_angle(fighter: &mut L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let angle = if lr == -1.0 {
        180.0
    }
    else {
        0.0
    };
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
}

mod special_s_attack;

pub fn install(agent: &mut smashline::Agent) {
    special_s_attack::install(agent);
}