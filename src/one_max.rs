use crate::framework::{Chromosome, IFramework, Population};
use rand::Rng;

#[derive(Clone, Copy)]
pub struct OneMaxFramework {
    n: i32,
    m: i32,
    iter: i32,
}

impl OneMaxFramework {
    pub fn new(n: i32, m: i32, iter: i32) -> Self {
        OneMaxFramework {
            n: n,
            m: m,
            iter: iter,
        }
    }
}

impl IFramework<Vec<i8>> for OneMaxFramework {
    fn genotype(self) -> Population<Vec<i8>> {
        let mut rng = rand::thread_rng();
        let mut ret = vec![];
        for _ in 0..self.n {
            let mut row = vec![];
            for _ in 0..self.m {
                row.push(rng.gen_range(0..2));
            }
            ret.push(Chromosome::new(row));
        }
        ret
    }

    fn fitness_function(self, cromosome: &Chromosome<Vec<i8>>) -> i32 {
        cromosome.genes.iter().fold(0, |acc, n| acc + *n as i32)
    }

    fn termination_condition(self, population: Population<Vec<i8>>) -> bool {
        for chromosome in population {
            if chromosome.fitness >= 1000.0 {
                return true;
            }
        }
        false
    }

    fn mutate_chromosome(self, chromosome: &mut Chromosome<Vec<i8>>) {
        let mut rng = rand::thread_rng();
        for i in 0..chromosome.genes.len() {
            if chromosome.genes[i] == 0 && rng.gen_range(0..100) < 5 {
                chromosome.genes[i] = rng.gen_range(0..2);
            }
        }
    }

    fn max_iters(self) -> i32 {
        self.iter
    }
}
