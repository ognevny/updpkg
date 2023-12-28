use {
    clap::Parser,
    log::{error, info},
    std::{env, path::PathBuf, process::Command},
};

#[derive(Debug, Parser)]
struct Args {
    /// new version of package (git commit if `--git` flag is set)
    #[clap(short = 'V', long)]
    version: Option<String>,

    /// make all possible things verbosely
    #[clap(short, long)]
    verbose: bool,

    /// set directory where recipe is contained (otherwise it's set as `pwd`)
    #[clap(short, long)]
    directory: Option<PathBuf>,

    /// invoke `makepkg`
    #[clap(short, long)]
    make: bool,

    /// invoke `makepkg-mingw`
    #[clap(short = 'M', long = "make-mingw")]
    make_mingw: bool,

    /// set flags for `makepkg` (like you invoke it manually)
    #[clap(short, long, default_value = "")]
    flags: String,

    /// set if package source is downloaded from git
    #[clap(long)]
    git: bool,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let (version, verbose, directory, make, make_mingw, flags, git) = (
        args.version,
        args.verbose,
        args.directory,
        args.make,
        args.make_mingw,
        args.flags,
        args.git,
    );

    if verbose {
        env::set_var("VERBOSE", "1");
    }

    if let Some(dir) = directory {
        info!("changing directory to {}", dir.to_string_lossy());
        env::set_current_dir(dir).unwrap_or_else(|e| error!("couldn't change directory: {e}"));
    }

    if !git {
        if let Some(ver) = version {
            info!("setting pkgver as {ver}, setting pkgrel as 1");
            // assuming sed doesn't fail
            Command::new("sed")
                .args(["-i", "-e", &format!("s|^pkgver=.*|pkgver={ver}|; s|^pkgrel=.*|pkgrel=1|")])
                .status()
                .unwrap();
        }
    // bless clippy
    } else if git {
        if let Some(ver) = version {
            info!("setting commit as {ver}, setting pkgrel as 1");
            Command::new("sed")
                .args([
                    "-i",
                    "-e",
                    &format!("s|^_commit=.*|_commit={ver}|; s|^pkgrel=.*|pkgrel=1|"),
                ])
                .status()
                .unwrap();
        }
    }

    match Command::new("updpkgsums").status() {
        Ok(_) => (),
        Err(e) => error!("couldn't update checksums: {e}, see message above"),
    }

    if make && make_mingw {
        error!("can't invoke both `makepkg` and `makepkg-mingw`");
    } else if make {
        match Command::new("makepkg").arg(&flags).status() {
            Ok(_) => (),
            Err(e) => error!("couldn't make package: {e}, see message above"),
        }
    } else if make_mingw {
        match Command::new("makepkg-mingw").arg(&flags).status() {
            Ok(_) => (),
            Err(e) => error!("couldn't make package: {e}, see message above"),
        }
    }
}
