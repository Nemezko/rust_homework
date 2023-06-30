fn main() {
  
    let mut x = 2;
    println!("Значение x: {x}");
    x = 16;
    println!("Значение x: {x}");

  
    const TWO_HOURS_IN_SECONDS: u64 = 7100 + 50 * 2;
    println!("У меня обед через: {TWO_HOURS_IN_SECONDS} секунд. \n\r ");


    let x = 6;
    let x = x + 1;
    {
        let x = x * 3 + 4;
// Сейчас 7 затененна
        println!("Значение x в скобках: {x}.")
    }

    println!("Значение x: {x}.");


    let spaces = "   ";
    let spaces = spaces.len();
    println!("Количество пробелов: {spaces}.") 
}
