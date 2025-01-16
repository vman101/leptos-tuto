use leptos::html::*;
use leptos::view;
use leptos::prelude::{ *, };
use leptos::ev;

/// Simple Progress Bar Component
#[component]
fn ProgressBar(
	/// How much progress should be displayed
	#[prop(into)]
    progress: Signal<u32>,
    /// Maximum value of progressj
    #[prop(default = 100)]
    max: u32
) -> impl IntoView {
    view! {
        <progress
            value=progress
            max=max
        />
        <br/>
    }
}

#[allow(non_snake_case)]
#[component]
fn DynamicList(
	initial_length: usize
) -> impl IntoView {
	let mut next_counter_id = initial_length;
	let initial_counters = (0..initial_length)
	    .map(|id| (id, ArcRwSignal::new(id + 1)))
		.collect::<Vec<_>>();
	let (counters, set_counters) = signal(initial_counters);

	let add_counter = move || {
		let sig = ArcRwSignal::new(next_counter_id + 1);
		set_counters.update( move |counters| {
			counters.push((next_counter_id, sig));
		});
		next_counter_id += 1;
	};
}
/// A simple counter view.
// A component is really just a function call: it runs once to create the DOM and reactive system
pub fn counter(initial_value: i32, step: i32) -> impl IntoView {
    let (count, set_count) = signal(initial_value);
    div().child((
        button()
            // typed events found in leptos::ev
            // 1) prevent typos in event names
            // 2) allow for correct type inference in callbacks
            .on(ev::click, move |_| set_count.set(0))
            .child("Clear"),
        button()
            .on(ev::click, move |_| *set_count.write() -= step)
            .child("-1"),
        span().child(("Value: ", move || count.get(), "!")),
        button()
            .on(ev::click, move |_| *set_count.write() += step)
            .child("+1"),
    ))
}

#[allow(non_snake_case)]
#[component]
fn App() -> impl leptos::IntoView {
    let (count, set_count) = leptos::prelude::signal(0);

    div()
	    .style(("display", "flex"))
	    .style(("align-items", "center"))
	    .style(("justify-content", "center"))
	    .style(("height", "100vh"))
	    .style(("text-align", "cen)ter"))
	    .child("Hello, World!")
}

fn main() {
    // Better Stack Trace for wasm Errors
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App)
}
