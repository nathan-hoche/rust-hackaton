enum Event {
    Input(),
    Quit,
    Pause,
    Resume,
}

enum InputEvent {
    Click(),
    KeyPress(),
}

fn main() {
    let event = Event::Quit;
    event.log_event();

    let event = Event::Input(InputEvent::Click((45, 540)));
    event.log_event();

    let event = Event::Input(InputEvent::KeyPress('a'));
    event.log_event();

    let event = Event::Pause;
    event.log_event();

    let event = Event::Resume;
    event.log_event();
}
