use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	///Move dotfile to the repository and track changes and state
	Track {
		///Track dotfile as a template, cannot be used on directories
		#[arg(short, long, conflicts_with = "directory")]
		template: bool,
		///Track a directory instead of a file
		#[arg(short, long)]
		directory: bool,
	},
	///Move dotfile out of repository and stop tracking changes and state
	Release {},
	///Symlink files from the repository to home
	Link {},
	///Removes a symlink for a file and updates state
	Unlink {},
	///Performs a full health check of the repository, state and links
	Health {},
}
