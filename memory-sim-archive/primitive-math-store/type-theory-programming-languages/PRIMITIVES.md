# Type Theory and Programming Language Primitives

> Domain: Type Theory and Programming Languages — formal systems for program specification, proof, abstraction, and compositional reasoning about computation.

## Primitive Catalog

---

### TT.001: Simply Typed Lambda Calculus (STLC)
**Definition:** Core calculus with types → (function) and base types; variables, abstractions, applications.
**Cost Model:** Type-checking is O(n) for term of size n.
**Real Wall:** Church encodings enable booleans, naturals, products as terms.
**Cross-Domain Aliases:** stlc, lambda_types, church_stlc
**Notes:** Normalization: every well-typed term reduces to a normal form.

---

### TT.002: Type Inference (Hindley-Damas-Milner)
**Definition:** Principal type inference for let-polymorphic lambda calculus.
**Cost Model:** O(n·W) where W is size of principal type; polynomial.
**Real Wall:** Unification handles type constraints; occurs check prevents infinite types.
**Cross-Domain Aliases:** hml, damas_milner, let_polymorphism
**Notes:** Basis for ML, Haskell, Rust lifetime inference.

---

### TT.003: Unification (First-Order)
**Definition:** Find substitution σ making two terms equal; occurs check prevents x = t[x].
**Cost Model:** O(n) per unification step; O(n) steps for well-behaved constraints.
**Real Wall:** Most general unifier (mgu) is unique up to renaming.
**Cross-Domain Aliases:** unification, term_unification, mgu
**Notes:** Core of type inference; also used in logic programming (Prolog).

---

### TT.004: Principal Type Scheme
**Definition:** Most general type for a term; capture by ∀ quantified type variables.
**Cost Model:** Generalization: ∀α·τ if α not free in typing environment.
**Real Wall:** Type scheme prevents unsound polymorphism (impredicative ∀).
**Cross-Domain Aliases:** principal_type, type_scheme, poly_type
**Notes:** Let polymorphism: generalize type under ∴ but not under →.

---

### TT.005: Let-Binding Polymorphism
**Definition:** let x = t in u uses principal type of t; polymorphic in x.
**Cost Model:** Generalize at let; monomorphize at each use.
**Real Wall:** Contrast with var-binding: λx.t is monomorphic in x.
**Cross-Domain Aliases:** let_polymorphism, polymorphic_let
**Notes:** Standard in ML, Haskell, Scala; essential for code reuse.

---

### TT.006: Monomorphism Restriction
**Definition:** Restrict polymorphic bindings to single types when no type signature given.
**Cost Model:** Prevents duplicate work from generic instantiation.
**Real Wall:** Used in Haskell 98 to avoid ambiguity; relaxed in type families.
**Cross-Domain Aliases:** monomorphism, restricted_polymorphism
**Notes:** Defaulting rules solve most ambiguity automatically.

---

### TT.007: Subtyping (Structural)
**Definition:** σ ≤ τ if every term of type σ is also usable as type τ.
**Cost Model:** O(n) to check subtype relation on named records.
**Real Wall:** Width: adding fields is subtype; depth: strengthening fields.
**Cross-Domain Aliases:** subtyping, subtype_relation, width_depth
**Notes:** Structural vs nominal subtyping: name-based vs structure-based.

---

### TT.008: Variance (Covariance, Contravariance)
**Definition:** If T₁ ≤ T₂: T[F] is covariant if F preserves, contravariant if reverses order.
**Cost Model:** Check variance of type constructors; compile-time annotation.
**Real Wall:** Function: A₁ → B₁ ≤ A₂ → B₂ iff A₂ ≤ A₁ (contra) and B₁ ≤ B₂ (co).
**Cross-Domain Aliases:** variance, co_contra, covariant
**Notes:** Generic containers: List[T] is covariant; Function[-A,+B].

---

### TT.009: Bounded Polymorphism (F<:B)
**Definition:** Type variable α with upper bound B: α <: B means α is subtype of B.
**Cost Model:** Constraint solving with bound; can specify lower bounds too.
**Real Wall:** Enables methods on any subtype without full inheritance hierarchy.
**Cross-Domain Aliases:** bounded_quantifier, fsub, parametric_bounds
**Notes:** Essential for object-oriented type systems (Java generics, Scala).

---

### TT.010: Higher-Rank Polymorphism
**Definition:** Polymorphic functions passed as arguments; requires explicit type annotations.
**Cost Model:** Type inference undecidable at rank > 1; needs annotations.
**Real Wall:** ∀ cannot appear under → without annotation in Hindley-Milner.
**Cross-Domain Aliases:** higher_rank, rank_polymorphism, impredicative
**Notes:** System F has ∀ at any position; inference is complex.

---

### TT.011: System F (Second-Order Lambda Calculus)
**Definition:** λ→ extended with ∀ quantification over types; terms parameterized by types.
**Cost Model:** Type-checking with ∀ introduction/elimination rules.
**Real Wall:** Significantly more expressive than Hindley-Milner.
**Cross-Domain Aliases:** system_f, polymorphic_lambda
**Notes:** Encodes parametric polymorphism fully; not normalizing.

---

### TT.012: Impredicative Polymorphism
**Definition:** ∀ can quantify over types containing ∀; e.g., (∀α.α→α)→Bool.
**Cost Model:** Type comparison becomes complex; may require normalization.
**Real Wall:** System F is impredicative; inconsistent with excluded middle.
**Cross-Domain Aliases:** impredicative_poly, system_f
**Notes:** Paradoxes avoided by avoiding type:Type (no Type in itself).

---

### TT.013: Predicative Polymorphism
**Definition:** ∀α.T restricts α to smaller types; Stratified Type:Type levels.
**Cost Model:** Type levels: Type⁰ < Type¹ < Type²; predicative restriction.
**Real Wall:** MLTT, CoC use predicative hierarchy to avoid paradoxes.
**Cross-Domain Aliases:** predicative, cumulative_universe, universes
**Notes:** Russell-style stratification; Coq's Set, Prop, Type(ℓ).

---

### TT.014: Existential Types (Package/ unpack)
**Definition:** ∃α.T hides implementation type α; abstract data types.
**Cost Model:** Pack: give witness type τ; unpack: use with concrete type.
**Real Wall:** Corresponds to module signatures; ADT hiding.
**Cross-Domain Aliases:** existential, hide_type, abstract_type
**Notes:** Introduction: pack τ with witness; elimination: unpack with witness.

---

### TT.015: Dependent Types (Π, Σ Types)
**Definition:** Types depend on values; Πx:A.B(x) is functions, Σx:A.B(x) is pairs.
**Cost Model:** Type-checking requires evaluating type-level functions.
**Real Wall:** Can express precise invariants: Vec n (length n list).
**Cross-Domain Aliases:** dependent, pidot, type_depend_value
**Notes:** Core of proof assistants (Coq, Agda, Idris, Lean).

---

### TT.016: Lambda Cube (Barendregt's Cube)
**Definition:** Three axes: → (terms), ∀ (types), λ (dependent); 8 systems at corners.
**Cost Model:** Each corner has known properties: normalizability, type safety.
**Real Wall:** λ→ (STLC), λ2 (F), λP (Fω), λP2 (CP), etc.
**Cross-Domain Aliases:** lambda_cube, barendregt_cube, cube_systems
**Notes:** Foundation for understanding type system expressiveness.

---

### TT.017: Pure Type Systems (PTS)
**Definition:** Abstract framework parameterized by sorts, axioms, rules.
**Cost Model:** Generalizes λ cube; all well-studied systems are instances.
**Real Wall:** Unified treatment of type theory; meta-theory verified generically.
**Cross-Domain Aliases:** pts, pure_type_system
**Notes:** Specified by (S, A, R) where S=sorts, A=axioms, R=rules.

---

### TT.018: Linear Types (Affine, Relevant)
**Definition:** Resource types used exactly once (linear), at most once (affine).
**Cost Model:** Tracking resources; no duplication or discarding.
**Real Wall:** Memory management: mutable references, file handles.
**Cross-Domain Aliases:** linear_type, affine_type, resource_type
**Notes:** !A is exponential (can be used multiple times); linear logic foundation.

---

### TT.019: Session Types (Communication)
**Definition:** Protocol types for communication channels; linear types + sequencing.
**Cost Model:** Type-checked communication: send/receive must match protocol.
**Real Wall:** Can ensure no deadlocks in concurrent programs.
**Cross-Domain Aliases:** session_type, channel_type, protocol_type
**Notes:** Dual types: & (branch) and ⊕ (select); used in multiparty protocols.

---

### TT.020: Effect Types (Row Polymorphism)
**Definition:** Type + effect annotations: T { eff }; effect variables row-polymorphic.
**Cost Model:** Effect inference similar to type inference; extensible rows.
**Real Wall:** Tracks IO, exceptions, state, nondeterminism.
**Cross-Domain Aliases:** effect_row, effect_polymorphism, row_type
**Notes:** Koka, Eff, Frank use algebraic effect handlers.

---

### TT.021: Gradual Typing
**Definition:** Mix static and dynamic typing; ?A is maybe-typed; casts at boundaries.
**Cost Model:** Blame tracking pinpoints boundary where type error occurs.
**Real Wall:** Compromises: not fully static nor fully dynamic.
**Cross-Domain Aliases:** gradual_type, dynamic_static, soft_type
**Notes:** TypeScript, Python with mypy, Hack; untyped = ?.

---

### TT.022: Cast Insertion (Gradual)
**Definition:** Insert runtime casts between static/dynamic boundaries.
**Cost Model:** O(n) cast insertion; runtime cast checks may fail.
**Real Wall:** Cast from Int to Bool: runtime check; may raise error.
**Cross-Domain Aliases:** cast_insertion, gradual_annotations
**Notes:** Blame calculus: assigns responsibility for type errors.

---

### TT.023: Type Soundness (Progress + Preservation)
**Definition:** Progress: well-typed term is value or steps; Preservation: steps preserve type.
**Cost Model:** Prove by induction on typing derivation + step relation.
**Real Wall:** Subject reduction: t : T and t → t' implies t' : T.
**Cross-Domain Aliases:** type_safety, soundness_theorem
**Notes:** Standard type soundness theorem (Wright, Felleisen).

---

### TT.024: Algorithmic Typing (Bidirectional)
**Definition:** Separate checking (synthesize type for term) and inference (check term against type).
**Cost Model:** O(n) for bidirectional; avoids constraint generation.
**Real Wall:** Allows local type inference without full constraint solving.
**Cross-Domain Aliases:** bidirectional_tc, checking_inference
**Notes:** Modern systems: e.g., λ→ with bidirectional rules.

---

### TT.025: Normalization by Evaluation (NbE)
**Definition:** Evaluate in semantic domain, then reify to normal form.
**Cost Model:** O(n) type comparison after normalization; efficient.
**Real Wall:** Used in proof assistants for term comparison.
**Cross-Domain Aliases:** nbe, semantic_normalization
**Notes:** Reduces type checking to evaluation + reification.

---

### TT.026: Strong Normalization
**Definition:** Every well-typed term reduces to a normal form in finite steps.
**Cost Model:** Proved by reducibility candidates or logical relations.
**Real Wall:** STLC is strongly normalizing; System F is not (can encode recursion).
**Cross-Domain Aliases:** strong_normalization, sn_proof
**Notes:** Key property: no infinite reduction sequences; consistency.

---

### TT.027: Church Encoding (Data as Lambdas)
**Definition:** Encode booleans, naturals, products as λ-terms: true = λa.λb.a.
**Cost Model:** Church numerals: c₀ = λf.λx.x; cₙ = λf.λx.fⁿx.
**Real Wall:** Works in untyped λ-calculus; inefficient (β-reduction).
**Cross-Domain Aliases:** church_encoding, lambda_encoding
**Notes:** Basis of Scott encoding; Scott more efficient for recursion.

---

### TT.028: Scott Encoding
**Definition:** Encode algebraic data types as functions: list = λnil.λcons.x.
**Cost Model:** More efficient than Church: no extra β-reductions.
**Real Wall:** Can encode recursive types directly (lazy self-application).
**Cross-Domain Aliases:** scott_encoding, lazy_encoding
**Notes:** Used in polymorphic lambda calculi with fix.

---

### TT.029: Recursion (Fix-Point Combinator)
**Definition:** fix = λf.(λx.f(xx))(λx.f(xx)); enables general recursion.
**Cost Model:** Y f = f (Y f) by β-reduction; can diverge.
**Real Wall:** Enables loops and general recursion in typed setting.
**Cross-Domain Aliases:** fixpoint, y_combinator, recursive_term
**Notes:** Requires type systems to be normalizing to avoid inconsistency.

---

### TT.030: Least Fixed Point (Types)
**Definition:** μX.F[X] is least solution to X = F[X]; recursive types.
**Cost Model:** Isorecursive: fold/unfold are explicit casts.
**Real Wall:** Enables recursive data structures: List = μX.1+X×X.
**Cross-Domain Aliases:** recursive_type, isorecursive, mu_type
**Notes:** Equirecursive: X = F[X] implicitly; more convenient but complex.

---

### TT.031: Induction (Natural Numbers)
**Definition:** ℕ has constructors 0 : ℕ, succ : ℕ → ℕ; induction principle.
**Cost Model:** recursor computes from constructors; structural recursion.
**Real Wall:** Eliminator: rec_Nat : (C 0) → ((n:C)C → C) → C.
**Cross-Domain Aliases:** nat_induction, recursor_nat, inductive_nat
**Notes:** Type-theoretic induction built into eliminators.

---

### TT.032: Elimination Rules (Tait's Method)
**Definition:** Prove reducibility of terms via candidate sets.
**Cost Model:** Reducibility candidates: CR0-3; prove by induction on type.
**Real Wall:** Used for strong normalization proofs.
**Cross-Domain Aliases:** tait_method, reducibility_candidates
**Notes:** Foundation for normalization proofs in type theory.

---

### TT.033: Logical Relation
**Definition:** Semantic interpretation of types as relations on terms.
**Cost Model:** Prove properties by induction on type structure.
**Real Wall:** Strong normalization, type safety, parametricity proofs.
**Cross-Domain Aliases:** logical_relation, parametricity
**Notes:** Reynolds' abstraction theorem: parametric functions respect relations.

---

### TT.034: Parametricity (Wadler's Theorem)
**Definition:** Polymorphic functions behave uniformly across all types.
**Cost Model:** Every ∀α.f : ∀α.T → T has a uniform implementation.
**Real Wall:** Theorems about polymorphic programs without seeing code.
**Notes:** "Theorems for free": e.g., length : ∀α.List α → Nat is parametric.

---

### TT.035: Natural Deduction (Propositional)
**Definition:** Rules for ∧, ∨, →, ⊥; introduction and elimination.
**Cost Model:** Each connective has I-rule (introduction) and E-rule (elimination).
**Real Wall:** Corresponds to intuitionistic logic (no proof by contradiction).
**Cross-Domain Aliases:** natural_deduction, propositional_logic
**Notes:** Foundation of type theory: proofs are terms, types are propositions.

---

### TT.036: Sequent Calculus
**Definition:** Γ ⊢ Δ: from antecedents Γ prove consequents Δ.
**Cost Model:** Structural rules (weakening, contraction, exchange).
**Real Wall:** Gentzen's 1934 formulation; proof search more systematic.
**Cross-Domain Aliases:** sequent_calculus, gentzen_calculus
**Notes:** LJ (intuitionistic) restricts to single conclusion.

---

### TT.037: Curry-Howard Isomorphism
**Definition:** Propositions ≅ Types; Proofs ≅ Terms; Computation ≅ Proof reduction.
**Cost Model:** Constructive logic ↔ λ-calculus; classical ↔ control operators.
**Real Wall:** ⊥ = empty type; ¬A = A → ⊥.
**Cross-Domain Aliases:** ch_isomorphism, proofs_as_programs
**Notes:** Foundational: connects logic and computation.

---

### TT.038: De Morgan Laws (Classical)
**Definition:** ¬(A∧B) ↔ (¬A∨¬B); ¬(A∨B) ↔ (¬A∧¬B).
**Cost Model:** Intuitionistically only one direction holds.
**Real Wall:** Classical logic requires excluded middle: A∨¬A.
**Cross-Domain Aliases:** de_morgan, classical_logic
**Notes:** Classical ↔ Classical (call/cc) via continuation encoding.

---

### TT.039: Double-Negation Translation
**Definition:** Translate classical logic to intuitionistic by ¬¬ translations.
**Cost Model:** Gödel-Gentzen: [[A]] replaces ¬¬ and ∨.
**Real Wall:** Preserves provability: A is classically provable ↔ translated is intuitionistically provable.
**Cross-Domain Aliases:** godel_gentzen, nnf_translation
**Notes:** Shows intuitionistic logic is sufficient for classical arithmetic.

---

### TT.040: Call/cc (Control Operator)
**Definition:** Capture current continuation; abort to any saved point.
**Cost Model:** CCC: types are functions; call/cc : ((A → ⊥) → A) → A.
**Real Wall:** Can encode Peirce's law → excluded middle.
**Cross-Domain Aliases:** callcc, control, abort
**Notes:** Griffin, Felleisen: classical proofs ↔ programs with control.

---

### TT.041: Classical Logic as Continuation-Passing
**Definition:** Translate classical to CPS: negate continuations; double-negation elimination.
**Cost Model:** A ⊢ B ↔ A ⊢ k: (B → ⊥) → ⊥.
**Real Wall:** Classical type theory via control operators.
**Cross-Domain Aliases:** cps_translation, classical_cps
**Notes:** Danvy, Filinski: CBN/CBV as delimited continuations.

