const std = @import("std");
const global = @import("global.zig");
const mem = @import("memory.zig");

pub const Value = f64;
pub const ValueArray = struct {
    count: usize,
    values: []Value,

    pub fn init() ValueArray {
        var value_arr: ValueArray = .{
            .count = 0,
            .values = undefined,
        };
        value_arr.values.len = 0;
        return value_arr;
    }

    pub fn deinit(value_array: *ValueArray) void {
        global.gpa().free(value_array.values);
        value_array.* = .init();
    }

    pub inline fn capacity(value_array: ValueArray) usize {
        return value_array.values.len;
    }

    pub fn grow(value_array: *ValueArray) !void {
        value_array.values =
            try if (value_array.capacity() == 0)
                global.gpa().alloc(Value, 8)
            else
                mem.grow_array(value_array.values);
    }

    pub fn write(value_array: *ValueArray, value: Value) !void {
        if (value_array.capacity() < value_array.count + 1) {
            try value_array.grow();
        }
        value_array.values[value_array.count] = value;
        value_array.count += 1;
    }
};

pub fn print(value: Value) !void {
    try global.stdout().print("{d}", .{value});
}
