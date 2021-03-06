use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Sym {
    Var(usize),
    Fwd(usize),
    Plus,
    Minus,
    Push,
    Pop,
}

pub struct LSystem {
    iterations: Vec<Vec<Sym>>,
    productions: HashMap<Sym, Vec<Sym>>,
    angle_radians: f64,
}

impl LSystem {
    pub fn new(axiom: Vec<Sym>, productions: HashMap<Sym, Vec<Sym>>, angle_radians: f64) -> LSystem {
        LSystem {
            iterations: vec![axiom],
            productions: productions,
            angle_radians: angle_radians,
        }
    }

    pub fn get(&mut self, index: usize) -> &[Sym] {
        let iter_count = self.iterations.len();

        if iter_count <= index {
            while self.iterations.len() <= index {
                let next = self.compute_iteration(
                    self.iterations.last().expect("iterations should never be empty"));

                self.iterations.push(next);
            }
        }

        &self.iterations[index]
    }

    pub fn angle_radians(&self) -> f64 {
        self.angle_radians
    }

    fn compute_iteration(&self, state: &[Sym]) -> Vec<Sym> {
        let mut result = vec![];

        for sym in state {
            if let Some(sub) = self.productions.get(sym) {
                result.extend_from_slice(&sub);
            } else {
                result.push(*sym);
            }
        }

        result
    }
}
