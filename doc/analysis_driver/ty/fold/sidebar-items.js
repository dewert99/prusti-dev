initSidebarItems({"fn":[["shift_region",""],["shift_vars",""]],"struct":[["BottomUpFolder",""],["RegionFolder","Folds over the substructure of a type, visiting its component types and all regions that occur free within it."],["ValidateBoundVars",""]],"trait":[["TypeFoldable","This trait is implemented for every type that can be folded. Basically, every type that has a corresponding method in `TypeFolder`."],["TypeFolder","The `TypeFolder` trait defines the actual folding. There is a method defined for every foldable type. Each of these has a default implementation that does an “identity” fold. Within each identity fold, it should invoke `foo.fold_with(self)` to fold each sub-item."],["TypeVisitor",""]]});