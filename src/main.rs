use dpbook::presentation::cli::PhonebookApp;

fn main() {
    if let Err(e) = PhonebookApp::run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
