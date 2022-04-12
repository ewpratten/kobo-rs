#! /bin/bash
set -ex

# We need the ip address and binary paths as arguments
if [ $# -ne 2 ]; then
    echo "Usage: $0 <ip address> <binary path>"
    exit 1
fi

# Use lftp to copy the binary to the target's /mnt/onboard directory
lftp -c "open -u root, $1;cd /mnt/onboard;put $2"