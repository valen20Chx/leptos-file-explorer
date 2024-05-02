use leptos::*;
use leptos_router::A;
use serde::{Deserialize, Serialize};
use std::{env, fmt, fs, path::Path};

// TODO: Should be configurable
const ROOT_PATH: &str = "./";

enum ValidPathError {
    OutOfScope(String),
    Canonicalize(String),
    RootPath,
}

impl ValidPathError {
    fn as_str(&self) -> String {
        match self {
            ValidPathError::OutOfScope(path) => format!("Path out of scope : '{}'", path),
            ValidPathError::Canonicalize(path) => format!("Canonicalization failed : '{}'", path),
            ValidPathError::RootPath => format!("Root path invalid"),
        }
    }
}

#[derive(Clone)]
struct ValidDirPath(String);

#[derive(Clone)]
struct ValidFilePath(String);

impl ValidDirPath {
    fn new(path: &Path) -> Self {
        // TODO : check if a dir. But might be redondent
        Self(path.to_string_lossy().to_string())
    }
}

impl ValidFilePath {
    fn new(path: &Path) -> Self {
        // TODO : check if a file. But might be redondent
        Self(path.to_string_lossy().to_string())
    }
}

enum ValidPathEnum {
    Dir(ValidDirPath),
    File(ValidFilePath)
}

impl ValidPathEnum {
    fn new(path: String) -> Result<Self, ValidPathError> {
        let path = ROOT_PATH.to_string() + &path;

        let path = fs::canonicalize(&path).map_err(|e| {
            ValidPathError::Canonicalize(format!("path=\"{}\", err=\"{}\"", path, e.to_string()))
        })?;
        let root_path = fs::canonicalize(ROOT_PATH).map_err(|_| ValidPathError::RootPath)?;

        if !path.starts_with(root_path) {
            Err(ValidPathError::OutOfScope(
                path.to_string_lossy().to_string(),
            ))
        } else {
            if path.is_dir() {
                Ok(ValidPathEnum::Dir(ValidDirPath::new(&path)))
            } else {
                Ok(ValidPathEnum::File(ValidFilePath::new(&path)))
            }
        }
    }

    fn as_str(&self) -> &str {
        self.as_str()
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

async fn get_dir_content(path: ValidDirPath) -> Result<Vec<FsEntry>, ServerFnError> {
    let mut entries = fs::read_dir(path.0.as_str())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .map(|res| {
            let name = res
                .as_ref()
                .map(|e| e.file_name().into_string())
                .unwrap_or(Ok(String::from("Invalid format")));

            let current_path = env::current_dir().unwrap().to_string_lossy().to_string();

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
pub fn ExploreView(path: String) -> impl IntoView {
    let path = ValidPathEnum::new(path.clone());

    match path {
        Ok(path) => {
            match path {
                ValidPathEnum::Dir(path) => {
                    view! {
                        <div>
                            <ListView path=path/>
                        </div>
                    }
                },
                ValidPathEnum::File(_path) => {
                    view! {
                        <div>
                            <p>File route for  not yet implemented</p>
                        </div>
                    }
                }
            }
        },
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
pub fn ListView(path: ValidDirPath) -> impl IntoView {
    let entries = create_resource(
        || (),
        move |_| {
            let path_clone = path.clone();
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
            <A href={format!("/explore{}", href)} class="hover:underline">
                {content}
            </A>
        </li>
    }
}
