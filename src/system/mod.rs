use crate::types::{LocationType, TemperatureType};

#[derive(Default)]
pub struct SystemConfig {
    bed_config: BedConfig,
    extruder_config: ExtruderConfig,
    global: GlobalConfig,
}

//------------------------------------------------------------------------------------------------
#[derive(Default)]
struct GlobalConfig {
    units_config: UnitsConfig,
    coordinates_config: CoordinatesConfig,
}

#[derive(Default)]
enum UnitsConfig {
    #[default]
    Millimeters,
    Inches,
}

#[derive(Default)]
enum CoordinatesConfig {
    #[default]
    Absolute,
    Relative,
}

//------------------------------------------------------------------------------------------------
#[derive(Default)]
struct BedConfig {
    /// Some if origin is configured, None if it's pending to be configured
    origin: Option<Location>,
    /// Some if printing limit is configured, None if it's pending to be configured
    limit: Option<Location>,
}

//------------------------------------------------------------------------------------------------
#[derive(Default)]
struct ExtruderConfig {
    /// Off by default
    fan_enabled: bool,
    current_temp: TemperatureType,
    /// Always needs to have a value, and its value will be relative to the origin
    /// When printer boots, it'll be 0, 0, 0
    current_location: Location,
}

//------------------------------------------------------------------------------------------------
/// Used to identify locations in a three dimentional space.
/// Coordinates can be negative for out-of-bounds locations
#[derive(Default)]
struct Location {
    x: LocationType,
    y: LocationType,
    z: LocationType,
}

//------------------------------------------------------------------------------------------------
