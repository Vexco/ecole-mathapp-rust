mod key_helper;
mod message_helper;
mod cipher_helper;

use key_helper::KeyHelper;
use message_helper::MessageHelper;
use cipher_helper::CipherHelper;

fn main() {
    // Génération des clés
    let (a, b) = KeyHelper::generate_key();
    let c = KeyHelper::find_c(a);
    let key = (a, b);
    let pub_key = KeyHelper::generate_public_key(key, c);

    println!("Cle de chiffrement : (A', B') = ({}, {})", key.0, key.1);
    println!("Cle de déchiffrement : (C', B') = ({}, {})", pub_key.0, pub_key.1);

    let message = "HELLO";
    let int_message = MessageHelper::convert_message_to_integers(message);

    let int_cipher_message = CipherHelper::cipher(key, &int_message);
    let cipher_message = MessageHelper::convert_integers_to_message(&int_cipher_message);

    println!("Mon message: {}", message);
    println!("Mon message chiffre: {}", cipher_message);

    let decipher_int_message = CipherHelper::decipher(pub_key, int_cipher_message);
    let decipher_message = MessageHelper::convert_integers_to_message(&decipher_int_message);
    println!("Mon message dechiffre: {}", &decipher_message);
}
