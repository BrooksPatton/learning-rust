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

    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
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

    closures_side_affects();
}


fn generated_workout(intensity: u32, random: u32) {
    let mut long_calculation = Cacher::new(|intensity| {
            println!("Waiting for a really long time...");
            thread::sleep(Duration::from_secs(2));
            intensity
        });

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
fn closures_side_affects() {
    let mut x = 42;
    let access_x = |num| x + num;
    let mut does_not_work = move |num: i32| x += num;

    does_not_work(3);
    does_not_work(9);

    println!("x after being moved {}", x);
    println!("x is now {}", access_x(4));
}