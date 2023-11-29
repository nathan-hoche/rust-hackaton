enum Event {
    Input(InputEvent),
    Quit,
    Pause,
}

enum InputEvent {
    Click((i32, i32)),
    KeyPress(char),
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
}
