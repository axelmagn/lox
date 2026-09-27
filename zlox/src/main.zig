const std = @import("std");
const global = @import("global.zig");
const debug = @import("debug.zig");
const Chunk = @import("chunk.zig").Chunk;

pub fn main(init: std.process.Init) !u8 {
    global.init(init);

    var chunk: Chunk = .init();
    defer chunk.deinit();

    const constant = try chunk.add_constant(1.2);
    try chunk.write_op(.constant, 123);
    try chunk.write_byte(constant, 123);

    try chunk.write_op(.@"return", 123);

    try debug.disassemble_chunk(chunk, "test chunk");

    try global.flush();
    return 0;
}
