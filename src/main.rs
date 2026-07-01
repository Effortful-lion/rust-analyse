fn main() {
    let mut args = std::env::args().skip(1);

    if args.next().is_none() {
        eprintln!("用法: grammar-analyse <输入文件> [输出目录]");
        std::process::exit(1);
    }

    println!("{}", grammar_analyse::version_banner());
}
