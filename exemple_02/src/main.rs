// Exemple alternatif. Moins bien que le premier car il y a 2 blocs unsafe.

// Ici on utilise le type CStr qui va nous permettre de récupérer un string C depuis un pointeur.
use std::ffi::CStr;

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
    // On crée un buffer initialisé à 0, pour recevoir notre hostname.
    let mut buffer: Vec<i8> = vec![0; buffer_len];
    //                   ^-- ici on garde un i8 comparé à l'exemple précèdent. Comme ça pas
    //                   besoin de conversion par la suite.

    // On exécute la fonction C gethostname() via un bloc unsafe (normal c'est du C donc par
    // définition unsafe)
    let error = unsafe { gethostname(buffer.as_mut_ptr(), buffer_len) };

    // On panique en cas d'erreur.
    if error != 0 {
        panic!("The libc fail to get hostname.")
    }

    // Ici on récupère un pointeur sur le buffer et on utilise CStr::from_ptr pour obtenir un CStr.
    let hostname = unsafe { CStr::from_ptr(buffer.as_ptr()) }
        // Ensuite on utilise to_str() pour convertir en &str et on panique si la chaine est pas
        // UTF-8.
        .to_str()
        .expect("Fail to convert to a &str, is hostname UTF-8 encoded ?");

    // On convertit le &str en String pour renvoyer le bon type.
    hostname.to_string()
}

fn main() {
    println!("{}", hostname());
}

//  🦉 uggla   master  …  rfs  rfs_ffi  exemple_02  cargo run
//    Compiling exemple_02 v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_ffi/exemple_02)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.74s
//      Running `target/debug/exemple_02`
// ugglalaptop
