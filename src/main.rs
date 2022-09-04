use dioxus::prelude::*;
use dioxus_web::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut metadata = use_state(&cx, || 0);
    let mut timeupdate = use_state(&cx, || 0);
    let mut ended = use_state(&cx, || 0);
    let mut durationchange = use_state(&cx, || 0);

    cx.render(rsx!(
        h1 { "Audio Event Bugs" }
        table {
            thead {
                tr {
                    th { "Event" }
                    th { "Count" }
                }
            }
            tbody {
                tr {
                    td {
                        "onloadedmetadata"
                    }
                    td { "{metadata}" }
                }
                tr {
                    td {
                        "ontimeupdate"
                    }
                    td { "{timeupdate}" }
                }
                tr {
                    td {
                        "onended"
                    }
                    td { "{ended}" }
                }
                tr {
                    td {
                        "ondurationchange"
                    }
                    td { "{durationchange}" }
                }
            }
        }
        audio {
            src: "/static/sample.mp3",
            preload: "metadata",
            controls: "true",
            onloadedmetadata: move |_| metadata += 1,
            ontimeupdate: move |_| timeupdate += 1,
            onended: move |_| ended += 1,
            ondurationchange: move |_| durationchange += 1,
        }
    ))
}

