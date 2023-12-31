use std::io;

fn main(){
    let _ = d9_a();
}

fn d9_a() -> io::Result<()>{
    let mut total_a = 0i64;
    let mut total_b = 0i64;
    for line in std::io::stdin().lines(){
        let zwi = line.unwrap();
        let parts = zwi.split(' ').collect::<Vec<&str>>();
        let mut numbers =Vec::<Vec::<i32>>::new(); 
        numbers.push(Vec::<i32>::new());
        for i in parts{
            numbers[0].push(i.parse().unwrap());
        }
        let mut cnt = 0usize;
        while (!numbers[cnt].iter().all(|&x| x== 0)) && numbers[cnt].len() > 1{
            numbers.push(Vec::<i32>::new());
            let zwi: Vec::<i32> = numbers[cnt].clone();
            for i in 0..numbers[cnt].len() -1{
                numbers[cnt + 1].push(zwi[i + 1] - zwi[i]);
            }
            cnt += 1;
        }
        
        let mut cnt = (numbers.len() - 2) as i32;
        while cnt >= 0{
            let i = cnt as usize;
            let num = numbers[i][numbers[i].len()-1] + numbers[i + 1][numbers[i + 1].len() - 1];
            print!("{}", num);
            numbers[i].push(num);
            cnt -= 1;
        }
        print!("{:?}\n\n",numbers);
        total_a += numbers[0][numbers[0].len() - 1] as i64;
        let mut num = 0i32;
        cnt = (numbers.len() - 2) as i32;
        while cnt >= 0{
            let i = cnt as usize;
            num = numbers[i][0]-num;
            cnt -= 1;
        }
        total_b += num as i64;
    }
    print!("\ntotal A: {}\ntotal B: {}", total_a, total_b);
    Ok(())
}

fn d3_b() -> io::Result<()>{
    const card_num : usize = 220usize;
    let mut card_amount: [u64;card_num] = [1;card_num];
    let mut _cnt = 0usize;
    for line in std::io::stdin().lines(){
        let mut zwi = line.unwrap();
        while zwi.remove(0) != ':' {}
        let parts = zwi.split('|').collect::<Vec<&str>>();
        let front = parts[0].split(' ').collect::<Vec<&str>>();
        let back = parts[1].split(' ').collect::<Vec<&str>>();
        //parse the back:
        let mut cor_num = Vec::<u32>::new();
        for p in back{
            let zwi = p.parse();
            if let Ok(v) = zwi { cor_num.push(v) }
        }
        let mut num = 0u16;
        for i in front{
            let zwi = i.parse::<u32>();
            if let Ok(v) = zwi {
                if cor_num.contains(&v) {
                    num += 1;
                }
            }
        }
        for i in 1..num + 1{
            card_amount[_cnt + (i as usize)] += card_amount[_cnt];
        }

        _cnt += 1;
    }
    let mut total = 0u128;
    for i in 0..card_num { total += card_amount[i] as u128 }
    print!("\n{}", total);
    Ok(())
}

fn d3_a() -> io::Result<()>{
    let mut total = 0u64;
    for line in std::io::stdin().lines(){
        let mut zwi = line.unwrap();
        while zwi.remove(0) != ':' {}
        let parts = zwi.split('|').collect::<Vec<&str>>();
        let front = parts[0].split(' ').collect::<Vec<&str>>();
        let back = parts[1].split(' ').collect::<Vec<&str>>();
        //parse the back:
        let mut cor_num = Vec::<u32>::new();
        for p in back{
            let zwi = p.parse();
            if let Ok(v) = zwi { cor_num.push(v) }
        }
        let mut num = 0x01u16;
        for i in front{
            let zwi = i.parse::<u32>();
            if let Ok(v) = zwi {
                if cor_num.contains(&v) {
                    num <<= 1;
                }
            }
        }
        total += (num >> 1) as u64;
    }
    print!("{}", total);
    Ok(())
}

fn d2() -> io::Result<()>{
    let mut i = 0u32;
    let mut count = 0u32;
    let mut cnt2 = 0u32;
    for line in std::io::stdin().lines(){
        let mut zwi = line.unwrap();
        while zwi.remove(0) != ':' {}
        i += 1;
        let mut found = false;
        let mut min_b = 0u32;
        let mut min_g = 0u32;
        let mut min_r = 0u32;
        for split in zwi.split(';') {
            for v in split.split(','){
                //print!("{}", v);
                let parts = v.split(' ').collect::<Vec<&str>>();
                //print!("{}\n", parts[0]);
                let num: u32 = parts[1].parse().unwrap();
                match parts[2]{
                    "red" => {
                        if num > 12 { found = true }
                        if num > min_r { min_r = num}
                    }
                    "green" =>{
                        if num > 13 { found = true }
                        if num > min_g {min_g = num }
                    }
                    "blue" =>{
                        if num >14 { found = true }
                        if num > min_b {min_b = num }
                    }
                    _ => {}
                }
                //print!("{}", found);
            }
        }
        if ! found { count += i}
        print!("{} {} {}\n", min_r, min_g, min_b);
        cnt2 += min_r * min_g * min_b;
    }
    let mut _n = 0u32;
    for e in 1..100 { _n += e}
    print!("PartA: {}\nPartB: {}", count, cnt2);
    Ok(())
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
