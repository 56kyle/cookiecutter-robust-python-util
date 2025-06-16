use std::path::PathBuf;
use git2::{BranchType, ObjectType, Repository};
use git2::build::CheckoutBuilder;


/// Sets up the local git repo.
pub fn setup_git_local(repo_folder: PathBuf) -> eyre::Result<()> {
    let mut repo = match Repository::discover(&repo_folder) {
        Ok(repo) => repo,
        Err(_) => {
            println!("Initializing repo at {:?}", repo_folder);
            Repository::init(&repo_folder).expect("Failed to initialize repo")
        }
    };
    println!("Repo found at {:?}", repo_folder);
    _with_changes_stashed(&mut repo, _setup_branches).expect("Failed to set up branches");

    _checkout_branch(&repo, "develop").expect("Failed to checkout branch 'main'");
    Ok(())
}

/// Initialize the repository if needed.
fn _get_or_create_repository(repo_folder: PathBuf) -> Repository {
    println!("Looking for repo at {:?}", repo_folder);
    let repo = match Repository::discover(&repo_folder) {
        Ok(repo) => {
            println!("Repo found at {:?}", repo_folder);
            repo
        },
        Err(_) => {
            println!("Initializing repo at {:?}", repo_folder);
            Repository::init(&repo_folder).expect("Failed to initialize repo")
        }
    };
    repo
}

/// Stashes local changes and then executes the provided function before unstashing the changes.
fn _with_changes_stashed<F, T>(repo: &mut Repository, f: F) -> eyre::Result<T>
where F: FnOnce(&mut Repository) -> eyre::Result<T> {
    let oid = _stash_local_changes(repo)
            .expect("Failed to stash local changes");
    let result = f(repo)
            .expect("Failed to run the provided function with changes stashed");
    _unstash_local_changes(repo, &oid)
            .expect("Failed to unstash local changes");
    Ok(result)
}

/// Stash any local changes and return the Oid of the stash.
fn _stash_local_changes(repo: &mut Repository) -> eyre::Result<git2::Oid>{
    println!("Stashing local changes...");
    let signature = repo.signature()
            .expect("Failed to get a signature for the repo");
    let message = "meta: temporary stash while running setup-git";
    
    match repo.stash_save(&signature, &message, None) {
        Ok(oid) => Ok(oid),
        Err(e) => {
            panic!("Failed to stash local changes");
        }
    }
}

/// Unstash changes based on the Oid provided.
fn _unstash_local_changes(repo: &mut Repository, oid: &git2::Oid) -> eyre::Result<()> {
    let mut stash_index = None;
    repo.stash_foreach(|i, _, current_oid| {
        if current_oid == oid {
            stash_index = Some(i);
            return false
        }
        return true
    }).expect("Failed to find a stash with the expected Oid");
    match stash_index {
        Some(i) => {
            Ok(repo.stash_pop(i, None).expect("Failed to unstash changes"))
        },
        None => {panic!("Failed to find a stash with the expected Oid")}
    }
}

/// Checks out a branch using the provided branch_name and sets the HEAD to it.
fn _checkout_branch(repo: &Repository, branch_name: &str) -> eyre::Result<()> {
    let full_reference = repo.find_reference(&format!("refs/heads/{}", branch_name))
            .expect(&format!("Failed to find the head of branch '{}'", branch_name));
    let object = full_reference.peel(ObjectType::Commit)
            .expect(&format!("Failed to get the commit object for branch '{}'", branch_name));
    let reference_name = full_reference.name()
            .expect("Failed to get the name of the reference");

    repo.set_head(&reference_name)
            .expect(&format!("Failed to set the head of branch '{}'", branch_name));

    let mut checkout_opts = CheckoutBuilder::new();
    checkout_opts.force();

    repo.checkout_tree(&object, Some(&mut checkout_opts))
            .expect("Failed to checkout tree");

    Ok(())
}

/// Set up all desired branches for cookiecutter-robust-python.
fn _setup_branches(repo: &mut Repository) -> eyre::Result<()>{
    _setup_main_branch(repo).expect("Failed to set up branch 'main'");
    _setup_develop_branch(repo).expect("Failed to set up branch 'develop'");
    Ok(())
}

/// Set up the main branch.
fn _setup_main_branch(repo: &Repository) -> eyre::Result<git2::Branch> {
    let master_branch = repo.find_branch("master", BranchType::Local).ok();
    let main_branch = repo.find_branch("main", BranchType::Local).ok();

    let main = match (master_branch, main_branch) {
        (Some(_), Some(_)) => {
            panic!("Repo already has both a master and main branch. Please remove one to continue.")
        },
        (Some(mut master), None) => {
            println!("Renaming the existing branch 'master' to 'main'...");
            let main = master.rename("main", true)
                    .expect("Failed to rename 'master' to 'main'");
            main
        },
        (None, Some(main)) => {
            println!("Found branch 'main', skipping creation...");
            main
        },
        (None, None) => {
            _setup_branch_from_head(repo, "main").expect("Failed to create branch 'main'")
        }
    };
    Ok(main)
}

/// Set up the develop branch.
fn _setup_develop_branch(repo: &Repository) -> eyre::Result<git2::Branch> {
    _setup_branch_from_head(repo, "develop")
}

/// Sets up a branch relative to HEAD named after the branch_name provided.
fn _setup_branch_from_head<'repo>(
    repo: &'repo Repository,
    branch_name: &str
) -> eyre::Result<git2::Branch<'repo>> {
    println!("Creating branch '{}'...", branch_name);
    let head_ref = repo.head()
            .expect("Failed to get HEAD reference");
    let head_commit = head_ref.peel_to_commit()
            .expect("Failed to get HEAD commit");
    let main = repo.branch(branch_name, &head_commit, false)
            .expect(&format!("Failed to create a branch named '{}'", branch_name));
    Ok(main)
}

