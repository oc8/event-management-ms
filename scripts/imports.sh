#!/bin/bash

FILE_PATH="./libs/gen/src/event.v1.rs"

IMPORTS="use apistos::ApiComponent;\\nuse schemars::JsonSchema;\\n"

sed -i '' "1s/^/$IMPORTS/" "$FILE_PATH"
