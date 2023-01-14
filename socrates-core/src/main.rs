mod config;
mod arguments;

fn main() {
    let str_args = arguments::get_str_args();
    let args = arguments::parse_args(&str_args);

    let path = {
        match args
            .iter()
            .find(|p| matches!(p, arguments::Args::Dir(_)))
            .expect("There should be a dir")
        {
            arguments::Args::Dir(s) => s,
            _ => panic!("Something has gone wrong"),
        }
    };

    let _config = config::Config::new(path);
}
