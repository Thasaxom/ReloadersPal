pub struct Load {
    pub load_id: i32,
    pub powder_id: i32,
    pub casing_id: i32,
    pub projectile_id: i32,
    pub powder_weight: f64,
    pub primer_make: String,
    pub primer_lot: String,
    pub headstamp: String,
    pub brass_lot: String,
    pub trim_to_length: f64,
    pub cartridge_overall_length: f64,
    pub crimp_diameter: f64,
}

pub struct BallisticTest {
    pub test_id: i32,
    pub load_id: i32,
    pub air_pressure: f64,
    pub altitude: f64,
    pub air_temperature: f64,
    pub wind_speed: f64,
    pub wind_direction: String,
    pub barrel_length: f64,
    pub twist_rate: f64,
    pub distance_to_target: f64,
    pub date: String,
}

pub struct Casing {
    pub casing_id: i32,
    pub name: String,
    pub primer_size: String,
    pub case_type: String,
    pub max_psi: f64,
    pub max_cup: f64,
}

pub struct Projectile {
    pub projectile_id: i32,
    pub casing_id: i32,
    pub manufacturer: String,
    pub diameter: f64,
    pub weight: f64,
    pub projectile_type: String,
    pub length: f64,
    pub sectional_density: f64,
}

pub struct Powder {
    pub powder_id: i32,
    pub manufacturer: String,
    pub powder_type: String,
}
