extern crate ncurses;

struct Screen
{
    pixel: str
}

impl Drop for Screen
{
    fn drop(&mut self)
    {
        ncurses::endwin();
    }
}

impl Screen
{
    fn new() -> i32
    {
        ncurses::initscr();
    }
}