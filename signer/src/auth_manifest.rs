// Licensed under the Apache-2.0 license

use anyhow::Result;
use clap::{Args, Subcommand};
use mcu_builder::{CaliptraBuilder, ImageCfg};
use std::path::PathBuf;

// NOTE: This is a near-copy of xtask/src/auth_signer.rs.
// This version _can_ run outside of the cargo workspace.
//
// The changes are:
//     - `soc_image` is optional.
//     - You can supply `caliptra_rom`,`caliptra_firmware` and `vendor_pk_hash`.  These are
//       required when running outside of the cargo workspace.

#[derive(Subcommand)]
pub enum AuthManifestCommands {
    /// Create a Authentication Manifest
    Create(Create),
}

#[derive(Args)]
pub struct Create {
    /// List of soc images with format: <path>,<load_addr>,<staging_addr>,<image_id>,<exec_bit>
    /// Example: --soc_image image1.bin,0x80000000,0x60000000,2,2
    #[arg(long = "soc_image", value_name = "SOC_IMAGE", num_args = 1.., required = false)]
    images: Vec<ImageCfg>,

    /// MCU Image metadata: <path>,<load_addr>,<staging_addr>,<image_id>,<exec_bit>
    /// Example: --mcu_image mcu-runtime.bin,0xA8000000,0x60000000,2,2
    #[arg(
        long = "mcu_image",
        value_name = "MCU_IMAGE",
        num_args = 1,
        required = true
    )]
    mcu_image: ImageCfg,

    /// Path to the caliptra ROM
    #[arg(long = "caliptra_rom")]
    caliptra_rom: Option<PathBuf>,

    /// Path to the caliptra firmware
    #[arg(long = "caliptra_firmware")]
    caliptra_firmware: Option<PathBuf>,

    /// Sha384 hash of the vendor's public key.
    #[arg(long = "vendor_pk_hash")]
    vendor_pk_hash: Option<String>,

    /// Output file path
    #[arg(long, value_name = "OUTPUT", required = true)]
    output: String,
}

pub fn create(opts: &Create) -> Result<()> {
    let mut builder = CaliptraBuilder::new(
        false,
        opts.caliptra_rom.clone(),
        opts.caliptra_firmware.clone(),
        None, // soc_manifest
        opts.vendor_pk_hash.clone(),
        Some(opts.mcu_image.clone().path),
        Some(opts.images.clone()),
        Some(opts.mcu_image.clone()),
        None,
        None,
        None,
    )
    .with_components_config(false);
    let path = builder.get_soc_manifest(Some(opts.output.as_str()))?;
    println!("Auth Manifest created at: {path:?}");
    Ok(())
}
