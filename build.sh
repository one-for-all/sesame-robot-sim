#!/usr/bin/env bash
set -e # immediately exit if any command returns non-zero exit code

arduino-cli compile --fqbn esp32:esp32:esp32 --build-property "build.partitions=partitions_factory_only" --build-property "build.extra_flags=-DCORE_DEBUG_LEVEL=5" app --output-dir ./app/build
# partitions_factory_only.csv is placed in /Users/jay/Library/Arduino15/packages/esp32/hardware/esp32/3.3.7/tools/partitions/
# use --clean to force clean build

xtensa-esp32-elf-nm -n app/build/app.ino.elf > app/build/symbols.txt
