#!/bin/sh

# 1. ensure the current directory is the root of the project
if [ ! -f Cargo.toml ]; then
    echo "Error: please run this script from the root of the project."
    exit 1
fi

# 2. create the test assets directory
mkdir tests/assets

# 3. download the test assets
download_wqzhao() {
    curl https://wqzhao.org/assets/"$1.pdf" -o tests/assets/"$1.pdf"
}
download_wqzhao zhao2023ompl
download_wqzhao you2024beam
download_wqzhao zhao2024flexible
download_wqzhao zhao2024efficient
download_wqzhao zhao2023automatic
download_wqzhao dai2024local
download_wqzhao zheng2024enhancing
