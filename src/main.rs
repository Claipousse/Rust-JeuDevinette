use rand::Rng;
use std::io;
use std::cmp::Ordering;

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

        match n.cmp(&nombre_a_deviner) {
            Ordering::Less => println!("C'est plus ! ⤴️"),
            Ordering::Greater => println!("C'est moins ! ⤵️"),
            Ordering::Equal => {
                println!("✅ Bravo! Le nombre à deviner était bien {}.", nombre_a_deviner);
                return; //On sort de la boucle quand on a deviné le nombre
            }
        }
    }
}