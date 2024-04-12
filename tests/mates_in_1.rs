use ladybug::ladybug::Message;

mod common;

#[test]
fn puzzle_1() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/8/1Q6/8/7B/2R4N/5K1P/k7 w - - 11 70".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove c3a3"));
}

#[test]
fn puzzle_2() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/8/pppppppK/NBBR1NRp/nbbrqnrP/PPPPPPPk/8/Q7 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove a1h1"));
}

#[test]
fn puzzle_3() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 2k5/1p3R2/p2Bp3/P3P3/4bP2/2P3n1/4B2r/6K1 b - - 1 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));
    
    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove h2g2"));
}

#[test]
fn puzzle_4() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/R5p1/5p1p/4r1k1/6P1/5KP1/8/8 w - - 1 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove a7g7"));
}

#[test]
fn puzzle_5() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 2r3k1/1Q4p1/4p2p/8/p4P2/1n5P/1B3KP1/1q6 w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove b7g7"));
}

#[test]
fn puzzle_6() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 5b1k/5p2/3ppN1p/2n3p1/2P3P1/2n1P2P/2Q2P1B/4qBK1 w - - 1 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove c2h7"));
}

#[test]
fn puzzle_7() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/5p1p/2p2kp1/p1P5/B2p1P2/P5Pb/1P2RK1P/3r4 b - - 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove d1f1"));
}

#[test]
fn puzzle_8() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/7r/8/k1B5/2K5/8/8/1R6 w - - 1 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove b1a1"));
}

#[test]
fn puzzle_9() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/2p5/p1k5/1pP1K3/1P1Qp3/P6q/5P2/8 w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove d4d5"));
}

#[test]
fn puzzle_10() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 4N1k1/5p2/6p1/p5Q1/2p4P/3n2P1/5PbK/q7 b - - 1 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 1".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove a1h1"));
}