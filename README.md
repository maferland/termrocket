# termrocket

Animated rocket on `git push` for kitty terminal. Inspired by [gitrocket](https://github.com/bomanimc/gitrocket).

## Requirements

- [Kitty terminal](https://sw.kovidgoyal.net/kitty/)
- [Fish shell](https://fishshell.com/) (for auto-trigger)

## Install

```bash
fisher install maferland/termrocket
```

Binary downloads automatically on first `git push` (runs in background, won't slow down your push).

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
