initSidebarItems({"constant":[["CAPTURE_STRUCT_LOCAL",""],["COMMON_VTABLE_ENTRIES",""],["COMMON_VTABLE_ENTRIES_ALIGN",""],["COMMON_VTABLE_ENTRIES_DROPINPLACE",""],["COMMON_VTABLE_ENTRIES_SIZE",""],["INNERMOST",""]],"enum":[["AdtKind",""],["AssocItemContainer",""],["AssocKind",""],["BindingMode",""],["BorrowKind",""],["BoundConstness",""],["BoundRegionKind",""],["BoundTyKind",""],["BoundVariableKind",""],["ClosureKind","Represents the various closure traits in the language. This will determine the type of the environment (`self`, in the desugaring) argument that the closure expects."],["ConstKind","Represents a constant in Rust."],["ExistentialPredicate",""],["FloatTy",""],["GenericParamDefKind",""],["ImplOverlapKind",""],["ImplPolarity",""],["InferConst","An inference variable for a const, for use in const generics."],["InferTy","A placeholder for a type that hasn’t been inferred yet."],["InstanceDef",""],["IntTy",""],["IntVarValue",""],["PredicateKind",""],["RegionKind","Representation of regions. Note that the NLL checker uses a distinct representation of regions. For this reason, it internally replaces all the regions with inference variables – the index of the variable is then used to index into internal NLL data structures. See `rustc_const_eval::borrow_check` module for more information."],["TyKind","Defines the kinds of types."],["UintTy",""],["UpvarCapture","Information describing the capture of an upvar. This is computed during `typeck`, specifically by `regionck`."],["UpvarSubsts",""],["UserType","A user-given type annotation attached to a constant. These arise from constants that are named via paths, like `Foo::<A>::new` and so forth."],["ValTree","This datastructure is used to represent the value of constants used in the type system."],["Variance",""],["VarianceDiagInfo","Extra information about why we ended up with a particular variance. This is only used to add more information to error messages, and has no effect on soundness. While choosing the ‘wrong’ `VarianceDiagInfo` may lead to confusing notes in error messages, it will never cause a miscompilation or unsoundness."],["VarianceDiagMutKind",""],["VariantDiscr",""],["Visibility",""],["VtblEntry",""]],"fn":[["ast_int_ty",""],["ast_uint_ty",""],["float_ty",""],["int_ty",""],["is_ancestor_or_same_capture","Return true if the `proj_possible_ancestor` represents an ancestor path to `proj_capture` or `proj_possible_ancestor` is same as `proj_capture`, assuming they both start off of the same root variable."],["is_impl_trait_defn","Yields the parent function’s `DefId` if `def_id` is an `impl Trait` definition."],["place_to_string_for_capture",""],["provide",""],["suggest_arbitrary_trait_bound",""],["suggest_constraining_type_param","Suggest restricting a type param with a new bound."],["uint_ty",""]],"mod":[["_match",""],["adjustment",""],["binding",""],["cast",""],["codec","This module contains some shared code for encoding and decoding various things from the `ty` module, and in particular implements support for “shorthands” which allow to have pointers back into the already encoded stream instead of re-encoding the same thing twice."],["error",""],["fast_reject",""],["flags",""],["fold","Generalized type folding mechanism. The setup is a bit convoluted but allows for convenient usage. Let T be an instance of some “foldable type” (one which implements `TypeFoldable`) and F be an instance of a “folder” (a type which implements `TypeFolder`). Then the setup is intended to be:"],["inhabitedness",""],["layout",""],["normalize_erasing_regions","Methods for normalizing when you don’t care about regions (and aren’t doing type inference). If either of those things don’t apply to you, use `infcx.normalize(...)`."],["print",""],["query",""],["relate","Generalized type relating mechanism."],["subst",""],["tls",""],["trait_def",""],["util","Miscellaneous type-system utilities that are too small to deserve their own modules."],["vtable",""],["walk","An iterator over the type substructure. WARNING: this does not keep track of the region depth."]],"struct":[["AdtDef","The definition of a user-defined type, e.g., a `struct`, `enum`, or `union`."],["AdtFlags",""],["AdtSizedConstraint",""],["AssocItem",""],["AssocItems","A list of `ty::AssocItem`s in definition order that allows for efficient lookup by name."],["Binder","Binder is a binder for higher-ranked lifetimes or types. It is part of the compiler’s representation for things like `for<'a> Fn(&'a isize)` (which would be represented by the type `PolyTraitRef == Binder<'tcx, TraitRef>`). Note that when we instantiate, erase, or otherwise “discharge” these bound vars, we change the type from `Binder<'tcx, T>` to just `T` (see e.g., `liberate_late_bound_regions`)."],["BoundConst",""],["BoundRegion",""],["BoundTy",""],["BoundVar",""],["CReaderCacheKey",""],["CanonicalUserTypeAnnotation",""],["CaptureInfo","Part of `MinCaptureInformationMap`; describes the capture kind (&, &mut, move) for a particular capture as well as identifying the part of the source code that triggered this capture to occur."],["CapturedPlace","A composite describing a `Place` that is captured by a closure."],["ClosureSizeProfileData",""],["ClosureSubsts","A closure can be modeled as a struct that looks like:"],["ClosureSubstsParts","Struct returned by `split()`."],["CoercePredicate","Encodes that we have to coerce from the `a` type to the `b` type."],["Const","Typed constant value."],["ConstInt","A type for representing any integer. Only used for printing."],["ConstVid","A `const` variable ID."],["ConstnessAnd",""],["CrateInherentImpls","A map for the local crate mapping each type to a vector of its inherent impls. This is not meant to be used outside of coherence; rather, you should request the vector for a specific type via `tcx.inherent_impls(def_id)` so as to minimize your dependencies (constructing this map requires touching the entire crate)."],["CratePredicatesMap","The crate outlives map is computed during typeck and contains the outlives of every item in the local crate. You should not use it directly, because to do so will make your pass dependent on the HIR of every item in the local crate. Instead, use `tcx.inferred_outlives_of()` to get the outlives for a particular item."],["CrateVariancesMap","The crate variances map is computed during typeck and contains the variance of every item in the local crate. You should not use it directly, because to do so will make your pass dependent on the HIR of every item in the local crate. Instead, use `tcx.variances_of()` to get the variance for a particular item."],["CtxtInterners",""],["DebruijnIndex","A De Bruijn index is a standard means of representing regions (and perhaps later types) in a higher-ranked setting. In particular, imagine a type like this:"],["DelaySpanBugEmitted","A type that is not publicly constructable. This prevents people from making [`TyKind::Error`]s except through the error-reporting functions on a [`tcx`][TyCtxt]."],["Destructor",""],["EarlyBoundRegion",""],["ExistentialProjection","A `ProjectionPredicate` for an `ExistentialTraitRef`."],["ExistentialTraitRef","An existential reference to a trait, where `Self` is erased. For example, the trait object `Trait<'a, 'b, X, Y>` is:"],["FieldDef",""],["FloatVarValue",""],["FloatVid","An floating-point (`f32` or `f64`) type variable ID."],["FnSig","Signature of a function type, which we have arbitrarily decided to use to refer to the input/output types."],["FoundRelationships",""],["FreeRegion","A “free” region `fr` can be interpreted as “some region at least as big as the scope `fr.scope`”."],["FreeRegionInfo",""],["GenSig",""],["GeneratorInteriorTypeCause","Whenever a value may be live across a generator yield, the type of that value winds up in the `GeneratorInteriorTypeCause` struct. This struct adds additional information about such captured types that can be useful for diagnostics. In particular, it stores the span that caused a given type to be recorded, along with the scope that enclosed the value (which can be used to find the await that the value is live across)."],["GeneratorSubsts","Similar to `ClosureSubsts`; see the above documentation for more."],["GeneratorSubstsParts",""],["GenericParamCount",""],["GenericParamDef",""],["GenericPredicates","Bounds on generics."],["Generics","Information about the formal type/lifetime parameters associated with an item or method. Analogous to `hir::Generics`."],["GlobalCtxt",""],["ImplHeader","The “header” of an impl is everything outside the body: a Self type, a trait ref (in the case of a trait impl), and a set of predicates (from the bounds / where-clauses)."],["InlineConstSubsts","An inline const is modeled like"],["InlineConstSubstsParts","Struct returned by `split()`."],["Instance","A monomorphized `InstanceDef`."],["InstantiatedPredicates","Represents the bounds declared on a particular set of type parameters. Should eventually be generalized into a flag list of where-clauses. You can obtain an `InstantiatedPredicates` list from a `GenericPredicates` by using the `instantiate` method. Note that this method reflects an important semantic invariant of `InstantiatedPredicates`: while the `GenericPredicates` are expressed in terms of the bound type parameters of the impl/trait/whatever, an `InstantiatedPredicates` instance represented a set of bounds for some particular instantiation, meaning that the generic parameters have been substituted with their values."],["IntVid","An integral (`u32`, `i32`, `usize`, etc.) type variable ID."],["List","A wrapper for slices with the additional invariant that the slice is interned and no other slice with the same contents can exist in the same context. This means we can use pointer for both equality comparisons and hashing."],["MainDefinition",""],["OpaqueTypeKey",""],["OutlivesPredicate",""],["ParamConst",""],["ParamEnv","When type checking, we use the `ParamEnv` to track details about the set of where-clauses that are in scope at this particular point."],["ParamEnvAnd",""],["ParamTy",""],["Placeholder","The “placeholder index” fully defines a placeholder region, type, or const. Placeholders are identified by both a universe, as well as a name residing within that universe. Distinct bound regions/types/consts within the same universe simply have an unknown relationship to one another."],["Predicate",""],["ProjectionPredicate","This kind of predicate has no direct correspondent in the syntax, but it roughly corresponds to the syntactic forms:"],["ProjectionTy","Represents the projection of an associated type. In explicit UFCS form this would be written `<T as Trait<..>>::N`."],["RegionVid","A region (lifetime) variable ID."],["ReprFlags",""],["ReprOptions","Represents the repr options provided by the user,"],["ResolverOutputs",""],["ScalarInt","The raw bytes of a simple value."],["SubtypePredicate","Encodes that `a` must be a subtype of `b`. The `a_is_expected` flag indicates whether the `a` type is the type that we should label as “expected” when presenting user diagnostics."],["SymbolName",""],["TraitDef","A trait’s definition with type information."],["TraitObjectVisitor","Collect al types that have an implicit `'static` obligation that we could suggest `'_` for."],["TraitPredicate",""],["TraitRef","A complete reference to a trait. These take numerous guises in syntax, but perhaps the most recognizable form is in a where-clause:"],["TyCtxt","The central data structure of the compiler. It stores references to the various arenas and also houses the results of the various compiler queries that have been performed. See the rustc dev guide for more details."],["TyS",""],["TyVid","A type variable ID."],["TypeAndMut",""],["TypeFlags","Flags that we track on types. These flags are propagated upwards through the type during type construction, so that we can quickly check whether the type has various kinds of types in it without recursing over the type itself."],["TypeckResults",""],["Unevaluated","An unevaluated, potentially generic, constant."],["UniverseIndex","“Universes” are used during type- and trait-checking in the presence of `for<..>` binders to control what sets of names are visible. Universes are arranged into a tree: the root universe contains names that are always visible. Each child then adds a new set of names that are visible, in addition to those of its parent. We say that the child universe “extends” the parent universe with new names."],["UpvarBorrow",""],["UpvarId","Upvars do not get their own `NodeId`. Instead, we use the pair of the original var ID (that is, the root variable that is referenced by the upvar) and the ID of the closure expression."],["UpvarPath",""],["UserTypeAnnotationIndex",""],["VariantDef","Definition of a variant – a struct’s fields or an enum variant."],["VariantFlags",""],["WithOptConstParam","A `DefId` which, in case it is a const argument, is potentially bundled with the `DefId` of the generic parameter it instantiates."]],"trait":[["DefIdTree",""],["Lift","A trait implemented for all `X<'a>` types that can be safely and efficiently converted to `X<'tcx>` as long as they are part of the provided `TyCtxt<'tcx>`. This can be done, for example, for `Ty<'tcx>` or `SubstsRef<'tcx>` by looking them up in their respective interners."],["OnDiskCache",""],["ToPolyTraitRef",""],["ToPredicate",""],["TypeFoldable","This trait is implemented for every type that can be folded. Basically, every type that has a corresponding method in `TypeFolder`."],["TypeFolder","The `TypeFolder` trait defines the actual folding. There is a method defined for every foldable type. Each of these has a default implementation that does an “identity” fold. Within each identity fold, it should invoke `foo.fold_with(self)` to fold each sub-item."],["TypeVisitor",""],["WithConstness",""]],"type":[["Attributes",""],["CanonicalPolyFnSig",""],["CanonicalUserType","Canonicalized user type annotation."],["CanonicalUserTypeAnnotations","Mapping of type annotation indices to canonical user type annotations."],["MinCaptureInformationMap","Given the closure DefId this map provides a map of root variables to minimum set of `CapturedPlace`s that need to be tracked to support all captures of that closure."],["MinCaptureList","Part of `MinCaptureInformationMap`; List of `CapturePlace`s."],["PlaceholderConst",""],["PlaceholderRegion",""],["PlaceholderType",""],["PolyCoercePredicate",""],["PolyExistentialProjection",""],["PolyExistentialTraitRef",""],["PolyFnSig",""],["PolyGenSig",""],["PolyProjectionPredicate",""],["PolyRegionOutlivesPredicate",""],["PolySubtypePredicate",""],["PolyTraitPredicate",""],["PolyTraitRef",""],["PolyTypeOutlivesPredicate",""],["Region",""],["RegionOutlivesPredicate",""],["RootVariableMinCaptureList","Part of `MinCaptureInformationMap`; Maps a root variable to the list of `CapturedPlace`. Used to track the minimum set of `Place`s that need to be captured to support all Places captured by the closure starting at a given root variable."],["Ty",""],["TypeOutlivesPredicate",""],["UpvarCaptureMap",""],["UpvarListMap",""]]});