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

    /// set directory where recipe is contained (otherwise it's set as `pwd`)
    #[clap(short, long)]
    directory: Option<PathBuf>,

    /// invoke `makepkg`
    #[clap(short, long)]
    make: bool,

    /// set if you use msys2 suite
    #[clap(short = 'M', long)]
    msys2: bool,

    /// set if you use msys2 mingw suite (automaticly sets `msys2` as `true`)
    #[clap(long = "msys2-mingw")]
    msys2_mingw: bool,

    /// set flags for `makepkg` (like you invoke it manually)
    #[clap(short, long, default_value = "")]
    flags: String,

    /// set if package source is downloaded from git
    #[clap(long)]
    git: bool,

    /// ugly fix for PATH issue
    #[clap(long)]
    ci: bool,
}

fn get_updpkgsums(msys2: bool, ci: bool) -> PathBuf {
    // hardcode as not everyone set /usr/bin from msys2 in PATH
    if msys2 {
        PathBuf::from(r"C:\msys64\usr\bin\updpkgsums")
    } else if ci {
        PathBuf::from(r"D:\M\msys64\usr\bin\updpkgsums")
    } else {
        PathBuf::from("/usr/bin/makepkg")
    }
}

fn get_makepkg(msys2: bool, msys2_mingw: bool, ci: bool) -> PathBuf {
    if msys2_mingw {
        PathBuf::from(r"C:\msys64\usr\bin\makepkg-mingw")
    } else if msys2 {
        PathBuf::from(r"C:\msys64\usr\bin\makepkg")
    } else if ci {
        PathBuf::from(r"D:\M\msys64\usr\bin\makepkg-mingw")
    } else {
        PathBuf::from("/usr/bin/makepkg")
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("{PATH: {:?}", env::var("PATH").unwrap());

    let args = Args::parse();
    let (version, directory, make, mut msys2, msys2_mingw, flags, git, ci) = (
        args.version,
        args.directory,
        args.make,
        args.msys2,
        args.msys2_mingw,
        args.flags,
        args.git,
        args.ci,
    );

    if msys2_mingw {
        msys2 = true
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
                .args([
                    "-i",
                    "-e",
                    &format!("s|^pkgver=.*|pkgver={ver}|; s|^pkgrel=.*|pkgrel=1|"),
                    "PKGBUILD",
                ])
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
                    "PKGBUILD",
                ])
                .status()
                .unwrap();
        }
    }

    match Command::new(get_updpkgsums(msys2, ci)).status() {
        Ok(_) => (),
        Err(e) => error!("couldn't update checksums: {e}"),
    }

    if make && !msys2 {
        match Command::new(get_makepkg(msys2, msys2_mingw, ci))
            .arg(&flags)
            .status()
        {
            Ok(_) => (),
            Err(e) => error!("couldn't make package: {e}"),
        }
    } else if make && msys2 {
        match Command::new(get_makepkg(msys2, msys2_mingw, ci))
            .arg(&flags)
            .status()
        {
            Ok(_) => (),
            Err(e) => error!("couldn't make package: {e}"),
        }
    }
}
