// "Astronomical Algorithms", Meeus, p. 67
const std = @import("std");
const mod = std.math.mod;

const expect = std.testing.expect;

const MonthAndDay = struct {
    month: u8,
    day: u8,
};

const DivisionResult = struct {
    quotient: u16,
    remainder: u16,
};

/// Function to take numerator and denominator, and return quotient and remainder
fn get_quotient_and_remainder(num: u16, denom: u16) !DivisionResult {
    const quotient = num / denom;
    const remainder = try mod(u16, num, denom);

    return DivisionResult{ .quotient = quotient, .remainder = remainder };
}

pub fn easter_month_and_day(year: u16) !MonthAndDay {
    const a = try mod(u16, year, 19);

    const bc = try get_quotient_and_remainder(year, 100);
    const b = bc.quotient;
    const c = bc.remainder;

    const de = try get_quotient_and_remainder(b, 4);
    const d = de.quotient;
    const e = de.remainder;

    const f: u16 = (b + 8) / 25;
    const g: u16 = (b - f + 1) / 3;
    const h = try mod(u16, ((19 * a) + b - d - g + 15), 30);

    const ik = try get_quotient_and_remainder(c, 4);
    const i = ik.quotient;
    const k = ik.remainder;

    const l = try mod(u16, 32 + (2 * e) + (2 * i) - h - k, 7);
    const m: u16 = (a + (11 * h) + (22 * l)) / 451;

    // Calculate the sum of the month and day before applying correction
    const month_day_sum: u16 = h + l + 114;
    // Calculate the correction to be applied to the month
    const correction: u16 = 7 * m;

    const month_and_day = try get_quotient_and_remainder(month_day_sum - correction, 31);
    const num_of_month = @as(u8, @intCast(month_and_day.quotient));
    const day = @as(u8, @intCast(month_and_day.remainder + 1));

    return MonthAndDay{ .month = num_of_month, .day = day };
}

// The first valid year, from the Gregorian calendar
test "1583" {
    const md = try easter_month_and_day(1583);

    try expect(md.month == 4);
    try expect(md.day == 10);
}

// Examples from the book
test "1818" {
    const md = try easter_month_and_day(1818);

    try expect(md.month == 3);
    try expect(md.day == 22);
}

test "1954" {
    const md = try easter_month_and_day(1954);

    try expect(md.month == 4);
    try expect(md.day == 18);
}

test "1991" {
    const md = try easter_month_and_day(1991);

    try expect(md.month == 3);
    try expect(md.day == 31);
}

// Most recent
test "2024" {
    const md = try easter_month_and_day(2024);

    try expect(md.month == 3);
    try expect(md.day == 31);
}

// Upcoming
test "2025" {
    const md = try easter_month_and_day(2025);

    try expect(md.month == 4);
    try expect(md.day == 20);
}

// Extreme example from the book
test "2038" {
    const md = try easter_month_and_day(2038);

    try expect(md.month == 4);
    try expect(md.day == 25);
}

// An even more extreme from the book
test "2285" {
    const md = try easter_month_and_day(2285);

    try expect(md.month == 3);
    try expect(md.day == 22);
}
