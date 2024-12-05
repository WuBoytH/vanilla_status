#![allow(dead_code, improper_ctypes_definitions, unused_unsafe)]

use super::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct PaddedVec2 {
    pub x: f32,
    pub y: f32,
    pub padding: u64
}

impl PaddedVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            padding: 0
        }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            padding: 0
        }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[repr(C)]
pub struct KineticEnergyVTable {
    pub destructor: extern "C" fn(&mut KineticEnergy),
    pub deleter: extern "C" fn(*mut KineticEnergy),
    pub unk: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub update: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_speed: extern "C" fn(&mut KineticEnergy) -> *mut PaddedVec2,
    pub initialize: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_some_flag: extern "C" fn(&mut KineticEnergy) -> bool,
    pub set_some_flag: extern "C" fn(&mut KineticEnergy, bool),
    pub setup_energy: extern "C" fn(&mut KineticEnergy, u32, &Vector3f, u64, &mut BattleObjectModuleAccessor),
    pub clear_energy: extern "C" fn(&mut KineticEnergy),
    pub unk2: extern "C" fn(&mut KineticEnergy),
    pub set_speed: extern "C" fn (&mut KineticEnergy, &Vector2f),
    pub mul_accel: extern "C" fn(&mut KineticEnergy, &Vector2f),
    // ...

}

#[repr(C)]
pub struct KineticEnergy {
    pub vtable: &'static KineticEnergyVTable,
    pub _x8: u64, // probably padding
    pub speed: PaddedVec2,
    pub rot_speed: PaddedVec2,
    pub enable: bool,
    pub unk2: [u8; 0x3], // probably padding
    pub x34: f32,
    pub x38: f32,
    pub x3c: f32,
    pub accel: PaddedVec2,
    pub speed_max: PaddedVec2,
    pub speed_brake: PaddedVec2,
    pub speed_limit: PaddedVec2,
    pub _x80: u8,
    pub consider_ground_friction: bool,
    pub active_flag: bool, // no clue?
    pub _x83: u8,
    pub energy_reset_type: u32,
}

impl KineticEnergy {
    pub fn adjust_speed_for_ground_normal(speed: &PaddedVec2, module_accessor: &mut BattleObjectModuleAccessor) -> PaddedVec2 {
        #[skyline::from_offset(0x47b4f0)]
        extern "C" fn adjust_speed_for_ground_normal_internal(speed: smash_rs::cpp::simd::Vector2, module_accessor: &mut BattleObjectModuleAccessor) -> smash_rs::cpp::simd::Vector2;

        unsafe {
            let result = adjust_speed_for_ground_normal_internal(smash_rs::cpp::simd::Vector2 { vec: [speed.x, speed.y] }, module_accessor);
            PaddedVec2::new(result.vec[0], result.vec[1])
        }
    }

    pub fn process(&mut self, module_accessor: &mut BattleObjectModuleAccessor) {
        unsafe {
            #[skyline::from_offset(0x47bf90)]
            extern "C" fn process_energy(energy: &mut KineticEnergy, module_accessor: &mut BattleObjectModuleAccessor);

            process_energy(self, module_accessor)
        }
    }

    pub fn update(&mut self, module_accessor: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.update)(self, module_accessor)
        }
    }

    pub fn get_speed<'a>(&'a mut self) -> &'a mut PaddedVec2 {
        unsafe {
            std::mem::transmute((self.vtable.get_speed)(self))
        }
    }

    pub fn initialize(&mut self, module_accessor: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.initialize)(self, module_accessor)
        }
    }

    pub fn get_some_flag(&mut self) -> bool {
        unsafe {
            (self.vtable.get_some_flag)(self)
        }
    }

    pub fn set_some_flag(&mut self, flag: bool) {
        unsafe {
            (self.vtable.set_some_flag)(self, flag)
        }
    }

    pub fn setup_energy(&mut self, reset_type: u32, incoming_speed: &Vector3f, some: u64, module_accessor: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.setup_energy)(self, reset_type, incoming_speed, some, module_accessor)
        }
    }

    pub fn clear_energy(&mut self) {
        unsafe {
            (self.vtable.clear_energy)(self)
        }
    }

    pub fn unk2(&mut self) {
        unsafe {
            (self.vtable.unk2)(self)
        }
    }

    pub fn set_speed(&mut self, speed: &Vector2f) {
        unsafe {
            (self.vtable.set_speed)(self, speed)
        }
    }

    pub fn mul_accel(&mut self, mul: &Vector2f) {
        unsafe {
            (self.vtable.mul_accel)(self, mul)
        }
    }

}

