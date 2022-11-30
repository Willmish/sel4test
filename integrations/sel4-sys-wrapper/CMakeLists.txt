# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

project(sel4_sys)
cmake_minimum_required(VERSION 3.8.2)

# NB: caller must provide Rust configuration; e.g. from the cmd line
#  -DRustTarget=riscv32imac-unknown-none-elf \
#  -DRustCFlags="" \
#  -DRustVersion=nightly-2021-11-05

add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/target/libsel4_sys.a
    COMMAND ${CMAKE_COMMAND}
        -E env
            RUSTFLAGS="${RustCFlags}"
            SEL4_DIR=${LibSel4KernelPath}
            SEL4_OUT_DIR=${LibSel4KernelOutputPath}
        cargo +${RustVersion} build
        --release
        --target=${RustTarget}
        -Z unstable-options
        --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
        --out-dir target
        --target-dir target
)

add_custom_target(
    sel4_external
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/target/libsel4_sys.a
)

set(LibSel4ExternalLibraryPath ${CMAKE_CURRENT_BINARY_DIR}/target/libsel4_sys.a PARENT_SCOPE)
