use dioxus::prelude::*;
use time_tracking_dioxus::hooks_composed::{use_persistent, UsePersistent};
use time_tracking_parser::{parse_time_tracking_data, Time};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let time_entry = use_persistent("time_entry", String::new);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex",
            TimeEntryArea {
                time_entry,
            }
            TimeDisplay {
                time_entry,
            }
        }

    }
}

#[component]
pub fn TimeEntryArea(time_entry: UsePersistent<String>) -> Element {
    rsx! {
        div {
            id: "time-entry-area",
            textarea {
                id: "time-entry-input",
                value: "{time_entry.get()}",
                oninput: move |e| time_entry.set(e.value()),
                placeholder: "Enter your time entry here...",
                rows: "4",
                cols: "50"
            }
            // Additional content can be added here
        }
    }
}

#[component]
pub fn TimeDisplay(time_entry: UsePersistent<String>) -> Element {
    let data = use_memo(move || parse_time_tracking_data(&time_entry.get()));

    let start_time = use_memo(move || data.read().formatted_start_time());
    let end_time = use_memo(move || data.read().formatted_end_time());
    let total_decimal = use_memo(move || data.read().formatted_total_decimal());
    let total = use_memo(move || data.read().formatted_total_minutes());
    let dead = use_memo(move || data.read().formatted_dead_time_minutes());
    let dead_decimal = use_memo(move || data.read().formatted_dead_decimal());
    let projects = use_memo(move || data.read().projects.clone());

    rsx! {
        div {
            class: "flex flex-col p-4",
            p {
                "Start Time: {start_time} End Time: {end_time}"
            }
            p {
                "Total Working Time: {total} ({total_decimal} hours)"
            }
            p {
                "Total Dead Time: {dead} ({dead_decimal} hours)"
            }
            for project in projects.iter() {
                h3 {
                    class: "text-xl",
                    "Project: {project.name} Total Time: {Time::format_duration_minutes(project.total_minutes)} ({Time::format_duration_decimal(project.total_minutes)} hours)"
                }
                for note in &project.notes {
                    p {
                        class: "ml-4",
                        "- {note}"
                    }
                }
            }
        }
    }
}

// #[component]
// pub fn Hero() -> Element {
//     rsx! {
//         div {
//             id: "hero",
//             img { src: HEADER_SVG, id: "header" }
//             div { id: "links",
//                 a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
//                 a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
//                 a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
//                 a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
//                 a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
//                 a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
//             }
//         }
//     }
// }
