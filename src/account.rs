use std::ops::{Add, Sub};

#[derive(Debug)]
struct Account {
    money: u32,
}
impl Account {
    fn add(&mut self, money: u32) {
        self.money = self.money.add(money)
    }
    fn subtract(&mut self, money: u32) {
        self.money = self.money.sub(money)
    }
}

macro_rules! exchange {
    (Give $amount:literal to $name:ident) => {
        $name.add($amount)
    };
    (Take $amount:literal from $name:ident) => {
        $name.subtract($amount)
    };
    (Give $amount:literal from $giver:ident to $receiver:ident) => {
        $giver.subtract($amount);
        $receiver.add($amount)
    };
}

pub fn transact() {
    let mut the_poor = Account { money: 0 };
    let mut the_rich = Account { money: 200 };
    exchange!(Give 20 to the_poor);
    exchange!(Take 10 from the_rich);
    exchange!(Give 30 from the_rich to the_poor);
    println!("Poor: {:?}, rich: {:?}", the_poor, the_rich);
}
