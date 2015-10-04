extern crate rand;
use rand::distributions::{IndependentSample, Range};

const NUM_CALLERS: u64 = 200;

struct Server {
    name: String,
    pub busy_until: u64,
    pub idle_time: u64,
    // For service times.
    rng: rand::ThreadRng,
    range: Range<u64>,
}
impl Server {
    fn new(name: String) -> Self {
        Server {
            name: name,
            busy_until: 0,
            idle_time: 0,
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
        self.busy_until = *clock + service_time;
        caller.server = Some(self.name.clone());
        caller.start_time = Some(*clock);
        caller.service_time = Some(service_time);
        caller.exit_time = Some(*clock + service_time);
    }
}


#[derive(Debug)]
struct Caller {
    interarrival: u64,
    server:       Option<String>,
    waiting_time: Option<u64>,
    arrival_time: Option<u64>,
    service_time: Option<u64>,
    start_time:   Option<u64>,
    exit_time:    Option<u64>,
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
            server: None,
            waiting_time: None,
            service_time: None,
            arrival_time: None,
            start_time: None,
            exit_time: None,
        })
    }
}

fn main() {
    let caller_queue = CallerQueue::new();
    let mut able = Server::new("Able".to_string());
    let mut baker = Server::new("Baker".to_string());
    let mut clock = 0;
    let callers = caller_queue.map(|mut caller| {
        // Set the arrival time.
        caller.arrival_time = Some(clock + caller.interarrival);
        clock = clock + caller.interarrival; // Advance the clock to next arrival time.

        // Choose a server.
        // If both idle, choose randomly.
        // If one idle, assign to it.
        // If both busy, choose soonest available.
        if (able.busy_until < clock) && (baker.busy_until < clock) {
            let mut rng = rand::thread_rng();
            let range = Range::new(0u64, 100);
            let rand_val = range.ind_sample(&mut rng);
            // No waiting.
            caller.waiting_time = Some(0);
            // There is an idle time!
            if rand_val <= 50 {
                able.idle_time = able.idle_time + (clock - able.busy_until);
                able.serve(&mut clock, &mut caller);
            } else {
                baker.idle_time = baker.idle_time + (clock - baker.busy_until);
                baker.serve(&mut clock, &mut caller);
            }
        } else if able.busy_until < clock {
            // No waiting.
            caller.waiting_time = Some(0);
            // There is an idle time for Able
            able.idle_time = able.idle_time + (clock - able.busy_until);
            able.serve(&mut clock, &mut caller);
        } else if baker.busy_until < clock {
            // No waiting.
            caller.waiting_time = Some(0);
            // There is an idle time for baker.
            baker.idle_time = baker.idle_time + (clock - baker.busy_until);
            baker.serve(&mut clock, &mut caller);
        } else {
            // Waiting time here!
            if able.busy_until < baker.busy_until {
                // Able can't serve until he's done!
                caller.waiting_time = Some(able.busy_until - clock);
                clock = able.busy_until;
                able.serve(&mut clock, &mut caller);
            } else {
                // Baker can't serve until he's done!
                caller.waiting_time = Some(baker.busy_until - clock);
                clock = baker.busy_until;
                baker.serve(&mut clock, &mut caller);
            }
        }

        // Set the exit time.
        caller.exit_time = Some(clock + caller.service_time.unwrap());

        caller
    }).take(NUM_CALLERS as usize).collect::<Vec<_>>();

    println!("Average Waiting Time: {}", callers.iter().fold(0.0f64, |mut acc, ref caller| {
        acc = acc + caller.waiting_time.unwrap() as f64;
        acc
    }) / NUM_CALLERS as f64);

    println!("Probability of Waiting: {}", callers.iter().fold(0.0f64, |mut acc, ref caller| {
        if caller.waiting_time.unwrap() > 0 {
            acc = acc + 1.0;
        }
        acc
    }) / NUM_CALLERS as f64);

    println!("Probability of Idle Server: {}", (able.idle_time + baker.idle_time) as f64 / clock as f64);

    println!("Average Service Time: {}",  callers.iter().fold(0.0f64, |mut acc, ref caller| {
        acc = acc + caller.service_time.unwrap() as f64;
        acc
    }) / NUM_CALLERS as f64);

    println!("Average Time Between Arrivals: {}",  callers.iter().fold(0.0f64, |mut acc, ref caller| {
        acc = acc + caller.interarrival as f64;
        acc
    }) / (NUM_CALLERS - 1) as f64);

    println!("Average Waiting time of those who wait: {}",  callers.iter().fold(0.0f64, |mut acc, ref caller| {
        acc = acc + caller.waiting_time.unwrap() as f64;
        acc
    }) / callers.iter().fold(0, |mut acc, ref caller| {
        if caller.waiting_time.unwrap() > 0 {
            acc = acc + 1
        }
        acc
    }) as f64);

    println!("Average Time Caller Spends in System: {}", callers.iter().fold(0.0f64, |mut acc, ref caller| {
        acc = acc + (caller.exit_time.unwrap() - caller.arrival_time.unwrap()) as f64;
        acc
    }) / NUM_CALLERS as f64);
}
