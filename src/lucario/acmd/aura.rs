use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "lucario", script = 0xed5bd8ca4 , category = ACMD_EFFECT, low_priority )]
unsafe fn luacrio_hadou_handl(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucario", script = 0xe2fb2b1c7 , category = ACMD_EFFECT, low_priority )]
unsafe fn luacrio_hadou_handr(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucario", script = 0xf94ec4c4d , category = ACMD_EFFECT, low_priority )]
unsafe fn luacrio_hadoum_handl(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucario", script = 0xf6ee3712e , category = ACMD_EFFECT, low_priority )]
unsafe fn luacrio_hadoum_handr(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucario", script = 0xf952e267a , category = ACMD_EFFECT, low_priority )]
unsafe fn luacrio_hadoul_handl(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "lucario", script = 0xf6f211b19 , category = ACMD_EFFECT, low_priority )]
unsafe fn luacrio_hadoul_handr(fighter: &mut L2CAgentBase) {
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
        luacrio_hadou_handl,
        luacrio_hadou_handr,
        luacrio_hadoum_handl,
        luacrio_hadoum_handr,
        luacrio_hadoul_handl,
        luacrio_hadoul_handr
    );
}