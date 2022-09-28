use std::io::Write;

use clap::Parser;
use directories::ProjectDirs;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,

    /// The name of the web app
    name: String,

    /// The default URL used for the app
    url: String,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Run,
    GenerateDesktopFile {
        #[arg(short, long)]
        /// Output filename. If not provided, the content will be printed to stdandard output
        output: Option<std::path::PathBuf>,

        #[arg(short, long)]
        /// Extra lines to be added to the desktop file. Accepts multiple values, each printed on a separate line.
        extra: Vec<String>,
    },
}

macro_rules! desktop_template {
    () => {
        r#"[Desktop Entry]
Type=Application
Exec=firefox-web-app-launcher "{name}" "{url}" run
Name={name}
StartupWMClass={name}
StartupNotify=true
Terminal=false"#
    };
}

fn main() {
    let args = Args::parse();

    let project_dir = ProjectDirs::from("", "", "firefox-web-app-launcher")
        .expect("Can't construct project directories.");

    let profile_path = project_dir.data_dir().join("profiles").join(&args.name);

    match args.action {
        Action::Run => {
            std::fs::create_dir_all(&profile_path)
                .expect("Failed to create profile base directory");
            std::process::Command::new("firefox")
                .arg("-new-instance")
                .arg("-profile")
                .arg(profile_path.as_os_str())
                .arg("--class")
                .arg(args.name)
                .arg(args.url)
                .spawn()
                .expect("Failed to run firefox")
                .wait()
                .expect("Firefox exits in error.");
        }
        Action::GenerateDesktopFile {
            output: None,
            extra,
        } => {
            println!(desktop_template!(), name = &args.name, url = &args.url);
            for extra in extra {
                println!("{}", extra);
            }
        }
        Action::GenerateDesktopFile {
            output: Some(filename),
            extra,
        } => {
            let mut file = std::fs::File::create(filename).expect("Failed to open file.");
            writeln!(
                file,
                desktop_template!(),
                name = &args.name,
                url = &args.url
            )
            .expect("Failed to write content.");
            for extra in extra {
                writeln!(file, "{}", extra).expect("Failed to write content.");
            }
        }
    }
}
