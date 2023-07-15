use std::backtrace::Backtrace;

pub fn install_asena_panic_hook() {
    std::panic::set_hook(Box::new(|error| {
        let message = error
            .message()
            .and_then(|message| message.as_str())
            .unwrap_or_default();

        match error.location() {
            Some(location) => {
                eprintln!("compiler panicked at '{}': '{}'", location.file(), message)
            }
            None => {
                eprintln!("compiler panicked in a unknown location: '{message}'")
            }
        }

        let backtrace = Backtrace::force_capture();
        eprintln!("{backtrace}");
        eprintln!("The compiler unexpectedly panicked. There's an issue.");
        eprintln!("  Please send a pull request, or an issue at the github repository.");
        eprintln!("  https://github.com/aripiprazole/asena");
        eprintln!();
    }))
}
