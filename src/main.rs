use {
    clap::Parser,
    log::{error, info, warn},
    std::{env, path::PathBuf, process::Command},
};

#[derive(Debug, Parser)]
#[clap(about, version, author, long_about = None)]
struct Args {
    /// path to directory with PKGBUILD
    #[clap(required = true)]
    directory: PathBuf,

    /// new version of package (tarball)
    #[clap(short, long = "ver", value_name = "VERSION")]
    ver: Option<String>,

    /// invoke `makepkg` with optional flags (like you are invoking it manually)
    #[clap(short, long, value_name = "FLAGS")]
    make: Option<String>,

    /// the same as `make`, but for `makepkg-mingw`
    #[clap(short = 'M', long = "make-mingw", value_name = "FLAGS")]
    make_mingw: Option<String>,

    /// specify commit SHA
    #[clap(long, value_name = "SHA")]
    git: Option<String>,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let (ver, dir, make, make_mingw, git) = (
        args.ver,
        args.directory,
        args.make,
        args.make_mingw,
        args.git,
    );

    info!("changing directory to {}", dir.to_string_lossy());
    env::set_current_dir(dir).unwrap_or_else(|e| error!("couldn't change directory: {e}"));

    if let Some(ver) = ver {
        info!("setting pkgver as {ver}, setting pkgrel as 1");
        match Command::new("sed")
            .args([
                "-i",
                "-e",
                &format!("s|^pkgver=.*|pkgver={ver}|; s|^pkgrel=.*|pkgrel=1|"),
                "PKGBUILD",
            ])
            .status()
        {
            Ok(_) => (),
            Err(e) => error!("couldn't sed PKGBUILD: {e}"),
        }
    }
    if let Some(ver_git) = git {
        info!("setting commit as {ver_git}, setting pkgrel as 1");
        match Command::new("sed")
            .args([
                "-i",
                "-e",
                &format!("s|^_commit=.*|_commit={ver_git}|; s|^pkgrel=.*|pkgrel=1|"),
                "PKGBUILD",
            ])
            .status()
        {
            Ok(_) => (),
            Err(e) => error!("couldn't sed PKGBUILD: {e}"),
        }
        if make.is_none() && make_mingw.is_none() {
            warn!("you may need to run `makepkg` manually to update `pkgver`");
        }
    }
    info!("updating checksums");
    match Command::new("sh").arg("updpkgsums").status() {
        Ok(_) => (),
        Err(e) => error!("couldn't update checksums: {e}"),
    }

    if make.is_some() && make_mingw.is_some() {
        error!("can't invoke both `makepkg` and `makepkg-mingw`");
    } else if let Some(flags) = make {
        info!("using `makepkg` with flags {flags}");
        match Command::new("sh").args(["makepkg", &flags]).status() {
            Ok(_) => (),
            Err(e) => error!("couldn't make package: {e}"),
        }
    } else if let Some(flags) = make_mingw {
        info!("using `makepkg-mingw` with flags {flags}");
        match Command::new("sh").args(["makepkg-mingw", &flags]).status() {
            Ok(_) => (),
            Err(e) => error!("couldn't make package: {e}"),
        }
    }
}
