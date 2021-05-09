fn main() {
    wh();
    fr();
}

fn wh() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is {}", a[index]);
        index = index + 1;
    }
}

fn fr() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}