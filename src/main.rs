use std::env;
use std::path::{Path, PathBuf};

use win_desktop_utils::DesktopApp;

const DOCS_URL: &str = "https://docs.rs/win-desktop-utils";
const REPO_URL: &str = "https://github.com/funwithcthulhu/win-desktop-utils";
const SHORTCUT_NAME: &str = "win-desktop-utils docs.url";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help(None);
        return Ok(());
    }

    let app = DesktopApp::with_company("Fun With Cthulhu", "Win Desktop Utils Demo")?;
    let Some(_guard) = app.single_instance()? else {
        println!("Another demo instance is already running.");
        return Ok(());
    };

    let local_data_dir = app.ensure_local_data_dir()?;

    if args.is_empty() {
        println!("Prepared demo desktop state.");
        println!("Local app data: {}", local_data_dir.display());
        println!();
        print_help(Some(&local_data_dir));
        return Ok(());
    }

    for arg in args {
        match arg.as_str() {
            "--elevation" => {
                println!("Process elevated: {}", win_desktop_utils::is_elevated()?);
            }
            "--open-docs" => {
                println!("Opening {DOCS_URL}");
                win_desktop_utils::open_url(DOCS_URL)?;
            }
            "--open-repo" => {
                println!("Opening {REPO_URL}");
                win_desktop_utils::open_url(REPO_URL)?;
            }
            "--reveal-data-dir" => {
                println!("Revealing {}", local_data_dir.display());
                win_desktop_utils::reveal_in_explorer(&local_data_dir)?;
            }
            "--create-docs-shortcut" => {
                let shortcut = create_docs_shortcut(&local_data_dir)?;
                println!("Created {}", shortcut.display());
            }
            other => {
                return Err(format!("unknown argument: {other}").into());
            }
        }
    }

    Ok(())
}

fn create_docs_shortcut(local_data_dir: &Path) -> win_desktop_utils::Result<PathBuf> {
    let shortcut = local_data_dir.join(SHORTCUT_NAME);
    win_desktop_utils::create_url_shortcut(&shortcut, DOCS_URL)?;
    Ok(shortcut)
}

fn print_help(local_data_dir: Option<&Path>) {
    println!("win-desktop-utils-demo");
    println!();
    println!("Runs a tiny app-startup flow with DesktopApp, app-data setup, and a");
    println!("single-instance guard. Visible shell actions only happen when requested.");
    println!();
    println!("Usage:");
    println!("  cargo run -- [option ...]");
    println!();
    println!("Options:");
    println!("  --elevation              Print whether this process is elevated");
    println!("  --open-docs              Open the docs.rs page in the default browser");
    println!("  --open-repo              Open the GitHub repository");
    println!("  --reveal-data-dir        Reveal the demo app-data directory in Explorer");
    println!("  --create-docs-shortcut   Create a .url shortcut to the docs page");
    println!("  -h, --help               Show this help");

    if let Some(local_data_dir) = local_data_dir {
        println!();
        println!("Current app-data directory:");
        println!("  {}", local_data_dir.display());
    }
}
