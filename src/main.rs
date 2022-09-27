fn main() {
    let grid2d= create_grid(5, 5);
    println!("{:?}", grid2d);
}

fn create_grid(width: usize, height: usize) -> Vec<Vec<i32>> {
    let mut grid2d: Vec<Vec<i32>> = vec![vec![0; width]; height];
    grid2d[0][0] = 3;
    grid2d[0][1] = 4;
    return grid2d;
}
