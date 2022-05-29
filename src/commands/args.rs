use clap::{Parser, Subcommand};

use crate::{
    cache,
    commands::{list::list_templates, pull::pull_template},
};

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    Pull {
        template: Option<String>,

        #[clap(short, long, default_value = ".gitignore")]
        output: String,

        #[clap(long)]
        append: bool,

        #[clap(long)]
        overwrite: bool,

        #[clap(long)]
        no_overwrite: bool,
    },
    List,
    Purge,
}

impl Commands {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Commands::List => list_templates()?,
            Commands::Pull {
                output,
                template,
                append,
                overwrite,
                no_overwrite,
            } => pull_template(output, template.clone(), append, overwrite, no_overwrite)?,
            Commands::Purge => cache::purge()?,
        };

        Ok(())
    }
}

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(PartialEq, Eq)]
pub enum PullOpts {
    Append,
    Overwrite,
    NoOverwrite,
}

impl PullOpts {
    pub fn get_opt(append: &bool, overwrite: &bool, no_overwrite: &bool) -> Option<Self> {
        let mut opts_vec = Vec::<Self>::new();

        if *append {
            opts_vec.push(PullOpts::Append);
        }
        if *overwrite {
            opts_vec.push(PullOpts::Overwrite);
        }
        if *no_overwrite {
            opts_vec.push(PullOpts::NoOverwrite);
        }

        if opts_vec.len() > 1 {
            panic!("Only one pull option can be specified");
        }

        opts_vec.pop()
    }
}
