use gamepads::{Button, Gamepads};
use rusqlite::{Connection};

fn main() {
    let mut gamepads = Gamepads::new();
    let dbconn: Connection;
    let conn = Connection::open("mcrtfa.db3");
    let mut pocitadlo = 0;
    loop {
        gamepads.poll();
        for gamepad in gamepads.all() {
            // Use just_pressed_buttons() or currently_pressed_buttons().
            for button in gamepad.all_just_pressed() {
                // Individual buttons can be checked using
                // is_just_pressed() / is_currently_pressed():
                if gamepad.is_currently_pressed(Button::ActionDown) {
                    println!("Stisknuto tlacitko");
                    //conn.execute("UPDATE button_press SET press_counter = ?1", (pocitadlo,))?;
                    pocitadlo += 1;
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}