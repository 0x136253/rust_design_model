//因为Option实现了IntoIterator特性，它就可以用来当.extend()的参数:
//如果你需要将一个Option添加到已有的迭代器后面，你可以用 .chain():
fn main() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);

    // equivalent to
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    let turing = Some("Turing");
    let logicians = vec!["Curry", "Kleene", "Markov"];

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }
}
