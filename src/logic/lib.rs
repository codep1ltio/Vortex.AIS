slint::include_modules!();

#[macro_export]
macro_rules! time {
    ($name:expr, $block:block) => {{
        let time = std::time::Instant::now();
        let result = $block;
        let dur = time.elapsed();
        println!(
            ">Task: \x1b[32m{}\x1b[0m\n>Time: \x1b[34m{:.1?}\x1b[0m",
            $name, dur
        );
        result
    }};
}

#[macro_export]
macro_rules! work_in_progress {
    () => {
        println!(
            "\nFILE: {}\nLINE: {} <- work in progress\n",
            file!(),
            line!(),
        );
    };

    ($text:expr) => {{
        println!(
            "\n{}\n^^ is still work in progress\nFILE: {}\nLINE: {}\n",
            $text,
            file!(),
            line!(),
        );
    }};
}
