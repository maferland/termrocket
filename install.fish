#!/usr/bin/env fish
# Download termrocket binary during fisher install

set -l plugin_dir (dirname (status filename))
set -l bin_dir "$plugin_dir/bin"
set -l binary "$bin_dir/termrocket"

# Skip if already downloaded
if test -x "$binary"
    return 0
end

set -l os (uname -s | string lower)
set -l arch (uname -m)

# Determine binary name
set -l binary_name
if test "$os" = darwin
    if test "$arch" = arm64
        set binary_name termrocket-macos-arm64
    else
        set binary_name termrocket-macos-x86_64
    end
else if test "$os" = linux
    set binary_name termrocket-linux-x86_64
else
    echo "termrocket: unsupported platform $os-$arch" >&2
    return 1
end

# Get latest release URL
set -l url "https://github.com/maferland/termrocket/releases/latest/download/$binary_name"

mkdir -p "$bin_dir"

echo "Downloading termrocket binary..."

if type -q curl
    curl -sL "$url" -o "$binary"
else if type -q wget
    wget -q "$url" -O "$binary"
else
    echo "termrocket: curl or wget required" >&2
    return 1
end

chmod +x "$binary"

if test -x "$binary"
    echo "termrocket installed successfully!"
else
    echo "termrocket: download failed" >&2
    return 1
end
