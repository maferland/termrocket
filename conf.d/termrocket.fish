# termrocket - Animated rocket on git push for kitty terminal
set -l plugin_dir (dirname (status filename))/..
set -l bin_dir "$plugin_dir/bin"

# Add to PATH if binary exists
if test -x "$bin_dir/termrocket"
    fish_add_path --path "$bin_dir"
end
