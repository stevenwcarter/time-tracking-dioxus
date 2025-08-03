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
    let mut show_help = use_signal(|| false);

    rsx! {
        div {
            class: "w-full md:w-1/2 bg-white rounded-lg shadow-sm border border-gray-200",
            div {
                class: "p-6",
                div {
                    class: "flex justify-between items-center mb-4",
                    h2 {
                        class: "text-xl font-semibold text-gray-800",
                        "Time Entry"
                    }
                    button {
                        class: "px-3 py-1 text-sm bg-red-500 text-white rounded hover:bg-red-600 transition-colors",
                        onclick: move |_| time_entry.set(String::new()),
                        "Clear"
                    }
                }
                textarea {
                    id: "time-entry-input",
                    class: "w-full h-64 p-3 border border-gray-300 rounded-md resize-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors placeholder-gray-500 text-sm font-mono",
                    value: "{time_entry.get()}",
                    oninput: move |e| time_entry.set(e.value()),
                    placeholder: "Enter your time tracking data here...\n\nExample:\n11:45-12:15 code1\n- Comment explaining what you did\n12:15-1:30 code2\n- Comment about what you were doing\n1:30-2 code1\n2-4 code3",
                }
                
                // Help Section
                div {
                    class: "mt-4",
                    button {
                        class: "flex items-center text-sm text-blue-600 hover:text-blue-800 transition-colors",
                        onclick: move |_| {
                            let current = *show_help.read();
                            show_help.set(!current);
                        },
                        span {
                            class: "mr-1",
                            if *show_help.read() { "▼" } else { "▶" }
                        }
                        "How to use this tool"
                    }
                    
                    if *show_help.read() {
                        div {
                            class: "mt-3 p-4 bg-blue-50 rounded-lg border border-blue-200",
                            p {
                                class: "text-sm text-gray-700 mb-3",
                                "You should enter your time in the format shown below. \"code1\" and \"code2\" can be anything you'd like, and the time will be aggregated together, even if you work on other time codes in the interim. You can try copying the data below into the text area to see a sample report. From the report, you can then note the time and copy the comments into the notes field in your time tracker."
                            }
                            pre {
                                class: "text-sm font-mono bg-gray-100 p-3 rounded border text-gray-800 whitespace-pre-wrap",
                                "11:45-12:15 code1\n- Comment explaining what you did\n12:15-1:30 code2\n- Comment about what you were doing\n1:30-2 code1\n2-4 code3"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TimeDisplay(time_entry: UsePersistent<String>) -> Element {
    let data = use_memo(move || parse_time_tracking_data(&time_entry.get()));
    let mut clipboard = use_clipboard();

    info!("Parsed time tracking data: {data:?}");

    let start_time = use_memo(move || data.read().formatted_start_time());
    let end_time = use_memo(move || data.read().formatted_end_time());
    let total_decimal = use_memo(move || data.read().formatted_total_decimal());
    let total = use_memo(move || data.read().formatted_total_minutes());
    let dead = use_memo(move || data.read().formatted_dead_time_minutes());
    let dead_decimal = use_memo(move || data.read().formatted_dead_decimal());
    let projects = use_memo(move || data.read().projects.clone());
    let warnings = use_memo(move || data.read().warnings.clone());

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
                {
                    let dead_minutes = data.read().dead_time_minutes;
                    
                    if dead_minutes == 0 {
                        // Green display for no dead time
                        rsx! {
                            div {
                                class: "border-l-4 border-green-400 bg-green-50 p-4 mb-6",
                                h3 {
                                    class: "text-sm font-medium text-green-800 mb-1",
                                    "Dead Time"
                                }
                                p {
                                    class: "text-lg font-semibold text-green-700",
                                    "No dead time (gaps) found"
                                }
                            }
                        }
                    } else if dead_minutes < 90 {
                        // Yellow display for dead time under 90 minutes
                        rsx! {
                            div {
                                class: "border-l-4 border-yellow-400 bg-yellow-50 p-4 mb-6",
                                h3 {
                                    class: "text-sm font-medium text-yellow-800 mb-1",
                                    "Total Dead Time"
                                }
                                p {
                                    class: "text-lg font-semibold text-yellow-700",
                                    "{dead} ({dead_decimal} hours)"
                                }
                            }
                        }
                    } else {
                        // Red display for dead time >= 90 minutes
                        rsx! {
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
                    }
                }

                // Warnings Section
                if !warnings.read().is_empty() {
                    div {
                        class: "border-l-4 border-yellow-400 bg-yellow-50 p-4 mb-6",
                        h3 {
                            class: "text-sm font-medium text-yellow-800 mb-2",
                            "Warnings"
                        }
                        div {
                            class: "space-y-1",
                            for warning in warnings.read().iter() {
                                p {
                                    class: "text-sm text-yellow-700 flex items-start",
                                    span {
                                        class: "text-yellow-500 mr-2 mt-0.5 text-xs",
                                        "⚠"
                                    }
                                    span { "{warning}" }
                                }
                            }
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
