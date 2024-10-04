#!/bin/bash

CONFIG_PATH="./src/config/config.yaml"
ACCESS_LOG_PATH="./src/log/access.log"
ERROR_LOG_PATH="./src/log/error.log"

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --config-path=*)
            CONFIG_PATH="${1#*=}"
            ;;
        --access-log-path=*)
            ACCESS_LOG_PATH="${1#*=}"
            ;;
        --error-log-path*)
            ERROR_LOG_PATH="${1#*=}"
            ;;
        *)
            echo "Неизвестный параметр: $1"
            exit 1
            ;;
    esac
    shift
done

echo "// Code generated by "jxs"; DO NOT EDIT.
pub const CONFIG_PATH: &str = \"$CONFIG_PATH\";
pub const ACCESS_LOG_PATH: &str = \"$ACCESS_LOG_PATH\";
pub const ERROR_LOG_PATH: &str = \"$ERROR_LOG_PATH\";" \
  > ./src/config/default.rs
