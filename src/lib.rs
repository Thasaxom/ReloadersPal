struct Load {
    load_id: i32,
    powder_id: i32,
    casing_id: i32,
    projectile_id: i32,
    powder_weight: f32,
    primer_make: String,
    primer_lot: String,
    headstamp: String,
    brass_lot: String,
    trim_to_length: f32,
    cartridge_overall_length: f32,
    crimp_diameter: f32,
}

struct BallisticTest {
    test_id: i32,
    load_id: i32,
    air_pressure: f32,
    altitude: f32,
    air_temperature: f32,
    wind_speed: f32,
    wind_direction: String,
    barrel_length: f32,
    twist_rate: f32,
    distance_to_target: f32,
    date: String,
}

struct Casing {
    casing_id: i32,
    name: String,
    diameter: f32,
    primer_size: String,
    case_type: String,
    max_psi: f32,
    max_cup: f32,
}

struct Projectile {
    projectile_id: i32,
    casing_id: i32,
    manufacturer: String,
    diameter: f32,
    weight: f32,
    type: String,
    length: f32,
    sectional_density: f32,
}

struct Powder {
    powder_id: i32,
    manufacturer: String,
    type: String,
}
