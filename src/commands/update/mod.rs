use self_update::cargo_crate_version;
use clap::Args;

#[derive(Args, Debug)]
pub struct UpdateArgs {
    #[arg(long, default_value_t = false)]
    pub unstable: bool,
}

pub fn run(args: UpdateArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for updates...");
    println!("Current Version: {}", cargo_crate_version!());

    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("markuskooche")
        .repo_name("watcher")
        .build()?
        .fetch()?;

    let target = releases.iter().find(|r| {
        if args.unstable {
            true
        } else {
            !r.version.contains('-')
        }
    });

    match target {
        None => {
            println!("Already up to date.");
            Ok(())
        }
        Some(release) => {
            let current = semver::Version::parse(cargo_crate_version!())?;
            let latest = semver::Version::parse(&release.version)?;

            if latest <= current {
                println!("Already up to date.");
                return Ok(());
            }

            println!("New version available: {} -> {}", current, latest);

            let status = self_update::backends::github::Update::configure()
                .repo_owner("markuskooche")
                .repo_name("watcher")
                .bin_name("watcher")
                .show_download_progress(true)
                .target_version_tag(&format!("{}", release.version))
                .current_version(cargo_crate_version!())
                .build()?
                .update()?;

            match status.updated() {
                true  => println!("Successfully updated to version {}!", status.version()),
                false => println!("Already up to date."),
            }

            Ok(())
        }
    }
}
