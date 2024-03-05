use std::io; // Importe le module d'entrée/sortie standard de Rust.
use std::cmp::Ordering; // Importe l'énumération Ordering de Rust qui a trois variantes : Less, Greater et Equal. // Moins, plus, égal qui nous servira à guess le nombre secret
use rand::Rng; // En gros c'est l'import qui va permettre de génerer de la RNG de ce que j'ai compris 

fn main() { // Définit la fonction principale qui est le point d'entrée de mon programme, la fonction main en gros
    println!("Guess the number!"); // Affiche "Guess the number!" à l'écran, un simple print

    let secret_number = rand::thread_rng().gen_range(1, 101); // Génère un nombre aléatoire entre 1 et 100 et le stocke dans la variable secret_number.

    loop { // Commence une boucle infinie. Le jeu continuera jusqu'à ce que le joueur devine le bon nombre.
        println!("Please input your guess."); // Demande au joueur de saisir son input.

        let mut guess = String::new(); // Crée une nouvelle chaîne de caractères mutable pour stocker l'input du joueur

        io::stdin().read_line(&mut guess) // Appelle la fonction read_line sur l'entrée standard pour lire une ligne de l'utilisateur. 
            .expect("Erreur"); // Si read_line échoue, le programme affiche "Erreur".

        let guess: u32 = match guess.trim().parse() { // Convertit la chaîne de caractères guess en un entier non signé de 32 bits. Si la conversion échoue (par exemple, si l'utilisateur entre du texte qui ne peut pas être converti en nombre), le programme continue la boucle.
            Ok(num) => num, // Si la conversion réussit, le nombre est renvoyé.
            Err(_) => continue, // Si la conversion échoue, le programme continue la boucle.
        };

        println!("You guessed: {}", guess); // Affiche "You guessed: " suivi du nombre que le joueur a donné avantça 

        match guess.cmp(&secret_number) { // Compare la supposition du joueur au nombre secret.
            Ordering::Less => println!("Too small!"), // Si la supposition est inférieure au nombre secret, le programme affiche "Too small!"
            Ordering::Greater => println!("Too big!"), // Si la supposition est supérieure au nombre secret, le programme affiche "Too big!"
            Ordering::Equal => { // Si la supposition est égale au nombre secret, le programme affiche "You win!" et sort de la boucle.
                println!("You win!");
                break;
            }
        }
    }
}
