function _termrocket_download --description "Download termrocket binary in background"
    set -l plugin_dir (status dirname)/..
    set -l bin_dir "$plugin_dir/bin"
    set -l binary "$bin_dir/termrocket"

    # Skip if already exists
    test -x "$binary"; and return 0

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
        return 1
    end

    set -l url "https://github.com/maferland/termrocket/releases/latest/download/$binary_name"

    mkdir -p "$bin_dir"

    if type -q curl
        curl -sL "$url" -o "$binary" 2>/dev/null
    else if type -q wget
        wget -q "$url" -O "$binary" 2>/dev/null
    else
        return 1
    end

    chmod +x "$binary" 2>/dev/null
end
