use crate::imports::status_imports::*;

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_GlideStart_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_GLIDE_START, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn status_GlideStart_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
    let lr = PostureModule::lr(fighter.module_accessor);
    let glide_start_air_accel_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_start_air_accel_y_mul"));
    let glide_start_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_start_speed_x_mul"));
    let glide_start_speed_y_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_start_speed_y_add"));
    KineticEnergy::reset_energy(gravity, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: 0.0, y: -glide_start_air_accel_y_mul}, &Vector3f{x: 0.0, y: -glide_start_air_accel_y_mul, z: 0.0}, fighter.module_accessor);
    KineticEnergy::reset_energy(motion, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: glide_start_speed_x_mul * lr, y: 0.0}, &Vector3f{x: glide_start_speed_x_mul * lr, y: 0.0, z: 0.0}, fighter.module_accessor);
    KineticUtility::reset_enable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor, *ENERGY_STOP_RESET_TYPE_GLIDE_START, Vector2f{x: 0.0, y: glide_start_speed_y_add}, Vector3f{x: 0.0, y: glide_start_speed_y_add, z: 0.0});
    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_GlideStart_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(GlideStart_Main_Sub as *const () as _))
}

unsafe extern "C" fn GlideStart_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE) || 
    WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && 
        MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_GlideStart_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_GlideStart_Pre,
        status_GlideStart_Init,
        status_GlideStart_Main,
        status_GlideStart_End,
    );
}