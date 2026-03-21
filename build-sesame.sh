#!/usr/bin/env bash
set -e # immediately exit if any command returns non-zero exit code

arduino-cli compile --fqbn esp32:esp32:esp32 --board-options "CPUFreq=80" --build-property "build.partitions=partitions_factory_only" sesame --output-dir sesame/build
# partitions_factory_only.csv is placed in /Users/jay/Library/Arduino15/packages/esp32/hardware/esp32/3.3.7/tools/partitions/
# use --clean to force clean build
# --build-property "build.extra_flags=-DCORE_DEBUG_LEVEL=5"

xtensa-esp32-elf-nm -n  sesame/build/sesame.ino.elf > sesame/build/symbols.txt

cp -r sesame/build www/static/sesame/
cp sesame/sesame.ino www/src/assets/
cp sesame/movement-sequences.h www/src/assets/

# default compiled binary and symbol for when user hits stop
cp sesame/build/sesame.ino.bin www/src/assets/
cp sesame/build/symbols.txt www/src/assets/
