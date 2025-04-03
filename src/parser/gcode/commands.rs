use crate::types::{ExtrudeAmountType, FeedrateAmountType, LocationType, PowerType};
use std::fmt::Debug;

pub(super) trait GcodeExecutable: Debug {}

/// Rapid move
pub(super) struct G0Move {}

/// Linear move
#[derive(Default, Debug)]
pub(super) struct G1Move {
    /// Xnnn
    x_target: Option<LocationType>,
    /// Ynnn
    y_target: Option<LocationType>,
    /// Znnn
    z_target: Option<LocationType>,
    /// Ennn
    amount_to_extrude: Option<ExtrudeAmountType>,
    /// Fnnn
    feedrate_per_minute: Option<FeedrateAmountType>,
    //  Hnnn and Rnnn not supported ATM
    laser_power: Option<PowerType>,
}

impl GcodeExecutable for G1Move {}
