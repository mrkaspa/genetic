use rand::Rng;

type Matrix = Vec<Vec<i8>>;

fn main() {
    let mat = create_matrix(100, 1000);
    println!("running");
    algorithm(mat);
}

fn create_matrix(n: i32, m: i32) -> Matrix {
    let mut rng = rand::thread_rng();
    let mut ret = vec![];
    for _ in 0..n {
        let mut row = vec![];
        for _ in 0..m {
            row.push(rng.gen_range(0..2));
        }
        ret.push(row);
    }
    ret
}

fn algorithm(mut m: Matrix) {
    let (mut ext_idx, mut ext_max_val, mut ext_max_elem) = get_best_result(&m);
    let mut iter = 0;
    while ext_max_val < 1000 && iter < 1000 {
        iter += 1;
        println!("iter {:?}", iter);
        evaluate(&mut m);
        xover(&mut m);
        mutate(&mut m);
        let (idx, max_val, max_elem) = get_best_result(&m);
        ext_idx = idx;
        ext_max_val = max_val;
        ext_max_elem = max_elem;
    }
    println!("Total iter {:?}", iter);
    println!("Best idx {:?}", ext_idx);
    println!("Best val {:?}", ext_max_val);
}

fn get_best_result(m: &Matrix) -> (usize, i32, Vec<i8>) {
    m.iter()
        .cloned()
        .enumerate()
        .fold((0, 0, vec![]), |(pos, max_val, max_elem), (i, elem)| {
            let val: i32 = elem.iter().fold(0, |acc, n| acc + *n as i32);
            if val > max_val {
                (i, val, elem)
            } else {
                (pos, max_val, max_elem)
            }
        })
}

fn evaluate(m: &mut Matrix) {
    m.sort_by(|c1, c2| {
        let val_1 = c1.iter().fold(0, |acc, n| acc + *n as i32);
        let val_2 = c2.iter().fold(0, |acc, n| acc + *n as i32);

        val_1.cmp(&val_2).reverse()
    })
}

fn xover(m: &mut Matrix) {
    let mut i = 0;
    let mut rng = rand::thread_rng();
    let row_size = m[0].len();
    while i < m.len() {
        let p1 = m[i].clone();
        let p2 = m[i + 1].clone();
        let break_point = rng.gen_range(0..row_size);

        let (p1_1, p1_2) = p1.split_at(break_point);
        let (p2_1, p2_2) = p2.split_at(break_point);

        m[i] = [p1_1, p2_2].concat();
        m[i + 1] = [p2_1, p1_2].concat();

        i += 2;
    }
}

fn mutate(m: &mut Matrix) {
    let mut rng = rand::thread_rng();
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 0 && rng.gen_range(0..100) < 5 {
                m[i][j] = rng.gen_range(0..2);
            }
        }
    }
}
