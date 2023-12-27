use leptos::*;
use std::fs;

// #[server(List)]
pub async fn get_dir_content() -> Result<Vec<String>, ServerFnError> {
    let dir = ".";

    let mut entries = fs::read_dir(dir)
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

/// List
#[component]
pub fn ListView() -> impl IntoView {
    let names = create_resource(|| (), |_| async move { get_dir_content().await });

    view! {
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
    }
}

/// List item
#[component]
fn ListItem(content: String) -> impl IntoView {
    view! {
        <li class="w-full bg-sky-950 text-white p-2 border border-sky-900">{content}</li>
    }
}
