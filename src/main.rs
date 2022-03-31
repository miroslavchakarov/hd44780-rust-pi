//extern crate linux_embedded_hal;
//extern crate hd44780_hal;

use std::{thread, time};
use linux_embedded_hal::{Delay, Pin};
use linux_embedded_hal::sysfs_gpio::Direction;

use hd44780_hal::HD44780;

fn main() {

    screen_init();

    let rs = Pin::new(13);
    let en = Pin::new(19);

    let db4 = Pin::new(26);
    let db5 = Pin::new(16);
    let db6 = Pin::new(20);
    let db7 = Pin::new(21);

    rs.export().unwrap();
    en.export().unwrap();
    
    db4.export().unwrap();
    db5.export().unwrap();
    db6.export().unwrap();
    db7.export().unwrap();

    rs.set_direction(Direction::Low).unwrap();
    en.set_direction(Direction::Low).unwrap();
    
    db4.set_direction(Direction::Low).unwrap();
    db5.set_direction(Direction::Low).unwrap();
    db6.set_direction(Direction::Low).unwrap();
    db7.set_direction(Direction::Low).unwrap();

    //4-bit communication with display
    let mut lcd = HD44780::new_4bit(
        rs,
        en,
    
        db4,
        db5,
        db6,
        db7,
        Delay,
    );
    
    lcd.reset();
    lcd.clear();
    lcd.set_display_mode(true, false, false);
    lcd.write_str("Customer nr. 4");

    lcd.set_cursor_pos(40);
    lcd.write_str("Calculating...");
    lcd.set_cursor_pos(30);
    lcd.write_str("0 kg.");

    loop {
        // for i in 1..6 {
        //     lcd.set_cursor_pos(0);
        //     lcd.write_str("Customer nr. ");
        //     let s = format!("{}", i);
        //     lcd.write_str(&s);
        //     thread::sleep(time::Duration::from_millis(1000));
        // }

        //blinking dot
        for i in 0..4 {
            lcd.set_cursor_pos(40);
            lcd.write_str("Calculating     ");
            thread::sleep(time::Duration::from_millis(200));
            lcd.set_cursor_pos(40);
            lcd.write_str("Calculating.   ");
            thread::sleep(time::Duration::from_millis(200));
            lcd.set_cursor_pos(40);
            lcd.write_str("Calculating..   ");
            thread::sleep(time::Duration::from_millis(200));
            lcd.set_cursor_pos(40);
            lcd.write_str("Calculating...  ");
            thread::sleep(time::Duration::from_millis(200));
        }
    }
}

fn screen_init(){
    
}
