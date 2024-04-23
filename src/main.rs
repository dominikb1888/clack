use web_sys;
use web_time::{Instant, SystemTime};
use yew::{function_component, html, Html, Properties, Callback};
use yew::use_state;

use clack::Board;


#[function_component]
fn RenderStones(props: &Props) -> Html {
    let mut board = Board::new(800, 500, 24);
    board.place_stones();
    let stones = board.stones.clone(); // Clone stones to avoid move into closure
    let remove_stone_callback = props.remove_stone_callback.clone();

    stones.iter().map(|((x, y), stone)| {
        let cx = *x + stone.radius / 2;
        let cy = *y + stone.radius / 2;
        let color = format!("rgb({},{},{})", stone.color.0, stone.color.1, stone.color.2);
        let onclick = {
            let remove_stone_callback = remove_stone_callback.clone();
            Callback::from(move |_| {
                let now = SystemTime::now();
                let hostname = web_sys::window()
                    .and_then(|win| win.location().hostname().ok())
                    .unwrap_or_else(|| "Unknown".to_string());
                let info = format!("{:?}, {}, {}, {}", now, cx, cy, &hostname);
                web_sys::console::log_1(&info.into()); // Log to Browser Console first
                // Remove stone from the board
                remove_stone_callback.emit((cx, cy)); // Emit the stone coordinates to the parent component
            })
        };

        html! {
            <circle { onclick } cx={ cx.to_string() } cy={ cy.to_string() } r={ stone.radius.to_string() } fill={ color } />
        }
    }).collect::<Html>()
}

#[derive(Properties, Clone, PartialEq)]
struct Props {
    remove_stone_callback: Callback<(u32, u32)>, // Callback to remove stone from the board
}

#[function_component]
fn Canvas() -> Html {
    let (stones, set_stones) = use_state(Vec::new);

    let remove_stone_callback = {
        let set_stones = set_stones.clone();
        Callback::from(move |(x, y)| {
            // Update the board state to remove the stone
            let updated_stones = stones.iter().filter(|((stone_x, stone_y), _)| *stone_x != x || *stone_y != y).cloned().collect();
            set_stones.emit(updated_stones);
        })
    };

    html! {
        <svg width="800" height="500">
            <RenderStones remove_stone_callback= { remove_stone_callback }  />
        </svg>
    }
}

fn main() {
    yew::Renderer::<Canvas>::new().render();
}

