mod app;
mod backup_io;
mod components;
mod csv_io;
mod export_io;
mod formatting;
mod import_io;
mod model;
mod storage;
mod xlsx_export;
mod xlsx_formatting;
#[cfg(test)]
mod xlsx_formatting_tests;
mod xlsx_io;
mod xlsx_text;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
