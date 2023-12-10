use leptos::*;

struct List {
    names: Vec<String>,
}

#[server(List)]
pub async fn get_dir_content() -> Result<Vec<String>, ServerFnError> {
    Ok(vec!["John".to_string(), "Jane".to_string()])
}

/// List
#[component]
pub fn ListView() -> impl IntoView {
    // let names = get_dir_content();
    let names = vec!["John".to_string(), "Jane".to_string()];
    view! {
        <ul class="f-full">
            {names.iter().map(|name| view! {
                <ListItem content=name.to_string()/>
            }).collect::<Vec<_>>()}
        </ul>
    }
}

/// List item
#[component]
fn ListItem(content: String) -> impl IntoView {
    view! {
        <li class="w-full bg-sky-950 text-white p-2 border border-sky-900">{content}</li>
    }
}
