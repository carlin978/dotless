use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	///Initialize repository with state and configuration
	Init {
		///Update existing repository
		///Adds a new branch to the existing repository and continues initialization from there,
		///skipping Git initializion steps
		#[arg(short, long)]
		update: bool,
	},
	///Move dotfile to the repository and track changes and state
	Track {
		///Track dotfile as a template, cannot be used on directories
		#[arg(short, long, conflicts_with = "directory")]
		template: bool,
		///Track a directory instead of a file
		#[arg(short, long)]
		directory: bool,
		///Path to the dotfile
		path: PathBuf,
	},
	///Move dotfile out of repository and stop tracking changes and state
	Untrack {},
	///Symlink files from the repository to home
	Link {
		///Skip selection and link all files
		#[arg(short, long)]
		all: bool,
	},
	///Removes a symlink for a file and updates state
	Unlink {
		///Skip selection and unlink all files
		#[arg(short, long)]
		all: bool,
	},
	///Create commit with interactive file selection and an automatic commit message
	Commit,
	///Performs a full health check of the repository, state and links
	Health {},
}
