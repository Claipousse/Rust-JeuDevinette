use rand::Rng; //rand 0.9.0 importé dans .toml
use std::io; //Gère entrées/sorties
use std::cmp::Ordering; //Gère les comparaisons

fn main() {
    let mut rng = rand::rng();
    let nombre_a_deviner: u8 = rng.random_range(0..=100); //On code la variable sur 8 bits car 0 à 255 suffisent

    loop {
        let mut str_input = String::new();
        println!("Devine le nombre ! (0 à 100)");
        io::stdin().read_line(&mut str_input).expect("❌ Erreur lors de la lecture");

        let input: u8 = match str_input.trim().parse() {
            Ok(num) if (0..=100).contains(&num) => num, //Le nombre saisit doit être compris entre 0 et 100
            _ => {
                println!("❌ La saisie est incorrecte ! Veuillez rééssayer.");
                println!();
                continue; //On saute l'itération pour ne pas éxécuter le code dessous
            }
        };

        match input.cmp(&nombre_a_deviner) {
            Ordering::Less => {
                println!("C'est plus ! ⤴️");
                println!();
            },
            Ordering::Greater => {
                println!("C'est moins ! ⤵️");
                println!();
            },
            Ordering::Equal => {
                println!("✅ Bravo! Le nombre à deviner était bien {}.", nombre_a_deviner);
                return; //On sort de la boucle quand on a deviné le nombre
            }
        }
    }
}