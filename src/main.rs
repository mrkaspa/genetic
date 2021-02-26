mod framework;
mod one_max;

use framework::FrameworkSolver;
use one_max::OneMaxFramework;

fn main() {
    let fs = FrameworkSolver::new(OneMaxFramework::new(100, 1000, 500));
    fs.run();
}
