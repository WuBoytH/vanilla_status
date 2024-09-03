use super::*;

pub unsafe extern "C" fn lucario_special_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        lucario_special_air_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        lucario_special_set_ground(fighter);
        lucario_special_ground_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}

pub unsafe extern "C" fn lucario_special_set_ground(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
}

pub unsafe extern "C" fn lucario_special_set_air(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}

pub unsafe extern "C" fn lucario_special_ground_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
        MotionModule::change_motion_inherit_frame_keep_rate(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    }
}

pub unsafe extern "C" fn lucario_special_air_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
        MotionModule::change_motion_inherit_frame_keep_rate(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    }
}

pub unsafe extern "C" fn lucario_special_n_save_charge_status(fighter: &mut L2CFighterCommon) {
    let kind = fighter.global_table[KIND].get_i32();
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_global = fighter.global_table[STATUS_KIND].get_i32();
    let statuses = if kind != *FIGHTER_KIND_KIRBY {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    }
    else {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    };
    if status != statuses[0]
    || status_global == statuses[1]
    || status_global == statuses[3] {
        if status == statuses[1] {
            if status_global != statuses[2] {
                if status_global != statuses[3] {
                    lucario_special_n_save_charge_status_eff(fighter);
                    return;
                }
            }
        }
        else if status == statuses[2] {
            if status_global != statuses[3] {
                lucario_special_n_save_charge_status_eff(fighter);
                return;
            }
        }
        else if status != statuses[3] {
            lucario_special_n_save_charge_status_shoot(fighter);
            return;
        }
        else {
            if !ArticleModule::is_exist(fighter.module_accessor, statuses[5]) {
                lucario_special_n_save_charge_status_shoot(fighter);
                return;
            }
        }
    }
    lucario_special_n_save_charge_status_eff(fighter);
}

unsafe extern "C" fn lucario_special_n_save_charge_status_eff(fighter: &mut L2CFighterCommon) {
    let kind = fighter.global_table[KIND].get_i32();
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_global = fighter.global_table[STATUS_KIND].get_i32();
    let statuses = if kind != *FIGHTER_KIND_KIRBY {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    }
    else {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    };
    let save_charge = if status_global != statuses[4] {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        false
    }
    else {
        if status != statuses[1] {
            if status != statuses[2] {
                EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
                false
            }
            else {
                EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0);
                true
            }
        }
        else {
            true
        }
    };
    if !save_charge {
        FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, false, 0);
    }
    else {
        let article = ArticleModule::get_article(fighter.module_accessor, 0);
        if article != std::ptr::null_mut::<smash::app::Article>() {
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let auraball = sv_battle_object::module_accessor(object_id);
            let charge = WorkModule::get_int(auraball, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
            FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, true, charge);
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor, statuses[5], ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    lucario_special_n_save_charge_status_shoot(fighter);
}

unsafe extern "C" fn lucario_special_n_save_charge_status_shoot(fighter: &mut L2CFighterCommon) {
    let kind = fighter.global_table[KIND].get_i32();
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_global = fighter.global_table[STATUS_KIND].get_i32();
    let statuses = if kind != *FIGHTER_KIND_KIRBY {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    }
    else {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    };
    if status == statuses[0] {
        if status_global == statuses[3] {
            EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        }
    }
    if status == statuses[3] {
        FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, false, 0);
    }
}
