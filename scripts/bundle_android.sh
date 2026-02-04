#!/usr/bin/env bash

set -euo pipefail

app_dir=$1
output_dir=$2
icon_dir=$3
keystore_path=$4

start_dir=$(pwd)

cd "$app_dir"
./gradlew clean
### Icons
find app/src/main/res -name "*.webp" -type f -delete

if [ -d "$start_dir/$icon_dir/android" ] && [ "$(ls -A "$start_dir/$icon_dir/android")" ]; then
    cp -r "$start_dir/$icon_dir/android/"* app/src/main/res/
else
    echo "Warning: Icon directory $start_dir/$icon_dir/android is missing or empty, skipping icon copy."
    exit 1
fi

rm app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml

./gradlew assembleRelease

tmp_dir=$(mktemp -d)

zipalign -v -p 4 \
    app/build/outputs/apk/release/app-release-unsigned.apk \
    "$tmp_dir/app-release-aligned.apk"

cd "$start_dir"
apksigner sign \
    --ks "$keystore_path" \
    --ks-pass env:KEYSTORE_PASSWORD \
    --out "$output_dir/roommates.apk" \
    "$tmp_dir/app-release-aligned.apk"

rm -r "$tmp_dir"

echo "Bundle created at $output_dir/roommates.apk"
