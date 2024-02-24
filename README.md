<p align="center">
  <img src="img/b2m.png" />
</p>

<h1 align="center">
    B2M
</h1>

<h3 align="center">
    Garry's Mod binary modules manager
</h3>
<br>

### B2m - as the name implies, this is a binary module manager for Garry's Mod.
### For the most part, servers need it so that there are more opportunities to do something on the client.

<br><br>

# Installation
The installation is very simple - you just need to transfer the B2M folder to your addons folder:

    1. Download ZIP archive of B2M on this page
    2. Open archive
    3. Open your game folder
    4. Open garrysmod/addons folder (create if not exist)
    5. Drop folder b2m_master in /addons

[Installation guide](https://youtube.com/)

<br>

# Building
### Preparing the environment for compilation
```bash
rustup override set nightly
```
### Build
```bash
cargo build
```

# Examples

### Require a module
```lua
if not b2m.Require("chttp") then
    print("Binary module chttp cannot be loaded")

    return
end
```

### Binary module installation
##### *command for server console
```bash
b2m install chttp * --server-only // installing module only on server
b2m install chttp * --client-only // installing module only on client
b2m install chttp 1.0.0 // installing chttp with version 1.0.0 on client and server
```