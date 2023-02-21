#![deny(clippy::all)]

fn main() {
    let _first_name = "Fatih";
    let _last_name = "Özgen";
    let _age = 37;
    let _job = "engineer";
    let _percentage = 0.000_001;
    println!(
        "My name is {} {} and my job is {}",
        _first_name, _last_name, _job
    );

    println!("Aşkta şansım %{}", _percentage)
}
