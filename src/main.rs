use std::io;
use rand::seq::SliceRandom;

struct Board {
    tl: String,
    tc: String,
    tr: String,
    cl: String,
    cc: String,
    cr: String,
    bl: String,
    bc: String,
    br: String,
}

fn main() {

    let available = vec!["tl", "tc", "tr", "cl", "cc", "cr", "bl", "bc", "br"];

    println!("hello, I would like to play a game...");
    println!("you may start first as X...");

   let mut position = Board {
       tl: String::from("tl"),
       tc: String::from("tc"),
       tr: String::from("tr"),
       cl: String::from("cl"),
       cc: String::from("cc"),
       cr: String::from("cr"),
       bl: String::from("bl"),
       bc: String::from("bc"),
       br: String::from("br")
   };

   loop {
       loop {
            println!("_{}_|_{}_|_{}_", position.tl, position.tc, position.tr);
            println!("_{}_|_{}_|_{}_", position.cl, position.cc, position.cr);
            println!("_{}_|_{}_|_{}_", position.bl, position.bc, position.br);
            println!("select your space my entering the appropriate space designation...");

            let mut player = String::new();

            io::stdin()
                .read_line(&mut player)
                .expect("failed to read line");

            let player = player.trim();

            if player == "tl" && &position.tl != "_X" && &position.tl != "O_" {
                position.tl = String::from("_X");
                break;
                } else if player == "tc" && &position.tc != "_X" && &position.tc != "O_" {
                    position.tc = String::from("_X");
                    break;
                } else if player == "tr" && &position.tr != "_X" && &position.tr != "O_" {
                    position.tr = String::from("_X");
                    break;
                } else if player == "cl" && &position.cl != "_X" && &position.cl != "O_" {
                    position.cl = String::from("_X");
                    break;
                } else if player == "cc" && &position.cc != "_X" && &position.cc != "O_" {
                    position.cc = String::from("_X");
                    break;
                } else if player == "cr" && &position.cr != "_X" && &position.cr != "O_" {
                    position.cr = String::from("_X");
                    break;
                } else if player == "bl" && &position.bl != "_X" && &position.bl != "O_" {
                    position.bl = String::from("_X");
                    break;
                } else if player == "bc" && &position.bc != "_X" && &position.bc != "O_" {
                    position.bc = String::from("_X");
                    break;  //80
                } else if player == "br" && &position.br != "_X" && &position.br != "O_" {
                    position.br = String::from("_X");
                    break
                } else {
                    println!("That is an invalid entry, please try again.");
                }
            }
    //check win condition
        if &position.tl == "_X" && &position.tc == "_X" && &position.tr == "_X" {
            println!("You win...This time...");
            break;
        } else if &position.tl == "_X" && &position.cc == "_X" && &position.br == "_X" {
            println!("You win...This time...");
            break;
        } else if &position.tl == "_X" && &position.cl == "_X" && &position.bl == "_X" {
            println!("You win...This time...");
            break;
        } else if &position.tc == "_X" && &position.cc == "_X" && &position.bc == "_X" {
            println!("You win...This time...");
            break;
        } else if &position.tr == "_X" && &position.cc == "_X" && &position.bl == "_X" {
            println!("You win...This time...");
            break;
        } else if &position.tr == "_X" && &position.cr == "_X" && &position.br == "_X" {
            println!("You win...This time...");
            break;
        } else if &position.cl == "_X" && &position.cc == "_X" && &position.cr == "_X" {
            println!("You win...This time...");
            break;
        } else if &position.bl == "_X" && &position.bc == "_X" && &position.br == "_X" {
            println!("You win...This time...");
            break;
        }
    //computer turn
        loop {
            let me: Vec<_> = available 
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            if me[0] == &"tl" && &position.tl != "_X" && &position.tl != "O_" {
                position.tl = String::from("O_");
                break;
                } else if me[0] == &"tc" && &position.tc != "_X" && &position.tc != "O_" {
                    position.tc = String::from("O_");
                    break;
                } else if me[0] == &"tr" && &position.tr != "_X" && &position.tr != "O_" {
                    position.tr = String::from("O_");
                    break;
                } else if me[0] == &"cl" && &position.cl != "_X" && &position.cl != "O_" {
                    position.cl = String::from("O_");
                    break;
                } else if me[0] == &"cc" && &position.cc != "_X" && &position.cc != "O_" {
                    position.cc = String::from("O_");
                    break;
                } else if me[0] == &"cr" && &position.cr != "_X" && &position.cr != "O_" {
                    position.cr = String::from("O_");
                    break;
                } else if me[0] == &"bl" && &position.bl != "_X" && &position.bl != "O_" {
                    position.bl = String::from("O_");
                    break;
                } else if me[0] == &"bc" && &position.bc != "_X" && &position.bc != "O_" {
                    position.bc = String::from("O_");
                    break;
                } else if me[0] == &"br" && &position.br != "_X" && &position.br != "O_" {
                    position.br = String::from("O_");
                    break;
                } else {
                }
        println!("I have chosen space {:?}", me[0]);
        }
              
    //check computer win condition 
        if &position.tl == "O_" && &position.tc == "O_" && &position.tr == "O_" {
            println!("I win...Better luck next time...");
            break;
        } else if &position.tl == "O_" && &position.cc == "O_" && &position.br == "O_" {
            println!("I win...Better luck next time...");
            break;
        } else if &position.tl == "O_" && &position.cl == "O_" && &position.bl == "O_" {
            println!("I win...Better luck next time...");
            break;
        } else if &position.tc == "O_" && &position.cc == "O_" && &position.bc == "O_" {
            println!("I win...Better luck next time...");
            break;
        } else if &position.tr == "O_" && &position.cc == "O_" && &position.bl == "O_" {
            println!("I win...Better luck next time...");
            break;
        } else if &position.tr == "O_" && &position.cr == "O_" && &position.br == "O_" {
             println!("I win...Better luck next time...");
             break;
        } else if &position.cl == "O_" && &position.cc == "O_" && &position.cr == "O_" {
            println!("I win...Better luck next time...");
            break;
        } else if &position.bl == "O_" && &position.bc == "O_" && &position.br == "O_" {
            println!("I win...Better luck next time...");
            break;
        }
   }
    println!("_{}_|_{}_|_{}_", position.tl, position.tc, position.tr);
    println!("_{}_|_{}_|_{}_", position.cl, position.cc, position.cr);
    println!("_{}_|_{}_|_{}_", position.bl, position.bc, position.br);
}