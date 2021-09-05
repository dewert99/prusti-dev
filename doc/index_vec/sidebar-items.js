initSidebarItems({"macro":[["define_index_type","Generate the boilerplate for a newtyped index struct, for use with `IndexVec`."],["define_index_type","Generate the boilerplate for a newtyped index struct, for use with `IndexVec`."],["index_box","A macro similar to the stdlib’s `vec![]`, but producing an `Box<IndexSlice<I, [T]>>` (That is, an `IndexBox<I, [T]>`)."],["index_vec","A macro equivalent to the stdlib’s `vec![]`, but producing an `IndexVec`."]],"struct":[["IndexSlice","A slice that only accepts indices of a specific type. Note that the intended usage is as `IndexSlice<I, [T]>`."],["IndexVec","A Vec that only accepts indices of a specific type."]],"trait":[["Idx","Represents a wrapped value convertable to and from a `usize`."],["IdxRangeBounds","This trait to function in API signatures where `Vec<T>` or `[T]` use `R: RangeBounds<usize>`. There are blanket implementations for the basic range types in `core::ops` for all Idx types. e.g. `Range<I: Idx>`, `RangeFrom<I: Idx>`, `RangeTo<I: Idx>`, etc all implement it."],["IdxSliceIndex","This is the equivalent of the sealed `core::slice::SliceIndex` trait. It cannot be overridden from user, code nor should it normally need use directly (Outside of trait bounds, I guess)."]],"type":[["IndexBox","`IndexBox<I, [T]>`: An alias for indexed boxed slice."]]});