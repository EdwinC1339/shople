pub mod game;
pub mod loading;

fn main() {
    let municipalities = loading::get_municipalities().expect("Couldn't load municipalities");
    println!("Loaded {} municipalities...", municipalities.len());
    println!("Head:");
    municipalities
        .into_iter()
        .take(5)
        .for_each(|municipality| println!("{:?}", municipality));

    let vieques_retail = loading::get_retail_data("vieques").unwrap();

    println!("{:?}", vieques_retail)
}
