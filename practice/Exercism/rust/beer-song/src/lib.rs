pub fn verse(n: u32) -> String {    
    if n == 0 {
        let zero_bottle = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
        return zero_bottle.to_string();        
    } else if n == 1 {
        let abottle = 
    format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n);
        return abottle;
    } else if n == 2 {
        let abottle = 
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1);
        return abottle;
    } else {
    let bottles = 
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1);
        return bottles; 
    }

}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    
    for i in (end..=start).rev() {            
        result = result + &verse(i);
        if i != end {
            result = result + "\n";
        }
    }
    
    result
}
