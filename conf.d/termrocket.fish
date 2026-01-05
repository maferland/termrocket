# termrocket - Animated rocket on git push for kitty terminal
set -l plugin_dir (status dirname)/..
set -l bin_dir "$plugin_dir/bin"

# Add to PATH
fish_add_path --path "$bin_dir"
