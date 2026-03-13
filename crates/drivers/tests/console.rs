use domain::Rgb;
use drivers::Console;

#[test]
fn new_initializes_pixels_to_black() {
    let console = Console::new(5);
    assert!(console.as_ref().iter().all(|p| *p == Rgb::BLACK));
}

#[test]
fn pixel_count_matches_constructor_arg() {
    let console = Console::new(10);
    assert_eq!(console.as_ref().len(), 10);
}

#[test]
fn pixels_mut_allows_writing() {
    let mut console = Console::new(3);
    console.as_mut()[1] = Rgb::new(255, 0, 0);
    assert_eq!(console.as_ref()[1], Rgb::new(255, 0, 0));
    assert_eq!(console.as_ref()[0], Rgb::BLACK);
}
