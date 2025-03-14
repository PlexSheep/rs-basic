// most of this is stolen from the indicatif examples (also MIT)

use std::cmp::min;
use std::fmt::Write;
use std::thread::{self, sleep_ms};
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressIterator, ProgressState, ProgressStyle};
use rand::Rng;

fn main() {
    // b0();
    // b1();
    // b2();
    b3();
    b4();
    b5();
    b6();
}

fn b0() {
    let bar = ProgressBar::new(1_000);
    for _ in 0..1_000 {
        bar.inc(1);
        sleep_ms(1);
    }
    bar.finish();
}

fn b1() {
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    for _ in 0..1_000 {
        bar.inc(1);
        sleep_ms(1);
    }
    bar.finish();
}

fn b2() {
    for _ in (0..1_000).progress() {
        sleep_ms(1);
    }
}

fn b3() {
    let bar = ProgressBar::new(1_000);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("▄▌_"),
    );
    for _ in 0..1_000 {
        bar.inc(1);
        sleep_ms(1);
    }
    bar.finish();
}

fn b4() {
    use indicatif::{HumanBytes, HumanCount, HumanDuration, HumanFloatCount};

    assert_eq!("3.00 MiB", HumanBytes(3 * 1024 * 1024).to_string());
    assert_eq!(
        "8 seconds",
        HumanDuration(Duration::from_secs(8)).to_string()
    );
    assert_eq!("33,857,009", HumanCount(33857009).to_string());
    assert_eq!(
        "33,857,009.1235",
        HumanFloatCount(33857009.123456).to_string()
    );
}

fn b5() {
    let mut downloaded = 0;
    let total_size = 231331;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}

fn b6() {
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let n = 20;
    let pb = m.add(ProgressBar::new(n));
    pb.set_style(sty.clone());
    pb.set_message("todo");
    let pb2 = m.add(ProgressBar::new(n));
    pb2.set_style(sty.clone());
    pb2.set_message("finished");

    let pb3 = m.insert_after(&pb2, ProgressBar::new(1024));
    pb3.set_style(sty);

    m.println("starting!").unwrap();

    let mut threads = vec![];

    let m_clone = m.clone();
    let h3 = thread::spawn(move || {
        for i in 0..1024 {
            thread::sleep(Duration::from_millis(2));
            pb3.set_message(format!("item #{}", i + 1));
            pb3.inc(1);
        }
        m_clone.println("pb3 is done!").unwrap();
        pb3.finish_with_message("done");
    });

    for i in 0..n {
        thread::sleep(Duration::from_millis(15));
        if i == n / 3 {
            thread::sleep(Duration::from_secs(2));
        }
        pb.inc(1);
        let m = m.clone();
        let pb2 = pb2.clone();
        threads.push(thread::spawn(move || {
            let spinner = m.add(ProgressBar::new_spinner().with_message(i.to_string()));
            spinner.enable_steady_tick(Duration::from_millis(100));
            thread::sleep(
                rand::thread_rng().gen_range(Duration::from_secs(1)..Duration::from_secs(5)),
            );
            pb2.inc(1);
        }));
    }
    pb.finish_with_message("all jobs started");

    for thread in threads {
        let _ = thread.join();
    }
    let _ = h3.join();
    pb2.finish_with_message("all jobs done");
    m.clear().unwrap();
}
