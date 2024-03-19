use std::collections::HashMap;

/// # Task Scheduler
/// ## Arguments
/// * `tasks` - A vector with processor tasks' representation;
/// * `n` - a processor cooling time.
///
/// You are given an array of CPU tasks, each represented by letters A to Z, and a cooling time, n.
/// Each cycle or interval allows the completion of one task. Tasks can be completed in any order,
/// but there's a constraint: identical tasks must be separated by at least n intervals due to
/// cooling time.
///
/// Return the minimum number of intervals required to complete all tasks.

pub fn least_interval(mut tasks: Vec<char>, n: i32) -> i32 {
    let tasks_len = tasks.len();
    let mut chars_amount: HashMap<char, i32> = HashMap::new();
    let mut max: i32 = 1;
    let mut amount: i32;
    let mut amount_incremented: i32;
    for value in tasks {
        if !chars_amount.contains_key(&value) {
            chars_amount.insert(value, 1);
        } else {
            amount = chars_amount[&value];
            amount_incremented = amount + 1;
            chars_amount.insert(value, amount_incremented);
            if amount + 1 > max {
                max = amount_incremented;
            }
        }
    }
    let values = chars_amount.values().cloned().collect::<Vec<i32>>();
    let mut result = (max - 1) * n;
    let mut is_max = false;
    for value in values {
        if value == max {
            if is_max == false {
                is_max = true;
                continue;
            } else {
                result += 1;
            }
        }
        result -= value;
    }
    if result < 0 {
        return tasks_len as i32;
    }
    return tasks_len as i32 + result;
}
