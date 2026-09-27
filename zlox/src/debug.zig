const std = @import("std");
const global = @import("global.zig");
const value = @import("value.zig");
const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;

pub const trace_execution = true;

pub fn disassemble_chunk(chunk: Chunk, name: []const u8) !void {
    try global.stdout().print("== {s} ==\n", .{name});

    var offset: usize = 0;
    while (offset < chunk.count) {
        offset = try disassemble_instruction(chunk, offset);
    }
}
pub fn disassemble_instruction(chunk: Chunk, offset: usize) !usize {
    try global.stdout().print("{d:04} ", .{offset});

    if (offset > 0 and chunk.lines[offset] == chunk.lines[offset - 1]) {
        try global.stdout().print("   | ", .{});
    } else {
        try global.stdout().print("{d:4} ", .{chunk.lines[offset]});
    }

    const op = chunk.op(offset);
    return switch (op) {
        .constant => constant_instruction(op, chunk, offset),
        .@"return",
        .add,
        .subtract,
        .multiply,
        .divide,
        .negate,
        => simple_instruction(op, offset),
        _ => unknown_instruction(op, offset),
    };
}

fn simple_instruction(op: OpCode, offset: usize) !usize {
    var buf: [32]u8 = undefined;
    const name = try op_name(op, &buf);
    try global.stdout().print("{s}\n", .{name});
    return offset + 1;
}

fn constant_instruction(op: OpCode, chunk: Chunk, offset: usize) !usize {
    var buf: [32]u8 = undefined;
    const name = try op_name(op, &buf);
    const addr = chunk.code[offset + 1];
    const value_ = chunk.constants.values[addr];
    try global.stdout().print("{s:<16} {d:4} '", .{ name, addr });
    try value.print(value_);
    try global.stdout().print("'\n", .{});

    return offset + 2;
}

fn unknown_instruction(op: OpCode, offset: usize) !usize {
    try global.stdout().print("Unknown opcode {d}\n", .{op});
    return offset + 1;
}

fn op_name(op: OpCode, buf: []u8) ![]u8 {
    @memcpy(buf[0..3], "OP_");
    const name = @tagName(op);
    if (name.len > buf.len) return error.insufficient_buffer;
    for (0..name.len) |i| buf[i + 3] = std.ascii.toUpper(name[i]);
    return buf[0 .. name.len + 3];
}
