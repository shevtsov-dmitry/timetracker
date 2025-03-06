const std = @import("std");
const time = std.time;

pub fn main() !void {
    const begin_count_stamp = time.timestamp();
    while (true) {
        time.sleep(1 * std.time.ns_per_s);
        const current_time_stamp = time.timestamp();
        std.debug.print("time: {}\n", .{current_time_stamp - begin_count_stamp});
    }
}
