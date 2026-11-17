//! SigmaOS-flatpak-sandbox-management — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_flatpak_sandbox_management_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-flatpak-sandbox-management: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-flatpak-sandbox-management: ok");
}
