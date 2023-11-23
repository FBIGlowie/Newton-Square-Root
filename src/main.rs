fn main() {
    // following some stack exchange answer on the webz I need to setup sum variables
    const NUMBER: i128 = 401; //sqrt is 20
    let mut guess: f64 = 0.0;
    loop {
        if guess * guess >= NUMBER.abs() as f64 {
            println!("{},{}", guess, guess * guess);
            break;
        } else {
            guess += 0.000000001;
        }
    }
}
// this garbage is pretty self explanatory
