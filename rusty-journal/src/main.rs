mod cli;
use structopt::StructOpt;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    // get the command line agruments
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // unpack the journal file
    let journal_file = journal_file.expect("Failed to find journal file");

    // perform the action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),

    }
    .expect("Failed to perform action")
}