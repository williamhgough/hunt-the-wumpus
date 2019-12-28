use crate::util::*;

pub struct Game {
    pub arrows: u8,
    pub current_room: u8,
    pub messages: Vec<String>,
    pub wumpus: u8,
    bats: [u8; 2],
    pits: [u8; 2],
}

impl Game {
    fn configure_cave(&mut self) {
        self.messages.push(
            "You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold."
                .to_string(),
        );
        self.wumpus = js_rand(1, 20);
        self.bats[0] = self.get_empty_room();
        self.bats[1] = self.get_empty_room();
        self.pits[0] = self.get_empty_room();
        self.pits[1] = self.get_empty_room();
        self.warning_messages();
    }

    fn get_empty_room(&self) -> u8 {
        gen_range_avoiding(
            0,
            20,
            vec![
                self.current_room,
                self.wumpus,
                self.bats[0],
                self.bats[1],
                self.pits[0],
                self.pits[1],
            ],
        )
    }

    pub fn warning_messages(&mut self) {
        for adj in &room_exits(self.current_room).unwrap() {
            let t = *adj;
            if self.wumpus == t {
                self.messages
                    .push("You smell something horrific and rancid.".into());
            } else if self.pits.contains(&t) {
                self.messages
                    .push("You feel a cold updraft from a nearby cavern.".into());
            } else if self.bats.contains(&t) {
                self.messages
                    .push("You hear a faint but distinct flapping of wings.".into());
            }
        }
    }

    pub fn move_effects(&mut self) -> Option<String> {
        self.warning_messages();
        if self.current_room == self.wumpus {
            Some("You have been eaten slowly and painfully by the wumpus".into())
        } else if self.pits.contains(&self.current_room) {
            Some(
                "You have fallen into a bottomless pit and must now wait to die, falling all the while"
                    .into(),
            )
        } else if self.bats.contains(&self.current_room) {
            // Switch us to a random room
            let current = self.current_room;
            let next = self.get_empty_room();
            self.messages.push(format!(
                "A gigantic bat whisks you from room {} to room {} before you can even blink",
                current, next
            ));
            self.current_room = next;
            self.warning_messages();
            None
        } else {
            None
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut ret = Self {
            arrows: 5,
            current_room: 1,
            messages: Vec::new(),
            wumpus: 0,
            bats: [0, 0],
            pits: [0, 0],
        };
        ret.configure_cave();
        ret
    }
}

pub fn room_exits(id: u8) -> Option<[u8; 3]> {
    match id {
        1 => Some([2, 5, 8]),
        2 => Some([1, 3, 10]),
        3 => Some([2, 4, 12]),
        4 => Some([3, 5, 14]),
        5 => Some([1, 4, 6]),
        6 => Some([5, 7, 15]),
        7 => Some([6, 8, 17]),
        8 => Some([1, 7, 11]),
        9 => Some([10, 12, 19]),
        10 => Some([2, 9, 11]),
        11 => Some([8, 10, 20]),
        12 => Some([3, 9, 13]),
        13 => Some([12, 14, 18]),
        14 => Some([4, 13, 15]),
        15 => Some([6, 14, 16]),
        16 => Some([15, 17, 18]),
        17 => Some([7, 16, 20]),
        18 => Some([13, 16, 19]),
        19 => Some([9, 18, 20]),
        20 => Some([11, 17, 19]),
        _ => None,
    }
}
