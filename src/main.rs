pub fn get_bottle_text(n: u32) -> String {
    if n > 1 {
        String::from("bottles")
    } else {
        String::from("bottle")
    }
}

pub fn verse(n: u32) -> String {
    let no_bottles_text = "Go to the store and buy some more, 99 bottles of beer on the wall.\n";
    if n == 0 {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\n") + no_bottles_text
    } else if n == 1 {
        String::from("1 bottle of beer on the wall, 1 bottle of beer.") + "\n" + "Take it down and pass it around, no more bottles of beer on the wall.\n"
    } else {
        n.to_string() + " " + &get_bottle_text(n) + " of beer on the wall, " + &n.to_string() + " " + &get_bottle_text(n) + " of beer." + "\n" + "Take one down and pass it around, " + &(n - 1).to_string() + " " + &get_bottle_text(n - 1) + " of beer on the wall.\n"
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = String::new();
    let mut new_start = start;
    let new_end = if end != 0 {
        end - 1
    } else {
        end
    };

    while new_start > new_end  {
        res += &verse(new_start);
        if new_start != end {
            res += "\n";
        }
        new_start -= 1;
    }

    if new_start == 0 {
        res += &verse(0);
    }
    res
}

fn main() {
    assert_eq!(verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    assert_eq!(verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    assert_eq!(verse(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    assert_eq!(verse(8), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");

    assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
    assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}
