pub mod consequence;
pub mod mail;
pub mod seeds;
use std::collections::HashMap;

pub use consequence::*;

pub fn lorem_ipsum() -> String {
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed mollis neque a magna sollicitudin laoreet.\
     Sed pulvinar enim in libero aliquam convallis.\
     Ut fermentum scelerisque risus, in placerat risus volutpat sit amet.\
     Maecenas dapibus lobortis leo.\
     Sed quis velit ac massa porttitor congue.\
     Curabitur eu commodo purus.\
     Ut eget volutpat tellus, non elementum sem.\
     Ut porttitor elit eu lectus finibus scelerisque.\
     Integer metus libero, molestie at mollis in, commodo ac sem.\
     Mauris commodo sagittis quam, vitae cursus arcu vulputate sed.\
     Aliquam erat volutpat.\
     Nunc eu odio ut risus feugiat tincidunt condimentum id sem.\
     Donec a orci nec risus vestibulum tristique.\
     Pellentesque in risus et augue tincidunt pharetra.\
     Maecenas fringilla, urna ut cursus congue, lacus urna commodo urna, eu pharetra dolor metus vel magna."
        .to_string()
}

pub fn months() -> HashMap<u32, String> {
    vec![
        "Janvier",
        "Février",
        "Mars",
        "Avril",
        "Mai",
        "Juin",
        "Juillet",
        "Août",
        "Septembre",
        "Octobre",
        "Novembre",
        "Décembre",
    ]
    .into_iter()
    .map(move |m| m.to_string())
    .enumerate()
    .into_iter()
    .map(|(i, month)| ((i + 1) as u32, month))
    .collect()
}
