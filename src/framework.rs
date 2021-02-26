pub type Population<T> = Vec<T>;

pub trait IFramework<T> {
    fn genotype(self) -> Population<T>;
    fn fitness_function(self, cromosome: &T) -> i32;
    fn xover(self, population: &Population<T>) -> Population<T>;
    fn mutate(self, population: &Population<T>) -> Population<T>;
    fn max_iters(self) -> i32;
}

pub struct FrameworkSolver<T, F: IFramework<T>> {
    population: Vec<T>,
    framework: F,
}

impl<T, F> FrameworkSolver<T, F>
where
    T: Clone,
    T: Default,
    F: IFramework<T>,
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
        let (mut ext_max_val, mut ext_max_elem) = self.best_result();
        let mut iter = 0;
        while iter < self.framework.max_iters() {
            iter += 1;
            println!("iter {:?}", iter);
            self.population = self.framework.xover(&self.population);
            self.population = self.framework.mutate(&self.population);
            let (max_val, max_elem) = self.best_result();
            ext_max_val = max_val;
            ext_max_elem = max_elem;
        }
        println!("Total iter {:?}", iter);
        println!("Best val {:?}", ext_max_val);
    }

    fn best_result(&self) -> (i32, T) {
        self.population.iter().cloned().enumerate().fold(
            (0, T::default()),
            |(max_val, max_elem), (i, elem)| {
                let val: i32 = self.framework.fitness_function(&elem);
                if val > max_val {
                    (val, elem)
                } else {
                    (max_val, max_elem)
                }
            },
        )
    }
}
