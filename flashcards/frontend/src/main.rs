mod app;
mod components;
mod csv_io;
mod model;
mod storage;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
