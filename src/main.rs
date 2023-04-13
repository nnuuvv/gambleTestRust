use std::thread;
use std::thread::JoinHandle;

fn main() {
    println!("Hello, world!");

    let start: u128 = 10000000;
    let runsPerPercentRun = 5000;
    let gambleRuns: i32 = 100;
    let winPercent: f32 = 0.51;
    let runsPerPercent = 100;


    let mut averages: Vec<(u128, u128)> = Vec::new();


    for gamblePercent in 1..100 {
        let mut threadVector: Vec<JoinHandle<_>> = Vec::new();
        for run in 1..runsPerPercent {
            threadVector.push(thread::spawn(move || {

                let mut results: u128 = 0;

                for eachRun in 0..runsPerPercentRun {
                    results += gamble(start, gambleRuns, winPercent, gamblePercent);
                }

                return (results / runsPerPercent);
            }));
        }

        let mut avg: u128 = 0;
        let mut count = 0;
        for thread in threadVector {
            avg += thread.join().unwrap();
            count += 1;
        }
        averages.push((gamblePercent, avg/count));
    }




    averages.sort_by_key(|k| k.1);
    averages.reverse();

    println!("Top 5 by amount, with percentage");
    for top in 0..5
    {
        println!("{}. {} points at {} gamble percentage", top, averages[top].1, averages[top].0);
    }


}

fn gamble(mut total_points: u128, gambleRuns: i32, winPercent: f32, gamblePercent: u128 ) -> u128 {

    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    for each in 1..gambleRuns {
        let rand: f32 = rng.gen();
        if rand >= winPercent {
            total_points += total_points * gamblePercent / 100;
        } else {
            total_points -= total_points * gamblePercent / 100;
        }
    }

    total_points
}