use std::io;

fn main(){
    let _ = d1_b();
}

fn d1_b() -> io::Result<()>{
    let mut tot = 0i32;
    for line in std::io::stdin().lines() {
        let mut first = -1i32;
        let mut sec = -1i32;

        let mut one = 0u8;
        let mut too = 0u8;
        let mut  three = 0u8;
        let mut  four = 0u8;
        let mut  five = 0u8;
        let mut  six = 0u8;
        let mut  seven = 0u8;
        let mut  eigth = 0u8;
        let mut  nine = 0u8;
        for c in line.unwrap().chars() {
            if c.is_digit(10) {
                if first < 0 {
                    first = c.to_digit(10).unwrap() as i32;
                }
                else{
                    sec = c.to_digit(10).unwrap() as i32;
                }
            }
            else{
                match c {
                    'w' =>{
                        one = 0;
                        if too == 1 { too = 2}
                        else { too = 0 }
                        three = 0;
                        four = 0;
                        five = 0;
                        six = 0;
                        seven = 0;
                        eigth = 0;
                        nine = 0;

                    }
                    'o' =>{
                        one = 1;
                        if too == 2 {
                            if first < 0 { first = 2}
                            else { sec = 2}
                        }
                        too = 0;
                        three = 0;
                        if four == 1 { four = 2 }
                        five = 0;
                        six = 0;
                        seven = 0;
                        eigth = 0;
                        nine = 0;
                    }
                    'n' => {
                        if one == 1 { one += 1 }
                        else { one = 0 }
                        too = 0;
                        three = 0;
                        four = 0;
                        five = 0;
                        six = 0;
                        if seven == 4 { 
                            if first < 0 { first = 7}
                            else { sec = 7}
                            seven = 0
                        }
                        else { seven = 0 }
                        eigth = 0;
                        if nine == 2 { nine += 1 }
                        else { nine = 1 }
                        
                    },
                    'e' => {
                        if one == 2 {
                            if first < 0 { first = 1}
                            else { sec = 1}
                            one = 0
                        }
                        else { one = 0 }
                        too = 0;
                        if three == 2 { three += 1}
                        else if three == 3 {
                            if first < 0 { first = 3}
                            else { sec = 3}
                            three = 0
                        }
                        else { three = 0 }
                        four = 0;
                        if five == 3{
                            if first < 0 { first = 5}
                            else { sec = 5}
                            five = 0
                        }
                        else {five = 0}
                        six = 0;
                        if seven == 1 || seven == 3 { seven += 1 }
                        else { seven = 0 }
                        eigth = 1;
                        if nine == 3 {
                            if first < 0 { first = 9}
                            else { sec = 9}
                            nine = 0
                        }
                        else { nine = 0}
                    }
                    't' =>{
                        one = 0;
                        too = 1;
                        three = 1;
                        four = 0;
                        five = 0;
                        six = 0;
                        seven = 0;
                        if eigth == 4 {
                            if first < 0 { first = 8}
                            else { sec = 8}
                        }
                        eigth = 0;

                        if eigth == 3 { eigth = 4 }
                        else { eigth = 0 }
                        nine = 0;
                    }
                    'h' => {
                        one = 0;
                        too = 0;
                        if three == 1 { three = 2 }
                        else { three = 0 }
                        four = 0;
                        five = 0;
                        six = 0;
                        seven = 0;
                        if eigth == 3 { eigth = 4 }
                        else { eigth = 0 }
                        nine = 0;
                    }
                    'r' => {
                        one = 0;
                        too = 0;
                        if three == 2 { three = 3 }
                        else { three = 0}
                        if four == 3 {
                            if first < 0 { first = 4}
                            else { sec = 4}
                            four = 0
                        }
                        else {four = 0}
                        five = 0;
                        six = 0;
                        seven = 0;
                        eigth = 0;
                        nine = 0;
                    }
                    'f' =>{
                        one = 0;
                        too = 0;
                        three = 0;
                        four = 1;
                        five = 1;
                        six = 0;
                        seven = 0;
                        eigth = 0;
                        nine = 0;
                    }
                    'u' =>{
                        one = 0;
                        too = 0;
                        three = 0;
                        if four == 2 { four = 3}
                        else { four = 0}
                        five = 0;
                        six = 0;
                        seven = 0;
                        eigth = 0;
                        nine = 0;
                    }
                    'i' =>{
                        one = 0;
                        too = 0;
                        three = 0;
                        four = 0;
                        if five == 1 { five = 2}
                        else { five = 0}
                        if six == 1 { six = 2}
                        else { six = 0}
                        seven = 0;
                        if eigth == 1 { eigth = 2}
                        else { eigth = 0}
                        if nine == 1 { nine = 2}
                        else { nine = 0}
                    }
                    'v' => {
                        one = 0;
                        too = 0;
                        three = 0;
                        four = 0;
                        if five == 2 { five = 3 }
                        else { five = 0}
                        six = 0;
                        if seven == 2 { seven = 3 }
                        else { seven = 0}
                        eigth = 0;
                        nine = 0;
                    }
                    's' => {
                        one = 0;
                        too = 0;
                        three = 0;
                        four = 0;
                        five = 0;
                        six = 1;
                        seven = 1;
                        eigth = 0;
                        nine = 0;
                    }            
                    'x' => {
                        one = 0;
                        too = 0;
                        three = 0;
                        four = 0;
                        five = 0;
                        if six == 2{
                            if first < 0 { first = 6}
                            else { sec = 6}
                        }
                        six = 0;
                        seven = 0;
                        eigth = 0;
                        nine = 0;
                    }
                    'g' => {
                        one = 0;
                        too = 0;
                        three = 0;
                        four = 0;
                        five = 0;
                        six = 0;
                        seven = 0;
                        if eigth == 2{ eigth = 3}
                        else {eigth = 0}
                        nine = 0;
                    }
                    _ => {
                        one = 0;
                        too = 0;
                        three = 0;
                        four = 0;
                        five = 0;
                        six = 0;
                        seven = 0;
                        eigth = 0;
                        nine = 0;
                     }
                }
        
            }
        }
        if sec > 0{
            tot += first *10 + sec;
        }
        else{
            tot += first *10 +first;
        }
        //print!("{} {}\n", first, sec);
    }
    print!("{}", tot);

    Ok(())
}
fn d1_a() -> io::Result<()>{
    let mut tot = 0i32;
    for line in std::io::stdin().lines() {
        let mut first = -1i32;
        let mut sec = -1i32;
        for c in line.unwrap().chars() {
            if c.is_digit(10) {
                if first < 0 {
                    first = c.to_digit(10).unwrap() as i32;
                }
                else{
                    sec = c.to_digit(10).unwrap() as i32;
                }
            }
        }
        if sec > 0{
            tot += first *10 + sec;
        }
        else{
            tot += first *10 +first;
        }
    }
    print!("{}", tot);

    Ok(())
}
