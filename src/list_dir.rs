use leptos::*;
use std::fs;

// TODO: Should be configurable
const ROOT_PATH: &str = "./";

#[derive(Clone)]
struct ValidPath(String);

enum ValidPathError {
    EmptyPath,
    InvalidPath(String),
    UnknownError,
}

impl ValidPath {
    fn new(path: String) -> Result<Self, ValidPathError> {
        if path.is_empty() {
            return Err(ValidPathError::EmptyPath);
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

async fn get_dir_content(path: ValidPath) -> Result<Vec<String>, ServerFnError> {
    let mut entries = fs::read_dir(path.as_str())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .map(|res| {
            res.map(|e| match e.file_name().into_string() {
                Ok(s) => s,
                Err(_) => String::from("Invalid format"),
            })
        })
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    entries.sort();

    Ok(entries)
}

#[component]
pub fn ListView(path: Option<String>) -> impl IntoView {
    let path = ValidPath::new(path.clone().unwrap_or("./".to_string()));

    match path {
        Ok(valid_path) => {
            let names = create_resource(
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
                                names.get()
                                    .map(|names|
                                        match names {
                                            Err(e) => view! {
                                                <div>
                                                    <p><span style="text-bold">{"Error: "}</span>{e.to_string()}</p>
                                                </div>
                                            },
                                            Ok(names) => view! {
                                                <div>
                                                    <ul class="f-full">
                                                        {names.iter().map(|name| view! {
                                                            <ListItem content=name.to_string()/>
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
        Err(_) => {
            view! {
                <div>
                    <p><span style="text-bold">{"Error: "}</span>InvalidPath</p>
                </div>
            }
        }
    }
}

#[component]
fn ListItem(content: String) -> impl IntoView {
    view! {
        <li class="w-full bg-sky-950 text-white p-2 border border-sky-900">{content}</li>
    }
}
