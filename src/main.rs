use clack::Board;
use yew::{function_component, html, Html};

#[function_component]
fn RenderStones() -> Html {
        let mut board = Board::new(800, 500, 24);
        board.place_stones();
        let stones = board.stones;

        stones.iter().map(|((x, y), stone)| {
            let cx = *x + stone.radius / 2;
            let cy = *y + stone.radius / 2;
            let color = format!("rgb({},{},{})", stone.color.0.to_string(), stone.color.1.to_string(), stone.color.2.to_string());
            html! {
                <circle cx={ cx.to_string() } cy={ cy.to_string() } r={ stone.radius.to_string() } fill={ color } />
            }
        }).collect::<Html>()
    }


#[function_component]
fn Canvas() -> Html {
    html! { <svg width="800" height="500"> <RenderStones /> </svg> }
}

fn main() {
    yew::Renderer::<Canvas>::new().render();
}

