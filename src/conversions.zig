// "Astronomical Algorithms", Meeus, Ch. 1
const std = @import("std");

const Angle = struct {
    degrees: i32,
    minutes: u32,
    seconds: f32,
};

const RightAscension = struct {
    hours: i32,
    minutes: u32,
    seconds: f32,
};

// -----
// UTILS
// -----

fn to_f64(comptime T: type, value: T) f64 {
    return @as(f64, @floatFromInt(value));
}

pub fn tame_float(value: f64) f64 {
    return std.math.round(value * 1e6) / 1e6;
}

// -----
// FUNCTIONS
// -----

pub fn right_ascension_to_float(ra: RightAscension) f64 {
    const f = to_f64(i32, ra.hours) * 15.0 + to_f64(u32, ra.minutes) / 4.0 + @as(f64, @floatCast(ra.seconds)) / 240.0;

    return tame_float(f);
}

pub fn degrees_to_radians(degrees: f64) f64 {
    const rad = degrees * (std.math.pi / 180.0);

    return tame_float(rad);
}

pub fn degrees_to_float(degrees: Angle) f64 {
    return to_f64(i32, degrees.degrees) + to_f64(u32, degrees.minutes) / 60.0 + @as(f64, @floatCast(degrees.seconds)) / 3600.0;
}

// -----
// TESTS
// -----

test "degrees to radians: 0" {
    const expected: f64 = 0.0;
    const result = degrees_to_radians(0.0);

    try std.testing.expectEqual(expected, result);
}

test "degrees to float: 0" {
    const expected: f64 = 0.0;
    const result = degrees_to_float(Angle{ .degrees = 0, .minutes = 0, .seconds = 0 });

    try std.testing.expectEqual(expected, result);
}

test "degrees to float: 23º 26' 49\"" {
    const expected: f64 = 23.446_944_444_444_445;
    const result = degrees_to_float(Angle{ .degrees = 23, .minutes = 26, .seconds = 49 });

    try std.testing.expectEqual(expected, result);
}

test "tangent of right ascension: 9h 14m 55.8s" {
    const angle = RightAscension{ .hours = 9, .minutes = 14, .seconds = 55.8 };
    const expected: f64 = -0.877_517;
    var result: f64 = std.math.tan(degrees_to_radians(right_ascension_to_float(angle)));
    result = tame_float(result);

    try std.testing.expectEqual(expected, result);
}

test "right ascension to float" {
    const ra = RightAscension{ .hours = 9, .minutes = 14, .seconds = 55.8 };
    const expected: f64 = 138.7325;
    const result = right_ascension_to_float(ra);

    try std.testing.expectEqual(expected, result);
}