---

### TT.042: Identity Type (Path Induction)
**Definition:** Id_A(a,b) with constructor refl : Id_A(a,a); J eliminator.
**Cost Model:** J: equality reflection; transport along paths.
**Real Wall:** Uniqueness of identity proofs (UIP) not provable in HoTT.
**Cross-Domain Aliases:** identity_type, id_type, path_type
**Notes:** HoTT: identity is higher-dimensional; homotopy 1.0 = path.

---

### TT.043: Homotopy Type Theory (HoTT)
**Definition:** Types as ∞-groupoids; paths as identity proofs; higher equalities.
**Cost Model:** Univalence: equivalent types are equal; Voevodsky's axiom.
**Real Wall:** Univalence + UIP is inconsistent; must give up UIP.
**Cross-Domain Aliases:** hott, homotopy_level, univalence
**Notes:** Book HoTT: foundations of mathematics via ∞-toposes.

---

### TT.044: Univalence Axiom
**Definition:** (A ≃ B) → (A = B); equivalent types are propositionally equal.
**Cost Model:** Transport along equivalence is definitional equality.
**Real Wall:** Enables formalization of mathematical library univalence.
**Cross-Domain Aliases:** univalence, equivalence_equal
**Notes:** Cannot be derived in CIC; added as new primitive.

---

### TT.045: Higher Inductive Types (HIT)
**Definition:** Inductive types with path and higher-path constructors.
**Cost Model:** Circle S¹: base : S¹; loop : Id(base, base).
**Real Wall:** New definitional equalities from constructors.
**Cross-Domain Aliases:** hit, higher_inductive, quotient_type
**Notes:** Enables quotients, truncations, suspensions, etc.

---

### TT.046: Quotient Types
**Definition:** A/~ collapses equivalence classes; function extensionality needed.
**Cost Model:** introduction: quotient A R; elimination: f must be constant on classes.
**Real Wall:** Definitional equality replaced by propositional equality.
**Cross-Domain Aliases:** quotient, set quot, collapse_eqclass
**Notes:** HIT encoding: quotient by providing path constructors.

---

### TT.047: Function Extensionality
**Definition:** (∀x. f x = g x) → f = g; often added as axiom.
**Cost Model:** Propositional: equality of functions is proposition.
**Real Wall:** With univalence, implies uniqueness of identity proofs.
**Cross-Domain Aliases:** funext, fun_ext, eta_expansion
**Notes:** Eta expansion for functions: (λx.fx) = f.

---

### TT.048: Canonicity (Normalization)
**Definition:** Every closed term of type Nat is a Church numeral.
**Cost Model:** Strong normalization + decidability of type checking implies canonicity.
**Real Wall:** Important for consistency: can compute with terms.
**Cross-Domain Aliases:** canonicity, canonical_forms
**Notes:** CIC is normalizing but not confluent; canonicity fails for impredicative Prop.

---

### TT.049: Consistency (Logical)
**Definition:** No term of type ⊥ (empty type); cannot prove false.
**Cost Model:** Via normalization: ⊥ has no closed normal form.
**Real Wall:** Mutual consistency of type theory with arithmetic.
**Cross-Domain Aliases:** consistency, non_contradiction
**Notes:** Consistency = normalization + soundness of type system.

---

### TT.050: Relative Consistency
**Definition:** Prove theory T consistent relative to theory S.
**Cost Model:** Interpret T in S; if S consistent, T consistent.
**Real Wall:** ZFC + inaccessible → CIC consistency.
**Cross-Domain Aliases:** relative_consistency, consistency_proof
**Notes:** Proof of CIC relative to ZFC set theory.

---

### TT.051: Observational Type Theory (OTT)
**Definition:** Equality is observational (computable); supports canonicity + univalence.
**Cost Model:** Equality reflection: observational equality can compute.
**Real Wall:** Mixes intensional and extensional views.
**Cross-Domain Aliases:** ott, observational_equality
**Notes:** Chapman, Altenkirch; aims to combine HoTT and canonicity.

---

### TT.052: Intensional Type Theory (ITT)
**Definition:** Equality is definitional (erasable); types have computational content.
**Cost Model:** Type checking decidable; equality is proof-relevant.
**Real Wall:** Equality is not observable; no univalence.
**Cross-Domain Aliases:** intensional_tt, computational_tt
**Notes:** Standard ML type theory; good for extraction.

---

### TT.053: Extensional Type Theory (ETT)
**Definition:** Equality is untyped; type checking is undecidable.
**Cost Model:** Reflection: any proof of equality makes terms equal.
**Real Wall:** Convenient but loses canonicity and decidability.
**Cross-Domain Aliases:** extensional_tt, equality_reflection
**Notes:** Nuprl uses ETT; canonicity sacrificed for convenience.

---

### TT.054: Martin-Löf Type Theory (MLTT)
**Definition:** Intensional MLTT with Σ, Π, Id, W (well-founded), universes.
**Cost Model:** Predicative universes: Type₀ : Type₁ : Type₂....
**Real Wall:** Identity types with J; W for induction-recursion.
**Cross-Domain Aliases:** mltt, martin_lof_tt
**Notes:** Basis of Agda, Lean, Coq (CIC variant).

---

