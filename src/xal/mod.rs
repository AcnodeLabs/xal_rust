use fltk::{app::*, button::*, frame::*, window::*};

pub fn win() {
    let app = App::default().with_scheme(AppScheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(Box::new(move || frame.set_label("Hello World!")));
    app.run().unwrap();
}

pub fn pr(message: &str) {
    println!("{}", message);
}

pub fn inp(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_inp() {
        let abcd = inp("please input exactly ABCD");
        assert_eq!(abcd,"ABCD");
    }

    #[test] 
    fn test_pr() {
        pr("OK");
    }

    #[test] 
    fn test_win() {
        win();
    }

}


