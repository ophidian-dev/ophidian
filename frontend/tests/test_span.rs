use frontend::span::Span;

#[test]
// test the joining function for span joins spans correctly
fn test_span_join() {
    let span1 = Span::new(3, 4);
    let span2 = Span::new(2, 5);

    assert_eq!(span1.join(span2), Span::new(2, 5));
}

#[test]
fn test_span_join1() {
    let span1 = Span::new(3, 5);
    let span2 = Span::new(4, 4);

    assert_eq!(span1.join(span2), Span::new(3, 5));
}
