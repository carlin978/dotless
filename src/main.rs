#![forbid(unsafe_code)]
mod cli;
mod config;
mod path;
mod state;

const DOTFILES_DIR: &'static str = match option_env!("DOTLESS_DIR") {
	Some(v) => v,
	None => ".dotfiles", //default dotfiles directory
};
const DOTFILES_HOME: &'static str = match option_env!("DOTLESS_HOME") {
	Some(v) => v,
	None => "home", //default name for home in dotfiles directory
};

fn main() -> anyhow::Result<()> {
	use crate::path::{expand_path_if_in_home_dir, get_repo_path};
	use anyhow::anyhow;
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
				state::State::default().write_state()?;

				let home = repo_path.join(DOTFILES_HOME);
				fs::create_dir(&home)?;
				fs::write(home.join(".gitkeep"), "")?;

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

				repo.commit(
					Some("HEAD"),
					&sig,
					&sig,
					include_str!("../assets/initial_commit_message.txt"),
					&tree,
					&[],
				)?;
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

				repo.commit(
					Some("HEAD"),
					&sig,
					&sig,
					include_str!("../assets/initial_commit_message.txt"),
					&tree,
					&[],
				)?;
			}
		}
		cli::Commands::Track { non_dotfile, template, directory, path, no_link } => {
			if let Some((canonical_path, home_path)) = expand_path_if_in_home_dir(path) {
				let repo_path = get_repo_path();

				if home_path.starts_with(DOTFILES_DIR) {
					bail!("Cannot track files in the dotfiles repo")
				}

				let home_path_str =
					home_path.to_str().expect("Non-Unicode paths are not supported");

				let mut state = state::read_state()?;

				if directory {
					todo!()
				} else if template {
					todo!()
				} else {
					use std::fs;
					use std::os::unix::fs::symlink;

					let local_path = if non_dotfile {
						home_path_str
					} else {
						home_path_str.strip_prefix('.').ok_or(anyhow!(
							"Path provided is not a dotfile, did you mean to use --non-dotfile?"
						))?
					};

					let file = repo_path.join(DOTFILES_HOME).join(local_path);

					let dotfile = state::Dotfile::new_file(
						home_path_str.to_owned(),
						local_path.to_owned(),
						non_dotfile,
					);

					state.add_dotfile(dotfile);

					{
						fs::create_dir_all(
							file.parent().expect("In-repo file path somehow was root"),
						)?;

						fs::copy(&canonical_path, &file)?;
						fs::remove_file(&canonical_path)?;

						if !no_link {
							symlink(&file, &canonical_path)?;
						}

						state.write_state()?;
					}

					println!("Successfully tracking {home_path_str}");
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
		cli::Commands::Git { args } => {
			use std::os::unix::process::CommandExt;
			use std::process::Command;
			let repo_path = get_repo_path();

			bail!(Command::new("git").args(args).current_dir(repo_path).exec());
		}
	};

	Ok(())
}
