const std = @import("std");
const global = @import("global.zig");

pub inline fn grow_capacity(capacity: usize) usize {
    return if (capacity < 8) 8 else capacity * 2;
}

pub inline fn grow_array(array: anytype) !@TypeOf(array) {
    const new_capacity = grow_capacity(array.len);
    std.debug.assert(new_capacity > array.len);
    return try global.gpa().realloc(array, new_capacity);
}
