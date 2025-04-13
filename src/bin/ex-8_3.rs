// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company; for example, "Add Saily to
// Engineering" or "Add Amir to Sales." Then let the user retrieve a list of all
// people in a department or all people in the company by department, sorted
// alphabetically.

use getting_rusty::scanner::Scanner;

fn main() {
    let lexemes = Scanner::new("    ").scan();
    println!("{:?}", lexemes);
}
