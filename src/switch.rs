use clap::*;
use dialoguer::*;
use git2::*;

pub fn exec(matches: &ArgMatches) {
    // fetch local repository branches
    let branches = fetch_branches();

    // output to stdout select branch
    let selection_branch = get_selection_branch(branches);

    // git checkout select_branch
    checkout(selection_branch);
}
