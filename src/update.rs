use self_update::cargo_crate_version;

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for updates...");
    println!("Current Version: {}", cargo_crate_version!());

    let status = self_update::backends::github::Update::configure()
        .repo_owner("markuskooche")
        .repo_name("watcher")
        .bin_name("watcher")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    match status.updated() {
        true  => println!("Successfully updated to version {}!", status.version()),
        false => println!("Already up to date ({}).", status.version()),
    }

    Ok(())
}
