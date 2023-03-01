use rand::Rng;

//create an const array of the top 10 best movies around the world
pub const MOVIES: [&str; 10] = [
    "New York",
    "Tokyo",
    "Vancouver",
    "Rome",
    "Sydney",
    "Dubai",
    "Singapore",
    "Paris",
    "Beijing",
    "Shanghai",
];

//create a function that returns a random movie in the list above
pub fn random_movie() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..MOVIES.len());
    MOVIES[random_index]
}
