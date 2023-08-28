use crate::imports::acmd_imports::*;

#[acmd_script( agent = "lucario", script = "effect_hadou_l" , category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_hadou_handl(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou"),
            Hash40::new("handl"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

#[acmd_script( agent = "lucario", script = "effect_hadou_r" , category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_hadou_handr(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou"),
            Hash40::new("handr"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

#[acmd_script( agent = "lucario", script = "effect_hadoum_l" , category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_hadoum_handl(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou_m"),
            Hash40::new("handl"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

#[acmd_script( agent = "lucario", script = "effect_hadoum_r" , category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_hadoum_handr(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou_m"),
            Hash40::new("handr"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

#[acmd_script( agent = "lucario", script = "effect_hadoul_l" , category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_hadoul_handl(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou_l_l"),
            Hash40::new("handl"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

#[acmd_script( agent = "lucario", script = "effect_hadoul_r" , category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_hadoul_handr(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou_l_r"),
            Hash40::new("handr"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucario_hadou_handl,
        lucario_hadou_handr,
        lucario_hadoum_handl,
        lucario_hadoum_handr,
        lucario_hadoul_handl,
        lucario_hadoul_handr
    );
}
