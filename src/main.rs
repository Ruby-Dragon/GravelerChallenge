use std::thread;
use std::thread::Thread;
use rand::Rng;
use rand::rngs::ThreadRng;
use futures::executor::ThreadPool;

fn main() {
    let total_samples = 1000000000;
    let thread_count = 25;
    let samples_per_thread = total_samples / thread_count;
    let rolls = 231;

    let mut samples_completed = 0;
    let mut highest_samples_per_thread : Vec<u8> = vec!(0; thread_count);

    let mut highest = 0;
    for _i in 0..5
    {
        let result = sample(rolls, highest);
        if result > 0
        {
            highest = result;
        }
    }

    println!("Hello, world! {}", highest);
}

fn roll_d4s(mut rng: ThreadRng, number : usize) -> u32
{
    let mut amount = 0;
    for _i in 0..number
    {
        let roll = rng.gen_range(1..5);

        if roll == 1
        {
            amount += 1;
        }
    }

    amount
}

fn sample(number : usize, highest : u32) -> u32
{
    let rng = rand::thread_rng();
    let roll = roll_d4s(rng, number);

    if roll > highest
    {
        return roll
    }

    0
}