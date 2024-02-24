# B2M

Binary Module Manager (BMM/B2M)

## Preparing the environment for compilation
```bash
rustup override set nightly
```

## Building:
```bash
cargo build
```

# Binary modules requiring example

```lua
if not b2m.Require("chttp") then
    print("Binary module chttp cannot be loaded")

    return
end
```