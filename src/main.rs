use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::process::Command;

fn find_existing_loader(exe_dir: &Path, names: &[&str]) -> Option<PathBuf> {
    for name in names {
        let candidate = exe_dir.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

fn main() {

    // Define loaders
    let candidates = ["skse64_loader.exe", "fose_loader.exe", "nvse_loader.exe"];

    // Get path of our program
    let exe_path: PathBuf = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current exe path: {}", e);
            return;
        }
    };

    // Get the directory we are
    let exe_dir = match exe_path.parent() {
        Some(dir) => dir,
        None => {
            eprintln!("Failed to get parent directory of executable");
            return;
        }
    };

    // Find the loader used by this game.
    let loader = match find_existing_loader(exe_dir, &candidates) {
        Some(loader) => loader,
        None => {
            eprintln!("Failed to find a Script Extender loader for your Bethesda Game.");
            return;
        }
    };
    let args: Vec<String> = env::args().skip(1).collect();

    let status = Command::new(&loader)
        .args(&args)
        .status()
        .expect("Failed to execute loader");
    
    println!("Exit status: {}", status.code().unwrap_or(-1));
}


