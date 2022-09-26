/* Copyright (c) 2015 The Robigalia Project Developers
 * Licensed under the Apache License, Version 2.0
 * <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT
 * license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
 * at your option. All files in the project carrying such
 * notice may not be copied, modified, or distributed except
 * according to those terms.
 */

use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let cargo_target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
    // XXX(mwitkowski): For now tools lives in sel4-sys crate, so lets use that
    let tools_dir = format!("{}/../../../kata/apps/system/components/kata-os-common/src/sel4-sys/tools",
        env::var("CARGO_MANIFEST_DIR").unwrap());
    println!(
        "target_arch = {} target_pointer_width = {}",
        cargo_target_arch, cargo_target_pointer_width
    );

    // Default to python3 (maybe necessary for code divergence)
    let python_bin = env::var("PYTHON").unwrap_or_else(|_| "python3".to_string());

    // Default to "seL4" for backwards compat; can either use git submodule or
    // symbolic link (neither recommended)
    let sel4_dir = env::var("SEL4_DIR").unwrap_or_else(
        |_| format!("{}/kata/kernel", env::var("ROOTDIR").unwrap())
    );
    println!("sel4_dir {}", sel4_dir);

    // If SEL4_OUT_DIR is not set we expect the kernel build at a fixed
    // location relative to the ROOTDIR env variable.
    println!("SEL4_OUT_DIR {:?}", env::var("SEL4_OUT_DIR"));
    let sel4_out_dir = env::var("SEL4_OUT_DIR").unwrap_or_else(
        |_| format!("{}/out/kata/kernel", env::var("ROOTDIR").unwrap())
    );
    println!("sel4_out_dir {}", sel4_out_dir);

    // Dredge seL4 kernel config for settings we need as features to generate
    // correct code: e.g. CONFIG_KERNEL_MCS enables MCS support which changes
    // the system call numbering.
    let features = sel4_config::get_sel4_features(&sel4_out_dir);
    println!("features = {:?}", features);
    let mut has_mcs = false;
    for feature in features {
        println!("cargo:rustc-cfg=feature=\"{}\"", feature);
        if feature.as_str() == "CONFIG_KERNEL_MCS" { has_mcs = true; }
    }

    // Use CARGO_TARGET_ARCH and CARGO_TARGET_POINTER_WIDTH
    // to select the target architecture;
    // NB: this mimics the logic in lib.rs
    let (arch, archdir) = match cargo_target_arch.as_str() {
        "x86" => ("ia32", "x86"),
        "x86_64" => ("x86_64", "x86"),
        "arm" => match cargo_target_pointer_width.as_str() {
            "32" => ("aarch32", "arm"),
            "64" => ("aarch64", "arm"),
            _ => {
                panic!("Unsupported arm word size {}", cargo_target_pointer_width);
            }
        },
        "riscv32" => ("riscv32", "riscv"),
        "riscv64" => ("riscv64", "riscv"),
        _ => {
            panic!("Unsupported architecture {}", cargo_target_arch);
        }
    };

    let xml_interfaces_file = format!("{}/libsel4/include/interfaces/sel4.xml", sel4_dir);
    let outfile = format!("{}/{}_syscall_stub.rs", out_dir, arch);
    let xml_arch_file = &*format!(
        "{}/libsel4/arch_include/{}/interfaces/sel4arch.xml",
        sel4_dir, archdir
    );
    let xml_sel4_arch_file = format!(
        "{}/libsel4/sel4_arch_include/{}/interfaces/sel4arch.xml",
        sel4_dir, arch
    );

    let mut cmd = Command::new("/usr/bin/env");
    cmd.arg(&python_bin)
       .arg(&format!("{}/syscall_stub_gen.py", tools_dir));
    if has_mcs {
        cmd.arg("--mcs");
    }
    cmd.args(&[
        "-a",
        arch,
        "-w",
        cargo_target_pointer_width.as_str(),
        "--buffer",
        "-o",
        &*outfile,
        &*xml_interfaces_file,
        &*xml_arch_file,
        &*xml_sel4_arch_file,
    ]);
    println!("Running: {:?}", cmd);
    assert!(cmd.status().unwrap().success());
}
