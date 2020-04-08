use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
enum Conclusion {
    Winning,
    Losing,
    Unknown
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct State(u8 /* top heap (1 or 0) */, u8 /* heap 2 */, u8 /* heap 3 */, u8 /* heap 4 */);

struct SolutionMap(BTreeMap<State, Conclusion>);

impl SolutionMap {
    fn new() -> Self {
        let mut states: BTreeMap<_, _> = Default::default();
        // populate the states
        for heap0 in 0..=1u8 {
            for heap1 in 0..=3u8 {
                for heap2 in 0..=5u8 {
                    for heap3 in 0..=7u8 {
                        states.insert(State(heap0, heap1, heap2, heap3), Conclusion::Unknown);
                    }
                }
            }
        }

        // Mark (0) as a LOSING state
        *states.get_mut(&State(0, 0, 0, 0)).unwrap() = Conclusion::Losing;

        Self(states)
    }

    fn mark(&mut self, s: State, v: Conclusion) {
        *self.0.get_mut(&s).unwrap() = v;
    }

    fn is_losing(&self, s: State) -> bool {
        *self.0.get(&s).unwrap() == Conclusion::Losing
    }

    fn is_winning(&self, s: State) -> bool {
        *self.0.get(&s).unwrap() == Conclusion::Winning
    }

    fn find_winning_states(&self) -> Vec<State> {
        self.0.iter()
            .filter(|(_, v)| **v == Conclusion::Winning)
            .map(|(&s, _)| s)
            .collect()
    }

    fn parents_of(&self, child: State) -> Vec<State> {
        self.0.iter()
            .filter(move |(&parent, _)| child.is_child_of(parent))
            .map(|(&parent, _)| parent)
            .collect()
    }

    fn children_of(&self, parent: State) -> Vec<State> {
        self.0.iter()
            .filter(move |(&child, _)| child.is_child_of(parent))
            .map(|(&child, _)| child)
            .collect()
    }

    fn unsolved(&self) -> Vec<State> {
        self.0.iter()
            .filter(|(_, v)| **v == Conclusion::Unknown)
            .map(|(&s, _)| s)
            .collect()
    }

    fn is_solved(&self) -> bool {
        self.unsolved().len() == 0
    }
}

impl State {
    fn is_child_of(&self, parent: Self) -> bool {
        match (self.0.cmp(&parent.0), self.1.cmp(&parent.1), self.2.cmp(&parent.2), self.3.cmp(&parent.3)) {
            // If removing N sticks from EXACTLY one heap leads from parent -> self,
            // then self is a direct child of parent
            (Ordering::Less, Ordering::Equal, Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Equal, Ordering::Less, Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Equal, Ordering::Equal, Ordering::Less, Ordering::Equal) => true,
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Less) => true,
            _ => false,
        }
    }

    fn parity(&self) -> u8 {
        let parity_1 = parity_ones(&[self.0, self.1, self.2, self.3]);
        let parity_2 = parity_ones(&[self.0 >> 1, self.1 >> 1, self.2 >> 1, self.3 >> 1]);
        let parity_4 = parity_ones(&[self.0 >> 2, self.1 >> 2, self.2 >> 2, self.3 >> 2]);
        parity_1 + parity_2 + parity_4
    }
}

impl fmt::Display for SolutionMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (s, v) in &self.0 {
            writeln!(f, "{:?}: {:?} (parity {})", s, v, s.parity())?;
        }
        Ok(())
    }
}

fn parity_ones(values: &[u8]) -> u8 {
    let mut s = 0;
    for v in values {
        s += v & 1;
    }
    s % 2
}

fn main() {
    let mut sols = SolutionMap::new();

    while !sols.is_solved() {
        // All states which lead to ONLY losing states must be winning states. i.e. if you leave
        // the board in this state, you force your opponent into a losing state.
        for state in sols.unsolved() {
            if sols.children_of(state).into_iter().all(|s| sols.is_losing(s)) {
                sols.mark(state, Conclusion::Winning);
            }
        }

        // Any state which leads to at least ONE winning state must be a losing state. i.f. if you
        // leave the board in this state, your opponent MAY put it into a state where they win.
        let winning = sols.find_winning_states();
        for win in winning {
            for parent in sols.parents_of(win) {
                sols.mark(parent, Conclusion::Losing);
            }
        }
        // print the evolution ?
        // println!("{}", sols);
    }

    println!("{}", sols);
}

