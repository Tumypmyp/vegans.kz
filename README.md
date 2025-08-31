# Development


## Installing Dependencies

- `dioxus-cli`

### Windows, Linux

```bash
cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli --locked
```

### NixOS

```bash
nix develop
```

## Building

```bash
dx serve
```

## Publishing

```bash
dx bundle --web --ssg --release
cp ./target/../release/public ./public
```
