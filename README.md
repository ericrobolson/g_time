This is a library for game timers. 

Typical usage is you'll `tick()` the timer and depending if it was `Triggered` you'll either update state or do some interpolation.

```
use g_time::*;

let hz = 60;
let mut sim_timer = Timer::new(hz);

// Game loop
loop {
	match sim_timer.tick() {
		Triggered::Yes => {
			tick_simulation();
		},
		Triggered::No => {
			let delta_t = sim_timer.elapsed();
			interpolate(delta_t.as_secs_f32());
		}
	}
}
```