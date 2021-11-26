use wix::{clear, exit, pkg, question, Configuration, Information};

#[tokio::main]
async fn main() {
    let config: Configuration = Configuration {
        repo: "https:://github.com/m1ten/wix-pkgs".to_string(),
        mirror: None,
    };

    let info = Information {
        name: "wix".to_string(),
        author: "miten".to_string(),
        version: "0.1.0".to_string(),
        description: "cross platform package manager".to_string(),
        license: "zlib".to_string(),
        git: "https://github.com/m1ten/wix".to_string(),
    };

    let args = wix::args::Arguments::new(info.clone());

    println!("Wix!\n");

    if wix::setup::is_super() {
        eprintln!("{}", "Error: You are running wix as root.");
        eprintln!("{}", "Please run wix as a normal user.");
        exit!(1);
    }

    if !wix::setup::is_python_installed() {
        eprintln!("Error: Python >=3.8 is not installed.");
        eprintln!("Please install and add Python to path then try again.");
        exit!(127);
    }

    if !wix::setup::is_internet_connected().await {
        eprintln!("Error: Internet connection is not available.");
        eprintln!("Please check your internet connection and try again.");
        exit!(1);
    }

    // check if config file exists
    if !dirs::home_dir().unwrap().join("wix/wix.py").exists() {
        // run setup?
        println!("{:?}", info.clone());
        if question!("Would you like to run setup?") {
            wix::setup::run(info.clone(), config.clone(), args.clone());
        } else {
            exit!(1);
        }
    }

    // check if wix.py is up to date

    let pkg_name;
    let pkg_version;

    match args.package.get_index(0) {
        Some(p) => {
            pkg_name = p.0.clone();
            pkg_version = p.1.clone();
        },
        None => {
            pkg_name = "".to_string();
            pkg_version = "".to_string();
        },
    };

    let os = wix::setup::get_os();
    let arch = wix::setup::get_arch();
    let mut path = dirs::home_dir()
        .unwrap()
        .join("wix/cache/{name}/{os}-{arch}/{version}.py")
        .to_str()
        .unwrap()
        .to_string()
        .replace("{name}", pkg_name.as_str())
        .replace("{os}", os.as_str())
        .replace("{arch}", arch.as_str())
        .replace("{version}", pkg_version.as_str());

    if cfg!(windows) {
        path = path.replace("/", "\\");
    }

    let package = pkg::Package::get_package(
        pkg_name.clone().to_lowercase(),
        pkg_version.clone(),
        os.clone(),
        arch.clone(),
    )
    .await
    .unwrap();

    match args.status.as_str() {
        "install" => match package.as_str() {
            "404: Not Found" => {
                eprintln!(
                    "Error: {}@{} not found in repository.",
                    pkg_name, pkg_version
                );
                exit!(1);
            }
            _ => pkg::Package::install(package, pkg_name, path),
        },
        "uninstall" => match package.as_str() {
            "404: Not Found" => {
                eprintln!("Error: Package not found in repository.");
                exit!(1);
            }
            _ => pkg::Package::uninstall(package, pkg_name, path),
        },
        "search" => match package.as_str() {
            "404: Not Found" => {
                eprintln!("Error: Package not found in repository.");
                exit!(1);
            }
            _ => {
                println!(
                    "{} cloned to path '{}'.\nReview Script\n{}",
                    pkg_name, path, package
                );
                exit!(0);
            }
        },
        "update" => println!("Updating {}", pkg_name),
        "clean" => {
            println!("Cleaning up.");
            std::fs::remove_dir_all(dirs::home_dir().unwrap().join("wix/cache/"))
                .unwrap_or_else(|err| {
                    eprintln!("Error Cleaning Cache: {}", err);
                    exit!(1);
                });
            
            println!("Cache Cleaned!");
            exit!(0);
        },
        _ => {
            clear!();
            println!("{}", args.help);
            exit!(0);
        },
    }
}
