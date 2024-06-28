use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    // define a command with clap builder
    let cmdtest = Command::new("test").arg(arg!(--foo <VALUE>).required(true));

    // instead of using the args that were actually given to the executable, we just insert our own
    // vec
    let simulated_input = vec!["test", "--foo", "5"];
    let parsed = cmdtest.try_get_matches_from(simulated_input);
    dbg!(&parsed);
}
