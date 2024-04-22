use web_sys;
use web_time::{Instant, SystemTime};
use yew::{function_component, html, Html, Properties, Callback};

use clack::Board;

#[function_component]
fn RenderStones() -> Html {
        let mut board = Board::new(800, 500, 24);
        board.place_stones();
        let stones = board.stones;

        stones.iter().map( | ((x, y), stone) | {
            let cx = *x + stone.radius / 2;
            let cy = *y + stone.radius / 2;
            let color = format!("rgb({},{},{})", stone.color.0.to_string(), stone.color.1.to_string(), stone.color.2.to_string());
            let onclick = Callback::from(move |_| {
                let now = web_time::SystemTime::now();
                let hostname = web_sys::window()
                    .and_then(|win| win.location().hostname().ok())
                    .unwrap_or_else(|| "Unknown".to_string());
                let info = format!("{:?}, {}, {}, {}", now, cx.clone(), cy.clone(), &hostname);
                web_sys::console::log_1(&info.into()); // Log to Browser Console first
                // Put element on Players Stack
                put_on_stack((cx, cy), now, &hostname)
            });

            html! {
                <circle { onclick } cx={ cx.to_string() } cy={ cy.to_string() } r={ stone.radius.to_string() } fill={ color } />
            }
        }).collect::<Html>()
    }

fn put_on_stack(coords: (u8,u8), time: SystemTime, hostname: String) -> () {
    // Decide whether a click was -> How can we be aware of the current dice?
    // a. on the right stone, given the current dice
    // b. before any other players click, if two players clicked on the same stone at about the
    // same time

}

#[function_component]
fn Scoreboard() -> Html {
    // Unimplemented
}





#[function_component]
fn Canvas() -> Html {
    html! { <svg width="800" height="500"> <RenderStones /> <Scoreboard /> </svg> }
}

fn main() {
    yew::Renderer::<Canvas>::new().render();
}

