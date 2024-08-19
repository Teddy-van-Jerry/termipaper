#!/bin/sh

ORIGINAL_CONFIG=$(cargo run -q -- config --show-config-path | tail -1)
BACKUP_CONFIG="$ORIGINAL_CONFIG.bak"
if [ -f "$BACKUP_CONFIG" ]; then
    mv "$BACKUP_CONFIG" "$ORIGINAL_CONFIG"
fi
