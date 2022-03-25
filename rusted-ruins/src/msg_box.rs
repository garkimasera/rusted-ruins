use sdl2::messagebox::*;

pub fn lang_selector() -> &'static str {
    let result = show_message_box(
        MessageBoxFlag::INFORMATION,
        &[
            ButtonData {
                flags: MessageBoxButtonFlag::NOTHING,
                button_id: 0,
                text: "English",
            },
            ButtonData {
                flags: MessageBoxButtonFlag::NOTHING,
                button_id: 1,
                text: "日本語",
            },
        ],
        "Select language",
        "Please select language\nEnglish translation is incomplete. Some text is displayed in Japanese.",
        None,
        None,
    ).unwrap();

    match result {
        ClickedButton::CustomButton(ButtonData { button_id, .. }) if *button_id == 1 => "ja",
        _ => "en",
    }
}

pub fn exit_with_error(title: impl std::fmt::Display, e: impl std::fmt::Display) -> ! {
    let title = title.to_string();
    let e = e.to_string();
    show_simple_message_box(MessageBoxFlag::ERROR, &title, &e, None).unwrap();
    std::process::exit(1)
}
