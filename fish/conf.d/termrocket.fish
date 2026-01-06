# Add termrocket binary to PATH if installed via fisher
set -l plugin_dir (dirname (status filename))/..
if test -x "$plugin_dir/../target/release/termrocket"
    fish_add_path --path "$plugin_dir/../target/release"
end
