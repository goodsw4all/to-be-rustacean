use minesweeper::annotate;
use minesweeper::get_adjacent_list;

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn neghibour_list_coord_0x0() {
    let actual = get_adjacent_list((0, 0), (3, 3));
    let expected = vec![(1, 0), (0, 1), (1, 1)];
    assert_eq!(expected, actual);
}

#[test]
fn neghibour_list_coord_2x0() {
    let actual = get_adjacent_list((2, 0), (2, 2));
    let expected = vec![(1, 0), (1, 1), (2, 1)];
    println!("expected {:#?}", expected);
    assert_eq!(expected, actual);
}

#[test]
fn neghibour_list_coord_1x1() {
    let actual = get_adjacent_list((1, 1), (3, 3));
    let expected = vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (0, 1),
        (2, 1),
        (0, 2),
        (1, 2),
        (2, 2),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
#[ignore]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
#[ignore]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
#[ignore]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
#[ignore]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
#[ignore]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
#[ignore]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
#[ignore]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
#[ignore]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
#[ignore]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
#[ignore]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
#[ignore]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
