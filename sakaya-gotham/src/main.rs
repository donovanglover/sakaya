use gotham::state::State;

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "testing")
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    let _ = gotham::start(addr, || Ok(say_hello));
}
