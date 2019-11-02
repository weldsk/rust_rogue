extern crate ncurses;
use ncurses::*;

pub struct Pos
{
    x: i32,
    y: i32,
}

pub struct ColorChar
{
    character: char,
    color: i16,
}

pub struct Screen (Vec<Vec<ColorChar>>);

impl Screen
{
    pub fn new()
    {
        /* 初期化 */
        initscr();
        raw();

        /* 拡張キーの許可 (like F1). */
        keypad(stdscr(), true);
        noecho();

        /* カーソルの非表示 */
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    pub fn get_size() -> Pos
    {
        Pos
        {
            x: getmaxx(stdscr()),
            y: getmaxy(stdscr()),
        }
    }
}

// デストラクタ
impl Drop for Screen
{
    fn drop(&mut self)
    {
        ncurses::endwin();
    }
}