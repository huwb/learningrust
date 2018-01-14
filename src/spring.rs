pub struct Spring {
    x: f32,
    v: f32,
    x_target: f32,
    v_target: f32,
    undamped_angular_freq: f32,
    damping_ratio: f32,
}

impl Spring {
    const MAX_DT: f32 = 1.0 / 60.0;

    fn new() -> Spring {
        Spring {
            x: 0.0,
            v: 0.0,
            x_target: 0.0,
            v_target: 0.0,
            undamped_angular_freq: 1.0,
            damping_ratio: 1.0,
        }
    }

    fn update(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }

        let step_count_f = (dt / Spring::MAX_DT).ceil();
        let dt_substep = dt / step_count_f;
        let step_count = step_count_f as usize;

        for _ in 0..step_count {
            let a_spring =
                self.undamped_angular_freq * self.undamped_angular_freq * (self.x_target - self.x);
            let a_damp = 2.0 * self.damping_ratio * (self.v_target - self.v);

            self.x += self.v * dt_substep;
            self.v += (a_spring + a_damp) * dt_substep;
        }
    }
}

pub fn run() {
    let mut s = Spring::new();
    s.x_target = 1.0;
    s.v_target = 0.0;

    s.update(2.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(a: f32, b: f32) {
        let diff = (a - b).abs();
        assert!(
            diff < 0.001,
            "Floats not approximately equal: {} and {}",
            a,
            b
        );
    }

    #[test]
    fn default_values() {
        let s = Spring::new();
        assert_eq!(s.x, 0.0);
        assert_eq!(s.v, 0.0);
    }

    #[test]
    fn update_default_vals() {
        let mut s = Spring::new();
        s.x_target = 1.0;
        s.v_target = 0.0;

        s.update(1.5);

        assert_approx_eq(s.x, 0.4436);
    }

    #[test]
    fn update_stiff() {
        let mut s = Spring::new();
        s.x_target = 1.0;
        s.v_target = 0.0;

        s.undamped_angular_freq = 5.0;

        s.update(6.7);

        assert_approx_eq(s.x, 1.0);
    }

    #[test]
    fn nonpositive_timestep_doesnt_change() {
        let mut s = Spring::new();
        s.x_target = 1.0;
        s.v_target = 0.0;
        s.x = 0.5;
        s.v = 0.5;

        // update with dt == 0
        s.update(0.0);
        assert_eq!(s.x, 0.5);
        assert_eq!(s.v, 0.5);

        // update with dt < 0
        s.update(-0.1);
        assert_eq!(s.x, 0.5);
        assert_eq!(s.v, 0.5);
    }
}
