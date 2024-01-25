use std::collections::HashMap;
use std::hash::Hash;

pub fn find_final_state<S, F>(initial_state: S, func: F, repeats: usize) -> S
where
    S: Clone + Eq + Hash,
    F: Fn(S) -> S,
{
    let mut state = initial_state;
    let mut seen = HashMap::new();
    let mut counter = 0;
    while !seen.contains_key(&state) {
        seen.insert(state.clone(), counter);
        state = func(state);
        counter += 1;
    }

    let last_counter = seen[&state];
    let period = counter - last_counter;
    let offset = (repeats - last_counter) % period;
    (0..offset).fold(state, |it_state, _| func(it_state))
}
