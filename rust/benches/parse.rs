use std::str::FromStr;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn read_input() -> Vec<String> {
    let data = include_str!("data.txt");

    data.lines()
        .take(1000)
        .map(String::from)
        .collect::<Vec<_>>()
}

fn twitch(c: &mut Criterion) {
    let input = read_input();
    c.bench_with_input(
        BenchmarkId::new("twitch", "data.txt (whitelist)"),
        &input,
        |b, lines| {
            b.iter_with_setup(
                || lines.clone(),
                |lines| {
                    for line in lines {
                        black_box(
                            twitch::Message::parse_with_whitelist(
                                line,
                                twitch::whitelist!(TmiSentTs, UserId),
                            )
                            .expect("failed to parse"),
                        );
                    }
                },
            );
        },
    );
    c.bench_with_input(
        BenchmarkId::new("twitch", "data.txt (no whitelist)"),
        &input,
        |b, lines| {
            b.iter_with_setup(
                || lines.clone(),
                |lines| {
                    for line in lines {
                        black_box(twitch::Message::parse(line).expect("failed to parse"));
                    }
                },
            );
        },
    );
}

fn twitch_irc(c: &mut Criterion) {
    let input = read_input();
    c.bench_with_input(
        BenchmarkId::new("twitch_irc", "data.txt"),
        &input,
        |b, lines| {
            b.iter(|| {
                for line in lines.clone() {
                    black_box(
                        twitch_irc::message::IRCMessage::parse(&line).expect("failed to parse"),
                    );
                }
            })
        },
    );
}

fn irc_rust(c: &mut Criterion) {
    let input = read_input();
    c.bench_with_input(
        BenchmarkId::new("irc_rust", "data.txt"),
        &input,
        |b, lines| {
            b.iter(|| {
                for line in lines.clone() {
                    black_box(
                        irc_rust::Message::from_str(&line)
                            .expect("failed to parse")
                            .parse()
                            .expect("failed to parse"),
                    );
                }
            })
        },
    );
}

criterion_group!(benches, twitch, twitch_irc, irc_rust);
criterion_main!(benches);
