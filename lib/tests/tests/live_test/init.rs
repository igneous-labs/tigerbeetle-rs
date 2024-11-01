/*
use crate::common::live_test_client;

// TODO: seems like there's a bug in tigerbeetle C client where if you just init a client and then deinit without making any requests,
// `assert(connection.peer != .none);`
// at tigerbeetle/src/message_bus.zig:680:23
// fails, triggering a panic.
// uncomment this test and run it to see
#[test]
fn init_sanity() {
    live_test_client();
}
*/
