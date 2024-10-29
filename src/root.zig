const std = @import("std");
const testing = std.testing;

const conversions = @import("conversions.zig");
const julian_date = @import("julian_date.zig");
const symbols = @import("symbols.zig");

comptime {
    // Necessary comptime assignments to trigger test runs
    // using the later `test` block.
    // TODO: There's a better way to do this, so do that, instead. :D
    _ = conversions;
    _ = julian_date;
    _ = symbols;
}

test {
    testing.refAllDecls(@This());
}
