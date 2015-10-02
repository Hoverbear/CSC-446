extern crate rand;
use rand::distributions::{IndependentSample, Range};

struct Server {
    name: String,
    busy_until: u64,
    // For service times.
    rng: rand::ThreadRng,
    range: Range<u64>,
}
impl Server {
    fn new(name: String) -> Self {
        Server {
            name: name,
            busy_until: 0,
            rng: rand::thread_rng(),
            range: Range::new(0u64, 100),
        }
    }
    fn serve(&mut self, clock: &mut u64, caller: &mut Caller) {
        let rand_val = self.range.ind_sample(&mut self.rng);
        let service_time = if self.name == "Able" {
            if rand_val <= 30 {
                2
            } else if rand_val <= 58 {
                3
            } else if rand_val <= 83 {
                4
            } else {
                5
            }
        } else { //Baker
            if rand_val <= 35 {
                3
            } else if rand_val <= 60 {
                4
            } else if rand_val <= 80 {
                5
            } else {
                6
            }
        };
        self.busy_until = self.busy_until + service_time;
        caller.server = Some(self.name.clone());
        caller.service_time = Some(service_time);
    }
}


#[derive(Debug)]
struct Caller {
    interarrival: u64,
    arrival: Option<u64>,
    server: Option<String>,
    service_time: Option<u64>,
}

struct CallerQueue {
    rng: rand::ThreadRng,
    range: Range<u64>,
}
impl CallerQueue {
    fn new() -> Self {
        CallerQueue {
            rng: rand::thread_rng(),
            range: Range::new(0u64, 100),
        }
    }
}
impl Iterator for CallerQueue {
    type Item = Caller;
    fn next(&mut self) -> Option<Caller> {
        // Get interarrival time.
        let interarrival = {
            let val = self.range.ind_sample(&mut self.rng);
            if val <= 25 {
                1
            } else if val <= 65 {
                2
            } else if val <= 85 {
                3
            } else {
                4
            }
        };
        Some(Caller {
            interarrival: interarrival, 
            arrival: None,
            server: None,
            service_time: None,
        })
    }
}

fn main() {
    let mut caller_queue = CallerQueue::new();
    let mut able = Server::new("Able".to_string());
    let mut baker = Server::new("Baker".to_string());
    let mut clock = 0;
    let mut callers = caller_queue.map(|mut caller| {
        // Set the arrival time.
        caller.arrival = Some(clock + caller.interarrival);
        clock = clock + caller.interarrival; // Advance the clock to next arrival time.
        // Choose a server.
        // If both idle, choose randomly.
        // If one idle, assign to it.
        // If both busy, choose soonest available.
        if able.busy_until < clock && baker.busy_until < clock {
            let mut rng = rand::thread_rng();
            let range = Range::new(0u64, 100);
            let rand_val = range.ind_sample(&mut rng);
            if rand_val <= 50 {
                able.serve(&mut clock, &mut caller);
            } else {
                baker.serve(&mut clock, &mut caller);
            }
        } else if able.busy_until < clock {
            able.serve(&mut clock, &mut caller);
        } else if baker.busy_until < clock {
            baker.serve(&mut clock, &mut caller);
        } else {
            if able.busy_until < baker.busy_until {
                able.serve(&mut clock, &mut caller);
            } else {
                baker.serve(&mut clock, &mut caller);
            }
        }
        caller
    }).take(3).collect::<Vec<_>>();

    println!("{:?}", callers);
}
