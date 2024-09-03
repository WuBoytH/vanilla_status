use super::*;

pub unsafe extern "C" fn buddy_special_s_set_armor(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR) {
        buddy_special_s_remove_armor(fighter);
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP) {
            HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
        }
    }
}

pub unsafe extern "C" fn buddy_special_s_remove_armor(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP) {
        HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    }
}