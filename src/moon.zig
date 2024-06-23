// "Position of the Moon", Meeus, Ch. 45
const std = @import("std");

/// Geocentric longitude of the Moon
var @"λ" = undefined;

/// Geocentric latitude of the Moon
var @"β" = undefined;

/// Distance between centers of Earth and moon, in km
/// TODO: this note does not show up in ZLS; file a ticket
// var @"∆": f64 = undefined;

fn get_equitorial_horizontal_parallax(@"∆": f64) f64 {
    return std.math.asin(6378.14 / @"∆");
}
