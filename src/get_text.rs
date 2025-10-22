use io::std;
pub fn get_text() -> String {
    println!("Enter text message: ");
    let mut text_msg = String::new();
    io::stdin().read_line(&mut text_msg);
    return text_msg;
}