### TT.055: Calculus of Inductive Constructions (CIC)
**Definition:** MLTT + impredicative Prop (Coq's sort Prop).
**Cost Model:** Prop is impredicative: ∀ can quantify over Prop in Prop.
**Real Wall:** Allows encodings like Prop = ∀P:Prop.P → P (Girard's paradox).
**Cross-Domain Aliases:** cic, coq_calculus
**Notes:** Coq's foundation; Prop allows classical reasoning in proofs.

---

### TT.056: Prop vs Set (Universes)
**Definition:** Prop: impredicative propositions; Set: predicative sets (types with computational content).
**Cost Model:** Prop allows classical axioms; Set ensures extraction.
**Real Wall:** Code extracted from Set terms; Prop erased at runtime.
**Cross-Domain Aliases:** prop_sort, extraction, universe_polymorphism
**Notes:** Large elimination: define Set from Prop; computationally relevant.

---

### TT.057: Induction-Recursion
**Definition:** Simultaneously define type and functions on it by recursion.
**Cost Model:** Universe definition requires IR; codes and decode.
**Real Wall:** Dybjer-Setzer: codes for universes as IR.
**Cross-Domain Aliases:** induction_recursion, ir_definition
**Notes:** Powerful: can define universe closed under Π, Σ, W.

---

### TT.058: Induction-Induction
**Definition:** Simultaneously define two types A and B:A → Type.
**Cost Model:** Contexts and types as mutually defined families.
**Real Wall:** Syntax of type theory as inductive-inductive type.
**Cross-Domain Aliases:** induction_induction, ii_type
**Notes:** Extends IR; used for syntax with binding.

---

### TT.059: Normalization (Typed)
**Definition:** Every well-typed term has a unique normal form.
**Cost Model:** Confluence + termination = uniqueness of normal form.
**Real Wall:** Not all λ-calculi are confluent (λμ has critical pairs).
**Cross-Domain Aliases:** uniqueness_normal_form, unf
**Notes:** Strong normalization + confluence → UNF.

---

### TT.060: Confluence (Church-Rosser)
**Definition:** If t →* u and t →* v, exists w with u →* w and v →* w.
**Cost Model:** Proved by Newman or Tait's method; critical pairs.
**Real Wall:** Terminating + local confluence → confluence.
**Cross-Domain Aliases:** confluence, church_rosser, cr_property
**Notes:** β-reduction is confluent for λ→; η may not be.

---

### TT.061: Fixed-Point Combinator (Y)
**Definition:** Y = λf.(λx.f(xx))(λx.f(xx)); Yf = f(Yf).
**Cost Model:** Untyped: can diverge; typed: requires recursion type.
**Real Wall:** Hindley-Milner infers recursion; System F needs explicit fix.
**Cross-Domain Aliases:** y_combinator, fixpoint
**Notes:** Enables self-reference; must be handled carefully in types.

---

### TT.062: Product Type (Pair, fst, snd)
**Definition:** Introduction: (a,b); Elimination: fst, snd.
**Cost Model:** Canonical form: (t₁,t₂) : A×B.
**Real Wall:** Corresponds to ∧ (conjunction) in logic.
**Cross-Domain Aliases:** pair_type, sigma_type, conjunction
**Notes:** Πx:A.B(x) generalizes to dependent pairs: Σx:A.B(x).

---

### TT.063: Sum Type (Either, inl, inr)
**Definition:** Introduction: inl : A → A+B, inr : B → A+B; Elimination: match.
**Cost Model:** Pattern matching: cases exhaust all constructors.
**Real Wall:** Corresponds to ∨ (disjunction); empty type ⊥ = 0 constructors.
**Cross-Domain Aliases:** sum_type, disjoint_union, coproduct
**Notes:** Void = ⊥ (no constructors); unit = ⊤ (one constructor).

---

### TT.064: Unit Type
**Definition:** ⊤ with single element tt; used for effects without value.
**Cost Model:** Introduction: tt : ⊤; elimination: match tt returns B.
**Real Wall:** Return type for IO actions returning no value.
**Cross-Domain Aliases:** unit_type, toptype, singleton
**Notes:** Inhabited by exactly one canonical term.

---

### TT.065: Void / Empty Type
**Definition:** ⊥ with no constructors; uninhabited.
**Cost Model:** No introduction; elimination: match on ⊥ impossible.
**Real Wall:** ¬A ≡ A → ⊥; absurd : ⊥ → A (ex falso quodlibet).
**Cross-Domain Aliases:** empty_type, bottom_type, void
**Notes:** Corresponds to false in logic; enables proof by contradiction.

---

### TT.066: Universe Hierarchy (Type₀ : Type₁ : ...)
**Definition:** Predicative hierarchy: Type₀ : Type₁ : Type₂ : ....
**Cost Model:** Avoids paradoxes; code/types live at different levels.
**Real Wall:** Large programs may need many universe levels.
**Cross-Domain Aliases:** universe_hierarchy, predicative_universe
**Notes:** Cumulative: Type₀ ≤ Type₁ ≤ Type₂; avoids full impredicativity.

---

### TT.067: Russell's Paradox (Type Theory)
**Definition:** Set x = {y | y ∉ y} leads to x ∈ x ⇔ x ∉ x.
**Cost Model:** Resolved by Stratification: types prevent self-reference.
**Real Wall:** Girard's paradox: similar in System F with impredicative Prop.
**Cross-Domain Aliases:** russell_paradox, girard_paradox
**Notes:** Type theory avoids by preventing Type : Type.

---

### TT.068: Girard's Paradox
**Definition:** Encoding of set theory in System F leads to inconsistency.
**Cost Model:** Contradiction from impredicative quantification over all types.
**Real Wall:** Coq avoids by distinguishing Prop (impredicative) from Set (predicative).
**Cross-Domain Aliases:** girard_paradox, impredicative_trouble
**Notes:** Requires stratification or eliminability restrictions.

---

### TT.069: Well-Founded Recursion
**Definition:** Recursive definition must decrease on structurally smaller arguments.
**Cost Model:** Accessibility predicate: Acc(R,x) iff no infinite R-descending chain.
**Real Wall:** Structural recursion on inductive types is well-founded.
**Cross-Domain Aliases:** well_founded, structural_recursion, acc_predicate
**Notes:** No recursive definition can produce non-terminating loops.

---

### TT.070: Tactic-Based Proof (Ltac, Ltac2)
**Definition:** Programs that search for proof terms; backtracking search.
**Cost Model:** O(n) to O(2^n) depending on goal complexity.
**Real Wall:** Ltac is untyped; Ltac2 adds types.
**Cross-Domain Aliases:** ltac, proof_search, tactic
**Notes:** Refinement types: refine t₁ ⇒ t₂ → hole; fill with subgoals.

---

### TT.071: Refinement Types (Liquid Types)
**Definition:** Base type + predicate: {x:T | P(x)}; refine with SMT.
**Cost Model:** SMT solver checks refinement; Liquid Haskell.
**Real Wall:** Refines type theory with dependent types + SMT.
**Cross-Domain Aliases:** refinement_type, liquid_type
**Notes:** Can prove array bounds, null checks statically.

---

### TT.072: Ghost Types ( phantom, erased)
**Definition:** Types that don't affect runtime but carry specifications.
**Cost Model:** Erased at runtime; can encode preconditions, loop invariants.
**Real Wall:** F* uses ghost types for effect specifications.
**Cross-Domain Aliases:** ghost_type, phantom_type, erased_type
**Notes:** No runtime representation; purely for verification.

---

### TT.073: Indexed Types
**Definition:** Type families indexed by values: Vec : Nat → Type.
**Cost Model:** More flexible than parameterized types; index checking.
**Real Wall:** Dependent types with restricted dependency.
**Cross-Domain Aliases:** indexed_type, family_type
**Notes:** GADTs in Haskell: constructors can refine index.

---

### TT.074: Generalized Algebraic Data Types (GADT)
**Definition:** Constructor can return specialized type instance of type family.
**Cost Model:** Type indices can be refined at constructors.
**Real Wall:** Enables type-safe AST interpreters, typed EDSLs.
**Cross-Domain Aliases:** gadt, generalized_adts
**Notes:** E.g., Expr a where Lit : Int → Expr Int; not Expr α for Lit.

---

### TT.075: Kind Signatures
**Definition:** Kinds: * (Type), * → * (type constructor), □ (Kind).
**Cost Model:** Type : Kind; higher kinds: (→) : * → * → *.
**Real Wall:** System Fω has type-level computation.
**Cross-Domain Aliases:** kind_signature, kind_of_type, star_star
**Notes:** Higher-kinded polymorphism: Functor f : (* → *) → *.

---

### TT.076: Algebraic Effects (Handlers)
**Definition:** Effects described by operations; handlers interpret them.
**Cost Model:** Effect signature: state_get : Unit → S; state_set : S → Unit.
**Real Wall:** Koka, Frank, Eff languages; composable effects.
**Cross-Domain Aliases:** algebraic_effect, effect_handler
**Notes:** Handler: (eff op k → t) replaces effect with behavior.

---

### TT.077: Row Polymorphism (Disjoint)
**Definition:** Extensible records: {x:Int | r} where r is row variable.
**Cost Model:** Merge: {x:Int | r₁} ∪ {y:Bool | r₂} = {x:Int, y:Bool | r₁+r₂}.
**Real Wall:** Duplicate fields error: r₁ + r₂ undefined if overlap.
**Cross-Domain Aliases:** row_poly, extensible_record, disjoint_union_row
**Notes:** Trakthen, Pyret, Crepe; solves extensible records.

---

### TT.078: Kind Polymorphism
**Definition:** Type constructors parameterized over kinds; ∀κ. T.
**Cost Model:** Compute over kinds; e.g., Functor parameterized by kind.
**Real Wall:** Haskell's PolyKinds extension; enables kind-generic programming.
**Cross-Domain Aliases:** kind_poly, polykinded
**Notes:** Type family over kinds; levity polymorphism (runtime/phantom).

---

### TT.079: Levity Polymorphism
**Definition:** Type parameter that can be lifted or unlifted; * : BOX.
**Cost Model:** Separate runtime representation (unlifted) from compile-time (lifted).
**Real Wall:** Haskell's Levity Polymorphism; enables polymorphic seq, unsafeCoerce.
**Cross-Domain Aliases:** levity_poly, lifted_unlifted
**Notes:** Lifted: has ⊥ as value; unlifted: strict, no ⊥.

---

### TT.080: Implicit Arguments
**Definition:** Arguments inferable from context; written as {x:A} in type.
**Cost Model:** Type inference fills implicits; can be forced with @.
**Real Wall:** Implicit lambdas: λ{@x} = λx. invisible argument.
**Cross-Domain Aliases:** implicit_arg, meta_variable
**Notes:** Coq {}, Agda {}, Haskell {-# UNPACK #-}.

---

### TT.081: Metavariables ( unification variables)
**Definition:** Placeholder for unknown type/term; solved by unification.
**Cost Model:** O(n) constraint solving; occurs check prevents infinite solutions.
**Real Wall:** Flexible vs rigid: can instantiate or must wait.
**Cross-Domain Aliases:** metavariable, unification_var, hole
**Notes:** Bidirectional checking uses synthesis (metavars) and checking.

---

### TT.082: Explicit Polymorphism (forall)
**Definition:** ∀ explicitly written: Λα. t : ∀α. T; application: t [τ].
**Cost Model:** Type application t [τ] instantiates type parameter.
**Real Wall:** Contrast with implicit: inference fills implicit ∀.
**Cross-Domain Aliases:** explicit_poly, forall_type, rank1_poly
**Notes:** ML uses implicit; System F requires explicit.

---

### TT.083: Monomorphization
**Definition:** Instantiate generic code per concrete type; no runtime generics.
**Cost Model:** Compile-time expansion; code bloat vs performance trade-off.
**Real Wall:** C++ templates monomorphize; Java generics erase.
**Cross-Domain Aliases:** monomorphization, specialization
**Notes:** Rust monomorphizes; enables zero-cost abstractions.

---

### TT.084: Erasure (Type Erasure)
**Definition:** Types erased at runtime; compilation removes type information.
**Cost Model:** Erasure simplifies runtime; dependent types may be costly.
**Real Wall:** Scala's manifests, Haskell's TypeRep; keep some type info.
**Cross-Domain Aliases:** type_erasure, erase_types
**Notes:** Dependent types may require runtime representation.

---

### TT.085: Ghost Code ( Compile-Time Only)
**Definition:** Code that only runs at compile time; theorems, proofs.
**Cost Model:** Extract computational content; discard proof terms.
**Real Wall:** Coq's Extraction; Haskell's meta-programming.
**Cross-Domain Aliases:** compile_time, proof_extraction
**Notes:** Propositions as types: only computationally relevant parts extracted.

---

### TT.086: Refinement (Liquid) Type Inference
**Definition:** Infer refinement predicates from examples/test-cases.
**Cost Model:** Liquid Types: refine + SMT solve; infer loop invariants.
**Real Wall:** Liquid Haskell; Verifiable C; automatic annotation.
**Cross-Domain Aliases:** liquid_inference, auto_refinement
**Notes:** Based on SMT (Z3); predicate abstraction over base types.

---

### TT.087: Effect Inference
**Definition:** Infer effect of expression; row variables track unknown effects.
**Cost Model:** Infer row; check against required effect; allow extension.
**Real Wall:** ML's implicit effects; Haskell's IO monad is explicit.
**Cross-Domain Aliases:** effect_inference, row_inference
**Notes:** Scales to large programs; modular effect tracking.

---

### TT.088: Effect Subtyping
**Definition:** Larger effect can be used where smaller is required.
**Cost Model:** Sub-effect: {read, write} ≤ {read, write, throw}.
**Real Wall:** Conservative: may infer too much; can restrict.
**Cross-Domain Aliases:** effect_subtype, effect_coercion
**Notes:** Subtyping matches variance of effect operations.

---

### TT.089: Type Classes (ad-hoc Polymorphism)
**Definition:** Overloaded functions with interface constraint; dictionary passing.
**Cost Model:** Compile-time resolution; generates dictionary arguments.
**Real Wall:** Functor, Monad, Applicative as type classes.
**Cross-Domain Aliases:** type_class, ad_hoc_poly, dictionary
**Notes:** Haskell's overloading; coherent dictionaries.

---

### TT.090: Superclasses
**Definition:** Type class with prerequisite: Eq a ⇒ Ord a.
**Cost Model:** Dictionary for Eq is sub-dictionary of Ord.
**Real Wall:** Hierarchy of constraints; orphan instances problematic.
**Cross-Domain Aliases:** superclass, class_hierarchy
**Notes:** Can cause coherence issues; usually resolved by global uniqueness.

---

### TT.091: Functional Dependencies
**Definition:** Type-level function: class F a b | a → b.
**Cost Model:** Improves type inference; avoids ambiguous types.
**Real Wall:** a → b: given a, b is determined.
**Cross-Domain Aliases:** fundep, type_function_dep
**Notes:** MultiParamTypeClasses; improves type-level computation.

---

### TT.092: Associated Types (Type Families)
**Definition:** Type-level function associated with type class.
**Cost Model:** F a = ...; type family computed at compile time.
**Real Wall:** Open vs closed: open allows instances; closed is injective.
**Cross-Domain Aliases:** associated_type, type_family
**Notes:** Haskell's type families; similar to functional dependencies.

---

### TT.093: Closed Type Families
**Definition:** Type family with fixed set of equations; injective if overlapping.
**Cost Model:** Pattern match at type level; no additional equations.
**Real Wall:** Injectivity can be asserted; completeness checked.
**Cross-Domain Aliases:** closed_type_family, injective_tf
**Notes:** Contrast with open type families: extendable by instances.

---

### TT.094: Type-Level Naturals (Peano)
**Definition:** Type-level 0, S n; type family addition, multiplication.
**Cost Model:** Compute at type level via type family equations.
**Real Wall:** Enables length-indexed vectors, etc.
**Cross-Domain Aliases:** type_nat, peano_type, nat_index
**Notes:** GHC 7.8+ has promoted data kinds.

---

### TT.095: Singletons (Type = Value)
**Definition:** Type-level mirror of value-level; proof that type = value.
**Cost Model:** Sing : Nat → Type where Sing n is singleton for n.
**Real Wall:** Eliminates needs for external type-level computation.
**Cross-Domain Aliases:** singleton_type, type_reflect_value
**Notes:** Dependent Haskell; reflects value-level to type-level.

---

### TT.096: Evidence (Proof Terms)
**Definition:** Terms that prove propositions: refl, sym, trans, cong.
**Cost Model:** Constructors of identity type; computational rules.
**Real Wall:** Proving: build term of type P; term is proof.
**Cross-Domain Aliases:** proof_term, witness_term
**Notes:** Proof-relevant: proof matters; identity as equality.

---

### TT.097: Tactic monad (Proof State)
**Definition:** StateT goal (list subgoal) monad; search for proof terms.
**Cost Model:** Backtracking; exponential worst case.
**Real Wall:** Ltac, Ltac2, Mtac; custom tactic languages.
**Cross-Domain Aliases:** proof_state, tactic_monad
**Notes:** proofstate carries current goal; tactics produce subgoals.

---

### TT.098: Backtracking (Tactics)
**Definition:** Try alternative proof paths when one fails.
**Cost Model:** Search tree; exponential in depth.
**Real Wall:** Prunes dead ends; smart selection important.
**Cross-Domain Aliases:** backtrack_search, proof_search_backtrack
**Notes:** auto, try, repeat, etc.; can diverge or loop.

---

### TT.099: Unification (Higher-Order)
**Definition:** Unify terms containing λ; higher-order pattern restriction.
**Cost Model:** Unification of higher-order terms is decidable with restrictions.
**Real Wall:** Higher-order unification is undecidable in general.
**Cross-Domain Aliases:** higher_order_unif, pattern_unif
**Notes:** Miller's patterns: only heads of arguments are variables.

---

### TT.100: Type Inference (Bidirectional)
**Definition:** Check against known type or synthesize from term.
**Cost Model:** O(n) bidirectional vs O(n²) constraint-based.
**Real Wall:** Check: given type, verify term inhabits; Synth: infer type.
**Cross-Domain Aliases:** bidirectional_infer, check_synth
**Notes:** Allows local type annotations; good for dependent types.

---

### TT.101: Constraint-Based Type Inference
**Definition:** Generate constraints; solve via unification.
**Cost Model:** O(n) constraints; O(n·k) solver for k variables.
**Real Wall:** Generalizes bidirectional; handles complex dependencies.
**Cross-Domain Aliases:** constraint_gen, cbi
**Notes:** Hindley-Milner uses constraint generation + solving.

---

### TT.102: Subtype Constraint Solving
**Definition:** Solve constraints σ ≤ τ; recursive subtype checking.
**Cost Model:** O(n) for records with width+depth subtyping.
**Real Wall:** May require fixed-point iteration for recursive types.
**Cross-Domain Aliases:** subtype_constraint, subleq_solve
**Notes:** Records: check each field; variance rules apply.

---

### TT.103: Principal Type (Most General)
**Definition:** Type scheme ∀α.τ such that any instance is ≤ any other type.
**Cost Model:** Infer principal type; generalizes to constraint context.
**Real Wall:** Principal types exist for Hindley-Milner; not for all systems.
**Cross-Domain Aliases:** principal_scheme, mgu_type
**Notes:** Unique up to renaming of type variables.

---

### TT.104: Scope of Type Variables
**Definition:** Bound variables in ∀; free variables not quantified.
**Cost Model:** α free in Γ means α must be known; cannot generalize.
**Real Wall:** Let polymorphism: generalize over fresh variables.
**Cross-Domain Aliases:** type_var_scope, binding_scope
**Notes:** In λx.t, x's type is free in t; let-generalization captures.

---

### TT.105: Generalization (∀ Introduction)
**Definition:** ∀α.t is valid if α not free in typing environment Γ.
**Cost Model:** At let-binding: generalize free variables.
**Real Wall:** Contrast with λ: cannot generalize under →.
**Cross-Domain Aliases:** generalization, forall_intro
**Notes:** ML type inference: generalization at let boundaries.

---

### TT.106: Instantiation (∀ Elimination)
**Definition:** Replace bound ∀α by fresh type σ; substitute.
**Cost Model:** O(n) substitution; O(1) for fresh variable.
**Real Wall:** Each use of polymorphic binding gets fresh instance.
**Cross-Domain Aliases:** instantiation, forall_elim
**Notes:** Monomorphization: instantiate all at compile time.

---

### TT.107: Occurs Check (Unification)
**Definition:** Reject substitution σ with x = t where x ∈ FV(t).
**Cost Model:** O(n) scan of t for x; prevents infinite types.
**Real Wall:** Without occurs check: x = List x has infinite solution.
**Cross-Domain Aliases:** occurs_check, occurs_in
**Notes:** Essential for sound unification; Prolog uses it.

---

### TT.108: Most General Unifier (MGU)
**Definition:** Substitution σ that is most general: any other unifier = θ∘σ.
**Cost Model:** O(n) Robinson's algorithm; O(n) union-find variant.
**Real Wall:** Unique up to renaming of variables.
**Cross-Domain Aliases:** mgu, mgs
**Notes:** Compositional: σ = mgu(s,t); apply to rest of equation set.

---

### TT.109: Decidable Type Checking
**Definition:** Algorithm deciding whether Γ ⊢ t : T.
**Cost Model:** O(n) for STLC; O(n²) for full ML with polymorphism.
**Real Wall:** Dependent types may be undecidable; requires annotations.
**Cross-Domain Aliases:** decidable_checking, type_check_algorithm
**Notes:** Type checking for most practical systems is decidable.

---

### TT.110: Undecidability (Type Checking)
**Definition:** In System F with ∀ under →, type checking is undecidable.
**Cost Model:** Reduces from halting problem via parametricity violations.
**Real Wall:** Higher-rank polymorphism requires annotations.
**Cross-Domain Aliases:** undecidable_tc, higher_rank_trouble
**Notes:** Rank-1 polymorphism (Hindley-Milner) is decidable.

---

### TT.111: Consistency of Type Theory
**Definition:** No closed term of empty type; no proof of ⊥.
**Cost Model:** Normalization implies consistency.
**Real Wall:** Requires meta-theoretic proof; external consistency proof.
**Cross-Domain Aliases:** consistency_tt, type_theory_consistent
**Notes:** CIC relative to ZFC is standard meta-proof.

---

### TT.112: Normal Forms (Terms)
**Definition:** Neutral: variable, application, lambda on neutral; normal: no reducible redex.
**Cost Model:** O(n) to β-reduce to normal form; may not terminate.
**Real Wall:** β-normal η-long: maximal reduction + eta-expansion.
**Cross-Domain Aliases:** normal_form, beta_normal
**Notes:** Terms in normal form are canonical representatives.

---

### TT.113: β-Reduction (One Step)
**Definition:** (λx.t) u → β t[x/u]; substitute u for x in t.
**Cost Model:** Capture-avoiding substitution; O(n) for term size n.
**Real Wall:** α-conversion renames bound variables first.
**Cross-Domain Aliases:** beta_reduction, one_step_beta
**Notes:** Must avoid variable capture in substitution.

---

### TT.114: η-Conversion
**Definition:** λx. f x =η f (extensionality); f =η λx. f x if x ∉ FV(f).
**Cost Model:** Eta-reduction reduces; eta-expansion adds redex.
**Real Wall:** Not confluent in all systems; optional rule.
**Cross-Domain Aliases:** eta_conv, eta_reduction
**Notes:** Function extensionality: f = g iff ∀x. f x = g x.

---

### TT.115: Capture-Avoiding Substitution
**Definition:** t[x/u]: rename bound variables in t to avoid capture.
**Cost Model:** O(n) to compute fresh names + substitute.
**Real Wall:** Barendregt convention: bound vars distinct from free.
**Cross-Domain Aliases:** capture_avoid, subst, subst_lambda
**Notes:** Key operation; must be defined carefully.

---

### TT.116: Confluence (β-Reduction)
**Definition:** Church-Rosser: →* commutes; unique normal form.
**Cost Model:** Tait's method or parallel moves; O(n²) for proof.
**Real Wall:** β alone is confluent; adding η may break confluence.
**Cross-Domain Aliases:** confluence_beta, church_rosser
**Notes:** Parallel β-reduction: one step in multiple positions.

---

### TT.117: Termination (Strong Normalization)
**Definition:** Every reduction sequence terminates; no infinite β-reduction.
**Cost Model:** Reducibility candidates; logical relations.
**Real Wall:** System F is not strongly normalizing (recursion via fix).
**Cross-Domain Aliases:** strong_normalization, sn
**Notes:** STLC is SN; adding fix destroys SN.

---

### TT.118: Semantic Substitution
**Definition:** Interpret substitution in semantic domain; normalization by evaluation.
**Cost Model:** Evaluate in domain; reify to syntax; O(n).
**Real Wall:** Replaces syntactic substitution; more efficient.
**Cross-Domain Aliases:** semantic_subst, nbe_subst
**Notes:** Key technique for proof assistants and compilers.

---

### TT.119: Kripke Logical Relation
**Definition:** Worlds w ≤ w' extend with more resources; monotone interpretation.
**Cost Model:** Relate terms across worlds; worlds grow.
**Real Wall:** Used for parametricity and soundness of effects.
**Cross-Domain Aliases:** kripke_lr, monotone_relation
**Notes:** Modal logic semantics; worlds = contexts.

---

### TT.120: Relational Parametricity
**Definition:** Polymorphic functions respect relations; abstraction theorem.
**Cost Model:** Theorem: [[∀α.t]]R = λx. [[t]]R∪{α↦R}.
**Real Wall:** Proves "theorems for free" about polymorphic code.
**Cross-Domain Aliases:** parametricity_rel, relational_param
**Notes:** Reynolds' abstraction theorem: parametricity = logical relation.

---

### TT.121: Free Theorems (Wadler)
**Definition:** Consequences of parametricity; e.g., ∀α. [α] → [α] respects length.
**Cost Model:** Generate from type by substituting relations.
**Real Wall:** Automatic: tool generates theorems from polymorphic types.
**Cross-Domain Aliases:** free_theorem, parametricity_theorem
**Notes:** Useful for equational reasoning about polymorphic functions.

---

### TT.122: Representation Independence
**Definition:** Concrete implementation irrelevant as long as abstract interface preserved.
**Cost Model:** Proved via relational parametricity.
**Real Wall:** Module A with representation type R is abstractly equal to any other.
**Cross-Domain Aliases:** repr_indep, abstraction_barrier
**Notes:** ADT: representation independence; functorial semantics.

---

### TT.123: Bisimulation (Process Types)
**Definition:** Behavioral equivalence of processes; simulation game.
**Cost Model:** Checked via coinductive definition; may require fixed point.
**Real Wall:** Strong bisimulation: match all actions.
**Cross-Domain Aliases:** bisimulation, behavioral_eq
**Notes:** HoTT: bisimilarity as greatest fixed point of relation.

---

### TT.124: Coinductive Types
**Definition:** Types with unfolding rules; greatest fixed point μX.F[X].
**Cost Model:** Corecursion: define by observation, not construction.
**Real Wall:** Streams, lazy lists, infinite processes.
**Cross-Domain Aliases:** coinductive, greatest_fixpoint
**Notes:** Copattern matching: define by what you can observe.

---

### TT.125: Coinductive Proof
**Definition:** Prove bisimilarity by building simulation relation.
**Cost Model:** Coinductive hypothesis: assume relation R holds.
**Real Wall:** Bisimilarity is coinductive: greatest fixed point.
**Cross-Domain Aliases:** coinduction, coinductive_proof
**Notes:** Guardedness check ensures productivity.

---

### TT.126: Guarded Recursion (Clock Quantifiers)
**Definition:** Recursive definitions under guard; delay modality ▷.
**Cost Model:** Use clocks: ∀k. (▷^k A); tick advances clocks.
**Real Wall:** Agda's coinductive with sized types.
**Cross-Domain Aliases:** guarded_type, clock_quantifier
**Notes:** Ticked recursion: productive even for infinite objects.

---

### TT.127: Sized Types
**Definition:** Inductive types parameterized by size; recursive calls must decrease.
**Cost Model:** Size inference automatic; termination check syntactic.
**Real Wall:** E.g., Vec n with n : Size; guardedness improved.
**Cross-Domain Aliases:** sized_type, size_inference
**Notes:** Agda, Lean 4; more precise than structural recursion.

---

### TT.128: Indexed Fixpoint (Well-Founded)
**Definition:** Fixpoint with decreasing measure; ensures termination.
**Cost Model:** Structural recursion checks: smaller argument.
**Real Wall:** Measure: nat-valued function decreasing on recursive calls.
**Cross-Domain Aliases:** wf_fixpoint, measure_recursion
**Notes:** lexicographic and product measures extend to complex cases.

---

### TT.129: Type-Level Computation
**Definition:** Evaluate type-level functions at compile time.
**Cost Model:** Type family reduction; must be normalizing.
**Real Wall:** Type families can diverge; may require external solver.
**Cross-Domain Aliases:** type_level_compute, type_family_eval
**Notes:** Haskell type families reduce during type checking.

---

### TT.130: Template Haskell (Compile-Time)
**Definition:** Run Haskell code at compile time; splice into program.
**Cost Model:** Compilation-time evaluation; can be expensive.
**Real Wall:** Type-level naturals, GADT reflection.
**Cross-Domain Aliases:** th_splice, compile_time_compute
**Notes:** Quasi-quotation: embed syntax; meta-programming.

---

### TT.131: Macros (Syntax Transformer)
**Definition:** Syntax-level rewrite rules; expand before type checking.
**Cost Model:** Linear-time expansion; hygiene prevents capture.
**Cross-Domain Aliases:** macro, syntax_rule, hygiene
**Notes:** Racket, Rust proc-macros; syntax-case systems.

---

### TT.132: Hygiene (Macro Expansion)
**Definition:** Generated bindings cannot accidentally capture user bindings.
**Cost Model:** Gensym: generate fresh names; α-convert at expansion.
**Real Wall:** Hygienic macros prevent unintended variable capture.
**Cross-Domain Aliases:** hygiene, gensym, hygienic_macro
**Notes:** Originally from Scheme; Fennett's algorithm.

---

### TT.133: Monadic Reflection (Typing)
**Definition:** Represent effects as values; run via monad.
**Cost Model:** m : Type → Type; return : A → M A; bind : M A → (A → M B) → M B.
**Real Wall:** Haskell IO, ST, Except, State monads.
**Cross-Domain Aliases:** monad_tc, mtl_class
**Notes:** Type classes encode monad operations; modular.

---

### TT.134: Effect System (Monadic)
**Definition:** Track effects in types: IO a = ... ; pure a = no effects.
**Cost Model:** Widen effect row; check against allowed effects.
**Real Wall:** ML using monad transformers; Haskell using Effect.
**Cross-Domain Aliases:** effect_track, io_effect
**Notes:** More precise than implicit IO; enables safe reasoning.

---

### TT.135: Arrow (Computation Abstraction)
**Definition:** Arrow class: arr : (a → b) → (a `f` b); first, second, ***.
**Cost Model:** Less expressive than monad; more structure.
**Real Wall:** Arrow circuits; static computation structures.
**Cross-Domain Aliases:** arrow_type, arr_first
**Notes:** Hughes's arrows; category-theoretic computation.

---

### TT.136: Applicative (Functor + Apply)
**Definition:** Applicative extends Functor: pure : A → F A; <*>, fmap, liftA2.
**Cost Model:** Sequential effects; no dependency between args.
**Real Wall:** Alternative to Monad for non-dependent effects.
**Cross-Domain Aliases:** applicative, apply_functor
**Notes:** Between Functor and Monad; validates <$>, <*>, pure.

---

### TT.137: Traversable
**Definition:** Functor with controlled traversal: traverse : (a → F b) → T a → F (T b).
**Cost Model:** Default traversal from sequence; foldable.
**Real Wall:** Generalizes mapM, sequence; laws: naturality, identity, composition.
**Cross-Domain Aliases:** traversable, effect_traverse
**Notes:** Sequence: T (F a) → F (T a); requires Applicative.

---

### TT.138: Foldable
**Definition:** Structure that can be folded: foldMap : (a → M) → T a → M.
**Cost Model:** Generic fold over any Foldable; default implementations.
**Real Wall:** Provides sum, product, length, null; any container.
**Cross-Domain Aliases:** foldable, fold_map
**Notes:** Haskell 2014 introduced Foldable/Traversable.

---

### TT.139: Deriving ( Automatic Instances)
**Definition:** Derive type class instances from structure.
**Cost Model:** Generic deriving: Generic1, Generic2; derive automatically.
**Real Wall:** Eq, Ord, Show, Read, Functor, Foldable, Traversable.
**Cross-Domain Aliases:** deriving_clause, generic_derive
**Notes:** DeriveGeneric enables user-defined deriving.

---

### TT.140: Newtype (Zero-Cost Wrapper)
**Definition:** newtype N = N T; single constructor; no runtime overhead.
**Cost Model:** Coercion at runtime; zero-cost abstraction.
**Real Wall:** Different from data: newtype has exactly one constructor.
**Cross-Domain Aliases:** newtype_decl, zero_cost_wrap
**Notes:** Used for newtype deriving; type-level markers.

---

### TT.141: Type Role (Representational)
**Definition:** Phantom, nominal, representational type parameters.
**Cost Model:** Nominal: must be same type; representational: same representation.
**Real Wall:** GHC's roles prevent unsafe coerce through newtype.
**Cross-Domain Aliases:** type_role, nominal_repr
**Notes:** role infernominal = nominal; repr = representational.

---

### TT.142: Coercible ( Safe Coercion)
**Definition:** Coercible a b iff a and b have same runtime representation.
**Cost Model:** Coercion proof; coercible : Coercion a b.
**Real Wall:** Safe alternative to unsafeCoerce; checked by type checker.
**Cross-Domain Aliases:** coercible, safe_coerce
**Notes:** Proves representation equality; role-checked.

---

### TT.143: Type Safety (Proof of)
**Definition:** Type system guarantees well-typed programs don't go wrong.
**Cost Model:** Progress + Preservation theorem.
**Real Wall:** "Go wrong" = get stuck: apply non-function, project from non-pair.
**Cross-Domain Aliases:** type_safety, progress_preservation
**Notes:** Well-typed terms don't get stuck; foundational theorem.

---

### TT.144: Existential (Opaque Abstraction)
**Definition:** Existential: ∃α. P(α) hides α; pack with witness.
**Cost Model:** Abstract type with representation hidden.
**Real Wall:** Clients only see interface; cannot depend on representation.
**Cross-Domain Aliases:** existential_type, abstract_type
**Notes:** Corresponds to module signature; ADT encapsulation.

---

### TT.145: Type Reconstruction (HO)
**Definition:** Infer types in higher-order unification context.
**Cost Model:** Complex; may require search; semi-decidable.
**Real Wall:** Higher-order patterns restrict search; decidable.
**Cross-Domain Aliases:** ho_unif, higher_order_recon
**Notes:** Elf, Twelf; LF logical framework uses HOAS.

---

### TT.146: Mode ( input/ output)
**Definition:** Declarative: some arguments known, others inferred.
**Cost Model:** Bidirectional: infer outputs from inputs, check inputs against types.
**Real Wall:** Respects directionality; some args provided, others synthesized.
**Cross-Domain Aliases:** directional_tc, input_output_mode
**Notes:** Prolog modes; Mercury modes; useful for logic languages.

---

### TT.147: Mode Checking (Logic Program)
**Definition:** Static analysis: which arguments are input, which output.
**Cost Model:** Dataflow analysis of logic variables.
**Real Wall:** Detects mode errors: unbound output used as input.
**Cross-Domain Aliases:** mode_check, logic_mode
**Notes:** Mercury; type + mode system for logic languages.

---

### TT.148: Uniqueness Types
**Definition:** Value used exactly once; linear type with extra guarantee.
**Cost Model:** Uniqueness: at most once; linear: exactly once.
**Real Wall:** Can safely update in-place; uniqueness + laziness.
**Cross-Domain Aliases:** uniqueness_type, exactly_once
**Notes:** Clean language; safe destructive update via uniqueness.

---

### TT.149: Ownership Types (Rust)
**Definition:** Each value has owner; references borrow; lifetime tracked.
**Cost Model:** Lifetime inference; borrow checker enforces rules.
**Real Wall:** No data races; memory safety without GC.
**Cross-Domain Aliases:** ownership_type, borrow_check, lifetime
**Notes:** Lifetime 'a: outlives; 'static for globals.

---

### TT.150: Region Types
**Definition:** Memory region; allocation in region; region-polymorphic functions.
**Cost Model:** Regions allocated/stacked; freed in LIFO order.
**Real Wall:** CxLL, ML Kit; region-based memory management.
**Cross-Domain Aliases:** region_type, region_inference
**Notes:** Combines with linear/ownership types; can prevent leaks.

---

### TT.151: Effect Polymorphism
**Definition:** Functions can be polymorphic in effect: ∀e. A →_{e} B.
**Cost Model:** Effect variables in type; constrain at call site.
**Real Wall:** Eff, Frank; effect polymorphism enables effect reuse.
**Cross-Domain Aliases:** effect_poly, poly_effect
**Notes:** Monomorphic in effect: only specific effects allowed.

---

### TT.152: Gradual Effect Typing
**Definition:** Mix static effects and dynamic (?eff); casts at boundaries.
**Cost Model:** Blame tracking; ?effs can flow anywhere.
**Real Wall:** Compromises: static guarantees for static code; flexibility for dynamic.
**Cross-Domain Aliases:** gradual_effect, soft_effect
**Notes:** Alms, Retic; type and effect gradual typing.

---

### TT.153: Refinement ( Ghost State)
**Definition:** Separation logic: assertion about heap; |——{P} c {Q}.
**Cost Model:** Frame rule: {P}c{Q} → {P*R}c{Q*R}.
**Real Wall:** Separation: resources don't overlap; local reasoning.
**Cross-Domain Aliases:** separation_logic, hoare_logic
**Notes:** Reynolds' separation logic; used for verifying pointer programs.

---

### TT.154: Symbolic Execution (Type-Level)
**Definition:** Execute programs symbolically; derive constraints on types.
**Cost Model:** Symbolic terms as types; unification as execution.
**Real Wall:** Symbolic path explosion; requires pruning.
**Cross-Domain Aliases:** symbolic_exec, symbolic_type
**Notes:** Liquid types use symbolic execution + SMT.

---

### TT.155: Model Checking (Type Theory)
**Definition:** Exhaustively check all states of finite model.
**Cost Model:** O(2^n) worst case; symbolic BDDs improve.
**Real Wall:** Used for protocol verification; hardware model checking.
**Cross-Domain Aliases:** model_check, bounded_model_check
**Notes:** TLA+; integrated into Coq via model extraction.

---

### TT.156: Theorem Proving ( Interactive)
**Definition:** User guides proof via tactics; system checks each step.
**Cost Model:** Expert time; high confidence.
**Real Wall:** Coq, Isabelle, Lean, Agda; interactive proof assistants.
**Cross-Domain Aliases:** itp, proof_assistant, theorem_prove
**Notes:** CompCert, seL4, Feit-Thompson theorem proved in Coq.

---

### TT.157: Model Extraction ( Certified)
**Definition:** Extract executable code from proof of specification.
**Cost Model:** Extract computational content; erase proof terms.
**Real Wall:** CompCert: extract C from Coq proof of compiler correctness.
**Cross-Domain Aliases:** extraction, certified_code
**Notes:** Curry-Howard: propositions as types; code as proofs.

---

### TT.158: Proof Relevance
**Definition:** Proof terms matter; equality is evidence.
**Cost Model:** Identity type Id(a,b) with constructor refl.
**Real Wall:** UIP: any two proofs are equal; HoTT drops UIP.
**Cross-Domain Aliases:** proof_relevant, identity_refl
**Notes:** Propositional extensionality: equal props have equal proofs.

---

### TT.159: Prop ( Impredicative)
**Definition:** Sort Prop: propositions; universe-polymorphic; impredicative.
**Cost Model:** Predicates over Type also live in Prop; classical axioms.
**Real Wall:** Large elimination to define Set from Prop.
**Cross-Domain Aliases:** prop_sort, proposition_sort
**Notes:** Girard's paradox avoided by predicativity of Set.

---

### TT.160: Sorts ( Sort Hierarchy)
**Definition:** Prop ⊆ Set ⊆ Type₀ ⊆ Type₁ ⊆ ...; universes.
**Cost Model:** Cumulative: Set ⊆ Type; each is larger.
**Real Wall:** Type must not be Set; prevents paradox.
**Cross-Domain Aliases:** sort_system, universe_level
**Notes:** Coq's sorts: Set, Prop, Type(0), Type(1), ...

---

### TT.161: Mutual Inductive Types
**Definition:** Several types defined simultaneously; cross-references.
**Cost Model:** Mutual recursion: A and B defined together.
**Real Wall:** Syntax and values often mutually inductive.
**Cross-Domain Aliases:** mutual_inductive, corec_mutual
**Notes:** Tree and forest: Tree has forest of subtrees.

---

### TT.162: Nested Inductive Types
**Definition:** Inductive type containing another inductive type as field.
**Cost Model:** No simple recursion scheme; requires functor.
**Real Wall:** rose_tree = node (forest); forest = nil | cons tree forest.
**Cross-Domain Aliases:** nested_inductive, nonregular_inductive
**Notes:** Induction-recursion on tree: define tree and size together.

---

### TT.163: Indexed Inductive Family
**Definition:** Family of types indexed by values: Vec : Nat → Type.
**Cost Model:** Index checking; elimination depends on index.
**Real Wall:** GADTs: constructors can refine index.
**Cross-Domain Aliases:** indexed_family, idx_inductive
**Notes:** Equality type: Id : ∀A. A → A → Type.

---

### TT.164: Inductive-Recursive (Definition)
**Definition:** Define type D and function f:D → Set simultaneously.
**Cost Model:** Universe definition via codes; codes contain D.
**Real Wall:** Define semantics of type theory within itself.
**Cross-Domain Aliases:** inductive_recursive, universe_ir
**Notes:** Dybjer-Setzer: U as codes, El as decode; mutual IR.

---

### TT.165: Pattern Matching ( with-K)
**Definition:** Match on identity proofs; can inspect equality proof.
**Cost Model:** K axiom: J applied to refl gives refl; UIP.
**Real Wall:** Without K: HoTT; with K: intensional MLTT.
**Cross-Domain Aliases:** match_withk, uth_k
**Notes:** Univalent type theory omits K; different equalities.

---

### TT.166: Extensional Equality (Definitional)
**Definition:** Definitional equality: same normal form; η-expansion.
**Cost Model:** Type checking reduces terms; equality is syntactic after reduction.
**Real Wall:** Convenient but undecidable in general (reduces to βη).
**Cross-Domain Aliases:** definitional_eq, intensional_eq
**Notes:** Type checking uses normalization to compare.

---

### TT.167: Propositional Equality (Computational)
**Definition:** Id_A(a,b) with transport; computes on refl.
**Cost Model:** refl = β-reduction; transport by refl is identity.
**Real Wall:** Intensional: Id is type of proofs; not definitional.
**Cross-Domain Aliases:** propositional_eq, id_compute
**Notes:** With canonicity, closed terms reduce to canonical forms.

---

### TT.168: Type Checking Algorithm
**Definition:** Check Γ ⊢ t : T via inference + unification.
**Cost Model:** O(n·W) where W is size of type; polynomial for HM.
**Real Wall:** May require constraint solving; may need backtracking.
**Cross-Domain Aliases:** type_check_algo, inference_algo
**Notes:** Tinkerpop: bidirectional with flexible error recovery.

---

### TT.169: Type Inference (Bidirectional)
**Definition:** Check (synthesize): infer type from subterms; check: verify term inhabits type.
**Cost Model:** O(n) bidirectional vs O(n²) constraint-based.
**Real Wall:** Local annotations help inference; explicit types guide.
**Cross-Domain Aliases:** bidir_infer, infer_check
**Notes:** PTSA: Programming Language Foundations in Agda.

---

### TT.170: Universe Polymorphism
**Definition:** Definitions polymorphic over universe levels; ∀ℓ. T.
**Cost Model:** Level inference; level coherence via constraints.
**Real Wall:** Avoids duplicating code for different universe levels.
**Cross-Domain Aliases:** universe_poly, level_poly
**Notes:** Agda, Coq, Lean 4; crucial for generic programming.

---

### TT.171: Cumulative Universes
**Definition:** Universe subtyping: Type ℓ ≤ Type (ℓ+1); allows lifting.
**Cost Model:** Subtyping rules; cumulativity inference.
**Real Wall:** Simplifies universe handling; no explicit lifting needed.
**Cross-Domain Aliases:** cumulative_universe, universe_subtype
**Notes:** Coq's Set < Type₀ < Type₁; explicit cumulativity.

---

### TT.172: Irrelevance ( Proof Irrelevance)
**Definition:** All proofs of same proposition are equal; irrelevance annotation.
**Cost Model:** Prop is impredicative + irrelevant; erased at runtime.
**Real Wall:** Prop : Prop allowed in Coq (CIC); Set : Prop not allowed.
**Cross-Domain Aliases:** proof_irrelevance, irrelevance_annot
**Notes:** Prop is computationally inert; can add classical axioms.

---

### TT.173: SProp ( Squash)
**Definition:** Propositional squash: ‖P‖ collapses proof of P to mere existence.
**Cost Model:** No proof terms; computationally inert; truncation.
**Real Wall:** Squash: if P has any proof, ‖P‖ is inhabited.
**Cross-Domain Aliases:** squash_type, prop_truncate
**Notes:** HoTT's mere propositions; useful for specifications.

---

### TT.174: Truncation ( Propositional Resizing)
**Definition:** Move proposition between universes: ‖P‖_ℓ : Type (ℓ+1).
**Cost Model:** Truncation preserves provability; moves up universes.
**Real Wall:** Allows embedding Prop from lower universe into higher.
**Cross-Domain Aliases:** prop_resize, truncation_universe
**Notes:** For large definitions requiring higher universe levels.

---

### TT.175: Function Extensionality (Axiom)
**Definition:** funext : (∀x. f x = g x) → f = g; added as axiom.
**Cost Model:** Prop can hold axioms; may lose canonicity.
**Real Wall:** Univalence implies funext for functions over types.
**Cross-Domain Aliases:** funext_axiom, function_extensionality
**Notes:** Propositional extensionality: (P ↔ Q) → P = Q.

---

### TT.176: Univalence ( as Primitive)
**Definition:** ua : (A ≃ B) → (A = B); definitional equality from equivalence.
**Cost Model:** Transport along ua is equivalence; computes.
**Real Wall:** Cannot be derived in CIC; added as new primitive.
**Cross-Domain Aliases:** univalence_prim, ua_constructor
**Notes:** Voevodsky's axiom; foundation of HoTT library.

---

### TT.177: Cubical Type Theory
**Definition:** Kan operations on cubes; composition instead of J.
**Cost Model:** Normalizes; has canonicity + univalence.
**Real Wall:** Cubical AGDA, CCHM cubes; definitional K.
**Cross-Domain Aliases:** cubical_tt, kan_cubes
**Notes:** Canonicity proof: closed Nat terms reduce to numerals.

---

### TT.178: Observational Type Theory (Comp)
**Definition:** Equality is observable; definitional equality via reduction.
**Cost Model:** Computable equality; canonicity preserved.
**Real Wall:** UIP definitional; no univalence without axioms.
**Cross-Domain Aliases:** ott_comp, computational_equality
**Notes:** Abbott, Altenkirch, Ghani; combines HoTT and canonicity.

---

### TT.179: Normalization-by-Evaluation ( NbE)
**Definition:** Interpret in semantic domain; reify to normal form.
**Cost Model:** O(n) for comparison; efficient for large terms.
**Real Wall:** Used in type checkers for fast equality.
**Cross-Domain Aliases:** nbe_normalize, semantic_norm
**Notes:** Chapman's OTT; Abel's efficient implementation.

---

### TT.180: Definitional Normalization
**Definition:** Normalize to β-normal η-long form during type checking.
**Cost Model:** Full normalization O(n²) worst; cached O(n).
**Real Wall:** Ensures definitional equality is syntactic after reduction.
**Cross-Domain Aliases:** def_normalize, full_beta_eta_norm
**Notes:** Required for consistency and type safety proofs.

---

### TT.181: Erasure ( Computational Irrelevance)
**Definition:** Erasable arguments; not evaluated at runtime.
**Cost Model:** Prop arguments erasable; computation irrelevant.
**Real Wall:** Coq's [shuffles] are erasable; compile-time only.
**Cross-Domain Aliases:** erasable_arg, computational_irrelevant
**Notes:** Can annotate arguments as [irrelevant]; not evaluated.

---

### TT.182: Canonical Forms ( Inhabitation)
**Definition:** Every closed well-typed term is a canonical form (value).
**Cost Model:** Values are constructors applied to values.
**Real Wall:** Progress theorem: well-typed terms are values or step.
**Cross-Domain Aliases:** canonical_form, progress
**Notes:** For closed terms of base type: must be constructor.

---

### TT.183: Syntactic Soundness
**Definition:** All well-typed terms have semantic meaning in model.
**Cost Model:** Construct model of types; interpret terms.
**Real Wall:** Soundness: ⟦Γ ⊢ t : T⟧ ∈ ⟦T⟧.
**Cross-Domain Aliases:** semantic_soundness, model_interp
**Notes:** Consistency: empty type has no interpretation.

---

### TT.184: Cut Elimination (Proof Normalization)
**Definition:** Remove cuts from proofs; normalize to atomic cuts.
**Cost Model:** Gentzen's Hauptsatz; normalization terminates.
**Real Wall:** Eliminates logical complexity; implies consistency.
**Cross-Domain Aliases:** cut_elimination, normalisation_proof
**Notes:** Curry-Howard: cut = proof term containing itself.

---

### TT.185: Consistency ( Arithmetic)
**Definition:** Arithmetic consistent: no proof of ⊥.
**Cost Model:** Normalization of PA proof terms shows consistency.
**Real Wall:** Gödel's second: cannot prove consistency in system.
**Cross-Domain Aliases:** consistency_arith, godel_theorem
**Notes:** PA's consistency proven in type theory; requires ordinal analysis.

---

### TT.186: Normalization ( Ordinal Analysis)
**Definition:** Assign ordinal to each proof; decreasing ordinals terminate.
**Cost Model:** Ordinal notation; compute up to ε₀.
**Real Wall:** Proof-theoretic ordinal measures strength of theory.
**Cross-Domain Aliases:** ordinal_norm, proof_ordinal
**Notes:** PA proves termination up to ε₀; beyond requires stronger theory.

---

### TT.187: Dependent Pattern Matching
**Definition:** Patterns can refine index; match on identity proof.
**Cost Model:** Unify patterns against indices; refine types.
**Real Wall:** Allows powerful eliminators; e.g., transport on equality.
**Cross-Domain Aliases:** dependent_match, pattern_unify
**Notes:** Cockx, Abel; unification for dependent types.

---

### TT.188: Unification with Metavariables
**Definition:** Solve for metavariables (holes) in type context.
**Cost Model:** O(n) constraint solving; occurs check.
**Real Wall:** Metavariables represent incomplete proof terms.
**Cross-Domain Aliases:** meta_solve, hole_fill
**Notes:** Agda's goals: ? or {!!}; fill with term.

---

### TT.189: Program Extraction (from Proof)
**Definition:** Extract computational content from constructive proof.
**Cost Model:** Erase Prop; keep Set; compute.
**Real Wall:** Curry-Howard: proof of ∀n. P(n) gives algorithm for all n.
**Cross-Domain Aliases:** extraction, computational_extract
**Notes:** Coq's Extraction: OCaml, Haskell, Scheme.

---

### TT.190: Proof Automation ( Hammer)
**Definition:** SMT/ATP integration for proof search; Mizar, Sledgehammer.
**Cost Model:** External provers; may time out or give wrong proof.
**Real Wall:** Reduces human effort; not 100% reliable.
**Cross-Domain Aliases:** proof_hammer, smt_integration
**Notes:** Auto2, Lean auto, Coq smt plugin; combination.

---

### TT.191: Certifying Compiler
**Definition:** Compiler that also proves its own correctness.
**Cost Model:** Coq compiler: CompCert; type-checked compilation.
**Real Wall:** Machine code is correct translation of source.
**Cross-Domain Aliases:** certifying_compiler, verified_compiler
**Notes:** CompCert: C compiler proven correct in Coq.

---

### TT.192: Type-Preserving Compilation
**Definition:** Types preserved through compilation; typed IR.
**Cost Model:** Intermediate languages with types; verified translation.
**Real Wall:** Prevent runtime type errors in compiled code.
**Cross-Domain Aliases:** type_preserve, typed_ir
**Notes:** LLVM's typed IL; CakeML's verified compilation chain.

---

### TT.193: Supercompilation ( Termination)
**Definition:** Transform program by specializing and reducing.
**Cost Model:** Specialize, fold, embed; may not terminate.
**Real Wall:** Can discover optimizations; exponential blowup possible.
**Cross-Domain Aliases:** supercompile, program_specialization
**Notes:** Turchin, Sørensen; powerful but complex.

---

### TT.194: Partial Evaluation ( Offline)
**Definition:** Specialize program given some inputs known.
**Cost Model:** Binding-time analysis; residual program.
**Real Wall:** Can achieve dramatic speedups for specialized code.
**Cross-Domain Aliases:** partial_eval, specialization
**Notes:** Similix, Cleve; online = dynamic compilation.

---

### TT.195: Defunctionalization
**Definition:** Replace higher-order functions with first-order data.
**Cost Model:** Encode continuations as sum-of-products.
**Real Wall:** Eliminates heap allocation for closures.
**Cross-Domain Aliases:** defunctionalize, closure_escape
**Notes:** Danvy, Nielsen; reverse of lambda lifting.

---

### TT.196: CPS Transformation
**Definition:** Transform to continuation-passing style; explicit continuations.
**Cost Model:** Every function takes continuation; tail calls explicit.
**Real Wall:** CPS conversion eliminates control flow complexity.
**Cross-Domain Aliases:** cps_transform, continuation_pass
**Notes:** Plotkin: call-by-name and call-by-value CPS.

---

### TT.197: A-Normal Form (ANF)
**Definition:** Every intermediate is let-bound; flat sequence of lets.
**Cost Model:** ANF conversion; variables bound to primitives.
**Real Wall:** Simplifies analysis; easier control flow.
**Cross-Domain Aliases:** anf_conversion, administrative_norm
**Notes:** Flattens nested applications; closure conversion target.

---

### TT.198: Closure Conversion ( Type-Theoretic)
**Definition:** Transform to explicitly passed environment; λ → closure {code, env}.
**Cost Model:** Eliminate free variables; first-class functions as closures.
**Real Wall:** Type-preserving closure conversion: embed env in type.
**Cross-Domain Aliases:** closure_conv, env_elimination
**Notes:** Minamide, Morrisett; typed closure conversion.

---

### TT.199: Typed Assembly Language (TAL)
**Definition:** Assembly language with type annotations; type-safe.
**Cost Model:** Well-typed TAL programs can't violate memory safety.
**Real Wall:** TAL-x86; verified via type checker.
**Cross-Domain Aliases:** tal, typed_assembly
**Notes:** Morrisett, Harper; compilation target for certified compilers.

---

### TT.200: LF ( Logical Framework)
**Definition:** Dependently typed lambda calculus for encoding logics/theories.
**Cost Model:** Judgment as type; constants for theory signature.
**Real Wall:** Twelf, Elf; meta-theorem proving.
**Cross-Domain Aliases:** lf_framework, dependently_typed_encoding
**Notes:** Harper, Honsell, Plotkin; foundational logical framework.

---

### TT.201: Twelf ( Meta-Theorem Prover)
**Definition:** Logic programming + LF; proves meta-theorems via search.
**Cost Model:** Search via logic program execution; may diverge.
**Real Wall:** Coverage checking; totality assertions.
**Cross-Domain Aliases:** twelf_prover, meta_theorem
**Notes:** Used to prove soundness of LF, compiler correctness.

---

### TT.202: Isabelle/ HOL ( Higher-Order Logic)
**Definition:** HOL in LCF style; meta-theory via derivations.
**Cost Model:** Basic inference primitives; derived rules build theories.
**Real Wall:** classicalReasoner, simp; powerful automation.
**Cross-Domain Aliases:** isabelle_hol, hol_system
**Notes:** Naproche (natural proof); Mizar style formalization.

---

### TT.203: Coq ( CIC / CoC)
**Definition:** Calculus of Inductive Constructions; Gallina language + Ltac.
**Cost Model:** Ltac search; O(n) to O(2^n) depending on automation.
**Real Wall:** CompCert, seL4, Coq standard library.
**Cross-Domain Aliases:** coq_system, cic_type
**Notes:** Definitional equality; Prop/Set/Type hierarchy.

---

### TT.204: Agda ( Indexed Type Theory)
**Definition:** Dependently typed language; pattern matching, termination.
**Cost Model:** O(n) type checking; may require annotations.
**Real Wall:** Cubical Agda; internal univalence.
**Cross-Domain Aliases:** agda_lang, indexed_type
**Notes:** Backend to Haskell; compile via Haskell.

---

### TT.205: Lean ( Tactical Theorem Prover)
**Definition:** Calculus of Inductive Constructions + tactics + metaprogramming.
**Cost Model:** Elaborator + tactic monad; Lean's mathlib.
**Real Wall:** Lean 4: Rust-based VM; metaprogramming in Lean.
**Cross-Domain Aliases:** lean_prover, lean4_tactic
**Notes:** Mathlib: community-maintained mathematics library.

---

### TT.206: Idris ( Type-Driven Development)
**Definition:** Dependently typed; type-first; totality checking.
**Cost Model:** Type-driven development; elaborate and check.
**Real Wall:** Compiles to Haskell, C, JavaScript, LLVM.
**Cross-Domain Aliases:** idris_lang, ttd_lang
**Notes:** Effects library; type-safe runtime code generation.

---

### TT.207: F* ( Effectful Theorem Prover)
**Definition:** SMT-backed verifier; dependent types + effects.
**Cost Model:** Z3 for verification conditions; may timeout.
**Real Wall:** Low* for low-level; EverCrypt, HACL*.
**Cross-Domain Aliases:** fst_verifier, effectful_verif
**Notes:** Dijkstra monads; verifies cryptographic code.

---

### TT.208: Why3 ( Verification Platform)
**Definition:** WhyML language; Why3 platform; external provers.
**Cost Model:** VC generation; SMT solver for verification.
**Real Wall:** Frama-C, SPARK; industrial formal methods.
**Cross-Domain Aliases:** why3_platform, whyml
**Notes:** Separation logic; pointer programs; modular verification.

---

### TT.209: Dafny ( Verifiable Language)
**Definition:** imperative + object-oriented; auto-active verification.
**Cost Model:** Boogie + Z3; loop invariants, pre/postconditions.
**Real Wall:** Compiles to C#, Java, JavaScript; verified examples.
**Cross-Domain Aliases:** dafny_verif, boogie_backend
**Notes:** Auto-active: minimal annotations + SMT solves rest.

---

### TT.210: Formally Verified Software
**Definition:** Software proven correct via formal methods.
**Cost Model:** Significant human effort; high assurance.
**Real Wall:** CompCert (C compiler), seL4 (microkernel), CertiKOS (OS).
**Cross-Domain Aliases:** verified_software, formal_correctness
**Notes:** 1-10× cost over unverified; essential for critical systems.

---

## Section: Lambda Calculus Extensions and Combinatory Logic

### TT.211: Untyped Lambda Calculus
**Definition:** Formal system with variables, abstraction, and application; no type discipline.
**Cost Model:** Turing-complete; reduction may not terminate.
**Real Wall:** Foundational model; Church 1936.
**Cross-Domain Aliases:** untyped_lc, pure_lambda, church_calculus
**Notes:** Church-Rosser theorem; confluence of beta reduction.

---

### TT.212: Combinatory Logic
**Definition:** Variable-free computation via combinators composed by application.
**Cost Model:** Equivalent expressive power to lambda calculus; size blowup possible.
**Real Wall:** Schönfinkel 1924; Curry developed extensively.
**Cross-Domain Aliases:** combinator_logic, variable_free_lc
**Notes:** Abstraction elimination via bracket abstraction; ties to point-free style.

---

### TT.213: SKI Combinator Calculus
**Definition:** Three-combinator basis: S = λxyz.xz(yz), K = λxy.x, I = λx.x.
**Cost Model:** Turing-complete; encoding lambda terms has quadratic blowup.
**Real Wall:** I = SKK; minimal computational basis.
**Cross-Domain Aliases:** ski_calculus, ski_basis
**Notes:** Curry showed S and K alone are functionally complete.

---

### TT.214: BCKW Combinator System
**Definition:** Combinators B (compose), C (flip), K (const), W (duplicate); spans relevance/affine sub-bases.
**Cost Model:** Various subsets correspond to substructural logics.
**Real Wall:** Curry-Feys; basis analysis for substructural logic.
**Cross-Domain Aliases:** bckw_system, relevance_combinators
**Notes:** BCI = linear logic; BCK = affine logic; BCKW = full intuitionistic.

---

### TT.215: Lambda-Mu Calculus
**Definition:** Extension of lambda calculus with μ-binder and named applications giving classical logic via Curry-Howard.
**Cost Model:** Adds control operators to typed lambda calculus.
**Real Wall:** Parigot 1992; computational interpretation of classical proofs.
**Cross-Domain Aliases:** lambda_mu, parigot_calculus
**Notes:** Captures call/cc-like control via duality with continuations.

---

### TT.216: Lambda-Bar-Mu-Mu-Tilde Calculus
**Definition:** Symmetric calculus with terms, contexts, and commands; computational dual of Lambda-Mu.
**Cost Model:** Confluent under call-by-name or call-by-value strategy choice.
**Real Wall:** Curien-Herbelin 2000; foundations of classical computation.
**Cross-Domain Aliases:** lambda_bar_mu, curien_herbelin
**Notes:** Dual sequent calculus; computational analog of LK.

---

### TT.217: Call-With-Current-Continuation (callcc)
**Definition:** First-class control operator capturing the rest of the computation as a function.
**Cost Model:** Linear in continuation depth at capture; stack copying or segmented.
**Real Wall:** Scheme; Felleisen et al. semantics.
**Cross-Domain Aliases:** call_cc, callcc, current_continuation
**Notes:** Typing requires Peirce's law: ((A→B)→A)→A.

---

### TT.218: Delimited Continuations (Shift/Reset)
**Definition:** Continuation operators bounded by a prompt (reset) and captured by shift.
**Cost Model:** Localized; composable unlike full callcc.
**Real Wall:** Danvy-Filinski 1990; expressive enough for any side effect.
**Cross-Domain Aliases:** shift_reset, prompts, delimited_control
**Notes:** Foundations of algebraic effect handlers.

---

### TT.219: Krivine Abstract Machine
**Definition:** Lazy call-by-name abstract machine for lambda calculus using closures and stacks.
**Cost Model:** Tail-recursive; minimal heap allocation.
**Real Wall:** Jean-Louis Krivine; classical realizability.
**Cross-Domain Aliases:** krivine_machine, k_machine
**Notes:** Used in Krivine's classical realizability program.

---

### TT.220: CEK Machine
**Definition:** Control-Environment-Kontinuation abstract machine for call-by-value lambda calculus.
**Cost Model:** Linear in program steps; small-step structural.
**Real Wall:** Felleisen-Friedman 1986.
**Cross-Domain Aliases:** cek_machine, control_env_kont
**Notes:** Derived from CPS; basis for many language implementations.

---

### TT.221: SECD Machine
**Definition:** Stack-Environment-Control-Dump machine for lambda calculus, by Landin.
**Cost Model:** Constant-time primitive operations; explicit dump for function calls.
**Real Wall:** Landin 1964; ancestor of many functional language VMs.
**Cross-Domain Aliases:** secd_machine, landin_machine
**Notes:** First abstract machine for functional languages.

---

### TT.222: CK Machine
**Definition:** Control-Kontinuation abstract machine; CEK without environment.
**Cost Model:** Reduction via evaluation context decomposition.
**Real Wall:** Variant of CEK for substitution-based semantics.
**Cross-Domain Aliases:** ck_machine
**Notes:** Felleisen's reduction semantics.

---

## Section: Type Systems of the Lambda Cube

### TT.223: System F (Polymorphic Lambda Calculus)
**Definition:** Lambda calculus with universal type quantification: ∀α.τ and Λα.e.
**Cost Model:** Type inference is undecidable (Wells 1994); type checking decidable.
**Real Wall:** Girard 1972, Reynolds 1974 independently.
**Cross-Domain Aliases:** system_f, polymorphic_lc, second_order_lc
**Notes:** Foundation of ML and Haskell polymorphism.

---

### TT.224: System F-omega
**Definition:** System F extended with type-level functions and higher kinds.
**Cost Model:** Type equivalence requires beta-reduction at type level.
**Real Wall:** Used in Haskell's higher-kinded types.
**Cross-Domain Aliases:** f_omega, system_f_omega
**Notes:** Combines polymorphism with type operators.

---

### TT.225: System F-sub
**Definition:** System F with bounded quantification: ∀α<:τ.σ.
**Cost Model:** Subtype checking is undecidable (Pierce 1994).
**Real Wall:** Cardelli-Wegner; basis for many OO type systems.
**Cross-Domain Aliases:** f_sub, bounded_system_f
**Notes:** Used in object-oriented language formalizations.

---

### TT.226: Calculus of Constructions (CoC)
**Definition:** Higher-order dependent type theory unifying terms and types.
**Cost Model:** Type checking decidable; complexity non-elementary.
**Real Wall:** Coquand-Huet 1988; core of Coq.
**Cross-Domain Aliases:** coc, calculus_constructions
**Notes:** Top of the lambda cube; impredicative Prop universe.

---

### TT.227: Calculus of Inductive Constructions (CIC)
**Definition:** CoC + inductive types with associated elimination principles.
**Cost Model:** Adds primitive recursion schemes per inductive family.
**Real Wall:** Coquand-Paulin 1990; foundation of Coq and Lean.
**Cross-Domain Aliases:** cic, calc_inductive_constructions
**Notes:** Strong normalization; consistency proof requires large cardinals.

---

### TT.228: Pure Type Systems (PTS)
**Definition:** Generalized framework parameterized by sorts S, axioms A, and rules R.
**Cost Model:** Type checking decidability depends on rules.
**Real Wall:** Barendregt 1991; uniform presentation of type theories.
**Cross-Domain Aliases:** pts, generalized_type_systems
**Notes:** Lambda cube is a family of eight PTSs.

---

### TT.229: Lambda Cube
**Definition:** 3D classification of type systems by polymorphism, type operators, dependent types.
**Cost Model:** Eight vertices: STLC, F, F-omega, LF, P, P-omega, CoC, etc.
**Real Wall:** Barendregt 1991.
**Cross-Domain Aliases:** barendregt_cube, lambda_cube
**Notes:** Three axes: terms→types, types→types, types→terms.

---

### TT.230: Edinburgh Logical Framework (LF)
**Definition:** Dependently typed lambda calculus for specifying logical systems.
**Cost Model:** Type checking decidable; weak metatheory.
**Real Wall:** Harper-Honsell-Plotkin 1993; Twelf implementation.
**Cross-Domain Aliases:** lf_calculus, lambda_pi, edinburgh_lf
**Notes:** Higher-order abstract syntax via dependent types.

---

### TT.231: Expression Problem
**Definition:** Difficulty of extending datatypes and operations in both directions without modifying existing code.
**Cost Model:** Solved by various encodings: type classes, OO visitors, tagless final.
**Real Wall:** Wadler 1998 named it; identified by Reynolds 1975.
**Cross-Domain Aliases:** expression_problem, extension_dimension
**Notes:** Test of type system extensibility.

---

## Section: Polymorphism Forms

### TT.232: Parametric Polymorphism
**Definition:** Code that works uniformly over all types via type quantification.
**Cost Model:** Compile-time monomorphization or boxed at runtime.
**Real Wall:** Strachey 1967; formalized by System F.
**Cross-Domain Aliases:** parametric_poly, generics, universal_poly
**Notes:** Reynolds' abstraction theorem and parametricity.

---

### TT.233: Ad-Hoc Polymorphism
**Definition:** Same syntax with different implementations chosen by type; overloading.
**Cost Model:** Dictionary passing or instance resolution.
**Real Wall:** Type classes (Haskell), traits (Rust), implicits (Scala).
**Cross-Domain Aliases:** overloading, ad_hoc_poly
**Notes:** Strachey 1967; mechanism: type classes in Wadler-Blott 1989.

---

### TT.234: Bounded Polymorphism
**Definition:** Type variables constrained by subtype upper bounds: ∀α<:τ.σ.
**Cost Model:** Subtype check at each instantiation.
**Real Wall:** F-sub; Java/Scala generics with extends bounds.
**Cross-Domain Aliases:** bounded_quantification, constrained_generics
**Notes:** Cardelli-Wegner 1985.

---

### TT.235: F-Bounded Polymorphism
**Definition:** Bound may mention the bound type variable: ∀α<:F(α).σ.
**Cost Model:** Required for self-types and recursive interfaces.
**Real Wall:** Canning et al. 1989; foundation of generic interfaces.
**Cross-Domain Aliases:** f_bounded_poly, recursive_bounds
**Notes:** Encodes binary methods and Comparable<T> patterns.

---

### TT.236: Higher-Rank Polymorphism
**Definition:** Polymorphic types may appear under function arrows: (∀α.σ)→τ.
**Cost Model:** Inference undecidable beyond rank 2.
**Real Wall:** RankNTypes in Haskell; explicit annotations required.
**Cross-Domain Aliases:** higher_rank, rank_n_types
**Notes:** Wells 1994; ST monad requires rank-2.

---

### TT.237: Impredicative Polymorphism
**Definition:** Quantified types may be instantiated to polymorphic types.
**Cost Model:** Inference very difficult; ImpredicativeTypes in GHC.
**Real Wall:** Quick Look impredicativity (Serrano et al.).
**Cross-Domain Aliases:** impredicative_poly, quick_look
**Notes:** Loss of predicative stratification.

---

### TT.238: Predicative Polymorphism
**Definition:** Quantified type variables only instantiated with monotypes.
**Cost Model:** Decidable inference (Hindley-Milner family).
**Real Wall:** Damas-Milner restriction enforces predicativity.
**Cross-Domain Aliases:** predicative_poly, stratified_poly
**Notes:** Russell's vicious-circle principle.

---

### TT.239: Rank-N Types
**Definition:** Polymorphic types occurring N arrows deep; rank-N requires annotations beyond rank-2.
**Cost Model:** Rank-1 = HM; rank-2 = decidable; rank-N>2 needs annotations.
**Real Wall:** Kfoury-Wells; Peyton Jones et al. "Practical type inference".
**Cross-Domain Aliases:** rank_n, polymorphic_arguments
**Notes:** Necessary for runST and continuation-passing combinators.

---

### TT.240: Existential Types
**Definition:** ∃α.τ; types where the witness type is hidden.
**Cost Model:** Pack/unpack operations; introduces abstract types.
**Real Wall:** Mitchell-Plotkin 1988; equivalent to modules.
**Cross-Domain Aliases:** existentials, exists_type, abstract_data_types
**Notes:** Dual of universal quantification; foundation of ADT modules.

---

## Section: Subtyping

### TT.241: Subtype Polymorphism
**Definition:** A value of subtype S may be used where supertype T is expected.
**Cost Model:** May incur coercion or pointer-tag checks.
**Real Wall:** Cardelli; basis of OOP type systems.
**Cross-Domain Aliases:** subtype_poly, inclusion_poly
**Notes:** Liskov Substitution Principle.

---

### TT.242: Structural Subtyping
**Definition:** S <: T iff S has all members T requires, by shape.
**Cost Model:** Shape comparison; can be O(field-count).
**Real Wall:** OCaml objects, TypeScript, Go interfaces.
**Cross-Domain Aliases:** structural_sub, shape_subtyping
**Notes:** Duck typing made formal.

---

### TT.243: Nominal Subtyping
**Definition:** S <: T iff explicitly declared (extends/implements clause).
**Cost Model:** Constant-time class table lookup.
**Real Wall:** Java, C#, C++ class hierarchies.
**Cross-Domain Aliases:** nominal_sub, declaration_sub
**Notes:** Tradeoff: less flexible but more controlled.

---

### TT.244: Depth Subtyping
**Definition:** Allows subtyping inside record fields: {x:S} <: {x:T} when S <: T.
**Cost Model:** Requires field variance analysis.
**Real Wall:** Sound only for immutable fields (covariant positions).
**Cross-Domain Aliases:** depth_sub, internal_subtyping
**Notes:** Mutable fields require invariance.

---

### TT.245: Width Subtyping
**Definition:** Record with more fields is a subtype of one with fewer: {x:T,y:U} <: {x:T}.
**Cost Model:** Possibly requires record adjustment at coercion.
**Real Wall:** OCaml objects use width subtyping freely.
**Cross-Domain Aliases:** width_sub, forgetting_fields
**Notes:** Combine depth+width for full record subtyping.

---

### TT.246: Variance (Covariant/Contravariant/Invariant)
**Definition:** How parametric type constructor preserves, reverses, or ignores subtyping of its argument.
**Cost Model:** Variance check during subtype derivation.
**Real Wall:** Function types: contravariant in input, covariant in output.
**Cross-Domain Aliases:** type_variance, cov_contra
**Notes:** Reynolds; central to safe generics.

---

### TT.247: Declaration-Site Variance
**Definition:** Variance annotations placed on type parameter declarations (Scala, Kotlin).
**Cost Model:** Checked once at declaration; user sites need no annotation.
**Real Wall:** Scala +T (cov), -T (contra), T (invariant).
**Cross-Domain Aliases:** decl_site_variance, definition_variance
**Notes:** Igarashi-Viroli.

---

### TT.248: Use-Site Variance
**Definition:** Variance specified per usage of a type constructor (Java wildcards).
**Cost Model:** Each usage carries annotation.
**Real Wall:** Java `? extends T`, `? super T`.
**Cross-Domain Aliases:** use_site_variance, wildcards
**Notes:** Kennedy-Pierce; more flexible but verbose.

---

### TT.249: Variance Annotations
**Definition:** Syntactic markers controlling subtyping of generic parameters.
**Cost Model:** Static check; no runtime cost.
**Real Wall:** Required for sound generic collections.
**Cross-Domain Aliases:** variance_marks, plus_minus_annotations
**Notes:** Producer-extends, consumer-super (PECS rule).

---

## Section: Dependent Types and Universes

### TT.250: Pi-Types (Dependent Function Types)
**Definition:** Types of functions where return type depends on argument value: Π(x:A).B(x).
**Cost Model:** Type-checking requires reduction in types.
**Real Wall:** Foundation of dependent type theories.
**Cross-Domain Aliases:** pi_type, dep_function, dep_product
**Notes:** Martin-Löf; generalizes A → B when B independent of x.

---

### TT.251: Sigma-Types (Dependent Pair Types)
**Definition:** Pairs (a, b) where type of b depends on a: Σ(x:A).B(x).
**Cost Model:** Pattern matching with dependent eliminators.
**Real Wall:** Encodes existentials, subset types.
**Cross-Domain Aliases:** sigma_type, dep_pair, dep_sum
**Notes:** Generalizes A × B; foundation of refinements.

---

### TT.252: Dependent Pattern Matching
**Definition:** Pattern matching that refines indices and types in each branch.
**Cost Model:** Equality refinements; unification-based.
**Real Wall:** Coquand 1992; Agda, Idris use extensively.
**Cross-Domain Aliases:** dep_pat_match, ipm
**Notes:** Goguen, McBride, McKinna formalized.

---

### TT.253: Indexed Inductive Types
**Definition:** Inductive families parameterized by indices that vary per constructor.
**Cost Model:** Indices recovered by unification at elimination.
**Real Wall:** Vectors, finite sets, well-typed terms.
**Cross-Domain Aliases:** indexed_inductive, inductive_families
**Notes:** Dybjer; foundation of GADT-like reasoning.

---

### TT.254: Identity Types (Equality Types)
**Definition:** Type Id_A(a,b) of proofs that a = b in type A.
**Cost Model:** Path operations; J-eliminator.
**Real Wall:** Martin-Löf 1972; central object of HoTT.
**Cross-Domain Aliases:** id_type, equality_type, propositional_equality
**Notes:** Intensional vs extensional equality distinction.

---

### TT.255: Universes
**Definition:** Types of types, hierarchically stratified: Type_0 : Type_1 : Type_2 : ...
**Cost Model:** Universe checking; consistency via stratification.
**Real Wall:** Avoids Girard's paradox (Type:Type inconsistent).
**Cross-Domain Aliases:** universe, type_universe, sort_hierarchy
**Notes:** Type:Type leads to Burali-Forti-like paradoxes.

---

### TT.256: Universe Polymorphism
**Definition:** Definitions quantified over universe levels.
**Cost Model:** Universe constraint solving at use sites.
**Real Wall:** Coq, Agda, Lean 4 implement.
**Cross-Domain Aliases:** universe_poly, level_polymorphism
**Notes:** Required for category-theoretic libraries.

---

### TT.257: Cumulativity
**Definition:** Lower universes embed in higher ones: Type_i ⊆ Type_{i+1}.
**Cost Model:** Simplifies universe management; implicit lifts.
**Real Wall:** Coq has cumulativity; Agda does not.
**Cross-Domain Aliases:** universe_cumulativity, universe_inclusion
**Notes:** Avoids explicit lift operations.

---

### TT.258: Russell-Style Universes
**Definition:** Universes whose elements are themselves types: A : Type implies A is-a type.
**Cost Model:** Cleaner notation; implicit decoding.
**Real Wall:** Most modern proof assistants.
**Cross-Domain Aliases:** russell_universe, no_decoding
**Notes:** Easier to use than Tarski-style.

---

### TT.259: Tarski-Style Universes
**Definition:** Universes contain codes for types, with explicit decoding function El.
**Cost Model:** Explicit El(a) operation reveals meaning.
**Real Wall:** Used in foundational works (Martin-Löf 1984).
**Cross-Domain Aliases:** tarski_universe, code_decode
**Notes:** Models more transparent metatheoretically.

---

## Section: GADTs and Refinements

### TT.260: Generalized Algebraic Data Types (GADTs)
**Definition:** Algebraic data types whose constructors return specific type instances.
**Cost Model:** Pattern matching refines outer type variable.
**Real Wall:** Haskell, OCaml; Cheney-Hinze; Xi et al.
**Cross-Domain Aliases:** gadts, generalized_adts
**Notes:** Capture inductive families in non-dependent languages.

---

### TT.261: Refinement Types
**Definition:** Types augmented with predicates: {x:Int | x > 0}.
**Cost Model:** SMT solver for refinement implication.
**Real Wall:** Liquid Haskell, F*, Refinement ML.
**Cross-Domain Aliases:** liquid_types, predicate_subtyping
**Notes:** Rondon-Kawaguchi-Jhala; verification without dep types.

---

### TT.262: Dependent Records
**Definition:** Records where later field types depend on earlier field values.
**Cost Model:** Iterated Sigma-types.
**Real Wall:** Σ-telescopes in proof assistants.
**Cross-Domain Aliases:** dep_records, telescope_records
**Notes:** Encode algebraic structures (groups, rings) as records.

---

### TT.263: Telescopes
**Definition:** Sequences of dependent bindings (x₁:A₁) (x₂:A₂(x₁)) ... used for contexts.
**Cost Model:** Linear in telescope length.
**Real Wall:** de Bruijn's original notion.
**Cross-Domain Aliases:** dep_context, dep_telescope
**Notes:** Foundation of context manipulation in type theory.

---

### TT.264: Mutual Inductive Types
**Definition:** Set of inductive types defined simultaneously, with mutual constructors.
**Cost Model:** Joint termination/positivity check.
**Real Wall:** Forests/trees; even/odd; expressions/statements.
**Cross-Domain Aliases:** mutual_inductive, mutual_recursion_types
**Notes:** Schemes generated jointly.

---

### TT.265: Coinductive Types
**Definition:** Types defined by their observations; greatest fixed points.
**Cost Model:** Productivity check replaces termination.
**Real Wall:** Streams, infinite trees, processes.
**Cross-Domain Aliases:** coinductive, codata, greatest_fixpoint
**Notes:** Coq's CoFixpoint; copatterns (Abel et al.).

---

### TT.266: Indexed Coinductive Types
**Definition:** Coinductive types parameterized by indices: e.g., trace types by state.
**Cost Model:** Indices vary through observations.
**Real Wall:** Used for behavioral specifications.
**Cross-Domain Aliases:** indexed_coinductive, coinductive_families
**Notes:** Dual of indexed inductive types.

---

## Section: Cubical and Homotopy Type Theory

### TT.267: Homotopy Type Theory (HoTT)
**Definition:** Type theory interpreting types as spaces and equality as paths.
**Cost Model:** Univalence axiom blocks computation (in book HoTT).
**Real Wall:** HoTT Book 2013; Voevodsky.
**Cross-Domain Aliases:** hott, homotopy_tt
**Notes:** Marries type theory with homotopy theory.

---

### TT.268: Univalence Axiom
**Definition:** (A ≃ B) ≃ (A = B); equivalent types are identical.
**Cost Model:** Postulated in book HoTT; lacks computational content.
**Real Wall:** Voevodsky 2009; cubical type theory makes it compute.
**Cross-Domain Aliases:** univalence, voevodsky_axiom
**Notes:** Implies function extensionality.

---

### TT.269: Cubical Type Theory
**Definition:** Constructive type theory where univalence computes via interval and Kan operations.
**Cost Model:** Cubical operations (coe, hcomp) add reduction rules.
**Real Wall:** Cohen-Coquand-Huber-Mörtberg 2017; Cubical Agda.
**Cross-Domain Aliases:** cubical_tt, cctt, cubical_agda
**Notes:** Path-from-interval; tactic-free univalence.

---

### TT.270: Higher Inductive Types (HITs)
**Definition:** Inductive types with constructors producing paths, not just points.
**Cost Model:** Generalize inductive types with path constructors.
**Real Wall:** Lumsdaine, Shulman; circles, suspensions, quotients.
**Cross-Domain Aliases:** hits, higher_inductive
**Notes:** Quotient types as HITs; truncations.

---

### TT.271: Path Types
**Definition:** PathP A x y: type of paths from x to y in (possibly dependent) type A.
**Cost Model:** Builtin in cubical type theory.
**Real Wall:** Replaces inductively defined Id-type.
**Cross-Domain Aliases:** path_type, pathp
**Notes:** Function-from-interval representation.

---

### TT.272: Transport
**Definition:** Function moving a term along an equality: A = B implies A → B.
**Cost Model:** O(complexity of path).
**Real Wall:** Built from J-rule or path application.
**Cross-Domain Aliases:** transport, subst, coerce
**Notes:** Foundational operation.

---

### TT.273: J-Rule (Path Induction)
**Definition:** Induction principle for identity types: prove a property holds for all paths by proving it for reflexivity.
**Cost Model:** Single eliminator for equality.
**Real Wall:** Martin-Löf 1975.
**Cross-Domain Aliases:** j_rule, path_induction, id_elim
**Notes:** Equivalent forms: based J (Paulin-Mohring) and unbased J.

---

## Section: Linear and Substructural Types

### TT.274: Linear Types
**Definition:** Each variable must be used exactly once; no contraction or weakening.
**Cost Model:** Static usage tracking.
**Real Wall:** Girard 1987; Wadler 1990 in PL.
**Cross-Domain Aliases:** linear_types, exactly_once
**Notes:** Linear Haskell, ATS, Idris linearity quantifiers.

---

### TT.275: Affine Types
**Definition:** Each variable used at most once; allows weakening but not contraction.
**Cost Model:** Static usage tracking; less rigid than linear.
**Real Wall:** Rust ownership; uniqueness types.
**Cross-Domain Aliases:** affine_types, at_most_once
**Notes:** Rust borrow checker enforces affine ownership.

---

### TT.276: Relevant Types
**Definition:** Each variable used at least once; allows contraction but not weakening.
**Cost Model:** Static usage tracking.
**Real Wall:** Less common in PL; appears in relevance logic.
**Cross-Domain Aliases:** relevant_types, at_least_once
**Notes:** Dual of affine: discard forbidden.

---

### TT.277: Ordered Types
**Definition:** Variables must be used exactly once and in declaration order.
**Cost Model:** Stack-discipline usage.
**Real Wall:** Polakow-Pfenning; non-commutative linear logic.
**Cross-Domain Aliases:** ordered_types, non_commutative_linear
**Notes:** Models stack-based or string-rewriting computation.

---

### TT.278: Bunched Implications
**Definition:** Logic with two contexts: multiplicative (linear) and additive (intuitionistic).
**Cost Model:** Two implication connectives: -* and →.
**Real Wall:** O'Hearn-Pym 1999.
**Cross-Domain Aliases:** bunched_logic, bi_logic
**Notes:** Foundation of separation logic.

---

### TT.279: Separation Logic Types
**Definition:** Types track ownership of disjoint heap fragments; * for separating conjunction.
**Cost Model:** Frame inference; resource tracking.
**Real Wall:** Reynolds, O'Hearn; Iris framework.
**Cross-Domain Aliases:** sep_logic_types, separation_types
**Notes:** Foundation of modern program verification.

---

### TT.280: Uniqueness Types
**Definition:** Types ensuring a value has unique reference; permits in-place updates.
**Cost Model:** Static reference counting at type level.
**Real Wall:** Clean language; Mercury modes.
**Cross-Domain Aliases:** unique_types, uniqueness
**Notes:** Dual perspective on linearity, more permissive.

---

## Section: Effect Systems and Algebraic Effects

### TT.281: Algebraic Effects
**Definition:** Computational effects presented as algebraic operations with equations.
**Cost Model:** Free monad over signature; handlers provide interpretation.
**Real Wall:** Plotkin-Power 2001; Plotkin-Pretnar.
**Cross-Domain Aliases:** alg_effects, algebraic_effects
**Notes:** Effects = operations; handlers = models.

---

### TT.282: Effect Handlers
**Definition:** First-class operators that interpret algebraic operations in computation.
**Cost Model:** Captures and resumes delimited continuations.
**Real Wall:** Eff, Koka, OCaml 5, Frank, Multicore OCaml.
**Cross-Domain Aliases:** effect_handlers, eff_handlers
**Notes:** Pretnar 2010; generalizes exceptions and continuations.

---

### TT.283: Effect Rows
**Definition:** Row-polymorphic representation of effect sets: ⟨exn, st | ρ⟩.
**Cost Model:** Row unification; scoped labels (Leijen).
**Real Wall:** Koka uses row-typed effects.
**Cross-Domain Aliases:** effect_rows, row_effects
**Notes:** Daan Leijen; tracks effect set in type.

---

### TT.284: Polymorphic Effects
**Definition:** Effect-quantified types: ∀ε. (A → B!ε) → ... allowing effect-polymorphic combinators.
**Cost Model:** Polymorphism extended to effect variables.
**Real Wall:** Tofte-Talpin region inference; Koka, F*.
**Cross-Domain Aliases:** poly_effects, effect_polymorphism
**Notes:** Function composition needs effect polymorphism.

---

### TT.285: Region Inference
**Definition:** Static analysis assigning regions to allocations for safe stack-like deallocation.
**Cost Model:** Constraint-based inference; per-allocation region.
**Real Wall:** Tofte-Talpin 1994; MLKit.
**Cross-Domain Aliases:** region_inference, tofte_talpin
**Notes:** Replaces GC with region-based memory.

---

### TT.286: Region Types
**Definition:** Types annotated with regions tracking lifetime of values.
**Cost Model:** Region variables in types; subregion ordering.
**Real Wall:** Cyclone language; precursor to Rust lifetimes.
**Cross-Domain Aliases:** region_types, lifetime_types
**Notes:** Grossman-Hicks-Jim et al.

---

### TT.287: Capability-Based Types
**Definition:** Types carrying first-class capabilities granting access to resources.
**Cost Model:** Capabilities consumed/duplicated via substructural rules.
**Real Wall:** Walker-Crary-Morrisett; Pony language; E.
**Cross-Domain Aliases:** capability_types, ocap_types
**Notes:** Foundation of object-capability security.

---

## Section: Type Inference Algorithms

### TT.288: Algorithm W
**Definition:** Hindley-Milner type inference walking syntax tree and unifying.
**Cost Model:** Almost-linear with union-find; pathological exponential cases.
**Real Wall:** Damas-Milner 1982; canonical ML inference.
**Cross-Domain Aliases:** algorithm_w, damas_milner
**Notes:** Substitution-based; eager.

---

### TT.289: Algorithm M
**Definition:** Top-down variant of Algorithm W passing expected type as input.
**Cost Model:** Comparable to W; better error locality.
**Real Wall:** Lee-Yi 1998.
**Cross-Domain Aliases:** algorithm_m, top_down_hm
**Notes:** Sometimes better error messages.

---

### TT.290: Bidirectional Type Checking
**Definition:** Alternates between type-checking (check) and type-synthesis (infer) modes.
**Cost Model:** Linear in syntax; avoids expensive unification when possible.
**Real Wall:** Pierce-Turner 2000.
**Cross-Domain Aliases:** bidirectional, check_infer
**Notes:** Scales to dependent types.

---

### TT.291: Constraint-Based Type Inference
**Definition:** Generate constraints during traversal, then solve.
**Cost Model:** Modular; separates generation from solving.
**Real Wall:** HM(X) framework; Sulzmann.
**Cross-Domain Aliases:** constraint_inference, ci_inference
**Notes:** Foundation for OutsideIn(X).

---

### TT.292: OutsideIn(X) Algorithm
**Definition:** GHC's constraint-based inference: solve given constraints outside-in, never inside binders.
**Cost Model:** Specialized to handle GADTs and type families.
**Real Wall:** Vytiniotis et al. 2011.
**Cross-Domain Aliases:** outsidein, ghc_inference
**Notes:** Maintains predictable error behavior.

---

### TT.293: Local Type Inference
**Definition:** Type inference using only local information; doesn't propagate globally.
**Cost Model:** Modular; requires more annotations.
**Real Wall:** Pierce-Turner; Scala, Kotlin use variants.
**Cross-Domain Aliases:** local_inference, scala_inference
**Notes:** Combines with bidirectional checking.

---

### TT.294: Damas-Milner Restriction
**Definition:** Only let-bound expressions can be generalized to polymorphic types.
**Cost Model:** Ensures principal types exist.
**Real Wall:** Foundation of let-polymorphism.
**Cross-Domain Aliases:** dm_restriction, let_polymorphism
**Notes:** Lambda-bound variables remain monomorphic.

---

### TT.295: Value Restriction
**Definition:** Polymorphism restricted to syntactic values to ensure soundness with mutability.
**Cost Model:** Conservative; rules out polymorphism for some expressions.
**Real Wall:** Wright 1995; SML adopts.
**Cross-Domain Aliases:** value_restriction, wright_restriction
**Notes:** Prevents reference cell type unsoundness.

---

## Section: Monads in Programming Languages

### TT.296: IO Monad
**Definition:** Monadic encapsulation of input/output effects, preserving referential transparency.
**Cost Model:** State-passing world or sequencing primitives.
**Real Wall:** Wadler; Haskell core feature.
**Cross-Domain Aliases:** io_monad, world_monad
**Notes:** "Action that, when performed, produces a value."

---

### TT.297: State Monad
**Definition:** Monad threading a value through computations: State s a = s → (a, s).
**Cost Model:** Pure encoding; potential overhead vs mutation.
**Real Wall:** Standard Haskell pattern.
**Cross-Domain Aliases:** state_monad, threaded_state
**Notes:** ST monad for safe local mutable state.

---

### TT.298: Reader Monad
**Definition:** Monad providing read-only environment: Reader r a = r → a.
**Cost Model:** Environment passing; cheap.
**Real Wall:** Dependency injection in functional code.
**Cross-Domain Aliases:** reader_monad, env_monad
**Notes:** ask, local primitives.

---

### TT.299: Writer Monad
**Definition:** Accumulates a log value alongside computation: Writer w a = (a, w).
**Cost Model:** Requires monoid for log; lazy log can leak space.
**Real Wall:** Strict variant typically needed.
**Cross-Domain Aliases:** writer_monad, log_monad
**Notes:** tell primitive appends to log.

---

### TT.300: Maybe/Option Monad
**Definition:** Computations potentially yielding nothing: Maybe a = Nothing | Just a.
**Cost Model:** Short-circuits on Nothing.
**Real Wall:** Pervasive in functional languages.
**Cross-Domain Aliases:** maybe_monad, option_monad
**Notes:** Tony Hoare's "billion-dollar mistake" alternative.

---

### TT.301: Either Monad
**Definition:** Computations yielding success or labeled error: Either e a.
**Cost Model:** Short-circuits on Left.
**Real Wall:** Used for error handling.
**Cross-Domain Aliases:** either_monad, result_monad
**Notes:** Generalizes Maybe with informative errors.

---

### TT.302: List Monad
**Definition:** Monad of nondeterministic computations: List a = [a].
**Cost Model:** Cartesian product semantics.
**Real Wall:** List comprehensions desugar to this.
**Cross-Domain Aliases:** list_monad, nondet_monad
**Notes:** Generalizes to MonadPlus.

---

### TT.303: Continuation Monad
**Definition:** Monad of computations in continuation-passing style: Cont r a = (a → r) → r.
**Cost Model:** Manipulates control flow first-class.
**Real Wall:** Foundation of callcc semantics.
**Cross-Domain Aliases:** cont_monad, cps_monad
**Notes:** "Mother of all monads" (Hutton).

---

### TT.304: Free Monad
**Definition:** Inductively constructed monad over a functor: Free f a = Pure a | Free (f (Free f a)).
**Cost Model:** Walks data structure to interpret; can be slow.
**Real Wall:** Used for DSLs and effect modeling.
**Cross-Domain Aliases:** free_monad, term_monad
**Notes:** Initial monad over a functor.

---

### TT.305: Cofree Comonad
**Definition:** Dual of Free monad: streams of values labeled with functor structure.
**Cost Model:** Lazy infinite structure.
**Real Wall:** Used for histories, annotated ASTs.
**Cross-Domain Aliases:** cofree_comonad, terminal_comonad
**Notes:** Terminal comonad over a functor.

---

### TT.306: Codensity Monad
**Definition:** Right-Kan-extension based monad transformation: Codensity m a = ∀b. (a → m b) → m b.
**Cost Model:** Improves asymptotic performance of free monads.
**Real Wall:** Voigtländer 2008.
**Cross-Domain Aliases:** codensity, kan_monad
**Notes:** Defunctionalizes left-nested binds.

---

### TT.307: Monad Transformers
**Definition:** Type constructors stacking monads: StateT s m a, ReaderT r m a, etc.
**Cost Model:** Per-layer overhead; n-squared lift instances.
**Real Wall:** Liang-Hudak-Jones 1995; mtl library.
**Cross-Domain Aliases:** monad_transformers, mtl_style
**Notes:** Classes select layer by effect type.

---

### TT.308: Mtl-Style Effects
**Definition:** Effects encoded as type-class constraints: MonadState s m, MonadReader r m.
**Cost Model:** Dictionary passing; class-resolution overhead.
**Real Wall:** Haskell's mtl library.
**Cross-Domain Aliases:** mtl_style, classy_effects
**Notes:** Composes well; n² instance problem.

---

### TT.309: Polysemy / Extensible Effects
**Definition:** Effect system using open unions instead of monad stacks.
**Cost Model:** Effect dispatch via type-level open union.
**Real Wall:** Kiselyov-Ishii 2015; freer-simple, polysemy.
**Cross-Domain Aliases:** extensible_effects, polysemy_lib
**Notes:** Avoids transformer order issues.

---

### TT.310: Eff Monad
**Definition:** Free monad over effect signatures with handlers.
**Cost Model:** Comparable to mtl with optimization.
**Real Wall:** Kammar-Lindley-Oury; Eff language.
**Cross-Domain Aliases:** eff_monad, eff_lang
**Notes:** Direct embedding of algebraic effects.

---

## Section: Categorical Semantics

### TT.311: Cartesian Closed Category (CCC)
**Definition:** Category with finite products and exponential objects modeling lambda calculus.
**Cost Model:** Models STLC categorically.
**Real Wall:** Lambek-Scott 1986.
**Cross-Domain Aliases:** ccc, cart_closed
**Notes:** Lambda calculus = internal language of CCC.

---

### TT.312: Cartesian Category
**Definition:** Category with finite products (terminal object and binary products).
**Cost Model:** Pairing and projection.
**Real Wall:** Foundation for product types.
**Cross-Domain Aliases:** cartesian_cat, products_cat
**Notes:** Without exponentials yields combinatory logic without abstraction.

---

### TT.313: Monoidal Category
**Definition:** Category with tensor product and unit, associative and unital up to iso.
**Cost Model:** Tensor not necessarily cartesian.
**Real Wall:** Models linear/affine types.
**Cross-Domain Aliases:** monoidal_cat, tensor_cat
**Notes:** Symmetric monoidal closed for MLL.

---

### TT.314: Symmetric Monoidal Closed Category
**Definition:** Monoidal category with symmetric tensor and internal hom.
**Cost Model:** Models multiplicative linear logic.
**Real Wall:** Seely; coherence theorems.
**Cross-Domain Aliases:** smcc, sym_mon_closed
**Notes:** Semantics of MLL.

---

### TT.315: Fibrations (Type Theory)
**Definition:** Functor with cartesian lifting modeling dependent types.
**Cost Model:** Slicing over context category.
**Real Wall:** Jacobs 1999; categorical semantics of dep types.
**Cross-Domain Aliases:** fibrations, grothendieck_fib
**Notes:** Categories with families (CwF) related.

---

### TT.316: Sheaf Semantics
**Definition:** Semantics where types are sheaves over a site.
**Cost Model:** Sheaf condition: gluable consistent local data.
**Real Wall:** Lawvere-Tierney; topos-theoretic semantics.
**Cross-Domain Aliases:** sheaf_sem, topos_sem
**Notes:** Used for intuitionistic and modal logics.

---

## Section: Denotational Semantics

### TT.317: Domain Theory
**Definition:** Mathematical theory of partial orders modeling computation with bottom (undefined).
**Cost Model:** Continuous functions; least fixed points.
**Real Wall:** Scott 1969; Strachey-Scott semantics.
**Cross-Domain Aliases:** domain_theory, scott_domain
**Notes:** Models recursive types and recursion.

---

### TT.318: Scott Domains
**Definition:** Bounded-complete algebraic CPOs used for semantic models.
**Cost Model:** Compact elements approximate; directed sups exist.
**Real Wall:** Solves recursive domain equations.
**Cross-Domain Aliases:** scott_domains, bc_dcpo
**Notes:** D ≅ [D → D] solution for untyped lambda.

---

### TT.319: Directed-Complete Partial Orders (DCPO)
**Definition:** Posets where every directed subset has a least upper bound.
**Cost Model:** Forms category with Scott-continuous maps.
**Real Wall:** Standard category of domains.
**Cross-Domain Aliases:** dcpo, dcpos
**Notes:** Models partial functions naturally.

---

### TT.320: Scott-Continuous Functions
**Definition:** Functions preserving directed sups; monotone and preserve approximation.
**Cost Model:** Computable functions are continuous.
**Real Wall:** Foundation of denotational semantics.
**Cross-Domain Aliases:** scott_continuous, omega_continuous
**Notes:** Continuity = computability in this setting.

---

### TT.321: Fixed-Point Semantics
**Definition:** Recursive programs interpreted as least fixed points of continuous functionals.
**Cost Model:** Kleene-ascending chain converges to fixpoint.
**Real Wall:** Foundation for recursion.
**Cross-Domain Aliases:** fixpoint_semantics, lfp_semantics
**Notes:** μF = ⊔_n F^n(⊥).

---

### TT.322: Knaster-Tarski Fixed-Point Theorem
**Definition:** Every monotone endo-function on a complete lattice has a least and greatest fixed point.
**Cost Model:** Existence-only; not constructive.
**Real Wall:** Used in static analysis.
**Cross-Domain Aliases:** knaster_tarski, kt_fixpoint
**Notes:** General-purpose; foundation of abstract interpretation.

---

### TT.323: Kleene Fixed-Point Theorem
**Definition:** On a pointed DCPO, continuous endo-functions have least fixed point reachable by iteration from bottom.
**Cost Model:** Constructive; iterates to LFP.
**Real Wall:** Algorithmic content of fixpoint semantics.
**Cross-Domain Aliases:** kleene_fixpoint, kt_iteration
**Notes:** Foundation of denotational semantics.

---

### TT.324: Plotkin Powerdomain
**Definition:** Powerdomain for modeling nondeterminism with both upper and lower constraints (convex).
**Cost Model:** Nondeterministic choice; nontrivial structure.
**Real Wall:** Plotkin 1976.
**Cross-Domain Aliases:** plotkin_powerdomain, convex_powerdomain
**Notes:** Convex powerdomain.

---

### TT.325: Smyth Powerdomain
**Definition:** Powerdomain modeling demonic nondeterminism (upper closure).
**Cost Model:** Refinement-ordered.
**Real Wall:** Smyth 1978.
**Cross-Domain Aliases:** smyth_powerdomain, upper_powerdomain
**Notes:** Models must-not-fail behaviors.

---

### TT.326: Hoare Powerdomain
**Definition:** Powerdomain modeling angelic nondeterminism (lower closure).
**Cost Model:** Total-correctness perspective.
**Real Wall:** Models may-succeed behavior.
**Cross-Domain Aliases:** hoare_powerdomain, lower_powerdomain
**Notes:** Dual to Smyth's.

---

## Section: Operational Semantics

### TT.327: Small-Step Semantics
**Definition:** Reduction relation taking expression one step closer to value.
**Cost Model:** One reduction at a time; many steps to value.
**Real Wall:** Plotkin's SOS.
**Cross-Domain Aliases:** small_step, sos
**Notes:** Models intermediate states.

---

### TT.328: Big-Step Semantics
**Definition:** Evaluation relation directly relating expression to its value.
**Cost Model:** One judgment per term; recursive on syntax.
**Real Wall:** Kahn's natural semantics.
**Cross-Domain Aliases:** big_step, natural_semantics
**Notes:** Cannot describe nontermination directly.

---

### TT.329: Structural Operational Semantics (SOS)
**Definition:** Operational semantics defined by inductive rules over syntax.
**Cost Model:** Standard small-step framework.
**Real Wall:** Plotkin 1981 Aarhus notes.
**Cross-Domain Aliases:** sos_semantics, plotkin_sos
**Notes:** Standard framework for PL semantics.

---

### TT.330: Reduction Semantics
**Definition:** Operational semantics via evaluation contexts and contraction rules.
**Cost Model:** Decomposition into context + redex.
**Real Wall:** Felleisen-Hieb; refocusing optimization.
**Cross-Domain Aliases:** reduction_sem, eval_context_sem
**Notes:** Modular and concise; foundation of Redex tool.

---

### TT.331: Evaluation Contexts
**Definition:** Syntactic frames identifying where the next reduction occurs.
**Cost Model:** Decompose term into E[redex]; linear in term size.
**Real Wall:** Felleisen-Hieb 1992.
**Cross-Domain Aliases:** eval_contexts, redex_context
**Notes:** Foundation of context-sensitive reduction.

---

## Section: Game Semantics

### TT.332: Game Semantics
**Definition:** Interpretation of types as games and programs as strategies.
**Cost Model:** Innocent strategies; well-bracketed plays.
**Real Wall:** Abramsky-Jagadeesan-Malacaria, Hyland-Ong 1994.
**Cross-Domain Aliases:** game_semantics, ajm_semantics
**Notes:** First full abstraction for PCF.

---

### TT.333: Geometry of Interaction
**Definition:** Categorical interpretation of cut-elimination as token-passing in graphs.
**Cost Model:** Particle traversing proof net.
**Real Wall:** Girard 1989.
**Cross-Domain Aliases:** goi, geom_interaction
**Notes:** Operator-algebraic representation of computation.

---

### TT.334: Full Abstraction
**Definition:** Equivalence in semantics matches observational equivalence in language.
**Cost Model:** Hard property to achieve.
**Real Wall:** Milner 1977 for PCF; resolved by game semantics 1994.
**Cross-Domain Aliases:** full_abstraction, ok_match
**Notes:** Plotkin's PCF + parallel-or problem.

---

## Section: Recursion Schemes

### TT.335: Structural Recursion
**Definition:** Recursion guaranteed to terminate by recursing on proper subterm.
**Cost Model:** Termination by structural induction.
**Real Wall:** Coq, Agda guard checker.
**Cross-Domain Aliases:** structural_recursion, primitive_recursion
**Notes:** Foundation of recursion in proof assistants.

---

### TT.336: Well-Founded Recursion
**Definition:** Recursion proven terminating by a well-founded order on argument.
**Cost Model:** Requires accessibility proof.
**Real Wall:** General recursion in Coq via Fix/Acc.
**Cross-Domain Aliases:** wf_recursion, well_founded_rec
**Notes:** Subsumes structural recursion.

---

### TT.337: Sized Types
**Definition:** Types annotated with sizes proving termination.
**Cost Model:** Size constraints checked at type-check.
**Real Wall:** Hughes-Pareto-Sabry; Agda has sized types.
**Cross-Domain Aliases:** sized_types, agda_sizes
**Notes:** Productivity proof for coinductive.

---

### TT.338: Primitive Recursion
**Definition:** Recursion scheme of natural numbers: f(0)=g, f(n+1)=h(n, f(n)).
**Cost Model:** Computable; subset of total functions.
**Real Wall:** Gödel's T computes exactly primitive recursive functions.
**Cross-Domain Aliases:** prim_rec, primitive_rec
**Notes:** Gödel's System T.

---

### TT.339: μ-Types (Iso-Recursive)
**Definition:** Recursive types where μX.τ ≅ τ[μX.τ/X] only via explicit fold/unfold.
**Cost Model:** Explicit coercion at each unrolling.
**Real Wall:** Common in formal type theory presentations.
**Cross-Domain Aliases:** iso_recursive, mu_types
**Notes:** Pierce TAPL.

---

### TT.340: Equi-Recursive Types
**Definition:** Recursive types where μX.τ = τ[μX.τ/X] by definitional equality.
**Cost Model:** Requires regular tree comparison.
**Real Wall:** OCaml objects use equi-recursive types.
**Cross-Domain Aliases:** equi_recursive, equirec_types
**Notes:** Equality checking becomes regular-tree equality.

---

### TT.341: Y Combinator
**Definition:** Fixed-point combinator: Y = λf.(λx.f(x x))(λx.f(x x)).
**Cost Model:** Yields fixed-point of any function.
**Real Wall:** Curry; foundation of recursion in untyped lambda.
**Cross-Domain Aliases:** y_combinator, curry_y
**Notes:** Self-application causes non-termination if reduced eagerly.

---

### TT.342: Z Combinator
**Definition:** Strict fixed-point combinator: Z = λf.(λx.f(λv.x x v))(λx.f(λv.x x v)).
**Cost Model:** Eta-expanded Y; works in call-by-value.
**Real Wall:** Required for strict languages.
**Cross-Domain Aliases:** z_combinator, strict_y
**Notes:** Variant of Y.

---

### TT.343: Turing Fixed-Point Combinator
**Definition:** Θ = (λxy. y (x x y))(λxy. y (x x y)).
**Cost Model:** Self-applicative fixed-point operator.
**Real Wall:** Turing 1937.
**Cross-Domain Aliases:** turing_combinator, theta_combinator
**Notes:** Alternative to Y; self-replicating structure.

---

## Section: Type-Level Computation

### TT.344: Type Families
**Definition:** Functions at type level, defined by pattern matching on types.
**Cost Model:** Confluence and termination required for soundness.
**Real Wall:** GHC's TypeFamilies extension.
**Cross-Domain Aliases:** type_families, type_functions
**Notes:** Open and closed type families.

---

### TT.345: Associated Types
**Definition:** Type families parameterized by class instances.
**Cost Model:** Resolved with class dispatch.
**Real Wall:** Chakravarty-Keller-Peyton Jones 2005.
**Cross-Domain Aliases:** associated_types, class_assoc_types
**Notes:** Rust associated types; Haskell type families in classes.

---

### TT.346: Kind Polymorphism
**Definition:** Polymorphism over kinds (types of types).
**Cost Model:** Kind variables in signatures.
**Real Wall:** PolyKinds in GHC; Coq/Agda built-in.
**Cross-Domain Aliases:** kind_poly, polykinds
**Notes:** Yorgey-Weirich.

---

### TT.347: Type-Level Naturals
**Definition:** Promoted naturals (Z, S Z, S (S Z)) used at type level.
**Cost Model:** Arithmetic via type families.
**Real Wall:** DataKinds in GHC; Nat in Coq/Agda.
**Cross-Domain Aliases:** type_nats, promoted_naturals
**Notes:** Foundation of length-indexed vectors.

---

### TT.348: Type-Level Strings (Symbols)
**Definition:** Promoted strings/symbols usable at type level.
**Cost Model:** Used for labels, error messages.
**Real Wall:** GHC's Symbol kind; static labels.
**Cross-Domain Aliases:** type_strings, symbol_kind
**Notes:** Used in row-polymorphic records.

---

### TT.349: Singleton Types
**Definition:** Types with exactly one inhabitant, used to link types and values.
**Cost Model:** Encoding of value at type level.
**Real Wall:** singletons library; SNat n.
**Cross-Domain Aliases:** singletons, singleton_types
**Notes:** Eisenberg-Weirich.

---

## Section: Records and Rows

### TT.350: Row Polymorphism
**Definition:** Polymorphism over rows of labeled fields in records or variants.
**Cost Model:** Row unification.
**Real Wall:** Wand 1987; Rémy 1989.
**Cross-Domain Aliases:** row_polymorphism, row_poly
**Notes:** Foundation of extensible records.

---

### TT.351: Record Concatenation
**Definition:** Operation combining two records into one with union of fields.
**Cost Model:** Type-level field union; can require disjoint check.
**Real Wall:** Wand 1989; tricky type system interactions.
**Cross-Domain Aliases:** record_concat, record_union
**Notes:** Encoding via row variables.

---

### TT.352: Record Extension
**Definition:** Adding a new field to an existing record.
**Cost Model:** Functional update; O(1) with row types.
**Real Wall:** Leijen's scoped labels.
**Cross-Domain Aliases:** record_extension, row_extend
**Notes:** Trees that grow encoding.

---

### TT.353: Scoped Labels
**Definition:** Row system allowing duplicate labels with leftmost binding (Leijen).
**Cost Model:** No constraint solving; simple unification.
**Real Wall:** Daan Leijen 2005.
**Cross-Domain Aliases:** scoped_labels, leijen_rows
**Notes:** Implementation in Koka and Roc.

---

### TT.354: Anonymous Records
**Definition:** Records without nominal declaration, identified by structure.
**Cost Model:** Structural typing comparison.
**Real Wall:** TypeScript, F# anonymous records.
**Cross-Domain Aliases:** anon_records, structural_records
**Notes:** Reduce boilerplate.

---

## Section: Object Calculi and Type Classes

### TT.355: Abadi-Cardelli Object Calculus
**Definition:** Calculus modeling objects as records of fields and methods with self-reference.
**Cost Model:** Self-types and method update.
**Real Wall:** Abadi-Cardelli 1996.
**Cross-Domain Aliases:** object_calculus, abadi_cardelli
**Notes:** Theory of objects.

---

### TT.356: Multiple Dispatch
**Definition:** Method selection based on dynamic types of multiple arguments.
**Cost Model:** O(args × specificity comparisons).
**Real Wall:** CLOS, Julia, Dylan.
**Cross-Domain Aliases:** multimethods, multiple_dispatch
**Notes:** Julia exemplifies as core abstraction.

---

### TT.357: Mixins
**Definition:** Class fragments composable into other classes via inheritance.
**Cost Model:** Linearization of inheritance.
<br>**Real Wall:** Bracha-Cook 1990; Scala traits.
**Cross-Domain Aliases:** mixins, mixin_classes
**Notes:** Avoid diamond problem via linearization.

---

### TT.358: Traits
**Definition:** Composable units of behavior without state, used for code reuse.
**Cost Model:** Composition without conflict resolution required.
**Real Wall:** Scharli et al. 2003; Rust traits; Scala.
**Cross-Domain Aliases:** traits, rust_traits
**Notes:** Stronger guarantees than mixins.

---

### TT.359: Type Classes (Wadler-Blott)
**Definition:** Ad-hoc polymorphism via type class declarations and instances.
**Cost Model:** Dictionary-passing translation.
**Real Wall:** Wadler-Blott 1989.
**Cross-Domain Aliases:** type_classes, wadler_blott
**Notes:** Haskell's central abstraction.

---

### TT.360: Implicits (Scala)
**Definition:** Implicit parameters and conversions resolved at compile time.
**Cost Model:** Implicit search may be complex.
**Real Wall:** Scala; given/using in Scala 3.
**Cross-Domain Aliases:** implicits, given_using
**Notes:** Subsumes type classes.

---

### TT.361: Modular Type Classes
**Definition:** Type classes expressed as ML-style modules.
**Cost Model:** Module-based dispatch.
**Real Wall:** Dreyer-Harper-Chakravarty.
**Cross-Domain Aliases:** modular_type_classes, ml_type_classes
**Notes:** Bridge between type classes and modules.

---

## Section: Module Systems

### TT.362: ML Modules
**Definition:** Structures (collections of types/values), signatures (interfaces), functors (parameterized modules).
**Cost Model:** Compile-time module elaboration.
**Real Wall:** MacQueen, Harper, Tofte; SML/OCaml.
**Cross-Domain Aliases:** ml_modules, sml_modules
**Notes:** Most expressive practical module system.

---

### TT.363: Applicative Functors (Module-Level)
**Definition:** Functors where applying the same arguments yields equal modules.
**Cost Model:** OCaml default; sharing constraints.
**Real Wall:** Leroy 1995.
**Cross-Domain Aliases:** applicative_functors_modules, oc_functors
**Notes:** vs generative functors.

---

### TT.364: Generative Functors
**Definition:** Functor applications produce fresh abstract types each time.
**Cost Model:** SML default behavior.
**Real Wall:** OCaml supports both via syntax.
**Cross-Domain Aliases:** generative_functors, sml_functors
**Notes:** Allows unique abstract types per application.

---

### TT.365: Signature Sealing / Ascription
**Definition:** Restricting a module's interface by ascribing it a signature.
**Cost Model:** Hides components not in signature.
**Real Wall:** Translucent vs opaque sealing.
**Cross-Domain Aliases:** sealing, signature_ascription
**Notes:** Foundation of abstraction in ML.

---

### TT.366: First-Class Modules
**Definition:** Modules as runtime values that can be passed and stored.
**Cost Model:** Packing/unpacking; existential-like.
**Real Wall:** OCaml `(module M : S)`; Rossberg's 1ML.
**Cross-Domain Aliases:** first_class_modules, 1ml
**Notes:** Unifies modules and core language.

---

## Section: Gradual Typing

### TT.367: Gradual Type System
**Definition:** Type system supporting fluid integration of typed and untyped code.
**Cost Model:** Dynamic type ? consistent with all types.
**Real Wall:** Siek-Taha 2006.
**Cross-Domain Aliases:** gradual_types, gradual_typing
**Notes:** TypeScript, Reticulated Python, Typed Racket.

---

### TT.368: Blame Calculus
**Definition:** Calculus tracking which side of a contract violation is at fault.
**Cost Model:** Blame labels propagated through casts.
**Real Wall:** Wadler-Findler 2009.
**Cross-Domain Aliases:** blame_calc, well_typed_no_blame
**Notes:** "Well-typed programs can't be blamed" theorem.

---

### TT.369: Contracts (Design by Contract)
**Definition:** Runtime checks expressing preconditions, postconditions, invariants.
**Cost Model:** Runtime overhead at each contract boundary.
**Real Wall:** Meyer's Eiffel; Findler-Felleisen formal contracts.
**Cross-Domain Aliases:** contracts, dbc
**Notes:** Higher-order contracts via wrappers.

---

### TT.370: Threesomes
**Definition:** Optimized representation of gradual casts requiring two-pass collapse.
**Cost Model:** Constant space per cast chain.
**Real Wall:** Siek-Wadler 2010.
**Cross-Domain Aliases:** threesomes, siek_wadler_casts
**Notes:** Improves on naive double-cast representation.
