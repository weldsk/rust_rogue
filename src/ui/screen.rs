use ncurses::*;

struct Cursor{
    x: i32,
    y: i32,
}

struct ColorChar {
    character: char,
    fg_color: u8,
    bg_color: u8,
}

pub struct Screen (Vec<Vec<ColorChar>>);

impl Screen {
    pub fn new() {
        /* 初期化 */
        initscr();
        raw();

        /* 拡張キーの許可 (like F1). */
        keypad(stdscr(), true);
        noecho();

        /* カーソルの非表示 */
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    fn get_size() -> Cursor{
        Cursor{
            x: getmaxx(stdscr()),
            y: getmaxy(stdscr()),
        }
    }
}

// デストラクタ
impl Drop for Screen {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}