#[skyline::hook(offset = 0x3463300)]
unsafe extern "C" fn murabito_clayrocket_kinetic_type_handler(
    _vtable: u64,
    kinetic_type: i32,
    module_accessor: *mut BattleObjectModuleAccessor
) {
    if kinetic_type == *WEAPON_KINETIC_TYPE_MURABITO_CLAYROCKET_FALL {
        // let sum_speed = KineticModule::get_sum_speed(module_accessor, 1);
        KineticModule::unable_energy_all(module_accessor);

        let fall_decel = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("fall_decel"));
        let fall_gravity = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("fall_gravity"));
        let fall_limit_gravity = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("fall_limit_gravity"));

        let kinetic_energy = KineticModule::get_energy(module_accessor, 0) as *mut KineticEnergy;
        (*kinetic_energy).enable = true;
        (*kinetic_energy).accel = PaddedVec2::zeros();
        (*kinetic_energy).speed_max = PaddedVec2::zeros();
        (*kinetic_energy).speed_brake = PaddedVec2::new(fall_decel, 0.0);
        (*kinetic_energy).speed_limit = PaddedVec2::new(-(*kinetic_energy).speed_limit.x, 0.0);

        let kinetic_energy = KineticModule::get_energy(module_accessor, 2) as *mut KineticEnergy;
        (*kinetic_energy).enable = true;
        (*kinetic_energy).x34 = -fall_gravity;
        (*kinetic_energy).x38 = fall_limit_gravity;
    }
    else if kinetic_type == *WEAPON_KINETIC_TYPE_MURABITO_CLAYROCKET_FLY {
        let lr = PostureModule::lr(module_accessor);
        let control_speed = WorkModule::get_int(module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_INT_CONTROL_SPEED);
        let ground = WorkModule::is_flag(module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_GROUND);
        let ride = WorkModule::is_flag(module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_RIDE);
        let kinetic_type = KineticModule::get_kinetic_type(module_accessor);
        let speed = if kinetic_type == *WEAPON_KINETIC_TYPE_MURABITO_CLAYROCKET_FLY {
            KineticModule::get_sum_speed(module_accessor, 1)
        }
        else {
            let param = if ride {
                hash40("ride_init_speed")
            }
            else {
                hash40("fly_speed")
            };
            WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), param) * lr
        };

        KineticModule::unable_energy_all(module_accessor);

        let param = if ride {
            hash40("ride_accel")
        }
        else {
            hash40("fly_accel")
        };
        let accel = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), param);

        let param = if ground {
            hash40("fly_limit_speed_ground")
        }
        else {
            hash40("fly_limit_speed_air")
        };
        let mut limit = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), param);

        if ride {
            let param = if control_speed == 2 {
                hash40("ride_limit_speed_mul_negative")
            }
            else if control_speed == 1 {
                hash40("ride_limit_speed_mul_positive")
            }
            else if control_speed == 0 {
                hash40("ride_limit_speed_mul")
            }
            else {
                0
            };
            if param != 0 {
                limit *= WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), param);
            }
        }

        let gravity = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("fly_gravity"));
        let mut gravity_limit = WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("fly_limit_gravity"));
        if ride {
            gravity_limit *= WorkModule::get_param_float(module_accessor, hash40("param_clayrocket"), hash40("ride_limit_gravity_mul"));
        }

        let speed_max = if WorkModule::is_flag(module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_GET_OFF)
        && speed <= limit {
            speed
        }
        else {
            limit
        };

        let accel = accel * lr.signum();

        let kinetic_energy = KineticModule::get_energy(module_accessor, 0) as *mut KineticEnergy;

        (*kinetic_energy).setup_energy(
            0,
            &Vector3f{x: speed, y: 0.0, z: 0.0},
            0,
            &mut *module_accessor
        );
        
        (*kinetic_energy).enable = true;

        let kinetic_energy = KineticModule::get_energy(module_accessor, 0) as *mut KineticEnergy;

        (*kinetic_energy).accel = PaddedVec2::new(accel, 0.0);
        (*kinetic_energy).speed_max = PaddedVec2::new(speed_max, 0.0);
        (*kinetic_energy).speed_limit = PaddedVec2::new(speed_max, 0.0);

        let kinetic_energy = KineticModule::get_energy(module_accessor, 2) as *mut KineticEnergy;

        (*kinetic_energy).x38 = gravity_limit;
        (*kinetic_energy).enable = true;
        // set speed? maybe unused?
        (*kinetic_energy).x34 = -gravity;
    }
    else if kinetic_type == 2 {
        KineticModule::unable_energy_all(module_accessor);
    }
}

pub fn install() {
    skyline::install_hooks!(
        murabito_clayrocket_kinetic_type_handler
    );
}