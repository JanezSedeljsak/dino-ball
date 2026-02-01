#!/bin/bash

# Check if assets/icon.png exists
if [ ! -f "assets/icon.png" ]; then
    echo "assets/icon.png not found"
    exit 1
fi

ICONSET="icon.iconset"
mkdir -p $ICONSET

declare -a sizes=("16" "32" "128" "256" "512")

for size in "${sizes[@]}"; do
    size2=$((size * 2))
    sips -z $size $size assets/icon.png --out "${ICONSET}/icon_${size}x${size}.png" > /dev/null 2>&1
    sips -z $size2 $size2 assets/icon.png --out "${ICONSET}/icon_${size}x${size}@2x.png" > /dev/null 2>&1
done

iconutil -c icns $ICONSET -o assets/icon.icns

rm -rf $ICONSET

echo "Successfully created assets/icon.icns"
