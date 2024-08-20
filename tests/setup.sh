#!/bin/sh

################################################################################
# Step 0: check if the current directory is the root of the project
################################################################################
if [ ! -f Cargo.toml ]; then
    echo "Error: please run this script from the root of the project."
    exit 1
fi

################################################################################
# Step 1: make a copy of the original config file
################################################################################
# happily use the hidden option --show-config-path
ORIGINAL_CONFIG=$(cargo run -q -- config --show-config-path | tail -1)
BACKUP_CONFIG="$ORIGINAL_CONFIG.bak"
# if the backup config file exists, issue an error
if [ -f "$BACKUP_CONFIG" ]; then
    echo "Error: backup config file '$BACKUP_CONFIG' already exists."
    echo "       Please remove it manually if are sure to overwrite it."
    echo "       Otherwise, please run 'tests/restore_config.sh' to restore the original config file."
    exit 1
fi
# if the original config file exists, move it
if [ -f "$ORIGINAL_CONFIG" ]; then
    mv "$ORIGINAL_CONFIG" "$BACKUP_CONFIG"
fi

################################################################################
# Step 2: Remove the test papers directory
################################################################################
rm -rf tests/papers

################################################################################
# Step 3: initialize the config file for testing TermiPaper
################################################################################
cargo run -q -- config --owner.name        "TermiPaper Tester"     \
                       --owner.email       "termipaper@wqzhao.org" \
                       --owner.affiliation "GitHub"                \
                       --owner.link        "https://termipaper.wqzhao.org"
cargo run -q -- init tests/papers
cargo run -q -- info
