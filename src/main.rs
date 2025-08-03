use dioxus::prelude::*;
use dioxus_clipboard::prelude::use_clipboard;
use dioxus::logger::tracing::*;
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
            class: "min-h-screen bg-gray-50",
            div {
                class: "w-full max-w-7xl mx-auto px-4 py-8",
                div {
                    class: "flex flex-col md:flex-row gap-6 w-full",
                    TimeEntryArea {
                        time_entry,
                    }
                    TimeDisplay {
                        time_entry,
                    }
                }
            }
        }

    }
}

#[component]
pub fn TimeEntryArea(time_entry: UsePersistent<String>) -> Element {
    rsx! {
        div {
            class: "w-full md:w-1/2 bg-white rounded-lg shadow-sm border border-gray-200",
            div {
                class: "p-6",
                h2 {
                    class: "text-xl font-semibold text-gray-800 mb-4",
                    "Time Entry"
                }
                textarea {
                    id: "time-entry-input",
                    class: "w-full h-64 p-3 border border-gray-300 rounded-md resize-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors placeholder-gray-500 text-sm font-mono",
                    value: "{time_entry.get()}",
                    oninput: move |e| time_entry.set(e.value()),
                    placeholder: "Enter your time tracking data here...\n\nExample:\n9:00 AM - 5:00 PM\nProject Alpha: Working on feature X\nProject Beta: Bug fixes\n\n2:00 PM - 2:15 PM (break)",
                }
            }
        }
    }
}

#[component]
pub fn TimeDisplay(time_entry: UsePersistent<String>) -> Element {
    let data = use_memo(move || parse_time_tracking_data(&time_entry.get()));
    let mut clipboard = use_clipboard();

    let start_time = use_memo(move || data.read().formatted_start_time());
    let end_time = use_memo(move || data.read().formatted_end_time());
    let total_decimal = use_memo(move || data.read().formatted_total_decimal());
    let total = use_memo(move || data.read().formatted_total_minutes());
    let dead = use_memo(move || data.read().formatted_dead_time_minutes());
    let dead_decimal = use_memo(move || data.read().formatted_dead_decimal());
    let projects = use_memo(move || data.read().projects.clone());

    rsx! {
        div {
            class: "w-full md:w-1/2 bg-white rounded-lg shadow-sm border border-gray-200",
            div {
                class: "p-6",
                h2 {
                    class: "text-xl font-semibold text-gray-800 mb-6",
                    "Time Summary"
                }
                
                // Time Overview Section
                div {
                    class: "bg-blue-50 rounded-lg p-4 mb-6",
                    div {
                        class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                        div {
                            class: "text-center",
                            p {
                                class: "text-sm text-gray-600 font-medium",
                                "Start Time"
                            }
                            p {
                                class: "text-lg font-semibold text-blue-700",
                                "{start_time}"
                            }
                        }
                        div {
                            class: "text-center",
                            p {
                                class: "text-sm text-gray-600 font-medium",
                                "End Time"
                            }
                            p {
                                class: "text-lg font-semibold text-blue-700",
                                "{end_time}"
                            }
                        }
                    }
                }

                // Working Time Section
                div {
                    class: "border-l-4 border-green-400 bg-green-50 p-4 mb-4",
                    h3 {
                        class: "text-sm font-medium text-green-800 mb-1",
                        "Total Working Time"
                    }
                    p {
                        class: "text-lg font-semibold text-green-700",
                        "{total} ({total_decimal} hours)"
                    }
                }

                // Dead Time Section
                if !dead.read().is_empty() && *dead.read() != "0 minutes" {
                    div {
                        class: "border-l-4 border-red-400 bg-red-50 p-4 mb-6",
                        h3 {
                            class: "text-sm font-medium text-red-800 mb-1",
                            "Total Dead Time"
                        }
                        p {
                            class: "text-lg font-semibold text-red-700",
                            "{dead} ({dead_decimal} hours)"
                        }
                    }
                }

                // Projects Section
                if !projects.is_empty() {
                    div {
                        h3 {
                            class: "text-lg font-semibold text-gray-800 mb-4 border-b border-gray-200 pb-2",
                            "Projects"
                        }
                        div {
                            class: "space-y-4",
                            for project in projects.iter() {
                                div {
                                    class: "bg-gray-50 rounded-lg p-4 border border-gray-200 cursor-pointer hover:bg-gray-100 transition-colors",
                                    onclick: {
                                        let project_notes = project.notes.clone();
                                        move |_| {
                                            let notes_text = project_notes.iter()
                                                .map(|note| format!("- {note}"))
                                                .collect::<Vec<_>>()
                                                .join("\n");
                                            
                                            match clipboard.set(notes_text.clone()) {
                                                Ok(_) => {
                                                    info!("Successfully copied to clipboard: {notes_text}");
                                                }
                                                Err(e) => {
                                                    warn!("Failed to copy to clipboard: {e:?}");
                                                }
                                            }
                                        }
                                    },
                                    div {
                                        class: "flex flex-col sm:flex-row sm:items-center sm:justify-between mb-3",
                                        h4 {
                                            class: "text-base font-semibold text-gray-800",
                                            "{project.name}"
                                        }
                                        span {
                                            class: "text-sm font-medium text-blue-600 bg-blue-100 px-2 py-1 rounded-full mt-1 sm:mt-0",
                                            "{Time::format_duration_minutes(project.total_minutes)} ({Time::format_duration_decimal(project.total_minutes)} hrs)"
                                        }
                                    }
                                    if !project.notes.is_empty() {
                                        div {
                                            class: "space-y-1",
                                            for note in &project.notes {
                                                p {
                                                    class: "text-sm text-gray-600 flex items-start",
                                                    span {
                                                        class: "text-gray-400 mr-2 mt-0.5 text-xs",
                                                        "-"
                                                    }
                                                    span { "{note}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "text-center py-8 text-gray-500",
                        p {
                            class: "text-sm",
                            "No projects found. Enter your time tracking data to see the breakdown."
                        }
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
