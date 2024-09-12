use std::thread;
use rand::Rng;
use rand::rngs::ThreadRng;
use GravelerChallenge::ThreadPool;

fn main() {
    let total_samples = 1000000000;
    let thread_count = 30;
    let samples_per_thread = total_samples / thread_count;
    let rolls = 231;

    let mut samples_completed = 0;
    let mut highest_samples_per_thread : Vec<u8> = vec!(0; thread_count);
    let mut is_thread_working : Vec<bool> = vec!(false; thread_count);

    let mut highest = 0;
    for i in 0..total_samples
    {
        let result = sample(rolls, highest);

        if result > 0
        {
            highest = result;
        }

        if (i % 100 == 0)
        {
            println!("Sample {} Complete. Highest: {}\n", i, highest);
        }
    }

    println!("Highest: {}", highest);
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