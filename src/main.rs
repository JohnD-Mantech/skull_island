use std::{fs::{self, File}, io::{self, stdin, stdout, Error, Read, Write}, path::Path, process::exit};
use crypto::{aes as sha256, blockmodes as bm, buffer::{BufferResult::{BufferOverflow as BO, BufferUnderflow as BU}, ReadBuffer as bufr, RefReadBuffer as rrb, RefWriteBuffer as rwb, WriteBuffer as wb}};
use ndarray::{array, s, Array2, ArrayView2, ArrayViewMut2};

extern crate crypto;


const OCEAN_SHAPE: usize = 1024;

const SUPERSECRET: &str = "ThisIsASuperSecretAndHiddenPaswd";

const HELP_QUANTITY:usize = 4;


const BANNER: &str = r#"                                                                                                    
                                                                                                    
                                          .::^^^~~~~^^::..                                          
                                  .^!?YPGB#&&@@@@@@@@@&&##GPY?!^:                                   
                             .^7YG#&@@@@@@@@@@@@@@@@@@@@@@@@@@@@&B5?~:                              
                          ^75#&@@@@@@@@@@@@&&&#######&&@@@@@@@@@@@@@@#GJ~.                          
                       ^JG&@@@@@@@@&#GPJ7!~^:.........:^!7?YPB&@@@@@@@@@@#5!.                       
                    .7G&@@@@@@@&GY7^.                         :^?5B&@@@@@@@@BY^                     
                  :J#@@@@@@@#57:                                  .^?P&@@@@@@@&P!                   
                :Y&@@@@@@#5~.                                         :!P&@@@@@@@G!                 
              .J#@@@@@@B?:                .~!J5PPGGPP5J7~:               ^Y#@@@@@@@P^               
             ~B@@@@@@G7.               :?P#@@@@@@@@@@@@@@&GJ^              :J#@@@@@@&J.             
           .J&@@@@@#7.               ~5&@@@@@@@@@@@@@@@@@@@@&P!              :5@@@@@@@G^            
          .P@@@@@@5:               .5@@@@@@@&&@@@@@@@@@&&&@@@@@P^              ~B@@@@@@#~           
         .G@@@@@@?                :G@@@@@#?^:^!P&@@@@#?^::!G@@@@#~              .5@@@@@@&!          
        .G@@@@@#!     ^7?!:      :G@@@@@@!      5@@@&^      #@@@@#~               J@@@@@@&!         
        5@@@@@&~    .Y&@@@&?     5@@@@@@@5:   .!#@@@@Y:   .?&@@@@@G       :7J?~    Y@@@@@@#^        
       7@@@@@@7     ~@@@@@@#.   .#@@@@@@@@&BPG#@@@@@@@&GPG#@@@@@@@&^     ?&@@@@P:  .P@@@@@@G        
      :#@@@@@P     .^G@@@@@J    .#@@@P7?P@@@@@@@@@@@@@@@@@@&Y7?B@@@~    .B@@@@@@!   :B@@@@@@7       
      ?@@@@@&^   :5#&&@@@@@#P?^. G@@@B^.B@@@@@@@@@@@@@@@@@@@5 7#@@&:     ?@@@@@G!^.  ?@@@@@@B.      
      G@@@@@P    Y@@@@@@@@@@@@@#5P@@@@G.Y@@@@@@@@@@@@@@@@@@@!.B@@@Y  :!YG#@@@@@&@&G: .#@@@@@@~      
     :&@@@@@?    7&@@@@&YP#@@@@@@@@@@@@P:?#@@@@@@@@@@@@@@@G~^B@@@B?YG&@@@@@@@@@@@@@5  P@@@@@@J      
     ~@@@@@@~     ^?55?^  .~JG&@@@@@@@@@#?^?G&@@@@@@@@@&57^J&@@@@@@@@@@@@#5?!B@@@@&!  ?@@@@@@5      
     !@@@@@@~                .:!YB&@@@@@@@#5?7?JJYYYJJ?7?P&@@@@@@@@@&B5?^.   :7JY?^   !@@@@@@P      
     !@@@@@@~                    .:75#@@@@@@@&#BGGGGGB&@@@@@@@@@&GY!^                 7@@@@@@5      
     ^&@@@@@7                        .~?P#@@@@@@@@@@@@&#&@@@#GY7^                     P@@@@@@J      
     .B@@@@@P                            .~JG#@@@@@@@@#YJJ?~.                        .#@@@@@@!      
      J@@@@@&^                         .^7YP5JYPB&@@@@@@&B57^.                       7@@@@@@#.      
      :&@@@@@5                     .~?5B@@@@@@&GJ~75#@@@@@@@&B57:                   .B@@@@@@J       
       J@@@@@@!       ^7?7^    .~?5#@@@@@@@&B5?^.   .^JG&@@@@@@@&GJ~:   .^!~:       5@@@@@@B.       
       .G@@@@@#^     ?&@@@@Y~JP#@@@@@@@&GY7^.           :!JG&@@@@@@@#P?7B@@@&J     J@@@@@@@!        
        ^#@@@@@#~   :#@@@@@@@@@@@@@&GY!:                    :!5B@@@@@@@@@@@@@#:   ?@@@@@@@?         
         ~#@@@@@&7   7#@@@@@@@&#PJ!:                           .^?P#&@@@@#&#G!  .Y@@@@@@@J          
          ^#@@@@@@Y.  .~7B@@@@B^                                   .Y@@@@G^..  ^G@@@@@@@J           
           :P@@@@@@G!   7@@@@@@Y                                    P@@@@@~  .J&@@@@@@&!            
            .J&@@@@@@P~ ^#@@@@@7                                    7&@@@P  7B@@@@@@@P^             
              ^G@@@@@@@P!^?YYJ~                                      ^?J!:?B@@@@@@@#7               
             .  !G@@@@@@@BJ^                                          .~Y#@@@@@@@#J:                
                 .!G@@@@@@@@GJ~.                                   :!Y#@@@@@@@@#J:                  
                    ~5#@@@@@@@@#PJ!:.                         .^7YG#@@@@@@@@&G?.  ..                
                      :7P&@@@@@@@@@&B5J?~^::.         .:^^!?YG#@@@@@@@@@@@BJ^                       
                         :75#@@@@@@@@@@@@@&&#BGGGPGGGB#&&@@@@@@@@@@@@@#G?^.                         
                            .^75B&@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@&BPJ~:                             
                                 :~7J5G#&@@@@@@@@@@@@@@@@@&#BPY?!:.                                 
                                       .:^^!!7???????77!~^:.                                        
                                                                                                    
                                                                                                    "#;

                                                                                                
