const std = @import("std");

const debug = @import("debug.zig");
const global = @import("global.zig");
const vm = @import("vm.zig");

const Chunk = @import("chunk.zig").Chunk;

pub fn main(init: std.process.Init) !u8 {
    global.init(init);

    vm.init();
    defer vm.deinit();

    var chunk: Chunk = .init();
    defer chunk.deinit();

    var constant = try chunk.add_constant(1.2);
    try chunk.write_op(.constant, 123);
    try chunk.write_byte(constant, 123);

    constant = try chunk.add_constant(3.4);
    try chunk.write_op(.constant, 123);
    try chunk.write_byte(constant, 123);

    try chunk.write_op(.add, 123);

    constant = try chunk.add_constant(5.6);
    try chunk.write_op(.constant, 123);
    try chunk.write_byte(constant, 123);

    try chunk.write_op(.divide, 123);
    try chunk.write_op(.negate, 123);

    try chunk.write_op(.@"return", 123);

    try debug.disassemble_chunk(chunk, "test chunk");
    try global.stdout().print("-- RUNNING --\n", .{});
    _ = try vm.interpret(&chunk);

    try global.flush();
    return 0;
}
