// Copyright 2021-Present Datadog, Inc. https://www.datadoghq.com/
// SPDX-License-Identifier: Apache-2.0
// Fork of DataDog/libdatadog v48.0.0 (0be9d86) with the cbindgen header
// generation removed (same reason as libdd-common-ffi/build.rs). The
// Windows version info embedding is kept since it does not invoke
// cargo metadata.
fn main() {
    // No-op: header generation and version info intentionally skipped.
    // The version info embedding is Windows-only and irrelevant for
    // the Linux/macOS Python extension this fork builds.
}
