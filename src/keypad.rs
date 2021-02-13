use sfml::{window::{Key}};

pub fn is_keypad_pressed(key: u8) -> bool {
    if Key::Num1.is_pressed() {
        if key == 0x1 {
            return true;
        }
    }

    if Key::Num2.is_pressed() {
        if key == 0x2 {
            return true;
        }
    }

    if Key::Num3.is_pressed() {
        if key == 0x3 {
            return true;
        }
    }

    if Key::Num4.is_pressed() {
        if key == 0xC {
            return true;
        }
    }

    if Key::Q.is_pressed() {
        if key == 0x4 {
            return true;
        }
    }

    if Key::W.is_pressed() {
        if key == 0x5 {
            return true;
        }
    }

    if Key::E.is_pressed() {
        if key == 0x6 {
            return true;
        }
    }

    if Key::R.is_pressed() {
        if key == 0xD {
            return true;
        }
    }

    if Key::A.is_pressed() {
        if key == 0x7 {
            return true;
        }
    }

    if Key::S.is_pressed() {
        if key == 0x8 {
            return true;
        }
    }

    if Key::D.is_pressed() {
        if key == 0x9 {
            return true;
        }
    }

    if Key::F.is_pressed() {
        if key == 0xE {
            return true;
        }
    }

    if Key::Z.is_pressed() {
        if key == 0xA {
            return true;
        }
    }

    if Key::X.is_pressed() {
        if key == 0x0 {
            return true;
        }
    }

    if Key::C.is_pressed() {
        if key == 0xB {
            return true;
        }
    }

    if Key::V.is_pressed() {
        if key == 0xF {
            return true;
        }
    }

    return false;
}

pub fn get_keypad() -> u8 {
    if Key::Num1.is_pressed() {
        return 0x1;
    }

    if Key::Num2.is_pressed() {
        return 0x2;
    }

    if Key::Num3.is_pressed() {
        return 0x3;
    }

    if Key::Num4.is_pressed() {
        return 0xC;
    }

    if Key::Q.is_pressed() {
        return 0x4;
    }

    if Key::W.is_pressed() {
        return 0x5;
    }

    if Key::E.is_pressed() {
        return 0x6;
    }

    if Key::R.is_pressed() {
        return 0xD;
    }

    if Key::A.is_pressed() {
        return 0x7;
    }

    if Key::S.is_pressed() {
        return 0x8;
    }

    if Key::D.is_pressed() {
        return 0x9;
    }

    if Key::F.is_pressed() {
        return 0xE;
    }

    if Key::Z.is_pressed() {
        return 0xA;
    }

    if Key::X.is_pressed() {
        return 0x0;
    }

    if Key::C.is_pressed() {
        return 0xB;
    }

    if Key::V.is_pressed() {
        return 0xF;
    }
    return 0xff;
}