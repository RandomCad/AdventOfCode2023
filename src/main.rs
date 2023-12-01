use std::io;

fn main(){
    let _ = d1_a();
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
