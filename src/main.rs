use std::env;
use std::path::{Path, PathBuf};

use win_desktop_utils::DesktopApp;

const DOCS_URL: &str = "https://docs.rs/win-desktop-utils";
const REPO_URL: &str = "https://github.com/funwithcthulhu/win-desktop-utils";
const SHORTCUT_NAME: &str = "win-desktop-utils docs.url";
const APP_DATA_SIDE_EFFECTS: &str =
    "resolves the local app-data path and creates the directory if missing";
const STARTUP_SIDE_EFFECTS: &str =
    "creates local app-data if missing; holds a single-instance guard";

const NOT_EXPOSED_HELPERS: &[(&str, &str)] = &[
    (
        "Recycle Bin helpers",
        "not exposed by this demo; no recycle action is available",
    ),
    (
        "Elevation requests",
        "not exposed by this demo; --elevation only checks current state",
    ),
];

struct DemoAction {
    flag: &'static str,
    description: &'static str,
    side_effects: &'static str,
}

const DEMO_ACTIONS: &[DemoAction] = &[
    DemoAction {
        flag: "--elevation",
        description: "Print whether this process is elevated",
        side_effects: "checks process elevation; does not request elevation",
    },
    DemoAction {
        flag: "--open-docs",
        description: "Open the docs.rs page in the default browser",
        side_effects: "opens the default browser or registered URL handler",
    },
    DemoAction {
        flag: "--open-repo",
        description: "Open the GitHub repository",
        side_effects: "opens the default browser or registered URL handler",
    },
    DemoAction {
        flag: "--reveal-data-dir",
        description: "Reveal the demo app-data directory in Explorer",
        side_effects: "opens Explorer with the app-data directory selected",
    },
    DemoAction {
        flag: "--create-docs-shortcut",
        description: "Create a .url shortcut to the docs page",
        side_effects: "writes or overwrites the docs .url file in app data",
    },
];

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

    println!("App-data side effects: {APP_DATA_SIDE_EFFECTS}");
    let local_data_dir = app.ensure_local_data_dir()?;

    if args.is_empty() {
        println!("Prepared demo desktop state.");
        println!();
        print_capabilities(&local_data_dir);
        println!();
        print_help(Some(&local_data_dir));
        return Ok(());
    }

    for arg in args {
        match arg.as_str() {
            "--elevation" => {
                print_action_side_effects("--elevation");
                println!("Process elevated: {}", win_desktop_utils::is_elevated()?);
            }
            "--open-docs" => {
                print_action_side_effects("--open-docs");
                println!("Opening {DOCS_URL}");
                win_desktop_utils::open_url(DOCS_URL)?;
            }
            "--open-repo" => {
                print_action_side_effects("--open-repo");
                println!("Opening {REPO_URL}");
                win_desktop_utils::open_url(REPO_URL)?;
            }
            "--reveal-data-dir" => {
                print_action_side_effects("--reveal-data-dir");
                println!("Revealing {}", local_data_dir.display());
                win_desktop_utils::reveal_in_explorer(&local_data_dir)?;
            }
            "--create-docs-shortcut" => {
                print_action_side_effects("--create-docs-shortcut");
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
    for action in DEMO_ACTIONS {
        println!("  {:<24} {}", action.flag, action.description);
    }
    println!("  -h, --help               Show this help");

    if let Some(local_data_dir) = local_data_dir {
        println!();
        println!("Current app-data directory:");
        println!("  {}", local_data_dir.display());
    }
}

fn print_capabilities(local_data_dir: &Path) {
    println!("Capabilities:");
    println!(
        "  Detected platform: {} {} ({})",
        env::consts::OS,
        env::consts::ARCH,
        env::consts::FAMILY
    );
    println!("  App-data path: {}", local_data_dir.display());
    println!();
    println!("Demo actions:");
    println!(
        "  {:<24} supported: {:<18} side effects: {}",
        "startup flow",
        support_label(),
        STARTUP_SIDE_EFFECTS
    );

    for action in DEMO_ACTIONS {
        println!(
            "  {:<24} supported: {:<18} side effects: {}",
            action.flag,
            support_label(),
            action.side_effects
        );
    }

    println!();
    println!("Not exposed:");
    for (name, description) in NOT_EXPOSED_HELPERS {
        println!("  {name:<24} {description}");
    }
}

fn print_action_side_effects(flag: &str) {
    println!("Side effects: {}", side_effects_for(flag));
}

fn side_effects_for(flag: &str) -> &'static str {
    DEMO_ACTIONS
        .iter()
        .find(|action| action.flag == flag)
        .map(|action| action.side_effects)
        .unwrap_or("unknown")
}

fn support_label() -> &'static str {
    if cfg!(windows) {
        "yes"
    } else {
        "no (Windows-only)"
    }
}

#[cfg(test)]
mod tests {
    use super::{
        side_effects_for, DemoAction, APP_DATA_SIDE_EFFECTS, DEMO_ACTIONS, NOT_EXPOSED_HELPERS,
        STARTUP_SIDE_EFFECTS,
    };

    #[test]
    fn demo_action_list_is_metadata_only() {
        let actions = DEMO_ACTIONS
            .iter()
            .map(|action| {
                (
                    action.flag,
                    action.description,
                    side_effects_for(action.flag),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(actions.len(), 5);
        assert!(actions.iter().all(|(flag, description, side_effects)| {
            flag.starts_with("--") && !description.is_empty() && !side_effects.is_empty()
        }));
        assert!(!actions
            .iter()
            .any(|(_, _, side_effects)| *side_effects == "unknown"));
    }

    #[test]
    fn side_effect_labels_cover_visible_and_omitted_actions() {
        assert!(action("--open-docs")
            .side_effects
            .contains("default browser"));
        assert!(action("--open-repo")
            .side_effects
            .contains("registered URL handler"));
        assert!(action("--reveal-data-dir")
            .side_effects
            .contains("Explorer"));
        assert!(action("--create-docs-shortcut")
            .side_effects
            .contains("writes or overwrites"));
        assert!(action("--elevation")
            .side_effects
            .contains("does not request elevation"));

        assert!(APP_DATA_SIDE_EFFECTS.contains("creates the directory"));
        assert!(STARTUP_SIDE_EFFECTS.contains("single-instance guard"));
        assert!(not_exposed("Recycle Bin helpers").contains("no recycle action is available"));
        assert!(not_exposed("Elevation requests").contains("only checks current state"));
    }

    fn action(flag: &str) -> &'static DemoAction {
        DEMO_ACTIONS
            .iter()
            .find(|action| action.flag == flag)
            .unwrap_or_else(|| panic!("missing action metadata for {flag}"))
    }

    fn not_exposed(name: &str) -> &'static str {
        NOT_EXPOSED_HELPERS
            .iter()
            .find(|(helper_name, _)| *helper_name == name)
            .map(|(_, description)| *description)
            .unwrap_or_else(|| panic!("missing omitted-helper metadata for {name}"))
    }
}
