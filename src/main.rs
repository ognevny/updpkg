use {
    clap::Parser,
    log::{error, info, warn},
    std::{
        env, fs,
        io::Error,
        path::PathBuf,
        process::{Command, ExitStatus},
    },
};

#[derive(Debug, Parser)]
#[clap(about, version, long_about = None)]
struct Args {
    /// path to directory with PKGBUILD
    #[clap(default_value = ".")]
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

    /// removes files from directory and recipe
    #[clap(short, long, value_name = "FILES", num_args = 1..)]
    rm: Option<Vec<String>>,

    /// use sd instead of sed
    #[clap(long = "use-sd")]
    use_sd: bool,
}

#[inline]
fn sed(re: &str) -> Result<ExitStatus, Error> {
    Command::new("sed").args(["-i", re, "PKGBUILD"]).status()
}

#[inline]
#[cfg(feature = "sd")]
fn sd(old: &str, new: &str) -> Result<ExitStatus, Error> {
    Command::new("sd").args([old, new, "PKGBUILD"]).status()
}

fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let args = Args::parse();
    let (ver, dir, make, make_mingw, git, rm) =
        (args.ver, args.directory, args.make, args.make_mingw, args.git, args.rm);

    #[cfg(feature = "sd")]
    let use_sd = args.use_sd;

    if dir != PathBuf::from(".") {
        info!("changing directory to {}", dir.to_string_lossy());
        env::set_current_dir(dir).unwrap_or_else(|e| error!("couldn't change directory: {e}"));
    }

    if let Some(files) = rm {
        for file in files.iter() {
            info!("removing {file} from recipe");
            if use_sd {
                sd(&format!(r#"^.*{file}\s*"#), "").unwrap();
                sd(r#"^")"#, ")").unwrap();
                sd(r#"^"$"#, "").unwrap();
            } else {
                sed(&format!(r#"s|^.*{file}\s*||g; s|^")|)|g; s|^"$||g"#)).unwrap();
            }
            info!("removing {file} from directory");
            fs::remove_file(file).unwrap_or_else(|e| warn!("couldn't remove file: {e}"));
        }
    }

    if let Some(ver) = ver {
        info!("setting `pkgver` as {ver}, setting `pkgrel` as 1");
        if use_sd {
            sd("^pkgver=.*", &format!("pkgver={ver}")).unwrap();
            sd("^pkgrel=.*", "pkgrel=1").unwrap();
        } else {
            sed(&format!("s|^pkgver=.*|pkgver={ver}|; s|^pkgrel=.*|pkgrel=1|")).unwrap();
        }
    }

    if let Some(ver) = git {
        info!("setting commit as {ver}, setting pkgrel as 1");
        if use_sd {
            sd("^_commit=.*", &format!("_commit={ver}")).unwrap();
            sd("^pkgrel=.*", "pkgrel=1").unwrap();
        } else {
            sed(&format!("s|^_commit=.*|_commit={ver}|; s|^pkgrel=.*|pkgrel=1|")).unwrap();
        }
        if make.is_none() && make_mingw.is_none() {
            warn!("you may need to run `makepkg` manually to update `pkgver`");
        }
    }

    info!("updating checksums");
    // Windows doesn't support sh executables
    if cfg!(windows) {
        match Command::new("sh").arg("updpkgsums").status() {
            Ok(_) => (),
            Err(e) => error!("couldn't update checksums: {e}"),
        }
    } else {
        match Command::new("updpkgsums").status() {
            Ok(_) => (),
            Err(e) => error!("couldn't update checksums: {e}"),
        }
    }

    if make.is_some() && make_mingw.is_some() {
        error!("can't invoke both `makepkg` and `makepkg-mingw`");
    } else if let Some(flags) = make {
        info!("using `makepkg` with flags {flags}");
        if cfg!(windows) {
            match Command::new("sh").args(["makepkg", &flags]).status() {
                Ok(_) => (),
                Err(e) => error!("couldn't make package: {e}"),
            }
        } else {
            match Command::new("makepkg").arg(&flags).status() {
                Ok(_) => (),
                Err(e) => error!("couldn't make package: {e}"),
            }
        }
    // MSYS2 only
    } else if let Some(flags) = make_mingw {
        info!("using `makepkg-mingw` with flags {flags}");
        match Command::new("sh").args(["makepkg-mingw", &flags]).status() {
            Ok(_) => (),
            Err(e) => error!("couldn't make package: {e}"),
        }
    }
}
