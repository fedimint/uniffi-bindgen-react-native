/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
 */
ubrn_macros::build_foreign_language_testcases! {
    "tests/bindings/test_strict_byte_arrays.ts" => [Jsi, Wasm, Napi, Wasm2, Jsi2],
    "tests/bindings/test_rustbuffer_alloc.ts" => [Jsi, Jsi2],
    "tests/bindings/test_leak.ts" => [Jsi, Wasm, Napi, Wasm2, Jsi2],
    // [Jsi, Napi] only. Both Wasm flavours copy the argument into linear memory
    // as part of the ABI (classic Wasm at the wasm-bindgen boundary, wasm2
    // through `__ubrn_alloc`), so the fixture's allocator moves for a borrow and
    // a copy alike and the assertion cannot attribute the movement to the
    // lowering. Jsi2 does pass a borrowed `&[u8]` through to Rust by pointer,
    // but it frees a buffer Rust returns only when the JS view over it is
    // garbage collected, so the live-allocation counts the test compares do
    // not settle between its snapshots.
    "tests/bindings/test_borrowed_bytes_no_copy.ts" => [Jsi, Napi],
    // [Jsi, Napi] only: wasm2 aliases the argument's linear memory rather than
    // adopting a copy, and has no "already consumed" marker, so an explicit
    // rustbuffer_free after the callee frees the aliased buffer double-frees.
    // That defensive guard is a separate, tracked wasm2 follow-up.
    "tests/bindings/test_double_free.ts" => [Jsi, Napi, Jsi2],
}
