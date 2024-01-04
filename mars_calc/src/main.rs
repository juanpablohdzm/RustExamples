use std::io;

fn main()
{
    let mut input  = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight :f32  = input.trim().parse().unwrap();
    dbg!(weight);

    borrow_string(&input);
    own_string(input);

    calculate_weight_on_mars(weight);

}

fn borrow_string(s: &String)
{
    println!("{}", s);
}

fn own_string(s: String)
{
    println!("{}", s);
}



fn second_example()
{
    let mut input : String = String::new();
    some_fn(&mut input); // moving input 

    let s1 = &input;
    let s2 = &input;
    println!("{} {}",s1, s2); // this can be done because the compiler knows that the value will be the same before changing it


    io::stdin().read_line(&mut input); // borrowing
    // println!("{} {}",s1, s2); //can't be done because it has been changed 
}

// & like this is borrowing (e.g. const &)
// &mut is not const 
fn some_fn(s: &mut String) {
    s.push_str("a");
}

fn first_example() {
    println!("Hello this is a console for weight on mars"); // This is a macro not a function, we know because of the "!"
    println!("Weight on mars {}",calculate_weight_on_mars(90.0));
    let mut mars_weight: f32 = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on mars 100kg {}", mars_weight);
}

fn calculate_weight_on_mars(weight : f32) -> f32
{
    return (weight/9.81)*3.711;
}

// Each value in rust is owned by a variable

// There can only be one owner at a time