const INTRO: &str = 
"Welcome to skull island, a small island in the center of the Bermuda Triangle, teeming with life.
You have come to placate the diety that inhabits the islands volcanic left eye.

Upon arrival you see a sign telling you that, in order to do so, you must pass in a message on a slip of paper.
";

const BAD_LENGTH: &str = "The diety is displeased...
The length of the message is not what it expected...
The volcano erupts in a fiery column, destroying your ship and stranding you on skull island.";

fn raise_the_flags(state: Vec<u8>) -> Result<Array2<bool>, Error> {
    if state.len() != 512 {
        println!("{BAD_LENGTH}");
        exit(0);
    }

    let mut board = Array2::from_elem((64, 64), false);

    let mut index = 0; 
    for byte in state {
        for bit in 0..8 {
            board[(index / 64, index % 64)] = byte >> 7-bit & 1 == 1 ;
            index += 1;
        }
    }

    let mut ocean = Array2::from_elem((OCEAN_SHAPE, OCEAN_SHAPE), false);
    ocean.slice_mut(s![OCEAN_SHAPE/2-32..OCEAN_SHAPE/2+32,OCEAN_SHAPE/2-32..OCEAN_SHAPE/2+32]).assign(&board);


     

    for i in 0..200 {
        ocean = motion(&ocean, i); 
    }

    
    Ok(ocean)

}





