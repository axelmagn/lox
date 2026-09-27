const std = @import("std");
const global = @import("global.zig");
const debug = @import("debug.zig");
const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Value = @import("value.zig").Value;

const print_value = @import("value.zig").print;

// NOTE: we use a static VM global.
// it is not pub, so it lives here instead of `global.zig`
var vm: VM = undefined;

pub const VM = struct {
    chunk: *Chunk,
    ip: [*]u8,
    stack_top: usize,
    stack: [stack_max]Value,

    const stack_max = 256;
};

pub const InterpretError = error{
    compile_error,
    runtime_error,
};

pub fn init() void {
    reset_stack();
}
pub fn deinit() void {}

pub fn interpret(chunk: *Chunk) !void {
    vm.chunk = chunk;
    vm.ip = vm.chunk.code.ptr;
    return run();
}

pub fn push(value: Value) void {
    std.debug.assert(vm.stack_top < VM.stack_max);
    vm.stack[vm.stack_top] = value;
    vm.stack_top += 1;
}

pub fn pop() Value {
    std.debug.assert(vm.stack_top > 0);
    vm.stack_top -= 1;
    return vm.stack[vm.stack_top];
}

fn run() !void {
    while (true) {
        if (debug.trace_execution) {
            try global.stdout().print(" ", .{});
            for (0..vm.stack_top) |idx| {
                const slot = &vm.stack[idx];
                try global.stdout().print("[ ", .{});
                try print_value(slot.*);
                try global.stdout().print(" ]", .{});
            }
            try global.stdout().print("\n", .{});
            _ = try debug.disassemble_instruction(
                vm.chunk.*,
                @intCast(vm.ip - vm.chunk.code.ptr),
            );
        }

        switch (read_op()) {
            .constant => {
                const constant = read_constant();
                push(constant);
            },
            .add => {
                const b = pop();
                const a = pop();
                push(a + b);
            },
            .subtract => {
                const b = pop();
                const a = pop();
                push(a - b);
            },
            .multiply => {
                const b = pop();
                const a = pop();
                push(a * b);
            },
            .divide => {
                const b = pop();
                const a = pop();
                push(a / b);
            },
            .negate => {
                push(-pop());
            },
            .@"return" => {
                try print_value(pop());
                try global.stdout().print("\n", .{});
                return;
            },
            _ => return error.bad_instruction,
        }

        try global.stderr().flush();
        try global.stdout().flush();
    }
}

inline fn read_byte() u8 {
    defer vm.ip += 1;
    return vm.ip[0];
}

inline fn read_op() OpCode {
    defer vm.ip += 1;
    return @enumFromInt(vm.ip[0]);
}

inline fn read_constant() Value {
    return vm.chunk.constants.values[read_byte()];
}

inline fn reset_stack() void {
    vm.stack_top = 0;
}
