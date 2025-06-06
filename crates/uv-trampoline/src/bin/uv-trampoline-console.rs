#![no_main] // disable all rust entry points, requires enabling compiler-builtins-mem
#![windows_subsystem = "console"] // configures /SUBSYSTEM:CONSOLE

// Named according to https://docs.microsoft.com/en-us/cpp/build/reference/entry-entry-point-symbol
// This avoids having to define a custom /ENTRY:entry_fn in build.rs
#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() -> ! {
    uv_trampoline::bounce::bounce(false)
}
