use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;

pub type Branches = Vec<String>;
pub type BranchName = String;

pub fn fetch_branches() -> Branches {
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

pub fn get_selection_branch(branches: &Branches) -> BranchName {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select branch")
        .default(0)
        .items(&branches)
        .interact()
        .unwrap();

    (&branches[selection]).to_string()
}
