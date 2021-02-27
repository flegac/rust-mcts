pub mod screen;
pub mod dimension;
pub mod drawer;


#[cfg(test)]
mod tests {
    use screen::drawer::Drawer;
    use screen::screen::Screen;
    use screen::dimension::{Cursor, ScreenIndex};

    #[test]
    fn test_screen() {
        let mut scr = Screen::fill('.', 30, 10);
        let img = Screen::fill('x', 2, 5);


        scr.move_to(scr.index(2,4));
        scr.draw(&img);

        println!("{}", scr);
    }
}

