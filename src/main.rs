use std::io; // input - output 

use rand::Rng; 
use std::cmp::Ordering;

fn main(){
    
    // Цикл
    loop {
        // Создать строковую переменную
        let mut str = String::new();
        // Создать численную переменную в которой сохранится рандомное число
        let randVal = rand::thread_rng().gen_range(1, 10);
        println!("Write value: ");

        // Считать строку командной строки
        io::stdin().read_line(&mut str).expect("Not written !");
        // Метод trim для типа String устраняет любые пробелы в начале и конце.
        let str: u32 = match str.trim().parse(){ 
            Ok(num) => num, // Проверка - если зн-е явл. числом
            Err(_) => continue // Если что-то другое
        };

        // Вывести введенное зн-е и секретное значение
        println!("Writen value: {}", str);

        // Обработка - сравнение зн-я str и randVal
        match str.cmp(&randVal){
            // Если меньше 
            Ordering::Less => println!("To less !"),
            // Если равно
            Ordering::Equal => {
                println!("Equal !");
                // Остановить цикл
                break;
            },
            // Если больше
            Ordering::Greater => println!("Greater !")
        }
    }
}
