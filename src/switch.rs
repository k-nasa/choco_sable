use clap::*;
use dialoguer::*;
use git2::*;

pub fn exec(matches: &ArgMatches) {
    // fetch local repository branches
    let branches = fetch_branches();

    // output to stdout select branch
    let selection_branch = get_selection_branch(branches);
type Branches = Vec<String>;
type BranchName = String;

fn fetch_branches() -> Branches {
    let repo = Repository::open(".").expect("failed: open repository");

    let mut branches = Vec::new();

    let filter = None;
    for branch in repo.branches(filter).expect("failed: fetch branches") {
        let (branch, _) = branch.unwrap();

        // Result<Option<String>という構造になっている
        // unwrap().unwrap() するのも分かりづらいのでifで剥がす
        let name = if let Ok(Some(name)) = branch.name() {
            name
        } else {
            panic!("failed")
        };

        branches.push(name.to_string())

    }

    branches
}

    // git checkout select_branch
    checkout(selection_branch);
}
