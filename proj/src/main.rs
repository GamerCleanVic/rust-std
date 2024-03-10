const SECONDS_IN_MINUTE: u32 = 60;

fn main(){
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 =
    SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

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
}
