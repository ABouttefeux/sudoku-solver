(function() {var implementors = {
"sudoku":[["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/size/struct.GameSize.html\" title=\"struct sudoku::size::GameSize\">GameSize</a>"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/grid/struct.CellCoordinate.html\" title=\"struct sudoku::grid::CellCoordinate\">CellCoordinate</a>&lt;SQUARE_SIZE&gt;"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"sudoku/grid/enum.Direction.html\" title=\"enum sudoku::grid::Direction\">Direction</a>"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/cell/struct.Cell.html\" title=\"struct sudoku::cell::Cell\">Cell</a>&lt;SQUARE_SIZE&gt;<span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">{ _ }</a>]: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,</span>"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/grid/struct.Row.html\" title=\"struct sudoku::grid::Row\">Row</a>&lt;SQUARE_SIZE&gt;"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"sudoku/grid/enum.VerificationError.html\" title=\"enum sudoku::grid::VerificationError\">VerificationError</a>&lt;SQUARE_SIZE&gt;"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"sudoku/grid/enum.SolveError.html\" title=\"enum sudoku::grid::SolveError\">SolveError</a>&lt;SQUARE_SIZE&gt;"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/grid/struct.Column.html\" title=\"struct sudoku::grid::Column\">Column</a>&lt;SQUARE_SIZE&gt;"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"sudoku/grid/enum.VerificationResult.html\" title=\"enum sudoku::grid::VerificationResult\">VerificationResult</a>"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/grid/struct.Sudoku.html\" title=\"struct sudoku::grid::Sudoku\">Sudoku</a>&lt;SQUARE_SIZE&gt;<span class=\"where fmt-newline\">where\n    [[<a class=\"struct\" href=\"sudoku/cell/struct.Cell.html\" title=\"struct sudoku::cell::Cell\">Cell</a>&lt;SQUARE_SIZE&gt;; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">{ _ }</a>]; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">{ _ }</a>]: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    [[<a class=\"struct\" href=\"sudoku/cell/struct.Cell.html\" title=\"struct sudoku::cell::Cell\">Cell</a>&lt;SQUARE_SIZE&gt;; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">{ _ }</a>]; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">{ _ }</a>]: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,</span>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"sudoku/error/enum.ExampleError.html\" title=\"enum sudoku::error::ExampleError\">ExampleError</a>"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/grid/struct.Square.html\" title=\"struct sudoku::grid::Square\">Square</a>&lt;SQUARE_SIZE&gt;"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/cell/struct.CellNumber.html\" title=\"struct sudoku::cell::CellNumber\">CellNumber</a>&lt;SQUARE_SIZE&gt;"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/grid/struct.BackTracePositionTracker.html\" title=\"struct sudoku::grid::BackTracePositionTracker\">BackTracePositionTracker</a>&lt;SQUARE_SIZE&gt;"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"sudoku/error/enum.SetError.html\" title=\"enum sudoku::error::SetError\">SetError</a>"],["impl&lt;'de, const SQUARE_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.137/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"sudoku/grid/struct.CellPosition.html\" title=\"struct sudoku::grid::CellPosition\">CellPosition</a>&lt;SQUARE_SIZE&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()