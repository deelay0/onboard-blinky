Rust code to blink the onboard LED in a Raspberry Pi Zero 2 W

Sources:
    https://stirnemann.xyz/posts/rust_led/

Configuration changes made:
    - Added build target and linker script at compilation (at ./.cargo/config):
        
        [build]
        target = "armv7a-none-eabi"
        [target.'cfg(all(target_arch = "arm", target_os = "none"))']
        rustflags = ["-C", "link-arg=-Tlinker.ld",]
    
    - Added rust-analyzer settings to only check for our target, removing false-positive vscode errors (at ./.vscode/settings.json):
        
        "rust-analyzer.cargo.target": "thumbv7em-none-eabihf",
        "rust-analyzer.checkOnSave.allTargets": false
    
    - Added flags to disable stack unwinding when building for release & dev (at ./Cargo.toml):
        
        [profile.dev]
        panic = "abort"
        [profile.release]
        panic = "abort"

ELF to binary image:
    $ arm-none-eabi-objcopy -O binary ./target/armv7a-none-eabi/release/onboard_blinky ./kernel.img