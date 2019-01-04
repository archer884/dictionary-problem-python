use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
};

static TEACHERS: &[&str] = &[
    "Blake",
    "Cori",
    "Philip",
];

static CLASSES: &[&str] = &[
    "Underwater basket weaving",
    "How to train your dragon",
    "World domination for dummies",
];

fn main() {
    let hours = Uniform::from(1..4);
    let classes = (0..20)
        .map(|id| (
            id + 1,
            *TEACHERS.choose(&mut rand::thread_rng()).unwrap(),
            *CLASSES.choose(&mut rand::thread_rng()).unwrap(),
            hours.sample(&mut rand::thread_rng()),
        ));

    for (id, teacher, class, hours) in classes {
        println!("{},{},{},{}", id, teacher, class, hours);
    }
}
