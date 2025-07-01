struct Guess {
    weapon: String,
    room: String,
    person: String,
}
fn main() {
    let answer = Guess {
        weapon: String::from("knife"),
        room: String::from("kitchen"),
        person: String::from("Scarlet"),
    };
    println!("{}", answer.weapon);
    println!("{}", answer.room);
    println!("{}", answer.person);
}
