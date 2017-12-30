use std::thread;
use std::time;
use std::time::Instant;

struct Sim {
    dx: f32,
    c: f32,
    fr: f32,
    data: Vec<f32>,
    data_last: Vec<f32>,
    data_next: Vec<f32>,
}

impl Sim {
    fn new(dx: f32, c: f32, fr: f32, size: usize) -> Sim {
        let mut data = Vec::new();
        let mut data_last = Vec::new();
        let mut data_next = Vec::new();

        for _ in 0..size {
            data.push(0.0);
            data_last.push(0.0);
            data_next.push(0.0);
        }

        Sim {
            dx,
            c,
            fr,
            data,
            data_last,
            data_next,
        }
    }

    fn add_step(&mut self, start: usize, end: usize, y: f32) {
        for i in start..end {
            self.data[i] += y;
        }
    }

    fn simulate_elem(&mut self, dt: &f32, i: usize) {
        // (tp + tm - 2*t) / dt*dt = c*c*(xp + xm - 2*x)/dx*dx
        // tp = c*c*(xp + xm - 2*x)*dt*dt/dx*dx + 2*t - tm

        let t = self.data[i];
        let tm = self.data_last[i];
        let x = self.data[i];
        let xp = if i < self.data.len() - 1 {
            self.data[i + 1]
        } else {
            self.data[i]
        };
        let xm = if i > 0 {
            self.data[i - 1]
        } else {
            self.data[i]
        };

        let tp =
            self.c * self.c * (xp + xm - 2.0 * x) * dt * dt / (self.dx * self.dx) + 2.0 * t - tm;

        self.data_next[i] = self.fr * tp;
    }

    fn simulate(&mut self, dt: &f32, _t: f32) {
        // simulate
        for i in 0..self.data.len() {
            self.simulate_elem(dt, i);
        }

        //self.data[1] += f32::sin(_t * 8.);

        // cycle data
        for i in 0..self.data.len() {
            self.data_last[i] = self.data[i];
            self.data[i] = self.data_next[i];
        }
    }

    fn _render(&self) {
        for i in 0..self.data.len() {
            let abs = f32::abs(self.data[i]);

            if abs < 0.1 {
                print!("-");
            } else if abs < 0.7 {
                print!("~");
            } else if self.data[i] < 0.0 {
                print!(".");
            } else {
                print!("^");
            }
        }
        print!("\n");
    }
}

fn _simulate_and_render() {
    const DT: f32 = 1.0 / 60.0;
    const DX: f32 = 0.25;
    const C: f32 = 4.0;
    const FR: f32 = 0.9999;
    const N: usize = 80;

    let frame_delay = time::Duration::from_millis((DT * 1000.0) as u64);

    let mut sim = Sim::new(DX, C, FR, N);
    sim.add_step(0, 1, 0.2);

    let mut t = 0.;

    for _i in 0..1000 {
        sim.simulate(&DT, t);
        sim._render();

        thread::sleep(frame_delay);
        t += DT;

        if f32::abs(sim.data[sim.data.len() - 1]) > 0.05 {
            println!("Done!");
            println!("Time taken: {}", t);
            let dist = DX * (N as f32);
            println!("Distance: {}", dist);
            println!("Average vel: {}", dist / t);
            break;
        }
    }
}


fn do_measurement(dt: &f32, fr: f32, dx: f32, c: f32) {
    const N: usize = 80;

    let mut sim = Sim::new(dx, c, fr, N);
    sim.add_step(0, 1, 0.2);

    let mut t = 0.;

    const MAX_STEPS: usize = 10_000;

    for _i in 0..MAX_STEPS {
        sim.simulate(dt, t);

        t += dt;

        if f32::abs(sim.data[sim.data.len() - 1]) > 0.05 {
            let dist = dx * (N as f32);
            println!("{}\t{}\t{}\t{}\t{}", c, dt, dx, t, dist / t);
            break;
        }
    }
}

fn run_experiments() {
    let now = Instant::now();

    println!("C\tdt\tdx\tt\tave_speed");

    let cs = [0.5, 1.0, 2.0, 4.0, 8.0];
    let dts = [
        1. / 10.,
        1. / 20.,
        1. / 30.,
        1. / 50.,
        1. / 60.,
        1. / 90.,
        1. / 120.,
    ];
    let frs = [0.995, 0.997, 0.999, 0.9999];
    let dxs = [0.05, 0.1, 0.25, 0.5, 1.0, 2.0];

    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();

    for c in cs.iter() {
        for dt in dts.iter() {
            for fr in frs.iter() {
                for dx in dxs.iter() {
                    // do_measurement(dt, *fr, *dx, *c);

                    handles.push(thread::spawn(move || {
                        do_measurement(dt, *fr, *dx, *c);
                    }));
                }
            }
        }
    }

    for h in handles {
        h.join().map_err(|_| "Could not join a thread!").unwrap();
    }
    
    println!("Time elapsed: {}s", now.elapsed().as_secs());
}
