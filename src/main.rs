//!     ==== uqnote ====
//!     Ultra-Quick Note
//!
//! A bite-sized note-taking app with
//! multiple ways to create, edit, and save
//! notes.
//
// TODOS:
//
// 1. Clap implementation: 

// == Editing ==
// add "Note" -- Add a raw line to the temp. file
// newtag/nt "Name" "Refcode" -- Create a new group with name and an optional reference code
// clear/clr -- Clear the temp. file

// == File Output ==
// out [FILE] -- Output the current temp. file to a permanent file name provided by user
// oedit/oe [EDITOR] -- Open Editor, open the notebook in the uqnote internal editor. (TODO)
// == Visualization ==
// view/vw -- Print the current notes to terminal

use std::path::PathBuf;
use clap::{arg, Command};

fn cli() -> Command {
    Command::new("uqnote")
        .author("Ryan, ryanmc.webflow.io")
        .about("A command-line notetaking app.")
        .version("0.1")
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Add line to notebook.")
                .short_flag('a')
                .arg(arg!(<LINE> "Line to be added."))
                .arg(arg!(<TAG> "Optional tag the new line will be associated with.").required(false))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("clear")
                .about("Clear all lines from notebook.")
                .short_flag('c')

        )
        .subcommand(
            Command::new("newtag")
                .about("Create a new tag.")
                .short_flag('t')
                .arg_required_else_help(true)
                .arg(arg!(<TAG> "Creates a new tag"))
        )
        .subcommand(
            Command::new("out")
                .about("Output current notebook to file.")
                .short_flag('o')
                .arg_required_else_help(true)
                .arg(arg!(<PATH> "Path to output notebook."))
                .arg(arg!(<TAG> "Optional tag to output. Otherwise, will output full notebook.").required(false))
        )
        .subcommand(
            Command::new("view")
                .about("View a visualization of current notebook in the commandline.")
                .short_flag('v')
        )
}
fn main() {
    let matches = cli().get_matches();
    match matches.subcommand(){
        Some(("add", sub_matches)) => {println!("UQNOTE: '{}' Added to notebook.",sub_matches.get_one::<String>("LINE").expect("Couldn't get line."))}
        Some(("newtag", sub_matches)) => {println!("Tag '{}' created in notebook",sub_matches.get_one::<String>("TAG").expect("Couldn't get tag."))}
        Some(("clear", sub_matches)) => {println!("Notebook cleared.")}
        Some(("view", sub_matches)) => {println!("Printing notebook...")}
        Some(("out", sub_matches)) => {println!("Notebook saved to {}", sub_matches.get_one::<String>("PATH").expect("Couldn't get path."))}
        _ => unreachable!()

    }
}


