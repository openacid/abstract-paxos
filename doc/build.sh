#!/bin/sh

platform=zhihu
fn=src/v1-cn.md

md2zhihu \
    --platform         "$platform" \
    --code-width       600 \
    --refs             src/refs.yml \
    --output-dir       ./built \
    --asset-output-dir ./built/"$platform" \
    --md-output        ./built/"$platform"/ \
    "$fn"
