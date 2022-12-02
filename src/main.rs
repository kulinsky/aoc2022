pub mod day_1;
pub mod day_2;

fn main() {
    let data = day_1::get_data("input-1.txt");
    let res_1 = day_1::solve_1(data.clone());
    let res_2 = day_1::solve_2(data.clone());

    println!("res_1 = {res_1}; res_2 = {res_2}");
}
