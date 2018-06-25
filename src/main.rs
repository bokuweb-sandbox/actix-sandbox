extern crate actix;
extern crate futures;

use actix::prelude::*;
use futures::Future;

use std::{thread, time};

// `PlusOne` message implementation
struct PlusOne;

impl Message for PlusOne {
    type Result = u32;
}

// `CounterActor` implementation
struct CounterActor {
    count: u32,
}

impl CounterActor {
    pub fn new() -> CounterActor {
        CounterActor { count: 0 }
    }
}

impl Actor for CounterActor {
    type Context = Context<Self>;
}

impl Handler<PlusOne> for CounterActor {
    type Result = u32;

    fn handle(&mut self, _msg: PlusOne, _ctx: &mut Context<Self>) -> u32 {
        self.count += 1;
        let d = time::Duration::from_millis(1000);
        thread::sleep(d);
        self.count
    }
}

fn main() {
    let sys = actix::System::new("test");

    let counter: Addr<Syn, _> = Arbiter::start(|_| CounterActor::new());
    // let counter_addr_copy = counter.clone();

    let result = counter
        .send(PlusOne)
        .and_then(move |count| {
            println!("Count: {}", count);
            // counter_addr_copy.send(PlusOne)
            counter.send(PlusOne)
        })
        .map(|count| {
            println!("Count: {}", count);
            Arbiter::system().do_send(actix::msgs::SystemExit(0));
        })
        .map_err(|error| {
            println!("An error occured: {}", error);
        });

    Arbiter::handle().spawn(result);

    sys.run();
    println!("system exit");
}
