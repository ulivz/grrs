pub fn bar() {
    use indicatif::ProgressBar;

    let pb = ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn do_hard_work() {
    for i in 0..1000000 {}
}
