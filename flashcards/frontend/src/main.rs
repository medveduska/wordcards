mod app;
mod backup_io;
mod components;
mod csv_io;
mod export_io;
mod import_io;
mod model;
mod storage;
mod xlsx_export;
mod xlsx_io;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
