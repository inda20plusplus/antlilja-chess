use super::*;

#[test]
fn empty() {
    let empty = TaggedPiece::empty();
    assert_eq!(empty.0, 0);
}

#[test]
fn get_type() {
    fn piece_type_persists(r#type: PieceType) {
        assert_eq!(
            TaggedPiece::original(r#type, Color::White).get_type(),
            r#type
        );
        assert_eq!(
            TaggedPiece::original(r#type, Color::Black).get_type(),
            r#type
        );
    }

    piece_type_persists(PieceType::Pawn);
    piece_type_persists(PieceType::Rook);
    piece_type_persists(PieceType::Knight);
    piece_type_persists(PieceType::Bishop);
    piece_type_persists(PieceType::Queen);
    piece_type_persists(PieceType::King);
}

#[test]
fn get_color() {
    fn color_persists(r#type: PieceType) {
        assert_eq!(
            TaggedPiece::original(r#type, Color::White).color(),
            Color::White
        );
        assert_eq!(
            TaggedPiece::original(r#type, Color::Black).color(),
            Color::Black
        );
    }

    color_persists(PieceType::Pawn);
    color_persists(PieceType::Rook);
    color_persists(PieceType::Knight);
    color_persists(PieceType::Bishop);
    color_persists(PieceType::Queen);
    color_persists(PieceType::King);
}

#[test]
fn from_str() {
    fn test_original(s: &str, piece: PieceType) {
        assert_eq!(
            TaggedPiece::from_str(s).ok().unwrap(),
            TaggedPiece::original(piece, Color::White)
        );

        assert_eq!(
            TaggedPiece::from_str(&s.to_ascii_lowercase()).ok().unwrap(),
            TaggedPiece::original(piece, Color::Black)
        );
    }

    fn test_non_original(s: &str, piece: PieceType) {
        assert_eq!(
            TaggedPiece::from_str(s).ok().unwrap(),
            TaggedPiece::new(piece, Color::White)
        );

        assert_eq!(
            TaggedPiece::from_str(&s.to_ascii_lowercase()).ok().unwrap(),
            TaggedPiece::new(piece, Color::Black)
        );
    }

    assert_eq!(
        TaggedPiece::from_str(".").ok().unwrap(),
        TaggedPiece::empty()
    );

    test_original("P*", PieceType::Pawn);
    test_original("R*", PieceType::Rook);
    test_original("N*", PieceType::Knight);
    test_original("B*", PieceType::Bishop);
    test_original("Q*", PieceType::Queen);
    test_original("K*", PieceType::King);

    test_non_original("P", PieceType::Pawn);
    test_non_original("R", PieceType::Rook);
    test_non_original("N", PieceType::Knight);
    test_non_original("B", PieceType::Bishop);
    test_non_original("Q", PieceType::Queen);
    test_non_original("K", PieceType::King);
}
