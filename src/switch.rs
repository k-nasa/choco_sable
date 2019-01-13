use crate::util::*;
use std::process::Command;

pub fn exec() {
    let branches = fetch_branches();
    let selection_branch = get_selection_branch(&branches);

    checkout(&selection_branch);
}

fn checkout(branch: &BranchName) {
    let mut editor_process = Command::new("git")
        .arg("checkout")
        .arg(branch)
        .spawn()
        .unwrap();

    editor_process.wait().unwrap();
}
