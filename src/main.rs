use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let nombre_a_deviner: u8 = rng.random_range(0..=100);

    loop {
        let mut input = String::new();
        println!("Devine le nombre ! (0 à 100)");
        io::stdin()
            .read_line(&mut input)
            .expect("❌ Erreur lors de la lecture");
        let n: u8 = match input.trim().parse() {
            Ok(num) if (0..=100).contains(&num) => num,
            _ => {
                println!("❌ La saisie est incorrecte ! Veuillez rééssayer.");
                println!();
                continue;
            }
        };

        if n == nombre_a_deviner {
            println!("✅ Bravo! Le nombre à deviner était bien {}.", nombre_a_deviner);
            return; //On sort de la boucle quand on a deviné le nombre
        }
        else if n < nombre_a_deviner {
            println!("C'est plus ! ⤴️");
            println!(); //Permet saut à la ligne
        }
        else if n > nombre_a_deviner {
            println!("C'est moins ! ⤵️");
            println!();
        }
    }
}