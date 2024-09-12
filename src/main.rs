use std::thread;
use std::thread::available_parallelism;
use rand::Rng;
use rand::rngs::ThreadRng;

const TOTAL_SAMPLES : f32 = 1000000000.0;
const ROLLS : usize = 231;

fn main() {


    let mut samples_completed = 0;
    let mut highest_samples_per_thread = Vec::new();
    let mut handles = Vec::new();

    let THREAD_COUNT : usize = (available_parallelism().unwrap().get() - 2);

    let mut highest = 0;
    /*
    for i in 0.total_samples
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
    */


    for id in 0..THREAD_COUNT
    {
        handles.push(thread::spawn(move || {
            run_thread(THREAD_COUNT.clone())
        }));
    }

    for id in 0..THREAD_COUNT
    {
        let result = handles.pop().unwrap().join().unwrap();
        highest_samples_per_thread.push(result);
    }

    for id in 0..THREAD_COUNT
    {
        let current_value = highest_samples_per_thread.get(id).unwrap().clone();
        if highest < current_value
        {
            highest = current_value;
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

fn run_thread(thread_count : usize) -> u32
{
    let mut highest = 0;

    let samples_per_thread : usize = (TOTAL_SAMPLES / thread_count as f32).ceil() as usize;

    for run in 0..samples_per_thread
    {
        let result = sample(ROLLS, highest);

        if result > 0
        {
            highest = result;
        }

        if run % 1000 == 0
        {
            println!("A Thread is on run {}, total threads: {}", run, thread_count)
        }
    }

    highest
}