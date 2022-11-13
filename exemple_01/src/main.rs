// Un exemple d'appel de la fonction gethostname() de la lib de la vie (libc).
// Alors on pourrait se simplifier la vie avec le binding et la définition des types C grace au
// crate libc, mais c'est RFS ici, donc on fait sans. C'est plus metal ! 🤘

// On déclare la fonction externe gethostname().
extern "C" {
    // On fait le binding de la fonction C gethostname() en faisant correspondre les types.
    // int gethostname(char *nom, size_t lg);
    pub fn gethostname(name: *mut i8, size: usize) -> i32;
}

//                     |--- On pourrait améliorer et renvoyer un Result mais ça complexifie un peu
//                     v    l'exemple
pub fn hostname() -> String {
    // On définit la taille de la chaine à récupérer d'après la doc un hostname ne peut pas faire +
    // de 255 char. 64 en fait sous Linux.
    let buffer_len: usize = 255;
    // On crée un buffer pour recevoir notre hostname.
    let mut buffer: Vec<u8> = vec![0; buffer_len];
    //                   ^--- on définit en u8 car pour utiliser plus tard String::from_utf8 il
    //                   faut un Vec<u8>.
    //

    // On exécute la fonction C gethostname() via un bloc unsafe (normal c'est du C donc par
    // définition unsafe)
    let error = unsafe { gethostname(buffer.as_mut_ptr() as *mut i8, buffer_len) };
    //                                                       ^-- conversion de pointeur de u8 vers i8, car
    //                                                       la fonction C attend un pointer sur i8 (*char)

    // On panique en cas d'erreur.
    if error != 0 {
        panic!("The libc fail to get hostname.")
    }

    // On cherche la position du premier 0 dans le buffer et ensuite on tronque le Vec à la bonne
    // longueur (sans le 0). Si on trouve pas de 0 avant la fin alors on utilise la valeur max (255 --> buffer_len)
    let len = buffer.iter().position(|b| *b == 0).unwrap_or(buffer_len);
    buffer.truncate(len);

    // On convertie le buffer tronqué en une String en supposant que c'est de l'UTF-8, si c'est pas
    // le cas alors on panique.
    String::from_utf8(buffer).expect("Fail to convert to a String, is hostname UTF-8 encoded ?")
}

fn main() {
    println!("{}", hostname());
}

//  🦉 uggla   master  …  rfs  rfs_ffi  exemple_01  cargo run
//    Compiling exemple_01 v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_ffi/exemple_01)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.59s
//      Running `target/debug/exemple_01`
// ugglalaptop
