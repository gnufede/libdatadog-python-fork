// Copyright 2021-Present Datadog, Inc. https://www.datadoghq.com/
// SPDX-License-Identifier: Apache-2.0
// Fork of DataDog/libdatadog v48.0.0 (0be9d86) with the cbindgen header
// generation removed: the upstream build script invokes cargo metadata
// on the workspace, which fails against CI mirrors whose checksums
// differ from the internal lock file. The generated C headers are
// consumed by dd-trace-py's C/C++ profiling integration, not by the
// Python cdylib that depends on this fork.
fn main() {
    // No-op: header generation intentionally skipped.
}
