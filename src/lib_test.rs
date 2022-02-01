use super::*;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick  three.\n\
        Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick three.\n\
        Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