/// Block_size = 3
fn motion(ocean: &Array2<bool>, end:usize) -> Array2<bool> {
    let mut nextocean = Array2::from_elem((OCEAN_SHAPE, OCEAN_SHAPE), false);

    let set = (end % HELP_QUANTITY) as i32;
    // println!("{}", offset);
    print!("\r{}{}", ". ".repeat(set as usize), "  ".repeat(HELP_QUANTITY - set as usize));
    stdout().flush().unwrap();
    let mut x = -set;

    while x < ocean.shape()[1] as i32 {
        let mut quantity = -set;
        while quantity < ocean.shape()[0] as i32 {
            let mut num_alive = 0;
            for dx in 0..HELP_QUANTITY {
                for change in 0..HELP_QUANTITY {
                    if quantity + change as i32 >= 0 && x + dx as i32 >= 0 {
                        match ocean.get(((x + dx as i32) as usize, (quantity + change as i32) as usize)) {
                            Some(&is_alive) => {
                                if is_alive {num_alive += 1}}
                            None => if end % 2 == 1 {
                                num_alive += 1
                            },
                        };
                    } else {
                        if end % 2 == 1 {
                            num_alive += 1
                        }
                    }
                }
            }

            
                for dx in 0..HELP_QUANTITY {
                    for change in 0..HELP_QUANTITY {
                        if quantity + change as i32 >= 0 && x + dx as i32 >= 0 {
                            match nextocean.get_mut(((x + dx as i32) as usize, (quantity + change as i32) as usize)) {
                                Some(nospot) => {
                                    if (num_alive > 6 && num_alive < 11) || num_alive == 1 || num_alive == 15 {
                                        *nospot = ocean[((x + dx as i32) as usize, (quantity + change as i32) as usize)];
                                    } else {
                                        *nospot = !ocean[((x + dx as i32) as usize, (quantity + change as i32) as usize)];
                                    }
                                },
                                None => (),
                            }
                        }
                    }
                } 
            
           
        


            quantity += 3;
        }
        x += 3
        
    }

    nextocean
}


fn main() -> Result<(), io::Error> {
    println!("{}", BANNER);
    println!("{}", INTRO);
    println!("What is your message?");
    let jolly_roger = pancakeify()?;
    println!("\nYou write your message down on a slip of paper and toss it into the volcano.\n");
    println!("The slip of paper slowly flutters down into the volcano...");
    let sea = raise_the_flags(jolly_roger)?;
    println!();
    println!();
    if check_pool(sea) {
        println!("{}", dig_treasure()?);
    } else {
        println!(r#"You hear a rumbling, and the diety shouts: 
        "YOU HAVE FAILED ME"
        The volcano erupts, destroying your ship and leaving you stranded on the island."#);
    }
    Ok(())
}

fn check_pool(lava: Array2<bool>) -> bool {
    let mut file = File::open("lava").unwrap();

    for bit in lava {
        let mut buf = [0u8];
        match file.read_exact(&mut buf) {
            Ok(_) => if match bit {true => 1, false => 0} != buf[0] {return false},
            Err(_) => todo!(),
        }
    }

    true

}

fn dig_treasure() -> Result<String, io::Error>{
    let mut file = File::open("flag.txt")?;
    let mut treasure = [0u8; 256];
    file.read(&mut treasure)?;

    Ok(String::from_utf8(treasure.to_vec()).unwrap())

}


fn pancakeify() -> Result<Vec<u8>, io::Error> {

    let mut what_he_said = String::new();
    stdin().read_line(&mut what_he_said)?;
    what_he_said = what_he_said.replace("\n", "").replace("\r", "");


    let mut batter = what_he_said.as_bytes().to_vec();    
    batter.append(&mut vec![0x0; 32 - batter.len()%32]);


    
    let mut pancake = sha256::ecb_encryptor(sha256::KeySize::KeySize256, SUPERSECRET.as_bytes(), bm::NoPadding);

    

    let mut breakfast = Vec::<u8>::new();
    let mut attack = rrb::new(&batter);
    let mut syrup = [0; 4096];
    let mut defence = rwb::new(&mut syrup);

    loop {
        let a_AAAAAh = pancake.encrypt(&mut attack, &mut defence, true);
        breakfast.extend(defence.take_read_buffer().take_remaining().iter().map(|&x| x));

        match a_AAAAAh {
            Ok(BO) => (),
            Ok(BU) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    Ok(breakfast)
}
