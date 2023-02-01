use std::env::current_dir;
use std::path::PathBuf;

pub trait Exercise: Send + Sync {
    fn solve(&self, exercise: String) -> String;

    fn read_input(&self, exercise: String) -> String {
        let current_dir = current_dir().unwrap();
        let path: PathBuf = current_dir
            .join("inputs")
            .join(exercise)
            .with_extension("txt");
        println!("Reading input from: {}", path.display());
        path.display().to_string()
    }
}
