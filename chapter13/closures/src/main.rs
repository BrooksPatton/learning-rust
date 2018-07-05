use std::thread;
use std::time::Duration;

struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl <T> Cacher<T> where T: Fn(u32) -> u32 {
    fn calculate(&mut self, intensity: u32) -> u32 {
        match self.value {
            Some(number) => number,
            None => {
                self.value = Some((self.calculation)(intensity));
                self.value.unwrap()
            }
        }
    }
}

fn main() {
    let simulated_intesity = 10;
    let simulated_random = 7;
    generated_workout(
        simulated_intesity,
        simulated_random
    );
}


fn generated_workout(intensity: u32, random: u32) {
    let mut long_calculation = Cacher {
        calculation: |intensity| {
            println!("Waiting for a really long time...");
            thread::sleep(Duration::from_secs(2));
            intensity
        },
        value: None
    };

    if intensity < 25 {
        println!("do {} pushups", long_calculation.calculate(intensity));
        println!("do {} situps", long_calculation.calculate(intensity));
    } else {
        if random == 3 {
            println!("Take a break today");
        } else {
            println!("Go run for {} minutes", long_calculation.calculate(intensity));
        }
    }
}