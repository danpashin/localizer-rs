#!/bin/bash

crate_name="localizer-rs"
cbindgen --config ${crate_name}/cbindgen.toml --crate ${crate_name} --output ./src/localizer.h ${crate_name}
