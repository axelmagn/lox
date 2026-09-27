const std = @import("std");
const global = @import("global.zig");
const mem = @import("memory.zig");
const Value = @import("value.zig").Value;
const ValueArray = @import("value.zig").ValueArray;

pub const Chunk = struct {
    count: usize,
    code: []u8,
    lines: []u32,
    constants: ValueArray,

    pub fn init() Chunk {
        var chunk: Chunk = .{
            .count = 0,
            .code = undefined,
            .lines = undefined,
            .constants = .init(),
        };
        chunk.code.len = 0;
        chunk.lines.len = 0;
        return chunk;
    }

    pub fn deinit(chunk: *Chunk) void {
        chunk.constants.deinit();
        global.gpa().free(chunk.code);
        global.gpa().free(chunk.lines);
        chunk.* = .init();
    }

    pub inline fn capacity(chunk: Chunk) usize {
        return chunk.code.len;
    }

    pub fn op(chunk: Chunk, offset: usize) OpCode {
        return @enumFromInt(chunk.code[offset]);
    }

    pub fn grow(chunk: *Chunk) !void {
        if (chunk.capacity() == 0) {
            chunk.code = try global.gpa().alloc(u8, 8);
            chunk.lines = try global.gpa().alloc(u32, 8);
        } else {
            chunk.code = try mem.grow_array(chunk.code);
            chunk.lines = try mem.grow_array(chunk.lines);
        }
        std.debug.assert(chunk.code.len == chunk.lines.len);
    }

    pub fn write_byte(chunk: *Chunk, byte: u8, line: u32) !void {
        if (chunk.capacity() < chunk.count + 1) {
            try chunk.grow();
        }
        chunk.code[chunk.count] = byte;
        chunk.lines[chunk.count] = line;
        chunk.count += 1;
    }

    pub fn write_op(chunk: *Chunk, op_: OpCode, line: u32) !void {
        try write_byte(chunk, @intFromEnum(op_), line);
    }

    pub fn add_constant(chunk: *Chunk, value: Value) !u8 {
        const addr: u8 = @intCast(chunk.constants.count);
        try chunk.constants.write(value);
        return addr;
    }
};

pub const OpCode = enum(u8) {
    constant,
    @"return",
    _,
};
