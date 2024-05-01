use leptos::*;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, env};

// TODO: Should be configurable
const ROOT_PATH: &str = "./";

enum ValidPathError {
    InvalidPath(String),
    UnknownError,
}

impl ValidPathError {
    fn as_str(&self) -> String {
        match self {
            ValidPathError::InvalidPath(path) => format!("Invalid path : '{}'", path),
            _ => "Unknown ValidPathError".to_string()
        }
    }
}

#[derive(Clone)]
struct ValidPath(String);

impl ValidPath {
    fn new(mut path: String) -> Result<Self, ValidPathError> {
        if path.is_empty() {
            path = "./".to_string();
        }

        let path = fs::canonicalize(path).map_err(|_| ValidPathError::UnknownError)?;
        let root_path = fs::canonicalize(ROOT_PATH).map_err(|_| ValidPathError::UnknownError)?;

        if !path.starts_with(root_path) {
            Err(ValidPathError::InvalidPath(
                path.to_string_lossy().to_string(),
            ))
        } else {
            Ok(ValidPath(path.to_string_lossy().to_string()))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct FsEntryData {
    name: String,
    path: String,
}

#[derive(Serialize, Deserialize, Clone)]
enum FsEntry {
    File(FsEntryData),
    Dir(FsEntryData),
}

impl fmt::Display for FsEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsEntry::File(data) => write!(f, "File: {}", data.name),
            FsEntry::Dir(data) => write!(f, "Dir: {}", data.name),
        }
    }
}

async fn get_dir_content(path: ValidPath) -> Result<Vec<FsEntry>, ServerFnError> {
    let mut entries = fs::read_dir(path.as_str())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .map(|res| {
            let name = res
                .as_ref()
                .map(|e| e.file_name().into_string())
                .unwrap_or(Ok(String::from("Invalid format")));

            let current_path = env::current_dir()
                .unwrap().to_string_lossy().to_string();

            // Path from relative root
            let path = res
                .as_ref()
                .map(|e| e.path())
                .unwrap_or(std::path::PathBuf::new())
                .to_string_lossy()
                .to_string();

            let (_, path) = path.split_at(current_path.len());
            let path = path.to_string();

            let is_dir = res
                .as_ref()
                .map(|e| e.file_type().map(|t| t.is_dir()))
                .unwrap_or(Ok(false))
                .unwrap_or(false);

            match is_dir {
                true => name.map(|name| FsEntry::Dir(FsEntryData { name, path })),
                false => name.map(|name| FsEntry::File(FsEntryData { name, path })),
            }
        })
        .collect::<Result<Vec<FsEntry>, _>>()
        .map_err(|_e| ServerFnError::ServerError("Unexpected error".to_string()))?;

    entries.sort_by(|a, b| match (a, b) {
        (FsEntry::Dir(a), FsEntry::Dir(b)) => a.name.cmp(&b.name),
        (FsEntry::File(a), FsEntry::File(b)) => a.name.cmp(&b.name),
        (FsEntry::Dir(_), FsEntry::File(_)) => std::cmp::Ordering::Less,
        (FsEntry::File(_), FsEntry::Dir(_)) => std::cmp::Ordering::Greater,
    });

    Ok(entries)
}

#[component]
pub fn ListView(path: String) -> impl IntoView {
    let path = ValidPath::new(path.clone());

    match path {
        Ok(valid_path) => {
            let entries = create_resource(
                || (),
                move |_| {
                    let path_clone = valid_path.clone();
                    async move { get_dir_content(path_clone).await }
                },
            );
            view! {
                <div>
                <Suspense
                    fallback=move || view! {
                        <p>{"Loading..."}</p>
                    }
                >
                            {move || {
                                entries.get()
                                    .map(|entries|
                                        match entries {
                                            Err(e) => view! {
                                                <div>
                                                    <p><span style="text-bold">{"Error: "}</span>{e.to_string()}</p>
                                                </div>
                                            },
                                            Ok(entries) => view! {
                                                <div>
                                                    <ul class="f-full">
                                                        {entries.iter().map(|entry| view! {
                                                            <ListItem entry=entry/>
                                                        }).collect::<Vec<_>>()}
                                                    </ul>
                                                </div>
                                            }
                                        }
                                    )
                            }}
                </Suspense>
                </div>
            }
        }
        Err(err) => {
            view! {
                <div>
                    <p><span style="text-bold">{"Error: "}</span>{err.as_str()}</p>
                </div>
            }
        }
    }
}

#[component]
fn ListItem<'a>(entry: &'a FsEntry) -> impl IntoView {
    let content = match entry {
        FsEntry::File(file) => file.name.as_str(),
        FsEntry::Dir(dir) => dir.name.as_str(),
    }
    .to_string();

    let href = match entry {
        // TODO : If it is a file, download the file
        FsEntry::File(file) => file.path.as_str(),
        FsEntry::Dir(dir) => dir.path.as_str(),
    }
    .to_string();

    view! {
        <li class="w-full bg-sky-950 text-white p-2 border border-sky-900">
            <a href={format!("/explore{}", href)} class="hover:underline">
                {content}
            </a>
        </li>
    }
}
