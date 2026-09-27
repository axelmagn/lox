const std = @import("std");

var gpa_: ?std.mem.Allocator = null;
var arena_: ?std.mem.Allocator = null;

var io_: ?std.Io = null;

var stdout_buf: [1024]u8 = undefined;
var stdout_: ?std.Io.File.Writer = null;
var stderr_buf: [1024]u8 = undefined;
var stderr_: ?std.Io.File.Writer = null;

pub fn init(init_: std.process.Init) void {
    gpa_ = init_.gpa;
    arena_ = init_.arena.allocator();
    io_ = init_.io;

    stdout_ = std.Io.File.stdout().writer(io(), &stdout_buf);
    stderr_ = std.Io.File.stderr().writer(io(), &stderr_buf);
}

pub fn flush() !void {
    try stderr().flush();
    try stdout().flush();
}

pub fn gpa() std.mem.Allocator {
    return gpa_.?;
}

pub fn arena() std.mem.Allocator {
    return arena_.?;
}

pub fn io() std.Io {
    return io_.?;
}

pub fn stdout() *std.Io.Writer {
    return &stdout_.?.interface;
}

pub fn stderr() *std.Io.Writer {
    return &stderr_.?.interface;
}
