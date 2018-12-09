extern crate regex;
extern crate chrono;

use regex::Regex;
use std::collections::HashMap;
use chrono::prelude::*;
use lazy_static::lazy_static;


pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    let hash_map = calc_sleep_hist_per_guard(input);

    // find id with most sleeping minutes
    let mut max_sum = 0;
    let mut max_id = 0;
    for (id, v) in hash_map.iter(){
        let sum_minutes: i32 = v.iter().sum();
        if sum_minutes > max_sum{
            max_sum = sum_minutes;
            max_id = *id;
        }
    }

    let res = hash_map[&max_id].iter().enumerate().max_by_key(|x| x.1).unwrap();

    max_id * res.0 as i32
}

fn part_2(input: &str) -> i32 {
    let hash_map = calc_sleep_hist_per_guard(input);

    let res = hash_map.iter().map(get_max).max_by_key(|x| x.2).unwrap();

    res.0 * res.1
}

fn calc_sleep_hist_per_guard(input : &str) -> HashMap<i32, [i32; 60]> {
    
    let mut events: Vec<Event> = input.lines().map(|x| Event::from_str(&x)).collect();
    events.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    let mut hash_map : HashMap<i32, [i32; 60]> = HashMap::new();
    let mut id = 0;
    let mut asleep_min = 0;
    for evt in events{
        match evt {
            Event{ evt_type: EventType::BeginsShift(i), ..} => id = i,
            Event{ date_time: dt, evt_type: EventType::FallsAsleep} => asleep_min = dt.time().minute(),
            Event{ date_time: dt, evt_type: EventType::WakesUp} => {
                let entry = hash_map.entry(id).or_insert([0; 60]);
                for m in asleep_min..dt.time().minute(){
                    entry[m as usize] += 1;
                }
            }
        }
    }
    hash_map
}

fn get_max(elmt: (&i32, &[i32; 60])) -> (i32, i32, i32){
    let (k, v) = elmt;

    let (min, freq) = v.iter().enumerate().max_by_key(|x| x.1).unwrap();

    (*k, min as i32, *freq as i32)
}

#[derive(Debug)]
enum EventType {
    FallsAsleep,
    WakesUp,
    BeginsShift(i32)
}

#[derive(Debug)]
struct Event {
    date_time: DateTime<Utc>,
    evt_type: EventType
}

impl Event {
    fn from_str(s: &str) -> Event{
        lazy_static! {
            static ref RE : Regex = Regex::new(r"\[(.+)\] (.+)").unwrap();
            static ref RE2 : Regex = Regex::new(r"\d+").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        Event{
            date_time:Utc.datetime_from_str(&cap[1], "%Y-%m-%d %H:%M").unwrap(),
            evt_type: match &cap[2]{
                "falls asleep" => EventType::FallsAsleep,
                "wakes up" => EventType::WakesUp,
                s => {
                    let match_ = RE2.find(s).unwrap();
                    EventType::BeginsShift(match_.as_str().parse().unwrap())
                }
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("[1518-11-01 00:00] Guard #10 begins shift\n\
                    [1518-11-01 00:05] falls asleep\n\
                    [1518-11-01 00:25] wakes up\n\
                    [1518-11-01 00:30] falls asleep\n\
                    [1518-11-01 00:55] wakes up\n\
                    [1518-11-01 23:58] Guard #99 begins shift\n\
                    [1518-11-02 00:40] falls asleep\n\
                    [1518-11-02 00:50] wakes up\n\
                    [1518-11-03 00:05] Guard #10 begins shift\n\
                    [1518-11-03 00:24] falls asleep\n\
                    [1518-11-03 00:29] wakes up\n\
                    [1518-11-04 00:02] Guard #99 begins shift\n\
                    [1518-11-04 00:36] falls asleep\n\
                    [1518-11-04 00:46] wakes up\n\
                    [1518-11-05 00:03] Guard #99 begins shift\n\
                    [1518-11-05 00:45] falls asleep\n\
                    [1518-11-05 00:55] wakes up"), 
            240
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("[1518-11-01 00:00] Guard #10 begins shift\n\
                    [1518-11-01 00:05] falls asleep\n\
                    [1518-11-01 00:25] wakes up\n\
                    [1518-11-01 00:30] falls asleep\n\
                    [1518-11-01 00:55] wakes up\n\
                    [1518-11-01 23:58] Guard #99 begins shift\n\
                    [1518-11-02 00:40] falls asleep\n\
                    [1518-11-02 00:50] wakes up\n\
                    [1518-11-03 00:05] Guard #10 begins shift\n\
                    [1518-11-03 00:24] falls asleep\n\
                    [1518-11-03 00:29] wakes up\n\
                    [1518-11-04 00:02] Guard #99 begins shift\n\
                    [1518-11-04 00:36] falls asleep\n\
                    [1518-11-04 00:46] wakes up\n\
                    [1518-11-05 00:03] Guard #99 begins shift\n\
                    [1518-11-05 00:45] falls asleep\n\
                    [1518-11-05 00:55] wakes up"),
            4455
        );
    }
}
