#!/bin/bash

function usage() {
  echo "Run the MCU emulator"
  echo
  echo "$1 --fw FILE --soc-manifest FILE"
  echo
  echo "    --fw FILE MCU firmware image to execute"
  echo "    --soc-manifest FILE Authentication manifest"
  echo "    --rom FILE MCU ROM image"
  echo "    --cptra-rom FILE Caliptra ROM image"
  echo "    --cptra-fw FILE Caliptra firmware image"
  echo "    --vendor-pk-hash STRING SHA384 of the vendor public keys"
}

# Provide sensible defaults for ROM and Firmware images.
DIR="$(dirname "$0")"
ROM="${DIR}/emulator/mcu-rom.bin"
CPTRA_ROM="${DIR}/emulator/cptra-rom.bin"
CPTRA_FW="${DIR}/emulator/cptra-firmware.bin"
VENDOR_PK_HASH="b17ca877666657ccd100e6926c7206b60c995cb68992c6c9baefce728af05441dee1ff415adfc187e1e4edb4d3b2d909"

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --rom)
      shift
      ROM="$1"
      shift
      ;;
    --fw)
      shift
      FW="$1"
      shift
      ;;
    --vendor-pk-hash)
      shift
      VENDOR_PK_HASH="$1"
      shift
      ;;
    --soc-manifest)
      shift
      SOC_MANIFEST="$1"
      shift
      ;;
    --cptra-rom)
      shift
      CPTRA_ROM="$1"
      shift
      ;;
    --cptra-fw)
      shift
      CPTRA_FW="$1"
      shift
      ;;
    -\?|-h|--help)
      usage $0
      exit 1
      ;;
    *)
      echo "Unknown argument: $1"
      exit 1
      ;;
  esac
done

: ${FW:?You must supply a firmware image file}
: ${SOC_MANIFEST:?You must supply a SOC manifest file}

${DIR}/tools/emulator \
    --rom ${ROM} \
    --firmware ${FW} \
    --i3c-port 65534 \
    --rom-offset 0x80000000 \
    --rom-size 0x8000 \
    --dccm-offset 0x50000000 \
    --dccm-size 0x4000 \
    --sram-offset 0x40000000 \
    --sram-size 0x80000 \
    --pic-offset 0x60000000 \
    --i3c-offset 0x20004000 \
    --i3c-size 0x1000 \
    --mci-offset 0x21000000 \
    --mci-size 0xe00000 \
    --mbox-offset 0x30020000 \
    --mbox-size 0x28 \
    --soc-offset 0x30030000 \
    --soc-size 0x5e0 \
    --otp-offset 0x70000000 \
    --otp-size 0x140 \
    --lc-offset 0x70000400 \
    --lc-size 0x8c \
    --caliptra-rom ${CPTRA_ROM} \
    --caliptra-firmware ${CPTRA_FW} \
    --soc-manifest ${SOC_MANIFEST} \
    --vendor-pk-hash ${VENDOR_PK_HASH}
