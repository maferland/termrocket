# termrocket

Animated rocket on `git push` for kitty terminal. Inspired by [gitrocket](https://github.com/bomanimc/gitrocket).

## Requirements

- [Kitty terminal](https://sw.kovidgoyal.net/kitty/)
- [Fish shell](https://fishshell.com/) (for auto-trigger)

## Install

### With Fisher (recommended)

```bash
fisher install maferland/termrocket
cd ~/.local/share/fisher/github.com/maferland/termrocket
cargo build --release
```

### Manual

```bash
git clone https://github.com/maferland/termrocket.git
cd termrocket
cargo build --release

# Add to PATH
cp target/release/termrocket ~/.local/bin/

# Add fish functions
cp -r fish/functions/* ~/.config/fish/functions/
```

## Usage

### Manual launch
```bash
termrocket launch
```

### Test terminal support
```bash
termrocket test
```

### Auto-trigger on git push
The fish wrapper automatically launches the rocket on successful `git push`.

## Support

If you enjoy termrocket, consider buying me a coffee:

[buymeacoffee.com/maferland](https://buymeacoffee.com/maferland)

## License

MIT
