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
