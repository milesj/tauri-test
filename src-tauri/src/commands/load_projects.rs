use crate::utils::{load_json_file, load_yaml_file};
use nodejs_package_json::{PackageJson, WorkspacesField};
use nodejs_package_managers::pnpm::PnpmWorkspaceYaml;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use wax::{Glob, LinkBehavior};

fn determine_package_workspaces(
    root_package: &PackageJson,
    repository_path: &Path,
) -> Result<Option<Vec<String>>, String> {
    if let Some(workspaces) = &root_package.workspaces {
        return Ok(Some(match workspaces {
            WorkspacesField::Globs(globs) => globs.to_owned(),
            WorkspacesField::Config { packages, .. } => packages.to_owned(),
        }));
    };

    let pnpm_workspaces_path = repository_path.join("pnpm-workspace.yml");

    if pnpm_workspaces_path.exists() {
        let pnpm_workspace: PnpmWorkspaceYaml = load_yaml_file(&pnpm_workspaces_path)?;

        return Ok(Some(pnpm_workspace.packages));
    }

    Ok(None)
}

fn load_packages_from_workspaces(
    workspaces: Option<&[String]>,
    repository_path: &Path,
) -> Result<HashMap<PathBuf, PackageJson>, String> {
    let mut packages = HashMap::new();

    let Some(workspaces) = workspaces else {
        return Ok(packages);
    };

    for workspace in workspaces {
        let pattern = if workspace.ends_with("package.json") {
            workspace.to_owned()
        } else {
            format!("{workspace}/package.json")
        };

        let glob =
            Glob::new(&pattern).map_err(|error| format!("Invalid workspaces glob: {error}"))?;

        for entry in glob.walk_with_behavior(repository_path, LinkBehavior::ReadFile) {
            let entry = entry.map_err(|error| error.to_string())?;

            if entry.file_type().is_file() {
                packages.insert(entry.path().to_owned(), load_json_file(entry.path())?);
            }
        }
    }

    Ok(packages)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectsOutput {
    root_package: PackageJson,
    packages: HashMap<PathBuf, PackageJson>,
    workspaces: Option<Vec<String>>,
}

#[tauri::command]
pub fn load_projects(repository_path: PathBuf) -> Result<ProjectsOutput, String> {
    let root_package_path = repository_path.join("package.json");

    if !root_package_path.exists() {
        return Err("No root package.json found, unable to load projects!".into());
    }

    let root_package = load_json_file(&root_package_path)?;
    let workspaces = determine_package_workspaces(&root_package, &repository_path)?;
    let packages = load_packages_from_workspaces(workspaces.as_deref(), &repository_path)?;

    Ok(ProjectsOutput {
        workspaces,
        root_package,
        packages,
    })
}
