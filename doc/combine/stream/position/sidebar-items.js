initSidebarItems({"struct":[["IndexPositioner","The `IndexPositioner<Item, Range>` struct maintains the current index into the stream `Input`.  The initial index is index 0.  Each `Item` committed increments the index by 1; each `range` committed increments the position by `range.len()`."],["SourcePosition","Struct which represents a position in a source file."],["Stream","The `Stream<Input>` struct maintains the current position in the stream `Input` using the `Positioner` trait to track the position."]],"trait":[["DefaultPositioned","Defines a default `Positioner` type for a particular `Stream` type."],["Positioner","Trait for tracking the current position of a `Stream`."],["RangePositioner","Trait for tracking the current position of a `RangeStream`."]]});