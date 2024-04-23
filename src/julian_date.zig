// "Astronomical Algorithms" by Jean Meeus
// Ch 2, Ch 21

const std = @import("std");
const testing = std.testing;

/// Calculate T from the Julian Ephemeris Date
pub fn calc_T(jde: f64) f64 {
    return (jde - 2_451_545.0) / 36525.0;
}

// TODO: revisit the check for Julian or Gregorian
// TODO: accept a date struct, instead
// TODO: think about the return type, too
pub fn calc_jd(y: f32, m: f32, day: f32) f32 {
    var year = y;
    var month = m;
    if (month <= 2) {
        year -= 1;
        month += 12;
    }
    const A: f32 = @trunc(year / 100.0);
    // Set to 0 if year is in Julian calendar
    const B: f32 = if (year < 1582)
        0
    else
        2.0 - A + @trunc(A / 4.0);
    const first: f32 = @trunc(365.25 * (year + 4716.0));
    const second: f32 = @trunc(30.6001 * (month + 1.0));

    return first + second + day + B - 1524.5;
}

test calc_jd {
    const expected_sputnik: f32 = 2_436_116.31;
    const jd_sputnik = calc_jd(1957, 10, 4.81);

    try testing.expectEqual(expected_sputnik, jd_sputnik);

    const expected_333: f32 = 1_842_713.0;
    const jd_333: f32 = calc_jd(333, 1, 27.5);

    try testing.expectEqual(expected_333, jd_333);
}
