use crate::framework::{IFramework, Population};
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
            ret.push(row);
        }
        ret
    }

    fn fitness_function(self, cromosome: &Vec<i8>) -> i32 {
        cromosome.iter().fold(0, |acc, n| acc + *n as i32)
    }

    fn xover(self, population: &Population<Vec<i8>>) -> Population<Vec<i8>> {
        let mut new_population = population.clone();
        let mut i = 0;
        let mut rng = rand::thread_rng();
        let row_size = new_population[0].len();
        while i < new_population.len() {
            let p1 = new_population[i].clone();
            let p2 = new_population[i + 1].clone();
            let break_point = rng.gen_range(0..row_size);

            let (p1_1, p1_2) = p1.split_at(break_point);
            let (p2_1, p2_2) = p2.split_at(break_point);

            new_population[i] = [p1_1, p2_2].concat();
            new_population[i + 1] = [p2_1, p1_2].concat();

            i += 2;
        }
        new_population
    }

    fn mutate(self, population: &Population<Vec<i8>>) -> Population<Vec<i8>> {
        let mut new_population = population.clone();
        let mut rng = rand::thread_rng();
        for i in 0..new_population.len() {
            for j in 0..new_population[i].len() {
                if new_population[i][j] == 0 && rng.gen_range(0..100) < 5 {
                    new_population[i][j] = rng.gen_range(0..2);
                }
            }
        }
        new_population
    }

    fn max_iters(self) -> i32 {
        self.iter
    }
}
