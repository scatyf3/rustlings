fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(x) = option{ //单个解构不要用for
        res += x;
    }
    println!("{res}");
}
