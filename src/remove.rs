use crate::util::*;
use std::process::Command;

pub fn exec() {
    let branches = fetch_branches();
    let selection_branch = get_selection_branch(&branches);

    delete_branch(&selection_branch);
}

fn delete_branch(branch: &BranchName) {
    let mut editor_process = Command::new("git")
        .arg("branch")
        .arg("-D")
        .arg(branch)
        .spawn()
        .unwrap();

    editor_process.wait().unwrap();
}
