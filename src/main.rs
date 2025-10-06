use clap::Parser;

mod cli;
mod config;
mod state;

fn main() {
	let cli = cli::Cli::parse();

	match cli.command {
		cli::Commands::Track { template, directory } => {
			if directory {
				todo!()
			} else if template {
				todo!()
			} else {
				todo!()
			}
		}
		cli::Commands::Release {} => todo!(),
		cli::Commands::Link {} => todo!(),
		cli::Commands::Unlink {} => todo!(),
		cli::Commands::Health {} => todo!(),
	};
}
