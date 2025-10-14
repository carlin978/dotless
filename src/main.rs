#![forbid(unsafe_code)]
mod cli;
mod config;
mod path;
mod state;

// Option::unwrap_or() is not stabilized as const at the time of writing
const DOTFILES_DIR: &'static str = match option_env!("DOTLESS_DIR") {
	Some(v) => v,
	None => ".dotfiles", //default dotfiles directory
};

fn main() -> anyhow::Result<()> {
	use crate::path::{expand_path_if_in_home_dir, get_repo_path};
	use crate::state::SerializeState;
	use anyhow::bail;
	use clap::Parser;
	use git2::Repository;
	use inquire::{Confirm, Text};

	let cli = cli::Cli::parse();

	match cli.command {
		cli::Commands::Init { update } => {
			let repo_path = get_repo_path();

			//Creates initial files and initial commit
			let initial_files = || -> anyhow::Result<()> {
				use crate::path::get_config_path;
				use std::fs;

				fs::write(get_config_path(), include_str!("../assets/config.toml"))?;
				fs::write(repo_path.join(".gitignore"), include_str!("../assets/gitignore"))?;
				fs::write(
					repo_path.join(".dotless.state"),
					state::State::default()
						.serialize_state()
						.expect("Default empty state should serialize"),
				)?;

				Ok(())
			};

			if update {
				use git2::Oid;

				let repo = Repository::open(repo_path.clone())?;

				if !Confirm::new("Any non-commited changes on the repository, including untracked and ignored files, will be lost! Continue?").prompt()? {
                    bail!("Canceled")
                }

				if repo.state() != git2::RepositoryState::Clean {
					bail!("Repository is not in a clean state")
				}

				{
					//Force clear working directory
					const EMPTY_TREE_OID: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
					let empty_tree_oid = Oid::from_str(EMPTY_TREE_OID)?;

					let empty_tree = repo.find_tree(empty_tree_oid)?;

					let mut checkout_builder = git2::build::CheckoutBuilder::new();
					checkout_builder.force();

					repo.checkout_tree(&empty_tree.into_object(), Some(&mut checkout_builder))?;
				}

				let branch = Text::new("Name of the new branch:").prompt()?;

				repo.set_head(&format!("refs/heads/{}", branch))?;

				initial_files()?;

				let mut index = repo.index()?;

				index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;

				index.write()?;

				let tree_id = index.write_tree()?;

				drop(index);

				let tree = repo.find_tree(tree_id)?;

				let sig = repo.signature()?;

				repo.commit(Some("HEAD"), &sig, &sig, "Init dotless", &tree, &[])?;
			} else {
				if repo_path.exists() {
					bail!("Repository location already exists, did you intend to use --update?")
				}

				let repo = Repository::init(repo_path.clone())?;

				initial_files()?;

				let mut index = repo.index()?;

				index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;

				index.write()?;

				let tree_id = index.write_tree()?;

				drop(index);

				let tree = repo.find_tree(tree_id)?;

				let sig = repo.signature()?;

				repo.commit(Some("HEAD"), &sig, &sig, "Init dotless", &tree, &[])?;
			}
		}
		cli::Commands::Track { template, directory, path } => {
			if let Some((canonical_path, home_path)) = expand_path_if_in_home_dir(path) {
				if directory {
					todo!()
				} else if template {
					todo!()
				} else {
					todo!()
				}
			} else {
				bail!("Path is not valid")
			}
		}
		cli::Commands::Untrack {} => todo!(),
		cli::Commands::Link { all } => todo!(),
		cli::Commands::Unlink { all } => todo!(),
		cli::Commands::Health {} => todo!(),
		cli::Commands::Commit => todo!(),
	};

	Ok(())
}
