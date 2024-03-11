//const SECONDS_IN_MINUTE: u32 = 60;

fn main(){
    /*const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 =
    SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

    //Inteiros positivos / unsigneds
    let total = 30;
    println!("\nTrabalhou {} horas\n", total);
    {
        let total = total + 40;
        println!("Trabalhou {} horas.\n", total);
    }
    let total_in_seconds = total + SECONDS_IN_HOUR;
    println!("Trabalhou {} segundos.\n", total_in_seconds);

    let total: &str = "Quarenta";
    println!("Trabalhou {} horas.\n", total);

    //Primitivos de tamanhos específicos
    let x: u8 = 5;
    let y: u8 = x * 51;
    println!("{} - 20 = {}\n", x, y);
    //Armazenar bytes
    let by: u8 = b'A';
    println!("O byte = {}.\n", by);
    //Armazenar bases (ex. binário)
    let b: i32 = 0b1111_0001;
    println!("O binário = {}", b); */

    /*N° Real / Float
    let floatNum = 42.1;
    println!("O n° real = {}", floatNum); */

    //Boolean
    fn teste_bool(){
        let bool_var: bool = false;
        if bool_var == true{
            println!("\nÉ verdadeiro! -> {}\n", bool_var);
        }else{
            println!("\nÉ falso! -> {}\n", bool_var);
        }
    }teste_bool();

    //Tupla
    let mut numbers: (i32, i32, f64) = (1, 2, 3.5);
    println!("Tupla.0 = {:?}", numbers.0);
    println!("Tupla = {:?}", numbers);
    let (a, b, c) = numbers;
    println!("Patern match b/numbers.1 = {}", b);
    numbers.0 = 50;
    println!("Numbers agora = {:?}", numbers);
    numbers = (4, 5, 6.2);
    println!("Nova Tupla = {:?}\n", numbers);

}
