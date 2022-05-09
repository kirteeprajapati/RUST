mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod structs;

fn main() {
    print:: run();
    vars::var();
    types::types();
    strings::strings();
    tuples::tup();
    arrays::arrays();
    vectors::vectors();
    conditionals::conditionals();
    structs::structs();
}
