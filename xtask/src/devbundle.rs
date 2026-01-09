// Licensed under the Apache-2.0 license

use anyhow::{anyhow, bail, Result};
use clap::Subcommand;
use mcu_builder::{get_build_metadata, CaliptraBuilder};
use std::fs::File;
use std::io::{Seek, Write};
use std::path::Path;
use std::process::Command;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[derive(Subcommand)]
pub enum DevBundleCommands {
    /// Create a Authentication Manifest
    Create {
        /// Output file path
        #[arg(value_name = "OUTPUT", required = true)]
        output: String,
    },
}

fn platform(fpga: bool) -> &'static str {
    if fpga {
        "fpga"
    } else {
        "emulator"
    }
}

fn add_file<W: Write + Seek, P: AsRef<Path>>(
    zip: &mut ZipWriter<W>,
    infile: P,
    archive_name: &str,
    options: SimpleFileOptions,
) -> Result<()> {
    zip.start_file(archive_name, options)?;
    let data = std::fs::read(infile)?;
    zip.write_all(&data)?;
    Ok(())
}

fn build_program<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    pkgname: &str,
    subdir: &str,
) -> Result<()> {
    let mut cmd = Command::new("cargo");
    let cmd = cmd
        .arg("build")
        .arg("--release")
        .arg("--message-format=json")
        .arg("-p")
        .arg(pkgname);

    println!("Executing {:?}", cmd);
    let result = cmd.output()?;
    if !result.status.success() {
        bail!("cargo failed to build emulator");
    }
    let stdout = String::from_utf8(result.stdout)?;
    let executable = get_build_metadata(&stdout, "executable")?;

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    add_file(
        zip,
        executable
            .as_str()
            .ok_or(anyhow!("executable isn't a string"))?,
        &format!("{subdir}/{pkgname}"),
        options,
    )?;
    Ok(())
}

fn build_hello<W: Write + Seek>(zip: &mut ZipWriter<W>, subdir: &str, fpga: bool) -> Result<()> {
    let feature = "test-exit-immediately".to_string();
    println!("Compiling test firmware {}", &feature);
    let test_runtime = mcu_builder::runtime_build_standalone(
        "tests/standalone_hello", // target_name
        &[feature.as_str()],      // features
        None,                     // output_name
        Some(platform(fpga)),     // platform
        None,                     //Some(memory_map()),     // memory_map,
        false,                    // use_dccm_for_stack
        None,                     // dccm_offset
        None,                     // dccm_size
        None,                     // log_flash_config
        None,                     // mcu_image_header
    )?;
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);
    add_file(zip, test_runtime, &format!("{subdir}/hello.bin"), options)?;
    Ok(())
}

fn build<W: Write + Seek>(zip: &mut ZipWriter<W>, subdir: &str, fpga: bool) -> Result<()> {
    zip.add_directory(subdir, SimpleFileOptions::default())?;
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    let mut builder = CaliptraBuilder::new(
        fpga, None, None, None, None, None, None, None, None, None, None,
    );
    add_file(
        zip,
        builder.get_caliptra_rom()?,
        &format!("{subdir}/cptra-rom.bin"),
        options,
    )?;
    add_file(
        zip,
        builder.get_caliptra_fw()?,
        &format!("{subdir}/cptra-firmware.bin"),
        options,
    )?;

    let platform = if fpga { "fpga" } else { "emulator" };
    let feature = "";
    let mcu_rom = mcu_builder::rom_build(Some(platform), feature)?;
    add_file(zip, mcu_rom, &format!("{subdir}/mcu-rom.bin"), options)?;
    Ok(())
}

pub fn create(output: &str) -> Result<()> {
    let file = File::create(output)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);
    add_file(
        &mut zip,
        "tests/standalone_hello/devbundle/module_bazel",
        "MODULE.bazel",
        options,
    )?;
    add_file(
        &mut zip,
        "tests/standalone_hello/devbundle/build_bazel",
        "BUILD.bazel",
        options,
    )?;

    build(&mut zip, "fpga", true)?;
    build_hello(&mut zip, "fpga", true)?;
    build(&mut zip, "emulator", false)?;
    build_hello(&mut zip, "emulator", false)?;
    zip.add_directory("tools", SimpleFileOptions::default())?;
    build_program(&mut zip, "emulator", "tools")?;
    build_program(&mut zip, "signer", "tools")?;

    add_file(
        &mut zip,
        "tests/standalone_hello/devbundle/README-devbundle.md",
        "README.md",
        options,
    )?;
    add_file(
        &mut zip,
        "tests/standalone_hello/devbundle/run.sh",
        "run.sh",
        options.unix_permissions(0o755),
    )?;

    zip.finish()?;
    Ok(())
}
