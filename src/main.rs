pub mod day_1;
pub mod day_2;

fn main() {
    let data = day_1::get_data("input-1.txt");
    let res_1 = day_1::solve_1(data.clone());
    let res_2 = day_1::solve_2(data.clone());
    println!("res_1 = {res_1}; res_2 = {res_2}");

    let data = day_2::get_data_1("input-2.txt");
    let res_1 = day_2::problem_1(data);
    let data_2_2 = day_2::get_data_2("input-2.txt");
    let res_2 = day_2::problem_2(data_2_2);

    println!("day 2: res_1 = {res_1}, res_2 = {res_2}");
}
