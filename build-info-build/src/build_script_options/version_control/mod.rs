use build_info_common::VersionControl;

#[cfg(feature = "git")]
mod git;

#[cfg(feature = "git")]
fn get_git_info() -> anyhow::Result<VersionControl> {
	git::get_info().map(VersionControl::Git)
}

#[cfg(not(feature = "git"))]
fn get_git_info() -> anyhow::Result<VersionControl> {
	Err(anyhow::anyhow!("Git support is disabled"))
}

pub(crate) fn get_info() -> Option<VersionControl> {
	if cfg!(feature = "git") {
		match get_git_info() {
			Ok(info) => Some(info),
			Err(err) => {
				println!("cargo:warning=Failed to collect git info: {err:#}");
				None
			}
		}
	} else {
		None
	}
}
