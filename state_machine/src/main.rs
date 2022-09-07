use std::io::{self, Write};

#[derive(Debug, Copy, Clone)]
enum State {
    LOCKED = 0,
    UNLOCKED = 1,
}

#[derive(Debug)]
enum Event {
    PUSH = 0,
    COIN = 1,
}

const FSM : [[State; 2]; 2] = [
    //Locked            Unlocked
    [State::LOCKED, State::LOCKED], //PUSH
    [State::UNLOCKED, State::UNLOCKED], //COIN
];

fn main() {
    let mut state = State::LOCKED;

    println!("State: {:#?}", state);
    print!("> ");
    io::stdout().flush().unwrap();  //unwrap means we are ignoring the error

    for line in io::stdin().lines() {
        match line.unwrap().as_str() {
            "coin" => state = next_state(state, Event::COIN),
            "push" => state = next_state(state, Event::PUSH),
            _ => println!("Invalid event")
        }
        println!("State: {:?}", state);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}

fn next_state(state: State, event: Event) -> State  {
    FSM[event as usize][state as usize]
}