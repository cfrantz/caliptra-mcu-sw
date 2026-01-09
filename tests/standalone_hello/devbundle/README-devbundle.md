# Dev Bundle

This archive is the Caliptra-MCU devbundle.  This archive contains files that
are needed to develop stand-alone MCU firmware outside of the main source tree.

This archive contains:

- For FPGA and the emulator:
  - The Caliptra ROM.
  - The Caliptra Firmware.
  - The MCU ROM.
  - A stand-alone "hello" MCU Firmware.
- The emulator binary.
- A signer binary that can create an auth manifest allowing MCU firmware to
  execute.

## Example: Hello Caliptra

In order to execute the "hello" binary, you must create an authorization
manifest that will permit it to run:

```
$ cd ${root-of-devbundle}
$ ./tools/signer auth-manifest create \
    --mcu_image emulator/hello.bin,0x40000000,0,2,2 \
    --caliptra_rom emulator/cptra-rom.bin \
    --caliptra_firmware emulator/cptra-firmware.bin \
    --vendor_pk_hash b17ca877666657ccd100e6926c7206b60c995cb68992c6c9baefce728af05441dee1ff415adfc187e1e4edb4d3b2d909 \
    --output manifest.bin
```

Once you have the authorization manifest, you can run the "hello" program in the
emulator:

```
$ cd ${root-of-devbundle}
$ ./run.sh --soc-manifest manifest.bin --fw emulator/hello.bin

...
[mcu-rom] Firmware boot reset detected
[mcu-rom] Starting fw boot reset flow
[mcu-rom] Jumping to firmware
Hello Caliptra
```
