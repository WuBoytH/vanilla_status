use super::*;

unsafe extern "C" fn shoot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_ATTACK
    );
    0.into()
}

unsafe extern "C" fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if GroundModule::is_touch(
        weapon.module_accessor,
        (
            *GROUND_TOUCH_FLAG_RIGHT |
            *GROUND_TOUCH_FLAG_LEFT |
            *GROUND_TOUCH_FLAG_DOWN_LEFT |
            *GROUND_TOUCH_FLAG_DOWN_RIGHT |
            *GROUND_TOUCH_FLAG_UP
        ) as u32
    ) {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_WIIFIT_SUNBULLET_INSTANCE_WORK_ID_FLAG_GROUND_TOUCH);
        let width = GroundModule::get_width(weapon.module_accessor);
        let part_size = AttackModule::part_size(weapon.module_accessor) as i32;
        if part_size > -1 {
            let mut count = 0;
            while count < part_size {
                if AttackModule::is_attack(weapon.module_accessor, count, false) {
                    AttackModule::set_size(weapon.module_accessor, count, width);
                }
                count += 1;
            }
        }
        StopModule::set_other_stop(weapon.module_accessor, 2, StopOtherKind(0));
    }

    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let pos_x = PostureModule::pos_x(weapon.module_accessor);
        let pos_y = PostureModule::pos_y(weapon.module_accessor);
        let pos_z = PostureModule::pos_z(weapon.module_accessor);

        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y + 0.1, z: pos_z});
    }

    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_WIIFIT_SUNBULLET_INSTANCE_WORK_ID_FLAG_GROUND_TOUCH) {
        weapon.clear_lua_stack();
        lua_args!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("wiifit_taiyo_shot"),
            Hash40::new("top"),
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            true,
            EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS,
            0,
            -1
        );
        let eff_handle = weapon.pop_lua_stack(1).get_u32();
        WorkModule::set_int(weapon.module_accessor, eff_handle as i32, *WEAPON_WIIFIT_SUNBULLET_INSTANCE_WORK_ID_INT_EFFECT_ID_SHOT);
    }

    weapon.shift(L2CValue::Ptr(shoot_shift as *const () as _))
}

unsafe extern "C" fn shoot_shift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let flag = if weapon.global_table[STATUS_FRAME].get_i32() > 0 {
        *GROUND_TOUCH_FLAG_DOWN
    }
    else {
        *GROUND_TOUCH_FLAG_RIGHT |
        *GROUND_TOUCH_FLAG_LEFT |
        *GROUND_TOUCH_FLAG_DOWN_LEFT |
        *GROUND_TOUCH_FLAG_DOWN_RIGHT |
        *GROUND_TOUCH_FLAG_UP
    };

    if GroundModule::is_touch(weapon.module_accessor, flag as u32) {
        let pos_x = PostureModule::pos_x(weapon.module_accessor);
        let pos_y = PostureModule::pos_y(weapon.module_accessor);
        let pos_z = PostureModule::pos_z(weapon.module_accessor);

        EffectModule::req(
            weapon.module_accessor,
            Hash40::new("wiifit_taiyo_hit"),
            &Vector3f{x: pos_x, y: pos_y, z: pos_z},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            1.0,
            *EFFECT_SUB_ATTRIBUTE_NONE as u32,
            -1,
            false,
            0
        );
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }

    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_WIIFIT_SUNBULLET_INSTANCE_WORK_ID_FLAG_GROUND_TOUCH)
    && !StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return 0.into();
        }
    }

    0.into()
}

unsafe extern "C" fn shoot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let shot_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_WIIFIT_SUNBULLET_INSTANCE_WORK_ID_INT_EFFECT_ID_SHOT);
    if shot_id != *EFFECT_HANDLE_NULL {
        EffectModule::kill(weapon.module_accessor, shot_id as u32, true, true);
    }
    EffectModule::detach_kind(weapon.module_accessor, Hash40::new("wiifit_taiyo_bullet"), 5);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("wiifit_taiyo_lensflare"), true, true);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_WIIFIT_SUNBULLET_STATUS_KIND_SHOOT, shoot_pre);
    agent.status(Main, *WEAPON_WIIFIT_SUNBULLET_STATUS_KIND_SHOOT, shoot_main);
    agent.status(End, *WEAPON_WIIFIT_SUNBULLET_STATUS_KIND_SHOOT, shoot_end);
}