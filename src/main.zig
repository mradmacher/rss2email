// zig run main.zig -lsqlite3 -lc

const std = @import("std");
const c = @cImport({
    @cInclude("sqlite3.h");
});
const print = std.debug.print;

const DB = struct {
    db: *c.sqlite3,

    fn init(db: *c.sqlite3) DB {
         return .{ .db = db };
    }

    fn deinit(self: DB) void {
        _ = c.sqlite3_close(self.db);
    }

    fn execute(self: DB, query: [:0]const u8) !void {
        var errmsg: [*c]u8 = undefined;
        if (c.SQLITE_OK != c.sqlite3_exec(self.db, query, null, null, &errmsg)) {
            defer c.sqlite3_free(errmsg);
            print("Exec query failed: {s}\n", .{errmsg});
            return error.execError;
        }
        return;
    }
};

pub fn main() !void {
    const version = c.sqlite3_libversion();
    print("libsqlite3 version is {s}\n", .{version});
}
