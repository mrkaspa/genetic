use rand::Rng;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Chromosome<T> {
    pub genes: T,
    size: i32,
    pub fitness: f32,
    age: i32,
}

impl<T> Chromosome<T> {
    pub fn new(genes: T) -> Self {
        Chromosome {
            genes: genes,
            size: 0,
            fitness: 0.0,
            age: 0,
        }
    }
}

impl<T> Default for Chromosome<T>
where
    T: Default,
{
    fn default() -> Chromosome<T> {
        Chromosome {
            genes: T::default(),
            size: 0,
            fitness: 0.0,
            age: 0,
        }
    }
}

pub type Population<T> = Vec<Chromosome<T>>;

pub trait IFramework<T> {
    fn genotype(self) -> Population<T>;
    fn fitness_function(self, chromosome: &Chromosome<T>) -> i32;
    fn termination_condition(self, population: Population<T>) -> bool;
    fn mutate_chromosome(self, chromosome: &mut Chromosome<T>);
    fn max_iters(self) -> i32;
}

pub struct FrameworkSolver<T, F: IFramework<T>> {
    population: Population<T>,
    framework: F,
}

impl<T, F> FrameworkSolver<Vec<T>, F>
where
    T: Clone,
    T: Default,
    T: Debug,
    F: IFramework<Vec<T>>,
    F: Copy,
{
    pub fn new(f: F) -> Self {
        FrameworkSolver {
            population: vec![],
            framework: f,
        }
    }

    pub fn run(mut self) {
        self.population = self.framework.genotype();
        let mut ext_max_elem = self.best_result();
        let mut iter = 0;
        while iter < self.framework.max_iters() {
            iter += 1;
            println!("iter {:?}", iter);
            self.xover();
            self.mutate();
            let max_elem = self.best_result();
            ext_max_elem = max_elem;
        }
        println!("Total iter {:?}", iter);
        println!("Age {:?}", ext_max_elem.age);
        println!("Best val {:?}", ext_max_elem.genes);
    }

    fn best_result(&mut self) -> Chromosome<Vec<T>> {
        let mut best_chromosome = &Chromosome::default();
        for chromosome in self.population.iter_mut() {
            let fitness: i32 = self.framework.fitness_function(&chromosome);

            chromosome.age = chromosome.age + 1;
            chromosome.fitness = fitness as f32;
            best_chromosome = if chromosome.fitness > best_chromosome.fitness {
                chromosome
            } else {
                best_chromosome
            }
        }
        best_chromosome.clone()
    }

    fn xover(&mut self) {
        let mut i = 0;
        let mut rng = rand::thread_rng();
        let row_size = self.population[0].genes.len();
        while i < self.population.len() {
            let p1 = self.population[i].genes.clone();
            let p2 = self.population[i + 1].genes.clone();
            let break_point = rng.gen_range(0..row_size);

            let (p1_1, p1_2) = p1.split_at(break_point);
            let (p2_1, p2_2) = p2.split_at(break_point);

            self.population[i].genes = [p1_1, p2_2].concat();
            self.population[i + 1].genes = [p2_1, p1_2].concat();

            i += 2;
        }
    }

    fn mutate(&mut self) {
        for i in 0..self.population.len() {
            self.framework.mutate_chromosome(&mut self.population[i])
        }
    }
}
