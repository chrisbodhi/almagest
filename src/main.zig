const std = @import("std");
const testing = std.testing;

const conversions = @import("conversions.zig");
const degrees_to_radians = conversions.degrees_to_radians;
const tame_float = conversions.tame_float;

// Accept angle, in degrees
fn big_sine(angle: f64) f64 {
    const radians = degrees_to_radians(angle);
    const sin_value = std.math.sin(radians);

    return tame_float(sin_value);
}

test "calc big number" {
    try testing.expectEqual(big_sine(36_000_030), 0.5);
}
