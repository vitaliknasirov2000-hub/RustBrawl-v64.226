# RustBrawl
Open-source server emulator for Supercell’s game Brawl Stars 64.226, written in Rust.

# ⚠️ Important Notice
This is the first time I’ve ever written anything in Rust
Yes, I know I mixed ``PascalCase`` and ``camelCase`` instead of following Rust’s ``snake_case`` convention.

# ❓ What is this?
RustBrawl is an **experimental Brawl Stars server emulator** built in **Rust**.
**Don’t expect this to work** in production it’s mostly for **learning** and **fun**.

# Client
You need to **create the Client yourself**.
The repository already includes the necessary scripts, you just need to add the Frida gadget into the apk/ipa.
I’ve prepared scripts for **all architectures** (Android ARM32, Android ARM64, iOS).

# Note
- Android scripts have not been tested, but the offsets are 100% correct.
- Feel free to fork, clone, and contribute, any type of contribution is welcome!