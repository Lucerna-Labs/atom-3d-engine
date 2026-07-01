# Decision Logic — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.
>
> This catalog covers the OPERATIONAL/RUNTIME side of logic: policies, decisions, planners, classifiers, monitors, contracts. For pure formal logic (SAT/SMT/model checking/Hoare proof systems), see `logic-reasoning/`. For agent-level reasoning frameworks, see `agentic-reasoning/`.

---

## Section 1 — Rules & Policies

### atomic-rule (cross-domain alias: `predicate-action-pair`, `if-then-rule`, `production-rule`)
**Domain:** Decision Logic
**Definition:** Minimal `(antecedent, consequent)` pair: a predicate over working memory plus an action to fire when it matches.
**Atom or composite:** Atom — the irreducible unit of a production system.
**Cost model:** O(P) predicate eval; activation pushed onto agenda, O(log A) priority insert.
**Real wall?** Yes — undecidable for arbitrary first-order predicates; restricted to decidable fragments at runtime.
**Cross-domain wiring:** `logic-reasoning/horn-clause` (formal); `agentic-reasoning/condition-action`; `operating-systems/syscall-filter`.
**Notes:** Drools, CLIPS, JESS units. The "atom" of every PDP, every BT leaf, every audit log entry.

### rule-antecedent (cross-domain alias: `lhs`, `condition`, `guard-predicate`)
**Domain:** Decision Logic
**Definition:** Left-hand side of a rule — a logical formula over working memory facts that must be true for activation.
**Atom or composite:** Composite — typically a conjunction of pattern matches over typed facts.
**Cost model:** RETE alpha-node hashing makes amortized cost O(facts-touched) instead of O(rules x facts).
**Real wall?** Yes — pattern-match completeness vs. expressiveness tradeoff; full FOL antecedents are undecidable.
**Cross-domain wiring:** `logic-reasoning/conjunction`; `database-streaming-sketching/predicate-pushdown`.
**Notes:** Drools `when`-block. Tokens stored in alpha memory keyed by fact-type-hash.

### rule-consequent (cross-domain alias: `rhs`, `action`, `effect`)
**Domain:** Decision Logic
**Definition:** Right-hand side — the action taken when the antecedent matches: assert/retract facts, call external function, emit event.
**Atom or composite:** Atom (single effect) or composite (sequence of effects in one fire).
**Cost model:** O(effect-count); side-effects may invalidate cached matches → cascade re-eval.
**Real wall?** Yes — non-monotonic effects break confluence; rule order matters.
**Cross-domain wiring:** `logic-reasoning/inference-step`; `distributed-systems/idempotency`; `databases/transaction`.
**Notes:** Drools `then`-block. Often wrapped in transaction for rollback.

### policy-matrix (cross-domain alias: `decision-table`, `domain-lane-op-matrix`)
**Domain:** Decision Logic
**Definition:** Multi-dimensional lookup `Domain × Lane × Operation → Decision` materializing the full policy space as a table.
**Atom or composite:** Composite — Cartesian product of typed dimensions with default fallback cells.
**Cost model:** O(1) lookup with perfect hash; O(D·L·O) storage; combinatorial blow-up at high dimensionality.
**Real wall?** Yes — table explosion; consistency checking is co-NP-hard for overlapping cells.
**Cross-domain wiring:** `logic-reasoning/truth-table`; `linear-algebra-matrix/tensor-index`; `database-streaming-sketching/lookup-table`.
**Notes:** DMN decision tables; Drools spreadsheets. Compiles down to a BDD or DT.

### policy-evaluation (cross-domain alias: `pdp-eval`, `decision-request`)
**Domain:** Decision Logic
**Definition:** The act of computing a decision for a request `(subject, action, resource, env)` against the policy set.
**Atom or composite:** Composite — combines matching, conflict resolution, and obligation assembly.
**Cost model:** O(R·P) naive; sub-linear with indexing or partial-eval residuals.
**Real wall?** Yes — request latency budget (typically <1ms for inline PDP).
**Cross-domain wiring:** `logic-reasoning/sat-check`; `agentic-reasoning/action-selection`; `networking/acl-eval`.
**Notes:** OPA `data.policy.allow`; XACML `Evaluate`. Hot path → must be deterministic.

### policy-decision-point (cross-domain alias: `pdp`, `policy-engine`)
**Domain:** Decision Logic
**Definition:** The component that, given a request and a policy bundle, returns Permit/Deny/NotApplicable/Indeterminate plus obligations.
**Atom or composite:** Composite — request parser + matcher + combiner + obligation pipeline.
**Cost model:** ms-range; sharded by policy domain for horizontal scale.
**Real wall?** Yes — PDP availability becomes a SPOF; sidecar pattern required.
**Cross-domain wiring:** `distributed-systems/sidecar`; `logic-reasoning/sat-solver`; `operating-systems/lsm-hook`.
**Notes:** OPA, Cedar, AuthZed. Decouples decision from enforcement.

### policy-enforcement-point (cross-domain alias: `pep`, `interceptor`, `gate`)
**Domain:** Decision Logic
**Definition:** The choke-point in the data path that queries the PDP and enforces the returned decision (allow/block/transform).
**Atom or composite:** Atom — single in-line gate per resource.
**Cost model:** O(1) per call + PDP round-trip; caching brings to O(1) amortized.
**Real wall?** Yes — every PEP added increases tail latency; must be on critical path.
**Cross-domain wiring:** `networking/middleware`; `operating-systems/syscall-hook`; `distributed-systems/service-mesh`.
**Notes:** Envoy ext_authz, Istio AuthorizationPolicy, Kubernetes admission controller.

### policy-administration-point (cross-domain alias: `pap`, `policy-store`)
**Domain:** Decision Logic
**Definition:** The system of record for policies — editing, versioning, signing, distributing to PDPs.
**Atom or composite:** Composite — git repo + CI + bundle server + sync protocol.
**Cost model:** Bulk push O(policies); pulled lazily by PDP cache.
**Real wall?** Yes — policy staleness window; eventual consistency between PAP and PDPs.
**Cross-domain wiring:** `distributed-systems/config-distribution`; `cryptography-advanced/signed-bundle`.
**Notes:** OPA bundle server; AWS IAM management console; Styra DAS.

### xacml (cross-domain alias: `xacml-policy`, `xacml-pdp`)
**Domain:** Decision Logic
**Definition:** OASIS standard XML language for ABAC policies: PolicySet→Policy→Rule with target/condition/effect.
**Atom or composite:** Composite — hierarchical with rule-combining and policy-combining algorithms.
**Cost model:** XML parsing dominates; compiled forms (e.g. AuthzForce) bring to ms.
**Real wall?** Yes — XML verbosity and decision-combinator semantics are hard to reason about.
**Cross-domain wiring:** `logic-reasoning/predicate-logic`; `cryptography-advanced/saml`.
**Notes:** AuthzForce, WSO2 Balana. Largely superseded by OPA/Cedar but still in enterprise SSO.

### opa-rego (cross-domain alias: `opa`, `rego-policy`, `open-policy-agent`)
**Domain:** Decision Logic
**Definition:** Open Policy Agent's declarative datalog-derived language for policy: `package`, rules, `default`, comprehensions, `with`.
**Atom or composite:** Composite — datalog-style rules over JSON documents.
**Cost model:** Partial evaluation compiles to constants/SQL/WASM; ms-range eval.
**Real wall?** Yes — non-determinism with negation/iteration is restricted; debugging requires `trace`.
**Cross-domain wiring:** `logic-reasoning/datalog`; `database-streaming-sketching/json-query`; `kubernetes/admission`.
**Notes:** CNCF graduated. Default policy engine for Kubernetes Gatekeeper.

### cedar-policy (cross-domain alias: `cedar`, `aws-cedar`)
**Domain:** Decision Logic
**Definition:** Amazon's policy language: `permit/forbid (principal, action, resource) when {...}`; statically analyzable via SMT.
**Atom or composite:** Composite — typed entity model + decidable subset of FOL.
**Cost model:** Microsecond-range due to typed schema; SMT equivalence checks in seconds.
**Real wall?** Yes — designed for decidability → expressiveness is bounded.
**Cross-domain wiring:** `logic-reasoning/smt`; `formal-verification/policy-equivalence`.
**Notes:** Verified Rust implementation. AVP (Amazon Verified Permissions), AuthZed alternative.

### aws-iam-policy (cross-domain alias: `iam-policy`, `aws-policy-json`)
**Domain:** Decision Logic
**Definition:** JSON policy with `Statement[]` of `(Effect, Action, Resource, Condition, Principal)` evaluated by AWS IAM PDP.
**Atom or composite:** Composite — bag of statements with explicit-deny semantics.
**Cost model:** O(statements × resources); AWS internally compiles to optimized form.
**Real wall?** Yes — confused-deputy, privilege escalation; condition-key combinatorics.
**Cross-domain wiring:** `logic-reasoning/predicate`; `cryptography-advanced/sts`.
**Notes:** Zelkova SMT-backed analyzer at AWS proves access-equivalence.

### azure-rbac (cross-domain alias: `azure-role-assignment`, `arm-rbac`)
**Domain:** Decision Logic
**Definition:** Azure's role-based model: built-in/custom Role Definitions (`Actions`/`NotActions`/`DataActions`) assigned at scope.
**Atom or composite:** Composite — role × scope × principal triples with hierarchical scope inheritance.
**Cost model:** Inheritance walk O(depth) per check; aggressively cached.
**Real wall?** Yes — wildcard semantics + scope inheritance produce surprise grants.
**Cross-domain wiring:** `agentic-reasoning/principal-of-least-privilege`; `operating-systems/role-model`.
**Notes:** Azure RBAC; ABAC conditions added in 2022 to bound wildcards.

### kubernetes-rbac (cross-domain alias: `k8s-rbac`, `rolebinding`)
**Domain:** Decision Logic
**Definition:** Kubernetes verb/resource RBAC: Role/ClusterRole granting verbs on resources, bound via RoleBinding.
**Atom or composite:** Composite — additive grants (no deny), default deny.
**Cost model:** O(1) authorizer hit after cache warm; APIServer caches all rolebindings in-memory.
**Real wall?** Yes — additive-only model means privilege escalation requires extra escalation/bind checks.
**Cross-domain wiring:** `operating-systems/capability-bit`; `distributed-systems/api-server`.
**Notes:** Often layered with OPA/Gatekeeper for ABAC concerns RBAC can't express.

### kubernetes-networkpolicy (cross-domain alias: `k8s-netpol`, `cni-policy`)
**Domain:** Decision Logic
**Definition:** Pod-selector based ingress/egress firewall rules enforced by the CNI plugin.
**Atom or composite:** Composite — pod selectors + namespace selectors + port/protocol matches.
**Cost model:** Compiled to iptables/eBPF/IPVS; O(1) per packet with ipset matching.
**Real wall?** Yes — additive (no explicit deny in v1); ordering across CNIs is undefined.
**Cross-domain wiring:** `networking/acl`; `operating-systems/ebpf`.
**Notes:** Cilium adds L7 + identity-aware; Calico adds GlobalNetworkPolicy.

### pod-security-policy (cross-domain alias: `psp`, `pod-security-standard`, `psa`)
**Domain:** Decision Logic
**Definition:** Cluster-level policy bounding what a pod-spec may request (capabilities, privileged, hostPath, runAsUser).
**Atom or composite:** Composite — admission-time guard checking many independent fields.
**Cost model:** Admission webhook ms-range; cached against pod template hash.
**Real wall?** Yes — PSP deprecated; PSA (Pod Security Admission) labels are coarser.
**Cross-domain wiring:** `operating-systems/capabilities`; `agentic-reasoning/least-privilege`.
**Notes:** PSP removed in k8s 1.25; replaced by Pod Security Admission + Kyverno/Gatekeeper.

### apparmor-profile (cross-domain alias: `apparmor`, `path-based-mac`)
**Domain:** Decision Logic
**Definition:** Path-based mandatory access control profile attached to a process via `aa_change_profile()`.
**Atom or composite:** Composite — list of file/cap/network rules with `allow`/`deny`/`audit`.
**Cost model:** O(1) per syscall via kernel hash on policy index.
**Real wall?** Yes — path-based confused-deputy via symlinks; profiles must enumerate every path.
**Cross-domain wiring:** `operating-systems/lsm`; `formal-verification/runtime-monitor`.
**Notes:** Default LSM on Ubuntu, SUSE. Generated by docker/podman from container image labels.

### selinux-type-enforcement (cross-domain alias: `selinux`, `type-enforcement`, `te`)
**Domain:** Decision Logic
**Definition:** Type-based MAC: every object has a type; rules specify which (source-type, target-type, class) tuples permit which permissions.
**Atom or composite:** Composite — millions of `allow` rules compiled into a binary policy.
**Cost model:** O(1) hash on policy db per access vector check.
**Real wall?** Yes — policy authoring complexity; refpolicy is ~100k rules.
**Cross-domain wiring:** `operating-systems/lsm`; `logic-reasoning/type-system`.
**Notes:** Default on RHEL/Fedora/Android. CIL (Common Intermediate Language) is the modern source.

### predicate-function (cross-domain alias: `boolean-test`, `match-fn`)
**Domain:** Decision Logic
**Definition:** Pure function `Context → Bool` used as a rule's guard or a filter on a stream of decisions.
**Atom or composite:** Atom — irreducible truth-valued evaluator.
**Cost model:** Caller-defined; ideally O(1)/O(log n); side-effect free for cacheability.
**Real wall?** Yes — must be total and terminating to be safely cached; partial fns leak.
**Cross-domain wiring:** `logic-reasoning/predicate`; `type-theory-programming-languages/refinement-type`.
**Notes:** Underlies every `if` in every PDP. Memoizable when pure.

### action-function (cross-domain alias: `effect-fn`, `do-fn`)
**Domain:** Decision Logic
**Definition:** Function executed when a rule fires; mutates state, emits events, or calls external systems.
**Atom or composite:** Atom — single effect; composed via sequencing.
**Cost model:** Dominated by external I/O; bound with timeouts.
**Real wall?** Yes — side effects break idempotence; need transactional semantics for replay.
**Cross-domain wiring:** `distributed-systems/idempotency-key`; `databases/saga`.
**Notes:** Underlies Drools `consequence`, BT action node, GOAP operator effect.

### guard-precondition (cross-domain alias: `pre-condition`, `precondition-check`)
**Domain:** Decision Logic
**Definition:** Predicate that must hold before a transition/action is permitted; violation aborts execution.
**Atom or composite:** Atom — single Boolean check at entry.
**Cost model:** O(predicate); short-circuits expensive action paths.
**Real wall?** Yes — incompleteness of precondition specs causes silent state corruption.
**Cross-domain wiring:** `logic-reasoning/hoare-precondition`; `type-theory-programming-languages/contracts`.
**Notes:** Eiffel `require`, .NET Code Contracts `Contract.Requires`, Dafny `requires`.

### invariant-postcondition (cross-domain alias: `invariant`, `post-condition`)
**Domain:** Decision Logic
**Definition:** Predicate that must remain true across a transition (invariant) or hold after (postcondition); checked at exit.
**Atom or composite:** Atom — single Boolean check at exit.
**Cost model:** O(predicate); often disabled in prod for hot paths.
**Real wall?** Yes — too-strong invariants block correct states; too-weak admit bugs.
**Cross-domain wiring:** `logic-reasoning/hoare-postcondition`; `formal-verification/runtime-assertion`.
**Notes:** Eiffel `ensure`/`invariant`, JML `@invariant`.

### permission (cross-domain alias: `right`, `grant`)
**Domain:** Decision Logic
**Definition:** A token (subject, action, resource) authorizing a specific operation; the unit grants are issued in.
**Atom or composite:** Atom — single (s,a,r) triple.
**Cost model:** O(1) set membership on indexed grant table.
**Real wall?** Yes — permission proliferation; manual review breaks at scale.
**Cross-domain wiring:** `operating-systems/file-mode`; `agentic-reasoning/authority`.
**Notes:** AWS IAM Action, k8s verb, POSIX r/w/x. Composes into roles.

### capability-bit (cross-domain alias: `linux-capability`, `cap-bit`)
**Domain:** Decision Logic
**Definition:** A single privilege flag in a per-process bitmap (e.g., `CAP_NET_BIND_SERVICE`) that gates a specific kernel operation.
**Atom or composite:** Atom — one bit, one privilege.
**Cost model:** O(1) bit-test in syscall path.
**Real wall?** Yes — coarse-grained; `CAP_SYS_ADMIN` is the catch-all "root-equivalent".
**Cross-domain wiring:** `operating-systems/capabilities(7)`; `agentic-reasoning/least-authority`.
**Notes:** 40+ caps in current Linux; CAP_BPF, CAP_PERFMON added recently.

### rule-conflict (cross-domain alias: `policy-conflict`, `contradictory-rule`)
**Domain:** Decision Logic
**Definition:** Situation where two or more applicable rules prescribe incompatible decisions for the same request.
**Atom or composite:** Composite — emergent from rule set, not a rule itself.
**Cost model:** Static detection is co-NP-hard; SAT/SMT used at build time.
**Real wall?** Yes — silent conflicts cause non-deterministic enforcement.
**Cross-domain wiring:** `logic-reasoning/satisfiability`; `formal-verification/policy-equivalence`.
**Notes:** Zelkova, Cedar's `is_authorized` proofs detect these.

### rule-priority (cross-domain alias: `salience`, `precedence`)
**Domain:** Decision Logic
**Definition:** Numeric weight on a rule that breaks ties on the agenda; higher priority fires first within an activation cycle.
**Atom or composite:** Atom — single scalar metadata.
**Cost model:** O(log n) priority-queue insert per activation.
**Real wall?** Yes — priorities create hidden coupling; refactoring becomes brittle.
**Cross-domain wiring:** `combinatorial-optimization/priority-queue`; `agentic-reasoning/preference-order`.
**Notes:** Drools `salience`, CLIPS `declare salience`.

### rule-shadowing (cross-domain alias: `subsumption`, `dead-rule`)
**Domain:** Decision Logic
**Definition:** A rule never fires because higher-priority or more general rules always preempt it.
**Atom or composite:** Composite — relational property between rules.
**Cost model:** Static analysis via SMT or unit propagation; build-time only.
**Real wall?** Yes — silent dead code in policy → maintenance debt.
**Cross-domain wiring:** `logic-reasoning/subsumption`; `combinatorial-optimization/dead-code-elimination`.
**Notes:** Cedar's analyzer flags subsumed permits/forbids.

### conflict-resolution-first-match (cross-domain alias: `first-applicable`, `ordered-rules`)
**Domain:** Decision Logic
**Definition:** Combiner: scan rules in order, return the decision of the first that matches.
**Atom or composite:** Atom — single combinator strategy.
**Cost model:** O(rules-until-match); ordering is part of the policy.
**Real wall?** Yes — semantics couples to insertion order; refactoring is dangerous.
**Cross-domain wiring:** `networking/acl-first-match`; `logic-reasoning/ordered-disjunction`.
**Notes:** Default for iptables, AWS SCP, XACML `first-applicable`.

### conflict-resolution-deny-overrides (cross-domain alias: `deny-overrides`, `safety-first`)
**Domain:** Decision Logic
**Definition:** Combiner: if any applicable rule denies, the final decision is Deny regardless of permits.
**Atom or composite:** Atom — single combinator.
**Cost model:** O(rules) — must scan all to confirm absence of deny.
**Real wall?** Yes — explicit deny scales poorly; encourages over-broad denies.
**Cross-domain wiring:** `logic-reasoning/conjunction`; `agentic-reasoning/safety-veto`.
**Notes:** XACML `deny-overrides`, AWS IAM (explicit deny always wins).

### conflict-resolution-permit-overrides (cross-domain alias: `permit-overrides`, `any-permit`)
**Domain:** Decision Logic
**Definition:** Combiner: if any applicable rule permits, the final decision is Permit regardless of denies.
**Atom or composite:** Atom — single combinator.
**Cost model:** O(rules); short-circuits on first permit.
**Real wall?** Yes — over-permissive by construction; rarely the right default.
**Cross-domain wiring:** `logic-reasoning/disjunction`; `agentic-reasoning/optimism-bias`.
**Notes:** XACML `permit-overrides`. Used in opt-in feature flags.

### conflict-resolution-ordered-permit-overrides (cross-domain alias: `ordered-permit-overrides`)
**Domain:** Decision Logic
**Definition:** Combiner: scan in order, first permit wins; if none, first deny; preserves rule order even when permitting.
**Atom or composite:** Atom — single combinator.
**Cost model:** O(rules).
**Real wall?** Yes — confusing semantics; rarely used outside XACML.
**Cross-domain wiring:** `logic-reasoning/ordered-disjunction`.
**Notes:** XACML `ordered-permit-overrides`. Useful for layered defaults.

### rule-cache (cross-domain alias: `decision-memoization`, `rule-result-cache`)
**Domain:** Decision Logic
**Definition:** Memoization of `(request-hash → decision)` tuples to skip repeated PDP evaluation.
**Atom or composite:** Composite — hash table + TTL + invalidation hook.
**Cost model:** O(1) hit; O(eval) miss; sizing critical for hit ratio.
**Real wall?** Yes — staleness window vs. policy update latency tradeoff.
**Cross-domain wiring:** `database-streaming-sketching/lru-cache`; `cryptography-hashing/request-hash`.
**Notes:** OPA's decision log; AWS IAM caches authorization decisions per session.

### decision-cache (cross-domain alias: `pep-cache`, `auth-cache`)
**Domain:** Decision Logic
**Definition:** Cache held at the PEP rather than PDP — avoids PDP round-trip entirely.
**Atom or composite:** Composite — LRU + invalidation channel.
**Result wall?** Yes.
**Cost model:** O(1) sub-microsecond; invalidation via pub-sub from PAP.
**Real wall?** Yes — invalidation delay = max policy staleness at PEP.
**Cross-domain wiring:** `distributed-systems/cache-invalidation`; `networking/edge-cache`.
**Notes:** Envoy CheckRequest caching; NGINX auth_request cache.

### hot-path-vs-slow-path-policy (cross-domain alias: `tiered-policy`, `fast-slow-pdp`)
**Domain:** Decision Logic
**Definition:** Two-tier evaluation: cheap deterministic check inline, fall through to full PDP only on ambiguous cases.
**Atom or composite:** Composite — guard + escape hatch.
**Cost model:** O(1) hot path; ms slow path.
**Real wall?** Yes — hot path must be conservative (deny by default on ambiguity).
**Cross-domain wiring:** `operating-systems/fast-path`; `networking/cilium-fast-path`.
**Notes:** Cilium fast-path eBPF + slow-path policy engine; iptables conntrack hot path.

### policy-compilation (cross-domain alias: `policy-compile`, `policy-build`)
**Domain:** Decision Logic
**Definition:** Lowering high-level policy (Rego, Cedar, XACML) to an efficient runtime form (BDD, WASM, machine code, eBPF).
**Atom or composite:** Composite — frontend → IR → backend.
**Cost model:** Build-time O(rules); runtime O(1) lookups after compile.
**Real wall?** Yes — compiled form invalidated by any policy change.
**Cross-domain wiring:** `type-theory-programming-languages/compiler`; `logic-reasoning/bdd-compile`.
**Notes:** OPA → WASM, OPA → SQL (partial eval), Cedar → typed AST.

### partial-evaluation-of-policies (cross-domain alias: `policy-partial-eval`, `residual-policy`)
**Domain:** Decision Logic
**Definition:** Specialize a policy with a known partial request; produce a residual policy that can be pushed down (e.g., into SQL).
**Atom or composite:** Composite — interpreter applied with bound vars.
**Cost model:** Eager: O(rules); produces smaller policy executed cheaply downstream.
**Real wall?** Yes — only valid for monotonic policy fragments.
**Cross-domain wiring:** `type-theory-programming-languages/partial-eval`; `database-streaming-sketching/predicate-pushdown`.
**Notes:** OPA `rego.partial`. Translates ABAC rules into row-filter SQL.

### policy-rewriting (cross-domain alias: `policy-normalization`, `policy-simplification`)
**Domain:** Decision Logic
**Definition:** Equivalence-preserving transformations: DNF/CNF normalization, dead-rule elimination, predicate factoring.
**Atom or composite:** Composite — pass pipeline.
**Cost model:** Build-time; can reduce policy size by 10x.
**Real wall?** Yes — equivalence must be proved, not assumed.
**Cross-domain wiring:** `logic-reasoning/cnf-normalization`; `combinatorial-optimization/expression-simplification`.
**Notes:** Cedar/Zelkova rewrite-then-solve.

### policy-diff-equivalence (cross-domain alias: `policy-equivalence-check`, `policy-diff`)
**Domain:** Decision Logic
**Definition:** Decide whether two policy versions accept the same request set; report counterexamples.
**Atom or composite:** Composite — SMT-backed analyzer.
**Cost model:** NP-hard in general; sec-range for typical Cedar/IAM policies.
**Real wall?** Yes — depends on decidable policy fragment.
**Cross-domain wiring:** `logic-reasoning/smt-solver`; `formal-verification/equivalence-checking`.
**Notes:** Zelkova (AWS), Cedar analyzer, OPA `eval --partial` + diff.

### policy-explanation-trace (cross-domain alias: `decision-trace`, `why-permit`)
**Domain:** Decision Logic
**Definition:** Structured record of which rules fired and which facts contributed to a decision; the receipt of evaluation.
**Atom or composite:** Composite — tree of (rule-id, bindings, sub-decision).
**Cost model:** O(rules-touched) extra log emission; opt-in due to volume.
**Real wall?** Yes — trace privacy (may leak inputs); volume of decision logs.
**Cross-domain wiring:** `agentic-reasoning/explanation`; `logic-reasoning/proof-tree`.
**Notes:** OPA decision logs; Cedar's `diagnostics`. Feeds audit + debug.

### drools-rete (cross-domain alias: `rete-network`, `rete-engine`)
**Domain:** Decision Logic
**Definition:** Forgy's RETE algorithm: incremental match via alpha/beta/join network sharing tests across rules.
**Atom or composite:** Composite — DAG of nodes with cross-rule sharing.
**Cost model:** Trade memory (partial matches) for time; amortized O(Δfacts) per assertion.
**Real wall?** Yes — memory blow-up on cross-products of partial matches.
**Cross-domain wiring:** `database-streaming-sketching/incremental-view-maintenance`; `logic-reasoning/datalog-eval`.
**Notes:** Drools, CLIPS, JESS. Variants: RETE-II, RETE-III, RETE-OO.

### rete-alpha-node (cross-domain alias: `alpha-memory`, `single-fact-test`)
**Domain:** Decision Logic
**Definition:** RETE node that tests a single fact against a single intra-condition predicate (e.g., `Order.amount > 100`).
**Atom or composite:** Atom — one test per node.
**Cost model:** O(1) per fact insertion; shared across rules.
**Real wall?** Yes — only handles intra-condition tests, not joins.
**Cross-domain wiring:** `database-streaming-sketching/filter-pushdown`.
**Notes:** Sharing alpha nodes across rules is the main RETE win for unselective tests.

### rete-beta-node (cross-domain alias: `beta-memory`, `join-node`)
**Domain:** Decision Logic
**Definition:** RETE node that joins partial matches from two upstream memories using inter-condition predicates.
**Atom or composite:** Atom — single binary join.
**Cost model:** O(left × right) worst case; indexed joins bring near-linear.
**Real wall?** Yes — join order determines memory blow-up.
**Cross-domain wiring:** `database-streaming-sketching/hash-join`; `logic-reasoning/conjunction`.
**Notes:** Most rule-engine perf work is on beta-node indexing.

### leaps-algorithm (cross-domain alias: `leaps`, `lazy-rete`)
**Domain:** Decision Logic
**Definition:** Lazy alternative to RETE: avoid materializing partial matches by enumerating tuples on demand.
**Atom or composite:** Composite — replaces RETE network with stack-based search.
**Cost model:** O(facts^k) per query, but O(1) memory for partial state.
**Real wall?** Yes — repeated queries over the same state recompute work.
**Cross-domain wiring:** `combinatorial-optimization/backtracking-search`.
**Notes:** OPS5 LEAPS, CLIPS LEAPS-mode. Good for low-fact-churn high-query.

### treat-algorithm (cross-domain alias: `treat`, `partial-treat`)
**Domain:** Decision Logic
**Definition:** RETE variant that drops beta memories entirely, re-deriving joins each cycle to save memory.
**Atom or composite:** Composite — modified RETE topology.
**Cost model:** More CPU per cycle, much less memory; favored when fact set is small/changing.
**Real wall?** Yes — memory vs. CPU tradeoff.
**Cross-domain wiring:** `database-streaming-sketching/streaming-join`.
**Notes:** Miranker 1990; influenced modern incremental query systems.

### production-system (cross-domain alias: `production-rule-system`, `ops5`)
**Domain:** Decision Logic
**Definition:** Architecture of (working memory, production memory, recognize-act cycle): match → conflict-resolve → fire.
**Atom or composite:** Composite — full control loop.
**Cost model:** O(match) per cycle; match dominates.
**Real wall?** Yes — non-confluent firing → order matters → debuggability suffers.
**Cross-domain wiring:** `agentic-reasoning/perception-action-loop`; `logic-reasoning/forward-chaining`.
**Notes:** OPS5, SOAR, ACT-R, Drools.

### ruleml (cross-domain alias: `ruleml-markup`)
**Domain:** Decision Logic
**Definition:** XML/RDF markup for sharing rules across engines: derivation rules, reaction rules, integrity constraints.
**Atom or composite:** Composite — interchange format, not an engine.
**Cost model:** Parse + transform to native form; engine-dependent.
**Real wall?** Yes — semantic round-trips between engines often lose information.
**Cross-domain wiring:** `logic-reasoning/horn-clauses`; `database-streaming-sketching/rdf`.
**Notes:** Reaction RuleML for ECA rules; Deliberation RuleML for derivation.

### jess-engine (cross-domain alias: `jess`, `java-expert-system-shell`)
**Domain:** Decision Logic
**Definition:** Java rule engine derived from CLIPS; provides scripting in JESS language + Java integration.
**Atom or composite:** Composite — RETE engine + LISP-like DSL.
**Cost model:** Comparable to CLIPS; JVM overhead dominates for small rule sets.
**Real wall?** Yes — proprietary; superseded by Drools in OSS world.
**Cross-domain wiring:** `agentic-reasoning/expert-system`.
**Notes:** Sandia National Labs origin; still used in legacy expert systems.

### clips-engine (cross-domain alias: `clips`, `c-language-integrated-production-system`)
**Domain:** Decision Logic
**Definition:** NASA-origin C-based forward-chaining rule engine with LISP-like syntax and RETE matcher.
**Atom or composite:** Composite — engine + DSL.
**Cost model:** Highly optimized RETE; sub-ms eval for tens of thousands of rules.
**Real wall?** Yes — single-threaded; embedding into multi-tenant services needs careful isolation.
**Cross-domain wiring:** `agentic-reasoning/expert-system`; `logic-reasoning/forward-chaining`.
**Notes:** Open source since 1986; still maintained.

### default-rule (cross-domain alias: `fallback-rule`, `catch-all`)
**Domain:** Decision Logic
**Definition:** Rule that fires when no more specific rule applies; encodes the system's default decision.
**Atom or composite:** Atom — single rule with lowest priority / broadest antecedent.
**Cost model:** O(1) — always last in evaluation order.
**Real wall?** Yes — default-permit vs. default-deny shapes the whole security posture.
**Cross-domain wiring:** `logic-reasoning/negation-as-failure`; `agentic-reasoning/baseline-behavior`.
**Notes:** Default-deny is the secure default; Reiter default logic underlies the formal semantics.

### exception-rule (cross-domain alias: `override-deny`, `carve-out`)
**Domain:** Decision Logic
**Definition:** Rule that overrides a more general rule for a specific case; encodes "X except when Y".
**Atom or composite:** Atom — high-priority specific rule.
**Cost model:** O(1) priority; depends on combinator semantics.
**Real wall?** Yes — exceptions accumulate, creating Swiss-cheese policies.
**Cross-domain wiring:** `logic-reasoning/non-monotonic-reasoning`; `agentic-reasoning/exception-handling`.
**Notes:** AWS IAM explicit-deny carve-outs; legal contracts modeled this way.

### override-rule (cross-domain alias: `policy-override`, `escalation-grant`)
**Domain:** Decision Logic
**Definition:** Rule that grants/denies despite conflicting normal rules — typically requires audit trail or break-glass approval.
**Atom or composite:** Atom — privileged rule with extra-strong priority.
**Cost model:** O(1) but with audit emission O(serialize-context).
**Real wall?** Yes — break-glass auth must be observable, time-bound, and revocable.
**Cross-domain wiring:** `agentic-reasoning/break-glass`; `cryptography-advanced/audit-log`.
**Notes:** PagerDuty break-glass; AWS IAM emergency role.

### policy-bundle (cross-domain alias: `signed-policy-bundle`, `opa-bundle`)
**Domain:** Decision Logic
**Definition:** Tarball of policies + data signed by the PAP and distributed to PDPs; the unit of policy versioning.
**Atom or composite:** Composite — manifest + policy files + data + signature.
**Cost model:** O(bundle-size) download + verify; cached aggressively.
**Real wall?** Yes — atomic swap required; partial-update is forbidden.
**Cross-domain wiring:** `cryptography-advanced/code-signing`; `distributed-systems/config-distribution`.
**Notes:** OPA bundle API; Styra DAS; Sigstore-signed bundles.

### policy-obligation (cross-domain alias: `obligation`, `decision-side-effect`)
**Domain:** Decision Logic
**Definition:** Side-effect the PEP must execute alongside the decision (e.g., log this access, mask field X, require MFA).
**Atom or composite:** Atom — single named side-effect with parameters.
**Cost model:** Adds to PEP critical path; per-obligation O(effect).
**Real wall?** Yes — failure to execute an obligation invalidates the decision (XACML "denied").
**Cross-domain wiring:** `agentic-reasoning/conditional-action`; `databases/audit-trigger`.
**Notes:** XACML obligations; OPA "advice" pattern.

### policy-advice (cross-domain alias: `xacml-advice`, `non-blocking-obligation`)
**Domain:** Decision Logic
**Definition:** Like an obligation but non-binding — failure to execute does not invalidate the decision.
**Atom or composite:** Atom.
**Cost model:** Optional best-effort; PEP may drop under load.
**Real wall?** Yes — distinguishing advice from obligation must be precise.
**Cross-domain wiring:** `distributed-systems/best-effort-delivery`.
**Notes:** XACML 3.0 advice expressions.

### abac-attribute (cross-domain alias: `attribute`, `claim`)
**Domain:** Decision Logic
**Definition:** A typed key-value pair on subject/resource/environment consumed by ABAC policies.
**Atom or composite:** Atom — single (k, v) pair.
**Cost model:** O(1) attribute fetch from attribute provider; caching mandatory.
**Real wall?** Yes — attribute provenance and freshness drive correctness; cache-poisoning is the attack.
**Cross-domain wiring:** `cryptography-advanced/jwt-claim`; `agentic-reasoning/context`.
**Notes:** OAuth2 claims, SAML attributes, AWS IAM `aws:PrincipalTag`.

### context-handler-pip (cross-domain alias: `pip`, `policy-information-point`)
**Domain:** Decision Logic
**Definition:** Component that fetches attribute values on demand for the PDP (LDAP, IDP, DB, HTTP API).
**Atom or composite:** Composite — adapter + cache + timeout.
**Cost model:** External RTT; cached with TTL.
**Real wall?** Yes — PIP latency blocks PDP eval; circuit-breaker needed.
**Cross-domain wiring:** `distributed-systems/sidecar`; `networking/service-mesh`.
**Notes:** XACML PIP role; OPA `http.send` and bundle-data fetches.

---

## Section 2 — Decision Making

### decision-tree-cart (cross-domain alias: `cart`, `classification-and-regression-tree`)
**Domain:** Decision Logic
**Definition:** Breiman's binary tree: each internal node tests `x_j <= t`, leaves carry class label or regression mean; built by greedy Gini/MSE splits.
**Atom or composite:** Composite — recursively built tree of axis-aligned splits.
**Cost model:** Train O(n·d·log n); inference O(depth).
**Real wall?** Yes — high variance; deep trees overfit; axis-aligned only.
**Cross-domain wiring:** `ml-training/cart`; `logic-reasoning/decision-table`; `combinatorial-optimization/greedy-tree`.
**Notes:** sklearn `DecisionTreeClassifier`; basis for RF/GBDT.

### decision-tree-id3 (cross-domain alias: `id3`, `iterative-dichotomiser-3`)
**Domain:** Decision Logic
**Definition:** Quinlan's information-gain greedy tree learner; categorical features only, no pruning.
**Atom or composite:** Composite — recursive entropy-split.
**Cost model:** Train O(n·d·log n); biased toward many-valued attributes.
**Real wall?** Yes — no handling of continuous or missing features.
**Cross-domain wiring:** `information-theory-coding/entropy`; `ml-training/tree-learner`.
**Notes:** Quinlan 1986. Superseded by C4.5.

### decision-tree-c45 (cross-domain alias: `c4.5`, `c45`)
**Domain:** Decision Logic
**Definition:** Quinlan's successor to ID3: gain-ratio splits, continuous features via thresholding, pessimistic pruning.
**Atom or composite:** Composite — recursive tree with pruning pass.
**Cost model:** Train O(n·d·log n); inference O(depth).
**Real wall?** Yes — gain-ratio still biased; pessimistic pruning is heuristic.
**Cross-domain wiring:** `ml-training/c45`; `information-theory-coding/gain-ratio`.
**Notes:** Reference implementation in J48 (Weka). Often baseline in benchmarks.

### decision-tree-c50 (cross-domain alias: `c5.0`, `see5`)
**Domain:** Decision Logic
**Definition:** Quinlan's commercial follow-on to C4.5: boosting, winnowing, multi-class trees, faster build.
**Atom or composite:** Composite — boosted tree ensemble + rule extraction.
**Cost model:** 5-10x faster than C4.5; lower memory; produces smaller trees.
**Real wall?** Yes — proprietary historically; open-sourced 2011.
**Cross-domain wiring:** `ml-training/boosting`; `logic-reasoning/rule-extraction`.
**Notes:** R `C50` package; rule extraction from boosted trees.

### decision-tree-chaid (cross-domain alias: `chaid`, `chi-square-automatic-interaction-detection`)
**Domain:** Decision Logic
**Definition:** Tree learner using chi-square (categorical target) or F-test (continuous) to choose splits; multi-way branching.
**Atom or composite:** Composite — multi-way splits via statistical tests.
**Cost model:** Train O(n·d²); inference O(depth).
**Real wall?** Yes — requires sufficient sample per cell for test validity.
**Cross-domain wiring:** `statistics-probability/chi-square-test`; `ml-training/tree`.
**Notes:** Kass 1980. Used heavily in market segmentation.

### decision-mars (cross-domain alias: `mars`, `multivariate-adaptive-regression-splines`)
**Domain:** Decision Logic
**Definition:** Friedman's piecewise-linear regression: hinge functions `max(0, x-t)`, greedy forward then pruning backward.
**Atom or composite:** Composite — basis expansion + linear combination.
**Cost model:** Train O(n²·M); inference O(M) basis evaluations.
**Real wall?** Yes — interpretability falls off as terms multiply.
**Cross-domain wiring:** `ml-training/gam`; `statistics-probability/spline-regression`.
**Notes:** `py-earth`, R `earth`.

### regression-tree (cross-domain alias: `cart-regression`, `mean-tree`)
**Domain:** Decision Logic
**Definition:** Decision tree predicting a continuous target via leaf-mean (or leaf-linear); split criterion is MSE reduction.
**Atom or composite:** Composite — recursive variance-reducing splits.
**Cost model:** Train O(n·d·log n); inference O(depth).
**Real wall?** Yes — piecewise-constant predictions; smooth functions need many leaves.
**Cross-domain wiring:** `ml-training/regression`; `statistics-probability/variance-decomposition`.
**Notes:** Building block of GBDT (XGBoost, LightGBM, CatBoost).

### classification-tree (cross-domain alias: `cart-classifier`, `leaf-class-tree`)
**Domain:** Decision Logic
**Definition:** Decision tree predicting a discrete label; leaf prediction is majority class, optionally with class probabilities.
**Atom or composite:** Composite.
**Cost model:** Train O(n·d·log n); inference O(depth).
**Real wall?** Yes — class imbalance bias; calibration of leaf probabilities required.
**Cross-domain wiring:** `ml-training/classification`; `statistics-probability/probability-calibration`.
**Notes:** sklearn `DecisionTreeClassifier(class_weight=...)`.

### random-forest-decision (cross-domain alias: `rf-vote`, `random-forest-classifier`)
**Domain:** Decision Logic
**Definition:** Ensemble of bagged decision trees; classification by majority vote, regression by mean.
**Atom or composite:** Composite — bootstrap + random subspace + aggregate.
**Cost model:** Train O(T·n·d·log n); inference O(T·depth) (embarrassingly parallel).
**Real wall?** Yes — memory linear in tree count; calibration drift across leaves.
**Cross-domain wiring:** `ml-training/ensemble`; `statistics-probability/bootstrap`.
**Notes:** Breiman 2001. Default ML baseline for tabular.

### extra-trees (cross-domain alias: `extremely-randomized-trees`, `extratrees`)
**Domain:** Decision Logic
**Definition:** Random-forest variant where split thresholds are sampled uniformly rather than optimized.
**Atom or composite:** Composite.
**Cost model:** Train faster than RF (no threshold search); inference identical.
**Real wall?** Yes — slightly more bias for less variance; needs more trees.
**Cross-domain wiring:** `ml-training/randomized-ensemble`.
**Notes:** Geurts/Ernst/Wehenkel 2006. sklearn `ExtraTreesClassifier`.

### ensemble-vote (cross-domain alias: `voting-classifier`, `majority-vote`)
**Domain:** Decision Logic
**Definition:** Decision combinator over an ensemble: hard (mode of labels) or soft (mean of class probabilities).
**Atom or composite:** Atom — single aggregation step over predictions.
**Cost model:** O(T) ensemble eval + O(T) aggregate.
**Real wall?** Yes — vote weights must reflect calibration; un-calibrated soft-vote degrades.
**Cross-domain wiring:** `statistics-probability/aggregation`; `agentic-reasoning/swarm-vote`.
**Notes:** sklearn `VotingClassifier`; the simplest stacking step.

### binary-decision-diagram (cross-domain alias: `bdd`, `obdd`)
**Domain:** Decision Logic
**Definition:** Canonical reduced ordered DAG representing a Boolean function: nodes are variables, two children for low/high.
**Atom or composite:** Composite — DAG with sharing.
**Cost model:** Operations O(|f|·|g|); size can be exponential in worst case.
**Real wall?** Yes — variable ordering matters; finding optimal order is NP-hard.
**Cross-domain wiring:** `logic-reasoning/bdd`; `formal-verification/model-checking`.
**Notes:** Bryant 1986. CUDD, BuDDy libraries.

### robdd (cross-domain alias: `reduced-ordered-bdd`, `canonical-bdd`)
**Domain:** Decision Logic
**Definition:** BDD with two reductions: shared subgraphs and elimination of redundant tests; canonical for a fixed order.
**Atom or composite:** Composite.
**Cost model:** Polynomial in result size; uniqueness enables O(1) equivalence test.
**Real wall?** Yes — order-dependent size blow-up.
**Cross-domain wiring:** `logic-reasoning/canonical-form`; `formal-verification/symbolic-mc`.
**Notes:** Used in NuSMV, CUDD. Underlies symbolic model checking.

### zdd (cross-domain alias: `zero-suppressed-bdd`, `zdd-set`)
**Domain:** Decision Logic
**Definition:** Zero-suppressed BDD: skips don't-care variables that are 0; efficient for sparse set families.
**Atom or composite:** Composite.
**Cost model:** Far smaller than BDD for sparse combinatorial sets.
**Real wall?** Yes — distinct algebra from BDD; conversion is non-trivial.
**Cross-domain wiring:** `combinatorial-optimization/set-family`; `logic-reasoning/symbolic-set`.
**Notes:** Minato 1993. Used in covering-problem solvers.

### add-algebraic-decision-diagram (cross-domain alias: `add`, `algebraic-dd`)
**Domain:** Decision Logic
**Definition:** Like BDD but leaves carry numbers (or elements of a semiring); represents functions Bool^n → ℝ.
**Atom or composite:** Composite.
**Cost model:** Operations O(|f|·|g|); often used in MDP value iteration.
**Real wall?** Yes — terminal proliferation when value space is large.
**Cross-domain wiring:** `control-numerical-opt/symbolic-mdp`; `logic-reasoning/semiring`.
**Notes:** Bahar et al. 1993. CUDD support.

### mdd (cross-domain alias: `multi-valued-dd`, `mdd-diagram`)
**Domain:** Decision Logic
**Definition:** Decision diagram where each node tests a finite-domain variable with k outgoing edges instead of two.
**Atom or composite:** Composite.
**Cost model:** Can be more compact than BDD when domains > 2; otherwise similar bounds.
**Real wall?** Yes — same variable-ordering pain as BDD.
**Cross-domain wiring:** `combinatorial-optimization/csp`; `logic-reasoning/finite-domain`.
**Notes:** Used in constraint programming and verification.

### decision-diagram-compaction (cross-domain alias: `dd-reduce`, `dd-minimize`)
**Domain:** Decision Logic
**Definition:** Variable-reordering, isomorphism merging, and don't-care substitution to shrink a DD.
**Atom or composite:** Composite — pass pipeline.
**Cost model:** Sifting reorder O(n²); often 10-100x size reduction.
**Real wall?** Yes — optimal reorder is NP-hard; sifting is heuristic.
**Cross-domain wiring:** `combinatorial-optimization/local-search`; `formal-verification/bdd-min`.
**Notes:** Rudell sifting; CUDD `Cudd_ReduceHeap`.

### score-function (cross-domain alias: `scoring`, `decision-score`)
**Domain:** Decision Logic
**Definition:** Function `Action × Context → ℝ` ranking choices; decision picks argmax/argmin.
**Atom or composite:** Atom.
**Cost model:** Caller-defined; O(features) typical.
**Real wall?** Yes — calibration across actions matters; raw scores rarely comparable.
**Cross-domain wiring:** `statistics-probability/scoring-rule`; `ml-training/scoring-model`.
**Notes:** Underlies ranking, retrieval, action selection.

### weight (cross-domain alias: `coefficient`, `importance-weight`)
**Domain:** Decision Logic
**Definition:** Scalar attached to a feature/rule/voter that scales its contribution to a decision.
**Atom or composite:** Atom.
**Cost model:** O(1) multiply.
**Real wall?** Yes — overfitting via weight inflation; needs regularization.
**Cross-domain wiring:** `linear-algebra-matrix/inner-product`; `ml-training/regularization`.
**Notes:** Logistic regression coefficient, ensemble vote weight, AHP weight.

### threshold (cross-domain alias: `cutoff`, `decision-boundary`)
**Domain:** Decision Logic
**Definition:** Scalar that partitions a score axis into decision regions: `score >= τ → action_a` else `action_b`.
**Atom or composite:** Atom.
**Cost model:** O(1) compare.
**Real wall?** Yes — operating-point selection is a domain choice, not optimal in any universal sense.
**Cross-domain wiring:** `statistics-probability/roc`; `signal-processing-rf/threshold-detector`.
**Notes:** Tuned via ROC, Youden, F-beta, cost-sensitive analysis.

### threshold-tuning-roc (cross-domain alias: `roc-tuning`, `operating-point-selection`)
**Domain:** Decision Logic
**Definition:** Selecting τ on the ROC curve to optimize TPR/FPR tradeoff or maximize Youden's J = TPR-FPR.
**Atom or composite:** Composite — sweep + objective evaluation.
**Cost model:** O(n·log n) sort + linear sweep.
**Real wall?** Yes — depends on class prevalence; PR curve preferred under imbalance.
**Cross-domain wiring:** `statistics-probability/youden-j`; `ml-training/operating-point`.
**Notes:** sklearn `roc_curve`. F-beta when costs asymmetric.

### cost-function (cross-domain alias: `loss`, `objective`)
**Domain:** Decision Logic
**Definition:** Function `Action × Outcome → ℝ` measuring penalty of taking action under realized outcome.
**Atom or composite:** Atom.
**Cost model:** O(1) evaluation; aggregate over distribution is the design choice.
**Real wall?** Yes — eliciting true costs is the hardest part of decision analysis.
**Cross-domain wiring:** `ml-training/loss-function`; `control-numerical-opt/objective`.
**Notes:** Squared, hinge, log-loss, asymmetric costs in fraud/medical.

### utility-function (cross-domain alias: `utility`, `preference-utility`)
**Domain:** Decision Logic
**Definition:** Real-valued representation of preferences over outcomes; rational decision maximizes expected utility.
**Atom or composite:** Atom.
**Cost model:** O(1) per eval; the integral is the expensive part.
**Real wall?** Yes — von Neumann-Morgenstern axioms; bounded utility avoids St. Petersburg.
**Cross-domain wiring:** `agentic-reasoning/utility-maximization`; `statistics-probability/expectation`.
**Notes:** Concavity encodes risk aversion (Bernoulli, Arrow-Pratt).

### expected-utility (cross-domain alias: `eu`, `expected-payoff`)
**Domain:** Decision Logic
**Definition:** ∑_o P(o|a) · U(o); the rational-actor decision criterion under uncertainty.
**Atom or composite:** Composite — inner product of probabilities and utilities.
**Cost model:** O(outcomes) per action; per-action argmax over actions.
**Real wall?** Yes — assumes calibrated P and elicited U; rarely both available.
**Cross-domain wiring:** `statistics-probability/expectation`; `agentic-reasoning/rational-agent`.
**Notes:** Foundation of decision theory. Often a strawman in practice.

### multi-attribute-utility (cross-domain alias: `mau`, `multi-attribute-utility-theory`)
**Domain:** Decision Logic
**Definition:** Utility as a function over multiple criteria, often additive `U = ∑ w_i · u_i(x_i)` with elicited weights.
**Atom or composite:** Composite — per-attribute utility + aggregation.
**Cost model:** O(attributes).
**Real wall?** Yes — additive form assumes preferential independence; rarely strictly true.
**Cross-domain wiring:** `agentic-reasoning/multi-criteria`; `statistics-probability/conjoint-analysis`.
**Notes:** Keeney & Raiffa 1976. SMART, SWING weight elicitation.

### risk-sensitive-utility (cross-domain alias: `cvar-utility`, `exponential-utility`)
**Domain:** Decision Logic
**Definition:** Utility shape (e.g., `U(x) = -exp(-αx)`) encoding risk aversion; equivalent to CVaR/entropic-risk objectives.
**Atom or composite:** Composite — utility transform + expectation.
**Cost model:** O(samples) Monte Carlo; closed form for Gaussian outcomes.
**Real wall?** Yes — choice of α reflects the operator's risk appetite, not data.
**Cross-domain wiring:** `statistics-probability/cvar`; `control-numerical-opt/risk-mdp`.
**Notes:** Howard-Matheson; entropic risk measure equivalence.

### cvar-decision (cross-domain alias: `cvar`, `conditional-value-at-risk`)
**Domain:** Decision Logic
**Definition:** Decision criterion minimizing the expected loss in the worst α-tail; coherent risk measure.
**Atom or composite:** Composite — tail expectation.
**Cost model:** O(n log n) from sample; closed form for parametric distributions.
**Real wall?** Yes — tail data is scarce; estimator variance high.
**Cross-domain wiring:** `statistics-probability/tail-expectation`; `combinatorial-optimization/cvar-optimization`.
**Notes:** Rockafellar-Uryasev 2000. Convex LP/QP reformulation.

### argmax-selector (cross-domain alias: `argmax`, `greedy-pick`)
**Domain:** Decision Logic
**Definition:** Selector returning the action with maximum score: `a* = argmax_a score(a)`.
**Atom or composite:** Atom.
**Cost model:** O(|A|) linear scan; O(log |A|) with sorted structure.
**Real wall?** Yes — ties broken arbitrarily; deterministic + brittle.
**Cross-domain wiring:** `control-numerical-opt/maximization`; `agentic-reasoning/greedy-policy`.
**Notes:** The default policy under perfect information.

### softmax-selector (cross-domain alias: `softmax`, `boltzmann-policy`)
**Domain:** Decision Logic
**Definition:** Stochastic selector: P(a) ∝ exp(score(a) / τ); temperature τ controls exploration.
**Atom or composite:** Atom.
**Cost model:** O(|A|) normalize + sample.
**Real wall?** Yes — τ-tuning is the hidden art; numerical underflow at low τ.
**Cross-domain wiring:** `ml-training/softmax`; `agentic-reasoning/boltzmann-exploration`.
**Notes:** Equivalent to maximum-entropy RL.

### epsilon-greedy (cross-domain alias: `eps-greedy`, `e-greedy`)
**Domain:** Decision Logic
**Definition:** With probability ε pick uniform random action, else argmax; the simplest exploration strategy.
**Atom or composite:** Composite — Bernoulli flip + argmax/uniform.
**Cost model:** O(|A|) for argmax; ε typically decayed over time.
**Real wall?** Yes — wastes exploration uniformly; no information-direction.
**Cross-domain wiring:** `ml-training/exploration`; `statistics-probability/uniform-sample`.
**Notes:** Watkins' original Q-learning exploration. Outperformed by UCB/Thompson.

### boltzmann-selection (cross-domain alias: `gibbs-sampling-policy`, `temperature-softmax`)
**Domain:** Decision Logic
**Definition:** Softmax over scores with explicit temperature schedule; classical RL/optimization exploration.
**Atom or composite:** Atom.
**Cost model:** O(|A|).
**Real wall?** Yes — schedule cooling rate is task-dependent.
**Cross-domain wiring:** `statistics-probability/gibbs-distribution`; `physics-diffusion/simulated-annealing`.
**Notes:** SA, Boltzmann machines, MaxEnt RL.

### thompson-sampling-decision (cross-domain alias: `thompson-sampling`, `posterior-sampling`)
**Domain:** Decision Logic
**Definition:** Sample reward parameters from posterior, then pick the action greedy w.r.t. the sample; Bayesian regret-optimal.
**Atom or composite:** Composite — posterior sample + argmax.
**Cost model:** O(posterior-sample + |A|).
**Real wall?** Yes — needs tractable posterior (Beta/Gaussian); else approximate sampling.
**Cross-domain wiring:** `statistics-probability/posterior-sampling`; `agentic-reasoning/bandit`.
**Notes:** Thompson 1933. Used in Microsoft's Azure Personalizer.

### ucb-selector (cross-domain alias: `ucb1`, `upper-confidence-bound`)
**Domain:** Decision Logic
**Definition:** Pick argmax of `μ̂_a + c·√(ln t / n_a)`; deterministic exploration via confidence interval inflation.
**Atom or composite:** Composite — empirical mean + uncertainty bonus.
**Cost model:** O(|A|).
**Real wall?** Yes — assumes bounded rewards; constants matter.
**Cross-domain wiring:** `statistics-probability/concentration-bound`; `agentic-reasoning/exploration`.
**Notes:** Auer et al. 2002. UCB1, UCB-V, KL-UCB variants.

### lexicographic-decision (cross-domain alias: `lex-order`, `priority-criteria`)
**Domain:** Decision Logic
**Definition:** Order criteria by priority; pick best on criterion 1, ties broken by criterion 2, etc.
**Atom or composite:** Composite — chained argmax.
**Cost model:** O(|A|·|criteria|).
**Real wall?** Yes — utterly insensitive to magnitude differences within a tier.
**Cross-domain wiring:** `agentic-reasoning/priority-ranking`; `combinatorial-optimization/lex-min`.
**Notes:** Hospital triage uses this. Easy to explain.

### satisficing (cross-domain alias: `satisficing-criterion`, `aspiration-level`)
**Domain:** Decision Logic
**Definition:** Pick the first action that exceeds an aspiration level; Simon's bounded-rationality model.
**Atom or composite:** Atom — threshold + first-match.
**Cost model:** O(|A|) until threshold met.
**Real wall?** Yes — aspiration must be calibrated; dynamic updates needed.
**Cross-domain wiring:** `agentic-reasoning/bounded-rationality`.
**Notes:** Simon 1956. Realistic model of human/agent behavior.

### bounded-rationality (cross-domain alias: `bounded-agent`, `procedural-rationality`)
**Domain:** Decision Logic
**Definition:** Decision under cognitive/compute limits; selects from feasible heuristics rather than truly optimal action.
**Atom or composite:** Composite — meta-policy over heuristics.
**Cost model:** Per-heuristic + meta-overhead.
**Real wall?** Yes — by definition cannot match unbounded optimum.
**Cross-domain wiring:** `agentic-reasoning/meta-reasoning`; `control-numerical-opt/anytime-algorithm`.
**Notes:** Simon (Nobel 1978). Anchors modern AI safety arguments.

### prospect-theory-decision (cross-domain alias: `prospect-theory`, `kahneman-tversky`)
**Domain:** Decision Logic
**Definition:** Decision model with reference-dependent value function (loss-averse, S-shaped) and probability weighting.
**Atom or composite:** Composite — value-fn + weighting-fn + reference-point.
**Cost model:** O(outcomes).
**Real wall?** Yes — empirical descriptive model, not normative; reference-point dependence makes it labile.
**Cross-domain wiring:** `agentic-reasoning/behavioral-agent`; `statistics-probability/probability-weighting`.
**Notes:** Kahneman-Tversky 1979. Used in modeling user/customer decisions.

### regret-minimization (cross-domain alias: `no-regret-learning`, `external-regret`)
**Domain:** Decision Logic
**Definition:** Choose actions so cumulative regret vs. best fixed action grows sublinearly; basis of online-learning.
**Atom or composite:** Composite — bandit/full-info update.
**Cost model:** O(|A|) per step; total regret O(√(T·|A|·log|A|)) for Hedge.
**Real wall?** Yes — adversarial setting; sublinear regret is best possible.
**Cross-domain wiring:** `ml-training/online-learning`; `agentic-reasoning/no-regret`.
**Notes:** Hedge/MWU, EXP3, EXP4. Foundation of CFR in poker.

### minimax-decision (cross-domain alias: `minimax`, `worst-case-decision`)
**Domain:** Decision Logic
**Definition:** Choose action minimizing the maximum possible loss across adversary's response; saddle-point.
**Atom or composite:** Composite — outer min, inner max.
**Cost model:** O(|A|·|states|) per ply; alpha-beta prunes substantially.
**Real wall?** Yes — assumes adversary picks worst; pessimistic when not warranted.
**Cross-domain wiring:** `combinatorial-optimization/game-tree`; `agentic-reasoning/adversarial`.
**Notes:** Von Neumann 1928. Chess engines pre-MCTS.

### maximin-decision (cross-domain alias: `maximin`, `wald-decision`)
**Domain:** Decision Logic
**Definition:** Choose action maximizing the minimum payoff across states of nature; safety-first under ignorance.
**Atom or composite:** Composite — outer max, inner min.
**Cost model:** O(|A|·|states|).
**Real wall?** Yes — ignores probability of bad states; overly conservative.
**Cross-domain wiring:** `agentic-reasoning/conservative-policy`; `combinatorial-optimization/robust-opt`.
**Notes:** Wald 1950. Used when probabilities are unknown.

### leximin-decision (cross-domain alias: `leximin`, `lexicographic-maximin`)
**Domain:** Decision Logic
**Definition:** Maximize the worst outcome, break ties by maximizing the second-worst, etc.; fair-allocation default.
**Atom or composite:** Composite — sorted-vector lex compare.
**Cost model:** O(|outcomes|·log|outcomes|).
**Real wall?** Yes — strict lex preference is ethically defensible but practically extreme.
**Cross-domain wiring:** `combinatorial-optimization/fair-allocation`; `agentic-reasoning/fairness`.
**Notes:** Rawlsian welfare. Used in network bandwidth allocation.

### hurwicz-criterion (cross-domain alias: `hurwicz`, `optimism-pessimism-index`)
**Domain:** Decision Logic
**Definition:** Blend maximin and maximax: `α·max + (1-α)·min`; α is the operator's optimism index.
**Atom or composite:** Composite — convex blend.
**Cost model:** O(|A|·|states|).
**Real wall?** Yes — α is an arbitrary choice; reduces to maximin/maximax at extremes.
**Cross-domain wiring:** `agentic-reasoning/optimism-pessimism`.
**Notes:** Hurwicz 1951. Bridges Wald (pessimistic) and Maximax.

### laplace-criterion (cross-domain alias: `laplace`, `principle-of-indifference`)
**Domain:** Decision Logic
**Definition:** Assume uniform prior over states of nature; pick action maximizing average payoff.
**Atom or composite:** Composite — uniform expectation.
**Cost model:** O(|A|·|states|).
**Real wall?** Yes — uniform prior is itself a strong assumption; rarely justified.
**Cross-domain wiring:** `statistics-probability/uniform-prior`.
**Notes:** Laplace's principle of insufficient reason. Convenient default.

### wald-criterion (cross-domain alias: `wald-minimax`, `worst-case-criterion`)
**Domain:** Decision Logic
**Definition:** Maximize the minimum payoff; the original distribution-free minimax decision rule.
**Atom or composite:** Composite.
**Cost model:** O(|A|·|states|).
**Real wall?** Yes — extreme pessimism; ignores all probability info.
**Cross-domain wiring:** `agentic-reasoning/robust-decision`.
**Notes:** Wald 1950. Foundational paper of statistical decision theory.

### decision-matrix (cross-domain alias: `payoff-matrix`, `action-state-matrix`)
**Domain:** Decision Logic
**Definition:** Tableau `Action × State → Payoff` summarizing outcomes; canonical form for static decision problems.
**Atom or composite:** Composite — 2D array of utilities.
**Cost model:** O(|A|·|S|) storage; same for each criterion's eval.
**Real wall?** Yes — combinatorial blow-up at high dimensionality; rarely fully specifiable.
**Cross-domain wiring:** `linear-algebra-matrix/tableau`; `agentic-reasoning/payoff-matrix`.
**Notes:** Textbook decision-theory representation. Game-theory matrix games.

### ahp (cross-domain alias: `analytic-hierarchy-process`, `saaty-ahp`)
**Domain:** Decision Logic
**Definition:** Multi-criteria method: pairwise compare criteria/alternatives, extract weights from principal eigenvector.
**Atom or composite:** Composite — pairwise matrix + eigenvector + hierarchical aggregation.
**Cost model:** O(n³) eigenvector per matrix; small n typical.
**Real wall?** Yes — rank reversal on adding alternatives; consistency-ratio check needed.
**Cross-domain wiring:** `linear-algebra-matrix/eigenvector`; `agentic-reasoning/preference-elicitation`.
**Notes:** Saaty 1980. Heavily used in procurement.

### topsis (cross-domain alias: `topsis-mcdm`, `ideal-solution-distance`)
**Domain:** Decision Logic
**Definition:** Rank alternatives by distance to the ideal positive solution and farness from negative ideal.
**Atom or composite:** Composite — normalize + compute distances + rank.
**Cost model:** O(n·m) for n alternatives, m criteria.
**Real wall?** Yes — sensitive to normalization choice and weight elicitation.
**Cross-domain wiring:** `combinatorial-optimization/distance-rank`; `linear-algebra-matrix/normalization`.
**Notes:** Hwang & Yoon 1981. Used in supplier selection.

### promethee (cross-domain alias: `promethee-i`, `promethee-ii`)
**Domain:** Decision Logic
**Definition:** Outranking method: pairwise preference functions per criterion, then positive/negative flow rankings.
**Atom or composite:** Composite — preference fns + flow computation.
**Cost model:** O(n²·m) pairwise comparisons.
**Real wall?** Yes — preference function shape (Gaussian/linear/U) is a modeling choice.
**Cross-domain wiring:** `combinatorial-optimization/outranking`; `agentic-reasoning/preference-flow`.
**Notes:** Brans 1982. PROMETHEE I (partial), II (complete) ranking.

### electre (cross-domain alias: `electre-method`, `electre-iii`)
**Domain:** Decision Logic
**Definition:** Outranking family using concordance/discordance indices and veto thresholds.
**Atom or composite:** Composite — concordance matrix + discordance + outranking graph.
**Cost model:** O(n²·m).
**Real wall?** Yes — many parameters (thresholds, vetoes) to elicit; sensitive to them.
**Cross-domain wiring:** `combinatorial-optimization/multi-criteria`; `logic-reasoning/outranking-relation`.
**Notes:** Roy 1968. ELECTRE I/II/III/IV/IS/TRI variants.

### voting-rule-plurality (cross-domain alias: `plurality-vote`, `first-past-the-post`)
**Domain:** Decision Logic
**Definition:** Each voter picks one alternative; winner is the option with most votes.
**Atom or composite:** Atom — count + argmax.
**Cost model:** O(voters + |A|).
**Real wall?** Yes — fails IIA, Condorcet; "spoiler" pathologies.
**Cross-domain wiring:** `combinatorial-optimization/voting`; `agentic-reasoning/preference-aggregation`.
**Notes:** Default voting; used in most national elections.

### voting-rule-borda (cross-domain alias: `borda-count`, `positional-vote`)
**Domain:** Decision Logic
**Definition:** Voters rank alternatives; points assigned by position; winner is highest total.
**Atom or composite:** Composite — rank-to-score + sum.
**Cost model:** O(voters · |A|).
**Real wall?** Yes — can violate Condorcet; vulnerable to clones.
**Cross-domain wiring:** `combinatorial-optimization/borda`.
**Notes:** Borda 1770. Used in Eurovision, MLB MVP voting.

### voting-rule-condorcet (cross-domain alias: `condorcet`, `pairwise-majority`)
**Domain:** Decision Logic
**Definition:** Pick the alternative that beats every other in head-to-head majority; may not exist (Condorcet paradox).
**Atom or composite:** Composite — pairwise majority matrix.
**Cost model:** O(voters · |A|²).
**Real wall?** Yes — Condorcet cycles; need a completion method (Schulze, Ranked Pairs).
**Cross-domain wiring:** `combinatorial-optimization/tournament`; `logic-reasoning/preference-cycle`.
**Notes:** Condorcet 1785. Debian uses Schulze.

### preference-aggregation (cross-domain alias: `social-choice`, `aggregator`)
**Domain:** Decision Logic
**Definition:** Combine multiple agents' preferences into a collective decision.
**Atom or composite:** Composite — multiple voting rules studied as aggregators.
**Cost model:** Method-dependent.
**Real wall?** Yes — Arrow's impossibility (next entry).
**Cross-domain wiring:** `agentic-reasoning/multi-agent-vote`; `combinatorial-optimization/social-choice`.
**Notes:** Studied as the central problem of social choice theory.

### arrow-impossibility (cross-domain alias: `arrows-theorem`, `arrow-impossibility-theorem`)
**Domain:** Decision Logic
**Definition:** No social welfare function with 3+ alternatives satisfies unanimity, IIA, non-dictatorship simultaneously.
**Atom or composite:** N/A — a theorem about decision systems.
**Cost model:** N/A.
**Real wall?** Yes — fundamental impossibility; all practical aggregators violate at least one axiom.
**Cross-domain wiring:** `logic-reasoning/impossibility-theorem`; `agentic-reasoning/multi-agent`.
**Notes:** Arrow 1951 (Nobel 1972). Foreground constraint when designing any voting/aggregation system.

---

## Section 3 — Inference & Reasoning

### fact-triple (cross-domain alias: `rdf-triple`, `eav-tuple`, `frame-slot`)
**Domain:** Decision Logic
**Definition:** Smallest assertion: `(subject, predicate, object)` (RDF) or `(entity, attribute, value)` (EAV).
**Atom or composite:** Atom — irreducible unit of factual content.
**Cost model:** O(1) hash insert; index per role for fast retrieval.
**Real wall?** Yes — open-world vs. closed-world semantics differ on missing triples.
**Cross-domain wiring:** `logic-reasoning/rdf`; `database-streaming-sketching/eav-store`.
**Notes:** RDF, Jena, RDFox, ABox in DL.

### inference-rule-forward (cross-domain alias: `forward-rule`, `production-rule`)
**Domain:** Decision Logic
**Definition:** Rule fired by matching antecedent against current fact base, asserting consequent.
**Atom or composite:** Atom.
**Cost model:** Depends on matcher (RETE amortizes); naive O(rules · facts).
**Real wall?** Yes — termination requires monotonic or well-founded semantics.
**Cross-domain wiring:** `logic-reasoning/forward-chaining`.
**Notes:** Drools, CLIPS, production systems.

### inference-rule-backward (cross-domain alias: `backward-rule`, `goal-driven-rule`)
**Domain:** Decision Logic
**Definition:** Rule used in reverse: to prove goal G, find a rule with G in consequent and recurse on antecedents.
**Atom or composite:** Atom.
**Cost model:** SLD resolution; can loop without occurs-check.
**Real wall?** Yes — search space explodes without memoization/tabling.
**Cross-domain wiring:** `logic-reasoning/backward-chaining`; `agentic-reasoning/goal-decomposition`.
**Notes:** Prolog, XSB tabled resolution.

### forward-chaining-rete (cross-domain alias: `forward-chain`, `data-driven-inference`)
**Domain:** Decision Logic
**Definition:** Apply forward rules using RETE incremental match until fixpoint or limit.
**Atom or composite:** Composite — match + fire loop.
**Cost model:** Per-cycle O(Δfacts·affected-rules) with RETE.
**Real wall?** Yes — non-termination if cycles unbounded; rule cycles guarded by stratification.
**Cross-domain wiring:** `database-streaming-sketching/incremental-eval`.
**Notes:** Default in Drools, CLIPS.

### backward-chaining-prolog (cross-domain alias: `prolog-style`, `sld-resolution`)
**Domain:** Decision Logic
**Definition:** Goal-driven proof search with unification; left-to-right, depth-first SLD resolution.
**Atom or composite:** Composite.
**Cost model:** Exponential worst case; tabling/memoization makes datalog-fragments polynomial.
**Real wall?** Yes — search may not terminate; cut/negation-as-failure are extra-logical.
**Cross-domain wiring:** `logic-reasoning/sld-resolution`; `agentic-reasoning/goal-search`.
**Notes:** SWI-Prolog, XSB, YAP.

### magic-sets (cross-domain alias: `magic-set-rewriting`, `query-driven-bottom-up`)
**Domain:** Decision Logic
**Definition:** Rewriting that lets bottom-up datalog simulate top-down (goal-driven) evaluation, focusing computation.
**Atom or composite:** Composite — rule rewrite pass.
**Cost model:** Same complexity class as datalog but with smaller constants.
**Real wall?** Yes — rewrite explosion for highly recursive rules.
**Cross-domain wiring:** `database-streaming-sketching/datalog-opt`; `logic-reasoning/program-transformation`.
**Notes:** Beeri & Ramakrishnan 1991. Core in modern Datalog engines.

### semi-naive-evaluation (cross-domain alias: `semi-naive`, `differential-datalog`)
**Domain:** Decision Logic
**Definition:** Incremental fixpoint: only join with new deltas from prior iteration, not full materialization.
**Atom or composite:** Composite — delta-tracking loop.
**Cost model:** O(work-per-new-tuple); avoids rederivation.
**Real wall?** Yes — needs set semantics; loses count info.
**Cross-domain wiring:** `database-streaming-sketching/differential-dataflow`.
**Notes:** Bancilhon-Ramakrishnan. Differential Datalog (Datafrog, DDlog).

### datalog-evaluation (cross-domain alias: `datalog-eval`, `bottom-up-datalog`)
**Domain:** Decision Logic
**Definition:** Polynomial fixpoint computation of datalog program: rules are function-free Horn clauses, no negation.
**Atom or composite:** Composite — semi-naive + magic-set + scheduler.
**Cost model:** PTIME data complexity for pure datalog.
**Real wall?** Yes — pure datalog cannot express arithmetic without extensions.
**Cross-domain wiring:** `logic-reasoning/datalog`; `database-streaming-sketching/recursive-cte`.
**Notes:** Soufflé, RDFox, LogicBlox, DDlog.

### recursive-datalog (cross-domain alias: `recursive-rule`, `transitive-closure-rule`)
**Domain:** Decision Logic
**Definition:** Datalog rules where head appears (transitively) in body; expresses reachability, transitive closure.
**Atom or composite:** Composite.
**Cost model:** PTIME; specific patterns optimized (linear recursion, mutual recursion).
**Real wall?** Yes — non-linear recursion can be expensive without left/right-linear rewrites.
**Cross-domain wiring:** `combinatorial-optimization/transitive-closure`; `logic-reasoning/fixpoint`.
**Notes:** Soufflé, LogicBlox, recursive CTE in SQL:1999.

### stratified-negation (cross-domain alias: `stratified-datalog`, `negation-as-failure-stratified`)
**Domain:** Decision Logic
**Definition:** Allow negation in datalog if program can be layered so negated predicates are fully computed before use.
**Atom or composite:** Composite — graph analysis + layered eval.
**Cost model:** Per-stratum PTIME; total PTIME.
**Real wall?** Yes — recursive negation breaks stratification → need well-founded/stable models.
**Cross-domain wiring:** `logic-reasoning/stratification`.
**Notes:** Apt-Blair-Walker. Default semantics in Datalog⁻.

### asp-grounding (cross-domain alias: `answer-set-grounding`, `gringo-ground`)
**Domain:** Decision Logic
**Definition:** Pre-process step in ASP: instantiate all variables in rules with constants, producing propositional program.
**Atom or composite:** Composite — variable-substitution pass.
**Cost model:** Exponential in arity; intelligent grounders prune aggressively.
**Real wall?** Yes — grounding bottleneck for large domains.
**Cross-domain wiring:** `logic-reasoning/answer-set-programming`; `combinatorial-optimization/grounding`.
**Notes:** Gringo, DLV grounder. Lazy grounding research mitigates.

### ontology-rdfs (cross-domain alias: `rdfs`, `rdf-schema`)
**Domain:** Decision Logic
**Definition:** Light ontology vocabulary: classes, subClassOf, properties, domain/range; supports simple entailment.
**Atom or composite:** Composite — small fixed rule set.
**Cost model:** PTIME entailment.
**Real wall?** Yes — too weak for many domains; OWL needed.
**Cross-domain wiring:** `logic-reasoning/ontology`; `database-streaming-sketching/rdf`.
**Notes:** W3C RDFS. Jena, RDFox support.

### ontology-owl-dl (cross-domain alias: `owl-dl`, `owl2-dl`)
**Domain:** Decision Logic
**Definition:** OWL 2 DL profile: SROIQ description logic — most expressive decidable OWL fragment.
**Atom or composite:** Composite.
**Cost model:** N2EXPTIME-complete; practical via tableau methods.
**Real wall?** Yes — undecidable if combined with arithmetic; reasoner heuristics needed.
**Cross-domain wiring:** `logic-reasoning/description-logic`; `formal-verification/tableau`.
**Notes:** Pellet, HermiT, Konclude, FaCT++.

### ontology-owl-el (cross-domain alias: `owl-el`, `el-profile`)
**Domain:** Decision Logic
**Definition:** PTIME OWL fragment for very large bio/medical ontologies: existentials only, no universals/inverse.
**Atom or composite:** Composite.
**Cost model:** PTIME classification.
**Real wall?** Yes — no universals or negation; limited expressivity.
**Cross-domain wiring:** `logic-reasoning/el-logic`.
**Notes:** ELK reasoner. Used in SNOMED CT, Gene Ontology.

### ontology-owl-ql (cross-domain alias: `owl-ql`, `dl-lite`)
**Domain:** Decision Logic
**Definition:** OWL profile based on DL-Lite, designed for query rewriting against relational DBs.
**Atom or composite:** Composite.
**Cost model:** AC0 data complexity; rewrites SPARQL into SQL.
**Real wall?** Yes — limited expressivity, but enables OBDA at scale.
**Cross-domain wiring:** `database-streaming-sketching/obda`; `logic-reasoning/dl-lite`.
**Notes:** Ontop, Mastro reasoners.

### ontology-owl-rl (cross-domain alias: `owl-rl`, `rule-language-profile`)
**Domain:** Decision Logic
**Definition:** OWL profile that can be implemented as forward-chaining Horn rules on RDF.
**Atom or composite:** Composite.
**Cost model:** PTIME via rule materialization.
**Real wall?** Yes — restrictive; trades expressivity for ease of implementation.
**Cross-domain wiring:** `logic-reasoning/horn`; `database-streaming-sketching/materialization`.
**Notes:** OWLRL, RDFox.

### dl-tbox (cross-domain alias: `tbox`, `terminological-box`)
**Domain:** Decision Logic
**Definition:** The schema part of a DL knowledge base: class definitions, subsumption axioms, property characteristics.
**Atom or composite:** Composite — set of axioms.
**Cost model:** Classified once; subsequent queries O(1) lookup in classification table.
**Real wall?** Yes — TBox+ABox consistency is hard in expressive logics.
**Cross-domain wiring:** `logic-reasoning/tbox`.
**Notes:** Classified by Pellet/HermiT/ELK.

### dl-abox (cross-domain alias: `abox`, `assertional-box`)
**Domain:** Decision Logic
**Definition:** The instance/data part of a DL KB: individual assertions (a:C, R(a,b)).
**Atom or composite:** Composite — set of assertions.
**Cost model:** Instance check is expensive; query rewriting helps.
**Real wall?** Yes — open-world means missing assertions don't disprove.
**Cross-domain wiring:** `logic-reasoning/abox`; `database-streaming-sketching/instance-store`.
**Notes:** Realization assigns instances to most-specific named class.

### dl-rbox (cross-domain alias: `rbox`, `role-box`)
**Domain:** Decision Logic
**Definition:** Role hierarchy and characteristics in DL: subPropertyOf, transitive, functional, inverse.
**Atom or composite:** Composite — set of role axioms.
**Cost model:** Affects reasoner complexity class.
**Real wall?** Yes — interactions (e.g., transitivity + cardinality) push toward undecidability.
**Cross-domain wiring:** `logic-reasoning/role-hierarchy`.
**Notes:** SROIQ supports complex RBoxes.

### instance-retrieval (cross-domain alias: `dl-instance-query`, `realization-retrieval`)
**Domain:** Decision Logic
**Definition:** Given concept C, find all individuals a such that the KB entails `a:C`.
**Atom or composite:** Composite — query + entailment check.
**Cost model:** Generally instance-check per individual; heuristics share work.
**Real wall?** Yes — expensive in expressive DL; OBDA mitigates with rewriting.
**Cross-domain wiring:** `database-streaming-sketching/dl-query`; `logic-reasoning/instance-check`.
**Notes:** Standard DL reasoning task.

### dl-classification (cross-domain alias: `taxonomy-classification`, `subsumption-classification`)
**Domain:** Decision Logic
**Definition:** Compute the subsumption hierarchy among all named classes in a TBox.
**Atom or composite:** Composite — O(n²) pairwise subsumption with traversal heuristics.
**Cost model:** Polynomial in EL, hard in SROIQ.
**Real wall?** Yes — expressive logics push toward NEXP/N2EXP.
**Cross-domain wiring:** `logic-reasoning/classification`.
**Notes:** ELK does SNOMED in seconds; HermiT slower but more expressive.

### dl-realization (cross-domain alias: `realization`, `most-specific-class`)
**Domain:** Decision Logic
**Definition:** Assign each ABox individual to its most-specific named class(es).
**Atom or composite:** Composite — instance-check sweep.
**Cost model:** O(individuals · classes); optimizations cluster.
**Real wall?** Yes — large ABox dominates reasoner cost.
**Cross-domain wiring:** `logic-reasoning/realization`.
**Notes:** Pellet, HermiT support.

### semantic-net (cross-domain alias: `semantic-network`, `frame-system`)
**Domain:** Decision Logic
**Definition:** Graph of concepts and labeled relations; frames provide slots with fillers and inheritance.
**Atom or composite:** Composite — node-edge graph.
**Cost model:** O(traverse) per query; inheritance walk O(depth).
**Real wall?** Yes — informal semantics; superseded by RDF/OWL formal foundations.
**Cross-domain wiring:** `logic-reasoning/frames`; `agentic-reasoning/knowledge-graph`.
**Notes:** Minsky's frames; KL-ONE precursor to DL.

### conceptual-graph (cross-domain alias: `cg`, `sowa-conceptual-graph`)
**Domain:** Decision Logic
**Definition:** Graph notation for FOL with concept and relation nodes; basis of CGIF.
**Atom or composite:** Composite.
**Cost model:** Inference via projection (graph matching).
**Real wall?** Yes — projection is NP-hard in general.
**Cross-domain wiring:** `logic-reasoning/conceptual-graph`; `computational-geometry/graph-match`.
**Notes:** Sowa 1976. ISO standard 24707 Common Logic.

### knowledge-graph-triple (cross-domain alias: `kg-edge`, `entity-relation-triple`)
**Domain:** Decision Logic
**Definition:** Directed labeled edge in a knowledge graph: `(head, relation, tail)`.
**Atom or composite:** Atom.
**Cost model:** O(1) per edge; index per role.
**Real wall?** Yes — open-world; missing edges are not negative.
**Cross-domain wiring:** `database-streaming-sketching/property-graph`; `agentic-reasoning/kg`.
**Notes:** Wikidata, DBpedia, ConceptNet, internal KGs at hyperscalers.

### rdf-star (cross-domain alias: `rdf*`, `rdf-1.2-quoted-triple`)
**Domain:** Decision Logic
**Definition:** RDF extension allowing triples as subject/object — meta-assertions about triples (provenance, confidence).
**Atom or composite:** Composite — nested triple.
**Cost model:** Modest overhead vs. reified RDF; native indexing in compliant stores.
**Real wall?** Yes — semantics of quoting/asserting interact subtly.
**Cross-domain wiring:** `database-streaming-sketching/rdf-star`; `logic-reasoning/meta-statement`.
**Notes:** RDF 1.2 / SPARQL-star; Stardog, GraphDB.

### named-graph (cross-domain alias: `quad`, `context-graph`)
**Domain:** Decision Logic
**Definition:** RDF graph identified by IRI; triples become quads `(s,p,o,g)` for context-scoped reasoning.
**Atom or composite:** Composite — graph as first-class entity.
**Cost model:** O(1) per-quad insert; queries scoped by graph filter.
**Real wall?** Yes — semantics of cross-graph entailment varies by spec.
**Cross-domain wiring:** `database-streaming-sketching/quad-store`; `agentic-reasoning/context-isolation`.
**Notes:** SPARQL `GRAPH` clause; Blazegraph, Stardog.

### sparql-inference (cross-domain alias: `sparql-entailment`, `owl-sparql`)
**Domain:** Decision Logic
**Definition:** SPARQL queries evaluated under entailment regime (RDF/RDFS/OWL) rather than simple pattern match.
**Atom or composite:** Composite — query + inference profile.
**Cost model:** Profile-dependent; PTIME for RDFS, harder for OWL DL.
**Real wall?** Yes — query rewriting / materialization tradeoff.
**Cross-domain wiring:** `database-streaming-sketching/sparql`.
**Notes:** W3C SPARQL 1.1 entailment regimes.

### owl-reasoner (cross-domain alias: `pellet`, `hermit`, `elk`, `konclude`)
**Domain:** Decision Logic
**Definition:** Software engine implementing OWL classification, consistency, query over OWL DL/EL/QL/RL.
**Atom or composite:** Composite — tableau/consequence-based algorithm.
**Cost model:** Varies wildly: ELK PTIME, HermiT/Pellet exponential worst case.
**Real wall?** Yes — undecidable extensions; engines target subsets.
**Cross-domain wiring:** `logic-reasoning/owl-reasoner`.
**Notes:** Konclude is current SOTA on OWL Reasoner Evaluation.

### dl-alc-reasoner (cross-domain alias: `alc-tableau`, `description-logic-alc`)
**Domain:** Decision Logic
**Definition:** Tableau-based reasoner for ALC (basic DL with ⊓, ⊔, ¬, ∃, ∀); decides consistency.
**Atom or composite:** Composite — tableau expansion + clash check.
**Cost model:** PSPACE-complete (ALC).
**Real wall?** Yes — blocking required to ensure termination.
**Cross-domain wiring:** `logic-reasoning/tableau`.
**Notes:** Schmidt-Schauss & Smolka 1991.

### classifier-binary (cross-domain alias: `binary-classifier`, `one-vs-rest-classifier`)
**Domain:** Decision Logic
**Definition:** Function returning a class label from {0,1}; the simplest predictive decision.
**Atom or composite:** Atom — fn from features to label.
**Cost model:** Train/inference depends on family; logreg O(d), trees O(depth).
**Real wall?** Yes — calibration of decision threshold separate from model fit.
**Cross-domain wiring:** `ml-training/binary-classification`; `statistics-probability/discrimination`.
**Notes:** Underpins spam, fraud, anomaly gates.

### classifier-multiclass (cross-domain alias: `multiclass-classifier`, `softmax-classifier`)
**Domain:** Decision Logic
**Definition:** Function returning a class label from K>2 options; typically softmax or one-vs-rest decomposition.
**Atom or composite:** Composite if OvR; atom if direct softmax.
**Cost model:** O(K·d) for softmax; K binary models for OvR.
**Real wall?** Yes — class imbalance + calibration per class.
**Cross-domain wiring:** `ml-training/multiclass`.
**Notes:** sklearn `multi_class` param.

### classifier-one-vs-rest (cross-domain alias: `ovr`, `one-vs-all`)
**Domain:** Decision Logic
**Definition:** Multiclass scheme: train K binary classifiers (class i vs rest), pick argmax score.
**Atom or composite:** Composite.
**Cost model:** O(K) train and inference.
**Real wall?** Yes — scores from different OvR models not comparable without calibration.
**Cross-domain wiring:** `ml-training/ovr`.
**Notes:** Often used with SVM/logreg.

### classifier-one-vs-one (cross-domain alias: `ovo`, `pairwise-classifier`)
**Domain:** Decision Logic
**Definition:** Train K(K-1)/2 binary classifiers for each pair; aggregate by voting.
**Atom or composite:** Composite.
**Cost model:** O(K²) train; O(K²) eval per prediction.
**Real wall?** Yes — quadratic blow-up at large K.
**Cross-domain wiring:** `ml-training/pairwise`.
**Notes:** libsvm default for multiclass.

### neural-classifier-in-loop (cross-domain alias: `nn-policy-gate`, `learned-classifier-policy`)
**Domain:** Decision Logic
**Definition:** Neural network deployed as a decision gate inside a policy loop (e.g., LLM safety classifier).
**Atom or composite:** Composite — model + threshold + fallback.
**Cost model:** Inference cost dominates; quantization/distillation typical.
**Real wall?** Yes — adversarial robustness, calibration, drift.
**Cross-domain wiring:** `ml-training/inference`; `agentic-reasoning/learned-policy`.
**Notes:** OpenAI moderation, Anthropic constitutional classifiers, Google Perspective.

### calibrated-classifier-platt (cross-domain alias: `platt-scaling`, `logistic-calibration`)
**Domain:** Decision Logic
**Definition:** Post-hoc logistic regression on classifier scores to recover well-calibrated probabilities.
**Atom or composite:** Composite — scoring model + sigmoid fit.
**Cost model:** O(n) fit; O(1) inference.
**Real wall?** Yes — Platt assumes sigmoid; misfit for some models.
**Cross-domain wiring:** `statistics-probability/calibration`; `ml-training/post-hoc-calibration`.
**Notes:** Platt 1999 for SVM; widely applied.

### calibrated-classifier-isotonic (cross-domain alias: `isotonic-regression`, `monotonic-calibration`)
**Domain:** Decision Logic
**Definition:** Fit a non-parametric monotonic function from scores to probabilities; more flexible than Platt.
**Atom or composite:** Composite.
**Cost model:** O(n log n) pool-adjacent-violators algorithm.
**Real wall?** Yes — needs more data than Platt; overfits with few samples.
**Cross-domain wiring:** `statistics-probability/isotonic-regression`.
**Notes:** sklearn `IsotonicRegression`.

### conformal-prediction (cross-domain alias: `conformal`, `inductive-conformal-prediction`)
**Domain:** Decision Logic
**Definition:** Wrap any classifier/regressor to produce prediction sets with guaranteed marginal coverage.
**Atom or composite:** Composite — scoring + calibration set + quantile.
**Cost model:** O(calib) for threshold; O(1) inference per sample.
**Real wall?** Yes — only marginal coverage; conditional guarantees harder.
**Cross-domain wiring:** `statistics-probability/conformal`; `agentic-reasoning/uncertainty-quantification`.
**Notes:** Vovk, Angelopoulos & Bates 2021. MAPIE, crepes libs.

### anomaly-detector-isolation-forest (cross-domain alias: `iforest`, `isolation-forest`)
**Domain:** Decision Logic
**Definition:** Anomaly detector: random-split trees; anomalies isolated with fewer splits → short path length.
**Atom or composite:** Composite — ensemble of random trees + path-length score.
**Cost model:** Train O(n log n); inference O(log n).
**Real wall?** Yes — assumes anomalies are sparse + different; fails on dense clusters.
**Cross-domain wiring:** `ml-training/anomaly-detection`; `combinatorial-optimization/tree-ensemble`.
**Notes:** Liu, Ting, Zhou 2008. sklearn `IsolationForest`.

### anomaly-detector-one-class-svm (cross-domain alias: `one-class-svm`, `ocsvm`)
**Domain:** Decision Logic
**Definition:** SVM that learns a boundary around normal data; points outside are anomalies.
**Atom or composite:** Composite.
**Cost model:** Train O(n²) (kernel matrix); inference O(SV·d).
**Real wall?** Yes — kernel selection + ν parameter tuning.
**Cross-domain wiring:** `ml-training/svm`; `statistics-probability/density-support`.
**Notes:** Schölkopf et al. sklearn `OneClassSVM`.

### anomaly-detector-lof (cross-domain alias: `lof`, `local-outlier-factor`)
**Domain:** Decision Logic
**Definition:** Score each point by density ratio relative to k-NN; high ratio = outlier.
**Atom or composite:** Composite — k-NN + reachability density.
**Cost model:** O(n²) naive; O(n log n) with index.
**Real wall?** Yes — k choice critical; doesn't scale to high dim.
**Cross-domain wiring:** `computational-geometry/k-nearest-neighbor`; `statistics-probability/density-estimation`.
**Notes:** Breunig et al. 2000.

### anomaly-detector-autoencoder (cross-domain alias: `ae-reconstruction-detector`, `autoencoder-anomaly`)
**Domain:** Decision Logic
**Definition:** Train AE on normal data; flag samples with high reconstruction error.
**Atom or composite:** Composite — AE + thresholded error.
**Cost model:** Train deep; inference O(forward pass).
**Real wall?** Yes — AE may generalize too well, reconstructing anomalies; needs regularization.
**Cross-domain wiring:** `ml-training/autoencoder`; `signal-processing-rf/reconstruction-error`.
**Notes:** Standard in industrial monitoring (e.g., Siemens MindSphere).

### anomaly-detector-kde (cross-domain alias: `kde-anomaly`, `kernel-density-anomaly`)
**Domain:** Decision Logic
**Definition:** Estimate density via KDE; flag points below density threshold.
**Atom or composite:** Composite.
**Cost model:** O(n) per query naive; tree methods O(log n).
**Real wall?** Yes — curse of dimensionality kills KDE above d≈10.
**Cross-domain wiring:** `statistics-probability/kde`.
**Notes:** sklearn `KernelDensity`.

### anomaly-detector-ewma (cross-domain alias: `ewma`, `exponentially-weighted-moving-average`)
**Domain:** Decision Logic
**Definition:** Online detector: track exponentially weighted mean; flag deviations beyond k·σ.
**Atom or composite:** Composite — recursive update + threshold.
**Cost model:** O(1) per sample.
**Real wall?** Yes — assumes stationarity except for jumps; misses gradual drift.
**Cross-domain wiring:** `signal-processing-rf/iir-filter`; `statistics-probability/control-chart`.
**Notes:** Roberts 1959. Standard SPC chart.

### anomaly-detector-cusum (cross-domain alias: `cusum`, `cumulative-sum-control`)
**Domain:** Decision Logic
**Definition:** Cumulative sum of deviations from target; signals when sum exceeds threshold.
**Atom or composite:** Composite — running cumsum + barrier.
**Cost model:** O(1) per sample.
**Real wall?** Yes — sensitive to drift magnitude; thresholds via ARL design.
**Cross-domain wiring:** `signal-processing-rf/change-detection`; `statistics-probability/sequential-test`.
**Notes:** Page 1954. Used in process control + fraud.

### anomaly-detector-hotelling-t2 (cross-domain alias: `hotelling-t2`, `multivariate-t-square`)
**Domain:** Decision Logic
**Definition:** Multivariate generalization of t-test: flag samples with high T² Mahalanobis-like statistic.
**Atom or composite:** Composite.
**Cost model:** O(d²) per sample with cached Σ⁻¹.
**Real wall?** Yes — assumes multivariate Gaussian; fails on heavy tails.
**Cross-domain wiring:** `statistics-probability/mahalanobis`; `linear-algebra-matrix/inverse-covariance`.
**Notes:** Hotelling 1931. Mainstay of multivariate SPC.

### anomaly-detector-hbos (cross-domain alias: `hbos`, `histogram-based-outlier-score`)
**Domain:** Decision Logic
**Definition:** Independent per-feature histograms; outlier score = sum of log inverse-bin-densities.
**Atom or composite:** Composite.
**Cost model:** Train O(n·d); inference O(d).
**Real wall?** Yes — assumes feature independence; misses joint outliers.
**Cross-domain wiring:** `statistics-probability/histogram`.
**Notes:** Goldstein & Dengel 2012. Very fast baseline.

### drift-detector-adwin (cross-domain alias: `adwin`, `adaptive-windowing`)
**Domain:** Decision Logic
**Definition:** Maintain adaptive window; cut when stats of two halves differ significantly.
**Atom or composite:** Composite — windowed two-sample test.
**Cost model:** O(log w) per sample.
**Real wall?** Yes — needs tunable significance; concept of "drift" is task-dependent.
**Cross-domain wiring:** `statistics-probability/sequential-test`; `ml-training/online-drift`.
**Notes:** Bifet & Gavaldà 2007. Default in scikit-multiflow.

### drift-detector-ddm (cross-domain alias: `ddm`, `drift-detection-method`)
**Domain:** Decision Logic
**Definition:** Monitor classifier error rate; signal warning/drift when error+std exceeds thresholds.
**Atom or composite:** Composite — error+std + thresholds.
**Cost model:** O(1) per sample.
**Real wall?** Yes — needs labeled samples; delayed labels limit applicability.
**Cross-domain wiring:** `ml-training/concept-drift`.
**Notes:** Gama et al. 2004.

### drift-detector-eddm (cross-domain alias: `eddm`, `early-drift-detection`)
**Domain:** Decision Logic
**Definition:** DDM variant using distance between errors instead of error rate; detects gradual drift earlier.
**Atom or composite:** Composite.
**Cost model:** O(1).
**Real wall?** Yes — harder to tune; can over-trigger on rare events.
**Cross-domain wiring:** `ml-training/gradual-drift`.
**Notes:** Baena-Garcia et al. 2006.

### drift-detector-ks-test (cross-domain alias: `kolmogorov-smirnov`, `ks-drift`)
**Domain:** Decision Logic
**Definition:** Two-sample Kolmogorov-Smirnov test on reference vs. recent windows; signals distribution shift.
**Atom or composite:** Composite — empirical CDFs + sup-distance.
**Cost model:** O(n log n) per check.
**Real wall?** Yes — univariate; multivariate needs per-feature or MMD.
**Cross-domain wiring:** `statistics-probability/ks-test`.
**Notes:** Alibi Detect, Evidently AI.

### drift-detector-page-hinkley (cross-domain alias: `page-hinkley`, `ph-test`)
**Domain:** Decision Logic
**Definition:** Sequential test on cumulative deviation from running mean; signals when threshold crossed.
**Atom or composite:** Composite — cumulative deviation + min tracker.
**Cost model:** O(1) per sample.
**Real wall?** Yes — drift direction (mean shift) assumed.
**Cross-domain wiring:** `statistics-probability/sequential-change-point`.
**Notes:** Page 1954, Hinkley 1971. Used in IoT.

### concept-drift (cross-domain alias: `p-y-given-x-drift`, `posterior-drift`)
**Domain:** Decision Logic
**Definition:** Change in `P(Y|X)`: same inputs map to different labels over time.
**Atom or composite:** Composite — phenomenon, not detector.
**Cost model:** Detection requires labels.
**Real wall?** Yes — invisible without labels; hardest drift to detect online.
**Cross-domain wiring:** `ml-training/concept-drift`.
**Notes:** Distinct from covariate shift; harder to correct.

### label-drift (cross-domain alias: `prior-shift`, `p-y-drift`)
**Domain:** Decision Logic
**Definition:** Change in marginal P(Y); class prevalence shifts.
**Atom or composite:** Composite.
**Cost model:** Detectable from label frequencies.
**Real wall?** Yes — requires recent labels.
**Cross-domain wiring:** `statistics-probability/prior-shift`.
**Notes:** EM-based correction possible; Saerens 2002.

### covariate-shift-detector (cross-domain alias: `covariate-shift`, `p-x-drift`)
**Domain:** Decision Logic
**Definition:** Detect change in `P(X)` only; assumes P(Y|X) stable.
**Atom or composite:** Composite — density-ratio or two-sample test.
**Cost model:** O(n) per test; MMD, KS, classifier two-sample tests.
**Real wall?** Yes — assumes Y|X stable; rarely strictly true.
**Cross-domain wiring:** `statistics-probability/density-ratio`.
**Notes:** Sugiyama 2007. KLIEP, RuLSIF for importance reweighting.

### ood-detector (cross-domain alias: `out-of-distribution`, `novelty-detector`)
**Domain:** Decision Logic
**Definition:** Detect inputs outside the training distribution; abstain or flag for review.
**Atom or composite:** Composite — score + threshold.
**Cost model:** Method-dependent; often O(inference).
**Real wall?** Yes — open-set recognition is unsolved at scale.
**Cross-domain wiring:** `ml-training/ood-detection`.
**Notes:** Used as a guard before downstream decisions.

### ood-detector-energy-based (cross-domain alias: `energy-ood`, `free-energy-score`)
**Domain:** Decision Logic
**Definition:** Use the logsumexp of logits (free energy) as OOD score; lower energy = in-distribution.
**Atom or composite:** Composite.
**Cost model:** O(inference).
**Real wall?** Yes — calibration of energy threshold per task.
**Cross-domain wiring:** `ml-training/energy-based-model`; `statistics-probability/log-partition`.
**Notes:** Liu et al. NeurIPS 2020.

### ood-detector-odin (cross-domain alias: `odin`, `odin-perturb`)
**Domain:** Decision Logic
**Definition:** OOD scoring via temperature-scaled softmax + gradient-based input perturbation.
**Atom or composite:** Composite — perturb + scale + max-softmax.
**Cost model:** Extra backward pass per sample.
**Real wall?** Yes — needs tuning on OOD validation set.
**Cross-domain wiring:** `ml-training/adversarial-perturb`.
**Notes:** Liang, Li, Srikant ICLR 2018.

### ood-detector-mahalanobis (cross-domain alias: `mahalanobis-ood`, `class-conditional-gaussian-ood`)
**Domain:** Decision Logic
**Definition:** Fit class-conditional Gaussian to feature representations; OOD score = min Mahalanobis distance.
**Atom or composite:** Composite.
**Cost model:** O(d²) per sample given precomputed Σ⁻¹.
**Real wall?** Yes — Gaussianity in feature space rarely holds exactly.
**Cross-domain wiring:** `statistics-probability/mahalanobis`; `ml-training/feature-distance`.
**Notes:** Lee et al. NeurIPS 2018.

### abductive-reasoning (cross-domain alias: `inference-to-best-explanation`, `abduction`)
**Domain:** Decision Logic
**Definition:** Given observation O and rules R, find hypothesis H s.t. R ∪ H ⊨ O; pick minimal/most-probable H.
**Atom or composite:** Composite — search over hypotheses + scoring.
**Cost model:** Σᵖ₂-complete in general.
**Real wall?** Yes — needs explicit explanatory rule base.
**Cross-domain wiring:** `logic-reasoning/abduction`; `agentic-reasoning/diagnosis`.
**Notes:** Peirce. Used in medical diagnosis (e.g., ATMS-based).

### default-reasoning-runtime (cross-domain alias: `default-logic`, `reiter-defaults`)
**Domain:** Decision Logic
**Definition:** Runtime application of default rules (A : B / C — "if A and consistently B, then C") with extension computation.
**Atom or composite:** Composite — extension search.
**Cost model:** Σᵖ₂ for credulous reasoning.
**Real wall?** Yes — multiple extensions, conflict resolution.
**Cross-domain wiring:** `logic-reasoning/default-logic`.
**Notes:** Reiter 1980.

### circumscription-eval (cross-domain alias: `circumscription`, `closed-world-circumscription`)
**Domain:** Decision Logic
**Definition:** Reasoning by minimizing extensions of certain predicates (parallel circumscription).
**Atom or composite:** Composite — second-order minimization.
**Cost model:** Hard in general; tractable fragments studied.
**Real wall?** Yes — second-order semantics; specialized solvers required.
**Cross-domain wiring:** `logic-reasoning/circumscription`.
**Notes:** McCarthy 1980. Implemented in ASP solvers via minimal models.

### well-founded-semantics-eval (cross-domain alias: `wfs`, `well-founded-model`)
**Domain:** Decision Logic
**Definition:** Three-valued semantics for logic programs with negation; tractable PTIME.
**Atom or composite:** Composite — fixpoint of alternating Gelfond-Lifschitz.
**Cost model:** PTIME data complexity.
**Real wall?** Yes — three-valued (true/false/undefined); some queries left undefined.
**Cross-domain wiring:** `logic-reasoning/wfs`.
**Notes:** Van Gelder, Ross, Schlipf 1991. XSB Prolog supports.

---

## Section 4 — Formal Verification at Runtime

### runtime-monitor (cross-domain alias: `runtime-verifier`, `online-monitor`)
**Domain:** Decision Logic
**Definition:** Component that consumes program events and decides whether the trace satisfies a property; emits verdicts.
**Atom or composite:** Composite — event stream + spec + verdict.
**Cost model:** Per-event O(transition); state size depends on property.
**Real wall?** Yes — only finite prefixes observable; some properties undecidable from prefixes.
**Cross-domain wiring:** `logic-reasoning/runtime-verification`; `formal-verification/monitor`.
**Notes:** Foundation of MOP, JavaMOP, RV-Match, Larva.

### runtime-verification (cross-domain alias: `rv`, `monitoring-based-verification`)
**Domain:** Decision Logic
**Definition:** Discipline of synthesizing monitors from formal specs (LTL, regex, contract) and running them alongside the system.
**Atom or composite:** Composite — spec compiler + runtime instrumentation.
**Cost model:** Monitor overhead 1-30% typical.
**Real wall?** Yes — only finds bugs on observed traces, not exhaustive.
**Cross-domain wiring:** `formal-verification/rv`; `agentic-reasoning/self-check`.
**Notes:** RV (Runtime Verification) conference. Industrial tools: NASA's R2U2, Galois' Copilot.

### mop-monitoring-oriented-programming (cross-domain alias: `mop`, `monitoring-oriented`)
**Domain:** Decision Logic
**Definition:** Framework where specs (regex/LTL/CFG) are attached to code points; tool generates instrumentation.
**Atom or composite:** Composite — spec language + weaver + runtime.
**Cost model:** AspectJ-style overhead; spec-dependent.
**Real wall?** Yes — instrumentation gaps mean unobserved violations.
**Cross-domain wiring:** `type-theory-programming-languages/aop`; `formal-verification/instrumentation`.
**Notes:** UIUC project. JavaMOP, ROSMOP, NASA Mojito.

### java-mop (cross-domain alias: `javamop`, `mop-java`)
**Domain:** Decision Logic
**Definition:** Java implementation of MOP — generates AspectJ aspects from formal specs to monitor Java executions.
**Atom or composite:** Composite.
**Cost model:** AspectJ weaving cost; <10% overhead typical.
**Real wall?** Yes — JVM-only; Java-only specs.
**Cross-domain wiring:** `type-theory-programming-languages/aspectj`.
**Notes:** Catches concurrency + API misuse. Successor RV-Monitor multi-language.

### larva-monitor (cross-domain alias: `larva`, `dynamic-automata-monitor`)
**Domain:** Decision Logic
**Definition:** Runtime verifier for Java using DATEs (Dynamic Communicating Automata with Timers and Events).
**Atom or composite:** Composite — DATE compiler + AspectJ weaver.
**Cost model:** Per-event O(transition); timer overhead.
**Real wall?** Yes — Java-only; AspectJ join-point granularity.
**Cross-domain wiring:** `formal-verification/timed-automaton`.
**Notes:** University of Malta. Used in banking compliance.

### mopbox (cross-domain alias: `mopbox`, `mop-library`)
**Domain:** Decision Logic
**Definition:** Java library for embedding monitors in code without external compilation; pluggable spec backends.
**Atom or composite:** Composite — embeddable runtime.
**Cost model:** JVM library overhead.
**Real wall?** Yes — manual integration; no auto weaving.
**Cross-domain wiring:** `formal-verification/embedded-monitor`.
**Notes:** Bodden 2010. Used in research prototypes.

### rv-match (cross-domain alias: `rv-match`, `k-runtime-verifier`)
**Domain:** Decision Logic
**Definition:** C/C++ runtime verifier built on K framework; detects undefined behavior + memory errors.
**Atom or composite:** Composite — K semantics + interpreter.
**Cost model:** ~100x slowdown; deep semantic monitoring.
**Real wall?** Yes — heavy overhead; debugging-only use.
**Cross-domain wiring:** `formal-verification/k-framework`; `type-theory-programming-languages/semantics`.
**Notes:** Runtime Verification Inc. commercial product.

### ltl-runtime-monitor (cross-domain alias: `ltl-monitor`, `ltl-online-checker`)
**Domain:** Decision Logic
**Definition:** Monitor that consumes events and tracks an LTL formula's satisfaction; emits ⊤/⊥/? per prefix.
**Atom or composite:** Composite — formula-to-automaton compile + step.
**Cost model:** Per-event O(automaton-states).
**Real wall?** Yes — many LTL properties only decidable on infinite traces.
**Cross-domain wiring:** `logic-reasoning/ltl`; `formal-verification/buchi-automaton`.
**Notes:** Bauer et al. LTL₃ semantics; Spot library.

### ltl3-three-valued (cross-domain alias: `ltl3`, `three-valued-ltl`)
**Domain:** Decision Logic
**Definition:** LTL semantics with values ⊤, ⊥, ? for finite prefixes; ? when both extensions remain possible.
**Atom or composite:** Composite — three-valued evaluation.
**Cost model:** Same automaton size as LTL₂; verdict via two parallel automata.
**Real wall?** Yes — many properties never go to ⊤/⊥ on finite traces.
**Cross-domain wiring:** `logic-reasoning/three-valued-logic`.
**Notes:** Bauer, Leucker, Schallhart 2011.

### ltl-runtime-semantics (cross-domain alias: `ltl-runtime`, `truncated-ltl`)
**Domain:** Decision Logic
**Definition:** Semantics for LTL over finite traces: includes good/bad/ugly prefix classifications.
**Atom or composite:** Composite — semantic definitions only.
**Cost model:** N/A.
**Real wall?** Yes — interpretation choice matters for monitor design.
**Cross-domain wiring:** `logic-reasoning/finite-trace-ltl`.
**Notes:** Eisner, Fisman, Havlicek; LTLf studied separately.

### automaton-based-monitor (cross-domain alias: `dfa-monitor`, `nfa-monitor`)
**Domain:** Decision Logic
**Definition:** Monitor implemented as a DFA/NFA; events drive transitions; verdict on reaching accept/reject states.
**Atom or composite:** Atom — the monitor representation.
**Cost model:** O(1) per event (DFA), O(|states|) (NFA).
**Real wall?** Yes — automaton size exponential in LTL formula size worst case.
**Cross-domain wiring:** `logic-reasoning/finite-automaton`.
**Notes:** Bridges between regex/LTL/policy specs and runtime code.

### buchi-runtime-monitor (cross-domain alias: `buchi-monitor`, `buchi-runtime`)
**Domain:** Decision Logic
**Definition:** Büchi automaton (acceptance on infinite traces) adapted for finite-prefix verdicts.
**Atom or composite:** Composite — Büchi + verdict mapping.
**Cost model:** Per-event O(|states|).
**Real wall?** Yes — infinite trace semantics doesn't naturally fit finite execution.
**Cross-domain wiring:** `formal-verification/buchi-automaton`.
**Notes:** Used in NASA R2U2 hardware monitors.

### proof-token (cross-domain alias: `attestation`, `signed-proof`)
**Domain:** Decision Logic
**Definition:** Cryptographically signed certificate that a verification step has completed (e.g., policy check passed).
**Atom or composite:** Composite — proof artifact + signature + expiry.
**Cost model:** O(sign) per emit, O(verify) per consume.
**Real wall?** Yes — token freshness, revocation, replay protection.
**Cross-domain wiring:** `cryptography-advanced/digital-signature`; `agentic-reasoning/capability-token`.
**Notes:** SPIFFE/SPIRE SVID, Macaroons, JWT, in-toto attestations.

### policy-attestation (cross-domain alias: `policy-receipt`, `verifiable-decision`)
**Domain:** Decision Logic
**Definition:** Signed receipt of a PDP decision used to convince downstream services without re-querying.
**Atom or composite:** Composite — decision + proof + signature.
**Cost model:** Adds signing latency to PDP; amortized.
**Real wall?** Yes — clock skew, revocation, key rotation.
**Cross-domain wiring:** `cryptography-advanced/x509`; `distributed-systems/audit-log`.
**Notes:** Biscuit tokens, Macaroons; SLSA attestations.

### hoare-triple-runtime (cross-domain alias: `runtime-hoare`, `assertion-triple`)
**Domain:** Decision Logic
**Definition:** `{P} S {Q}` enforced at runtime: assert P before S, assert Q after; not a proof but a check.
**Atom or composite:** Atom.
**Cost model:** O(assertion-eval).
**Real wall?** Yes — runtime checking only catches violations on observed paths.
**Cross-domain wiring:** `logic-reasoning/hoare-logic`; `formal-verification/contract-check`.
**Notes:** Eiffel, JML, Code Contracts ship runtime-checked Hoare triples.

### hoare-weakest-precondition (cross-domain alias: `wp`, `weakest-precondition`)
**Domain:** Decision Logic
**Definition:** `wp(S, Q)` — weakest P such that `{P} S {Q}` holds; computed by backward predicate transformer.
**Atom or composite:** Composite — recursive predicate transformer.
**Cost model:** Symbolic — depends on solver.
**Real wall?** Yes — undecidable for unbounded loops / FOL.
**Cross-domain wiring:** `logic-reasoning/wp-calculus`; `formal-verification/symbolic-execution`.
**Notes:** Dijkstra 1975. Used in Why3, Frama-C, Boogie.

### dijkstra-wp-sp (cross-domain alias: `wp-sp`, `predicate-transformer`)
**Domain:** Decision Logic
**Definition:** Dual operators: weakest precondition `wp` (backward), strongest postcondition `sp` (forward).
**Atom or composite:** Composite — pair of recursive transformers.
**Cost model:** Symbolic; SMT-backed.
**Real wall?** Yes — same undecidability barriers.
**Cross-domain wiring:** `formal-verification/predicate-transformer`.
**Notes:** Dijkstra's "Discipline of Programming" 1976.

### separation-logic-runtime (cross-domain alias: `runtime-separation-logic`, `dynamic-separation`)
**Domain:** Decision Logic
**Definition:** Runtime checks for ownership/disjointness conditions of separation logic (e.g., RustBelt, Verus).
**Atom or composite:** Composite — heap region tracker.
**Cost model:** Variable; lightweight if compile-time-checked.
**Real wall?** Yes — runtime tracking is partial; most enforcement static.
**Cross-domain wiring:** `logic-reasoning/separation-logic`; `type-theory-programming-languages/borrow-checker`.
**Notes:** Iris, RefinedC, Verus.

### ownership-runtime-check (cross-domain alias: `ownership-check`, `borrow-violation`)
**Domain:** Decision Logic
**Definition:** Runtime guard verifying a value's owner/borrow status (e.g., RefCell `borrow_mut`).
**Atom or composite:** Atom.
**Cost model:** O(1) atomic counter check.
**Real wall?** Yes — runtime panics rather than compile error.
**Cross-domain wiring:** `type-theory-programming-languages/rust-borrow`.
**Notes:** Rust `RefCell`, `Mutex`. Cost paid when static check is too restrictive.

### raii-pattern (cross-domain alias: `raii`, `resource-acquisition-is-initialization`)
**Domain:** Decision Logic
**Definition:** Bind resource lifetime to object lifetime; destructor enforces release; key runtime invariant pattern.
**Atom or composite:** Atom — ctor/dtor pair.
**Cost model:** O(0) overhead; structural.
**Real wall?** Yes — leaks possible if lifetimes are extended (e.g., via cycles).
**Cross-domain wiring:** `type-theory-programming-languages/raii`; `operating-systems/resource-tracking`.
**Notes:** C++, Rust idiom; finally-block analogue.

### design-by-contract-eiffel (cross-domain alias: `dbc`, `eiffel-contracts`)
**Domain:** Decision Logic
**Definition:** Meyer's discipline: every method has require/ensure clauses and class has invariant; checked at runtime.
**Atom or composite:** Composite — method + pre + post + invariant.
**Cost model:** Pre/post/invariant evaluation per call.
**Real wall?** Yes — heavy contracts → costly; often turned off in prod.
**Cross-domain wiring:** `type-theory-programming-languages/contracts`.
**Notes:** Eiffel; influenced JML, Spec#, Dafny.

### dotnet-code-contracts (cross-domain alias: `code-contracts`, `microsoft-contracts`)
**Domain:** Decision Logic
**Definition:** .NET library: `Contract.Requires`, `Contract.Ensures`, `Contract.Invariant`; backed by static analyzer.
**Atom or composite:** Composite — runtime + static check.
**Cost model:** Pre/post eval; static checker offline.
**Real wall?** Yes — Microsoft deprecated; community CodeContracts.NET maintained.
**Cross-domain wiring:** `formal-verification/static-contract`.
**Notes:** Originally based on Spec#.

### jml-contracts (cross-domain alias: `jml`, `java-modeling-language`)
**Domain:** Decision Logic
**Definition:** Behavioral interface specification language for Java: `requires`, `ensures`, `invariant`, ghost vars.
**Atom or composite:** Composite — spec language + compilers (jmlc, OpenJML, KeY).
**Cost model:** Runtime instrumentation + optional static proof.
**Real wall?** Yes — full FOL specs may not be runtime-decidable.
**Cross-domain wiring:** `formal-verification/jml`.
**Notes:** KeY tool proves JML statically; OpenJML hybrid.

### spec-sharp (cross-domain alias: `spec#`, `microsoft-spec-sharp`)
**Domain:** Decision Logic
**Definition:** Microsoft research language extending C# with non-null types, contracts, and Boogie-based verification.
**Atom or composite:** Composite.
**Cost model:** Runtime contract eval; verification cost offline.
**Real wall?** Yes — discontinued; lessons folded into Dafny.
**Cross-domain wiring:** `formal-verification/boogie`.
**Notes:** Barnett, Leino, Schulte 2005.

### dafny-contract (cross-domain alias: `dafny`, `dafny-specification`)
**Domain:** Decision Logic
**Definition:** Dafny language with `requires`, `ensures`, `decreases`, `invariant`; Z3-backed verification.
**Atom or composite:** Composite — spec + Boogie compile + Z3 query.
**Cost model:** Per-verification SMT calls; can be seconds.
**Real wall?** Yes — undecidable underlying logic; needs hints (`assert`, `triggers`).
**Cross-domain wiring:** `formal-verification/dafny`; `logic-reasoning/smt`.
**Notes:** Used in AWS s2n-tls verification.

### fstar-refinement (cross-domain alias: `fstar`, `f-star-refinement`)
**Domain:** Decision Logic
**Definition:** F* refinement types `x:t{P x}` checked by SMT; runtime check needed when SMT can't discharge.
**Atom or composite:** Composite — type + predicate.
**Cost model:** Static SMT + occasional runtime cast checks.
**Real wall?** Yes — gradual loss of refinement at module boundaries.
**Cross-domain wiring:** `type-theory-programming-languages/refinement-type`.
**Notes:** Project Everest (HACL*, miTLS).

### liquid-haskell (cross-domain alias: `lh`, `liquid-types-haskell`)
**Domain:** Decision Logic
**Definition:** Refinement types for Haskell via Liquid types; SMT-backed; ghost predicates and termination metrics.
**Atom or composite:** Composite.
**Cost model:** Compile-time SMT; runtime cost zero.
**Real wall?** Yes — refinement language is decidable fragment of FOL.
**Cross-domain wiring:** `type-theory-programming-languages/liquid-types`.
**Notes:** Vazou, Jhala. Used in fault-tolerant Redis driver.

### refinement-type-runtime (cross-domain alias: `gradual-refinement`, `runtime-cast-check`)
**Domain:** Decision Logic
**Definition:** When static refinement check can't discharge, insert runtime cast checking the predicate.
**Atom or composite:** Composite — cast + predicate eval.
**Cost model:** O(predicate).
**Real wall?** Yes — performance loss; blame at runtime.
**Cross-domain wiring:** `type-theory-programming-languages/gradual-typing`.
**Notes:** Sage, Stardust, Refined TypeScript.

### gradual-contract (cross-domain alias: `gradual-typing-contract`, `contract-import`)
**Domain:** Decision Logic
**Definition:** At typed/untyped boundary, attach contract that validates values crossing in both directions.
**Atom or composite:** Composite — bidirectional contract.
**Cost model:** O(contract) per boundary crossing.
**Real wall?** Yes — performance death of a thousand cuts (Takikawa et al.).
**Cross-domain wiring:** `type-theory-programming-languages/gradual`.
**Notes:** Racket, Typed Racket. Blame the right module.

### blame-tracking (cross-domain alias: `blame-calculus`, `contract-blame`)
**Domain:** Decision Logic
**Definition:** Track which module is responsible when a contract fails; report it as the blameworthy party.
**Atom or composite:** Composite — blame labels propagated through casts.
**Cost model:** Adds label propagation; usually constant.
**Real wall?** Yes — "well-typed programs can't be blamed" theorems hold only in restricted settings.
**Cross-domain wiring:** `logic-reasoning/blame-calculus`.
**Notes:** Findler-Felleisen 2002; Wadler-Findler 2009.

### runtime-assert (cross-domain alias: `assert`, `runtime-assertion`)
**Domain:** Decision Logic
**Definition:** Inline boolean expression that aborts the program if false; the simplest runtime invariant primitive.
**Atom or composite:** Atom.
**Cost model:** O(predicate); often `NDEBUG`-stripped in release.
**Real wall?** Yes — disabled asserts → silent corruption.
**Cross-domain wiring:** `type-theory-programming-languages/assertion`.
**Notes:** Every language ships one; Linux kernel `BUG_ON`.

### sanity-check (cross-domain alias: `sanity-test`, `internal-consistency-check`)
**Domain:** Decision Logic
**Definition:** Coarse-grained invariant check; less precise than assert, often pre/post a logical block.
**Atom or composite:** Atom.
**Cost model:** O(state-summary).
**Real wall?** Yes — gives false confidence if too coarse.
**Cross-domain wiring:** `formal-verification/inv-check`.
**Notes:** Linux `WARN_ON_ONCE`; Kubernetes admission.

### smoke-test (cross-domain alias: `smoke-test`, `liveness-smoke-check`)
**Domain:** Decision Logic
**Definition:** Minimal "does the system come up at all" test, run pre-prod and on deploy.
**Atom or composite:** Composite — script + pass/fail.
**Cost model:** Seconds to minutes.
**Real wall?** Yes — finds only catastrophic regressions.
**Cross-domain wiring:** `distributed-systems/deploy-gate`.
**Notes:** Origin: electronics; first turn-on test.

### canary-check (cross-domain alias: `canary-deploy`, `progressive-canary`)
**Domain:** Decision Logic
**Definition:** Deploy to small % of traffic, observe metrics; auto-rollback on regression beyond bound.
**Atom or composite:** Composite — deploy + metric eval + rollback.
**Cost model:** Hours of soak; small risk surface.
**Real wall?** Yes — slow-burn issues survive canary windows.
**Cross-domain wiring:** `distributed-systems/canary`; `statistics-probability/ab-test`.
**Notes:** Spinnaker, Argo Rollouts, Flagger.

### sentinel-value (cross-domain alias: `sentinel`, `magic-marker`)
**Domain:** Decision Logic
**Definition:** Special value (NaN, -1, ENOENT, null-object) marking absence or termination of valid data.
**Atom or composite:** Atom.
**Cost model:** O(1) compare.
**Real wall?** Yes — collisions when sentinel is a valid datum.
**Cross-domain wiring:** `type-theory-programming-languages/option-type`.
**Notes:** C-style errno, Rust `Option::None`, Python `None`.

### liveness-property (cross-domain alias: `eventually-property`, `progress-property`)
**Domain:** Decision Logic
**Definition:** "Something good eventually happens" — LTL `◇φ`; violated only on infinite traces.
**Atom or composite:** Atom — class of properties.
**Cost model:** Not finitely refutable.
**Real wall?** Yes — runtime can never confirm; only ◯-falsify with deadline.
**Cross-domain wiring:** `logic-reasoning/liveness`; `distributed-systems/eventual-consistency`.
**Notes:** Alpern-Schneider 1985 classification.

### safety-property (cross-domain alias: `always-property`, `nothing-bad-property`)
**Domain:** Decision Logic
**Definition:** "Nothing bad ever happens" — LTL `□¬bad`; finitely refutable on first bad prefix.
**Atom or composite:** Atom.
**Cost model:** Per-event O(transition).
**Real wall?** Yes — only catches what's observable.
**Cross-domain wiring:** `logic-reasoning/safety`; `formal-verification/invariant`.
**Notes:** Foundation of most runtime monitors.

### invariant-monitor (cross-domain alias: `invariant-checker`, `inv-monitor`)
**Domain:** Decision Logic
**Definition:** Continuous monitor that an invariant predicate holds across system states.
**Atom or composite:** Composite — sampler + predicate + alert.
**Cost model:** O(predicate) per sample; sampling rate determines coverage.
**Real wall?** Yes — gaps between samples mask violations.
**Cross-domain wiring:** `formal-verification/inv-monitor`.
**Notes:** Daikon discovers likely invariants from traces.

### schedulability-check (cross-domain alias: `schedulability-analysis`, `rt-schedulability`)
**Domain:** Decision Logic
**Definition:** Determine whether a real-time task set meets deadlines given a scheduling policy.
**Atom or composite:** Composite — utilization + response-time tests.
**Result wall?** Yes.
**Cost model:** Utilization O(n); response-time iterative.
**Real wall?** Yes — NP-hard for general task models; tractable for periodic.
**Cross-domain wiring:** `operating-systems/edf`; `combinatorial-optimization/scheduling`.
**Notes:** Liu-Layland; Joseph-Pandya. AUTOSAR, RTEMS.

### smt-backed-config-check (cross-domain alias: `config-smt`, `policy-smt`)
**Domain:** Decision Logic
**Definition:** Compile configuration constraints to SMT, check at deploy time.
**Atom or composite:** Composite — translator + solver.
**Cost model:** ms-sec per check; PSPACE+ for quantified.
**Real wall?** Yes — modeling fidelity vs. solver tractability.
**Cross-domain wiring:** `logic-reasoning/smt-solver`; `formal-verification/config-verification`.
**Notes:** Zelkova for AWS S3 policies; ConfigVer.

### opa-equivalence-check (cross-domain alias: `policy-equivalence`, `rego-equivalence`)
**Domain:** Decision Logic
**Definition:** Prove two Rego/OPA policies decide identically on all inputs (or find a counterexample).
**Atom or composite:** Composite — partial eval + SMT.
**Cost model:** Hard; tractable for typed datalog fragment.
**Real wall?** Yes — full Rego is Turing-complete; restricted profile needed.
**Cross-domain wiring:** `formal-verification/policy-equivalence`.
**Notes:** OPA `eval --partial`; commercial Styra.

### tla-plus (cross-domain alias: `tla+`, `tlaplus-spec`)
**Domain:** Decision Logic
**Definition:** Lamport's specification language for concurrent systems; TLC model-checker explores state.
**Atom or composite:** Composite — spec + temporal property + checker.
**Cost model:** State explosion bound; model-checking offline.
**Real wall?** Yes — finite-model checking only.
**Cross-domain wiring:** `formal-verification/tla`; `distributed-systems/spec`.
**Notes:** Used by AWS, Cosmos DB, Confluent, MongoDB.

### spin-model-checker (cross-domain alias: `spin`, `spin-promela`)
**Domain:** Decision Logic
**Definition:** Holzmann's explicit-state LTL model checker for Promela models of concurrent systems.
**Atom or composite:** Composite.
**Cost model:** Exponential in state; partial-order reduction key.
**Real wall?** Yes — state explosion.
**Cross-domain wiring:** `formal-verification/spin`.
**Notes:** Used in NASA Mars Pathfinder, Boeing 777.

### nusmv-symbolic (cross-domain alias: `nusmv`, `symbolic-mc`)
**Domain:** Decision Logic
**Definition:** BDD-based symbolic model checker for CTL/LTL over finite-state systems.
**Atom or composite:** Composite — symbolic encoding + fixpoint.
**Cost model:** Polynomial in BDD size; exponential worst case.
**Real wall?** Yes — variable ordering.
**Cross-domain wiring:** `formal-verification/symbolic-mc`; `logic-reasoning/bdd`.
**Notes:** NuSMV, nuXmv successor; widely used in hardware.

### model-check-as-build-gate (cross-domain alias: `mc-gate`, `ci-model-check`)
**Domain:** Decision Logic
**Definition:** Block deployment if a model-checker finds a counterexample in the deployable configuration.
**Atom or composite:** Composite — CI step + MC + report.
**Cost model:** Build-time minutes; offline.
**Real wall?** Yes — only catches what's in the model.
**Cross-domain wiring:** `formal-verification/ci-gate`.
**Notes:** AWS S3, IronFleet pattern.

### model-check-proof-token (cross-domain alias: `mc-proof-cert`, `verifiable-mc-output`)
**Domain:** Decision Logic
**Definition:** Output of MC tagged as a proof-token consumable by downstream PEPs (e.g., production-system gate).
**Atom or composite:** Composite — MC result + signature.
**Cost model:** Signing cost.
**Real wall?** Yes — model fidelity to deployed system.
**Cross-domain wiring:** `cryptography-advanced/signed-attestation`.
**Notes:** SLSA L4-style attestation.

### runtime-witness (cross-domain alias: `witness`, `evidence-record`)
**Domain:** Decision Logic
**Definition:** Concrete trace fragment that justifies a verdict (positive or negative); stored for audit.
**Atom or composite:** Composite — event-sequence + metadata.
**Cost model:** O(trace) storage.
**Real wall?** Yes — privacy + volume of witnesses.
**Cross-domain wiring:** `logic-reasoning/proof-witness`.
**Notes:** Counterexamples in model checking; positive witnesses in conformance.

### runtime-counterexample (cross-domain alias: `cex`, `failure-trace`)
**Domain:** Decision Logic
**Definition:** Specific trace observed at runtime that violates a property; minimized for debugging.
**Atom or composite:** Composite.
**Cost model:** Trace capture + minimization.
**Real wall?** Yes — minimization is NP-hard.
**Cross-domain wiring:** `formal-verification/counterexample`.
**Notes:** Slicing + delta-debugging.

### fault-injection (cross-domain alias: `fi`, `fault-injection-testing`)
**Domain:** Decision Logic
**Definition:** Inject controlled faults (latency, errors, crashes) at runtime to verify resilience properties.
**Atom or composite:** Composite — injector + observer.
**Cost model:** Test runtime; production-grade requires safety bounds.
**Real wall?** Yes — coverage of fault space is exponential.
**Cross-domain wiring:** `distributed-systems/chaos-engineering`; `formal-verification/fault-model`.
**Notes:** Jepsen tests, ChaosMesh, Gremlin.

### chaos-engineering-hook (cross-domain alias: `chaos-hook`, `chaos-monkey`)
**Domain:** Decision Logic
**Definition:** Instrumented site in code where chaos tools can swap behavior (delay, drop, error).
**Atom or composite:** Atom — sourceable hook.
**Cost model:** O(1) when disabled.
**Real wall?** Yes — must be safe by construction.
**Cross-domain wiring:** `distributed-systems/chaos`.
**Notes:** Netflix Chaos Monkey lineage.

### fault-tree-analysis-runtime (cross-domain alias: `fta`, `fault-tree`)
**Domain:** Decision Logic
**Definition:** Hierarchical model of fault propagation (AND/OR gates); used to compute system reliability.
**Atom or composite:** Composite.
**Cost model:** Polynomial in tree size; quantitative analysis NP-hard.
**Real wall?** Yes — modeling completeness limits validity.
**Cross-domain wiring:** `statistics-probability/reliability`; `formal-verification/safety-case`.
**Notes:** IEC 61025; SAE ARP4761.

### fmea-at-runtime (cross-domain alias: `fmea`, `failure-modes-effects-analysis-runtime`)
**Domain:** Decision Logic
**Definition:** Continuous tracking of identified failure modes vs. observed incidents to update risk priority numbers.
**Atom or composite:** Composite — incident log + classifier + RPN updater.
**Cost model:** Per-incident classification.
**Real wall?** Yes — only modes in catalog get tracked.
**Cross-domain wiring:** `agentic-reasoning/incident-mgmt`.
**Notes:** Automotive SPICE, aerospace AS9100.

---

## Section 5 — Planning & Orchestration

### goal-representation (cross-domain alias: `goal`, `intent`, `desired-state`)
**Domain:** Decision Logic
**Definition:** Formal representation of what the agent wants to achieve: predicate over states, target set, utility threshold.
**Atom or composite:** Atom — single goal description.
**Cost model:** O(predicate) per state-check.
**Real wall?** Yes — under-specified goals lead to wireheading / reward hacking.
**Cross-domain wiring:** `agentic-reasoning/intention`; `logic-reasoning/goal-state`.
**Notes:** PDDL `:goal`, BDI agent's desire.

### goap-planner (cross-domain alias: `goap`, `goal-oriented-action-planner`)
**Domain:** Decision Logic
**Definition:** Game-AI planner: A* search from current state to goal using actions with preconditions and effects.
**Atom or composite:** Composite — search + heuristic + action library.
**Cost model:** O(branching^depth); heuristic prunes.
**Real wall?** Yes — re-plans constantly in dynamic environments.
**Cross-domain wiring:** `combinatorial-optimization/a-star`; `agentic-reasoning/game-ai`.
**Notes:** Orkin's F.E.A.R., MachineGames games.

### htn-planner (cross-domain alias: `htn`, `hierarchical-task-network`)
**Domain:** Decision Logic
**Definition:** Planner decomposing high-level tasks into subtasks via methods until primitive operators remain.
**Atom or composite:** Composite — methods + operators + decomposition.
**Cost model:** Exponential worst case; PSPACE-complete for SHOP-style.
**Real wall?** Yes — author burden is hierarchical decomposition.
**Cross-domain wiring:** `agentic-reasoning/htn`; `logic-reasoning/task-decomposition`.
**Notes:** SHOP2, PANDA, IPC HTN track.

### strips (cross-domain alias: `strips-planner`, `strips-language`)
**Domain:** Decision Logic
**Definition:** Classical planning representation: states as fact sets, actions with add/delete/precondition lists.
**Atom or composite:** Composite.
**Cost model:** PSPACE-complete in worst case.
**Real wall?** Yes — closed-world; no resources, no time.
**Cross-domain wiring:** `logic-reasoning/strips`.
**Notes:** Fikes & Nilsson 1971. Underlies PDDL.

### pddl (cross-domain alias: `pddl`, `planning-domain-definition-language`)
**Domain:** Decision Logic
**Definition:** Standard language for classical planning problems: domains + problems with types/predicates/actions.
**Atom or composite:** Composite — DSL with many extensions (numeric, temporal, durative).
**Cost model:** Depends on extension; classical is PSPACE-complete.
**Real wall?** Yes — modeling effort dominates real applications.
**Cross-domain wiring:** `logic-reasoning/pddl`.
**Notes:** IPC standard; PDDL 2.1 (durative), 3.0 (preferences), 3.1 (numeric).

### adl-language (cross-domain alias: `adl`, `action-description-language`)
**Domain:** Decision Logic
**Definition:** Pednault's ADL: STRIPS + conditional effects + disjunctive preconditions + quantifiers.
**Atom or composite:** Composite — richer than STRIPS.
**Cost model:** Same complexity class; larger constants.
**Real wall?** Yes — many planners require ADL→STRIPS compilation.
**Cross-domain wiring:** `logic-reasoning/adl`.
**Notes:** Pednault 1989. Subsumed by PDDL.

### total-order-planner (cross-domain alias: `top-planner`, `linear-planner`)
**Domain:** Decision Logic
**Definition:** Plans are linear sequences; classical state-space search.
**Atom or composite:** Composite.
**Cost model:** O(branching^depth).
**Real wall?** Yes — over-constrains parallelism; commits to ordering early.
**Cross-domain wiring:** `combinatorial-optimization/sequencing`.
**Notes:** Early planners (STRIPS); now mostly via heuristic search.

### partial-order-planner (cross-domain alias: `pop`, `partial-order-planning`)
**Domain:** Decision Logic
**Definition:** Plans are partially ordered DAGs; only commit to orderings when needed (causal links).
**Atom or composite:** Composite — plan-space search.
**Cost model:** Smaller search space sometimes; harder heuristic guidance.
**Real wall?** Yes — backtracking on threats and open conditions.
**Cross-domain wiring:** `combinatorial-optimization/dag-scheduling`.
**Notes:** UCPOP, NOAH; less competitive now vs. heuristic state-space.

### plan-space-search (cross-domain alias: `plan-space`, `pop-search`)
**Domain:** Decision Logic
**Definition:** Search over partial plans rather than world states; nodes are plans, edges are refinements.
**Atom or composite:** Composite.
**Cost model:** Comparable to state-space; heuristic harder.
**Real wall?** Yes — heuristic estimation harder than in state-space.
**Cross-domain wiring:** `agentic-reasoning/plan-refinement`.
**Notes:** Foundation of POP planners.

### regression-planning (cross-domain alias: `goal-regression`, `backward-planning`)
**Domain:** Decision Logic
**Definition:** Search backward from goal: regress goal through action preconditions to find applicable actions.
**Atom or composite:** Composite.
**Cost model:** Branching often lower; many irrelevant actions pruned.
**Real wall?** Yes — heuristics in regression space harder.
**Cross-domain wiring:** `logic-reasoning/backward-planning`.
**Notes:** Used in Prodigy, some optimal planners.

### graphplan (cross-domain alias: `graphplan`, `planning-graph`)
**Domain:** Decision Logic
**Definition:** Build alternating proposition/action layered graph with mutex relations; extract plan via backward search.
**Atom or composite:** Composite — graph construction + search.
**Cost model:** Polynomial graph construction; NP-hard extraction.
**Real wall?** Yes — relaxed mutex structure limits scaling.
**Cross-domain wiring:** `combinatorial-optimization/layered-graph`.
**Notes:** Blum & Furst 1995.

### satplan (cross-domain alias: `satplan`, `sat-planning`)
**Domain:** Decision Logic
**Definition:** Encode planning problem with bounded horizon as SAT; iteratively increase bound; SAT solver finds plan.
**Atom or composite:** Composite — encoder + SAT solver.
**Cost model:** SAT-complete each iteration; competitive for hard problems.
**Real wall?** Yes — bound selection; long horizons explode formula size.
**Cross-domain wiring:** `logic-reasoning/sat`; `combinatorial-optimization/sat`.
**Notes:** Kautz & Selman; basis for IPC winners.

### ff-fast-forward (cross-domain alias: `ff`, `fast-forward-planner`)
**Domain:** Decision Logic
**Definition:** Hoffmann's heuristic forward planner using relaxed-plan-graph heuristic (h_FF) and enforced hill-climbing.
**Atom or composite:** Composite — heuristic + EHC + best-first fallback.
**Cost model:** Linear heuristic eval per state.
**Real wall?** Yes — local minima; falls back to BFS.
**Cross-domain wiring:** `combinatorial-optimization/heuristic-search`.
**Notes:** IPC-2 winner; baseline ever since.

### lama-planner (cross-domain alias: `lama`, `landmark-anytime-planner`)
**Domain:** Decision Logic
**Definition:** Anytime planner using landmark heuristic + FF heuristic in multi-heuristic best-first.
**Atom or composite:** Composite.
**Cost model:** Anytime: first solution fast, improves with time.
**Real wall?** Yes — anytime improvement plateaus.
**Cross-domain wiring:** `combinatorial-optimization/anytime-search`.
**Notes:** Richter & Westphal; IPC 2008 winner.

### fast-downward (cross-domain alias: `fd`, `fast-downward-planner`)
**Domain:** Decision Logic
**Definition:** Modular planning framework with SAS+ representation, many heuristics (lm-cut, merge-and-shrink, hmax).
**Atom or composite:** Composite.
**Cost model:** Heuristic-dependent.
**Real wall?** Yes — heuristic informedness vs. cost trade.
**Cross-domain wiring:** `combinatorial-optimization/sas-plus`.
**Notes:** Helmert 2006. Standard research planner.

### heuristic-search-a-star (cross-domain alias: `a-star`, `a*-search`)
**Domain:** Decision Logic
**Definition:** Best-first search with f(n) = g(n) + h(n); optimal if h admissible.
**Atom or composite:** Composite — open list + closed list + heuristic.
**Cost model:** O(b^d) worst; polynomial with good heuristic.
**Real wall?** Yes — memory blow-up for hard problems.
**Cross-domain wiring:** `combinatorial-optimization/a-star`; `graphics-rendering-lod/path-find`.
**Notes:** Hart, Nilsson, Raphael 1968.

### heuristic-search-ida-star (cross-domain alias: `ida-star`, `iterative-deepening-a*`)
**Domain:** Decision Logic
**Definition:** Iterative-deepening DFS with f-cost cutoff; linear memory in depth.
**Atom or composite:** Composite.
**Cost model:** Re-expansion overhead; constant memory.
**Real wall?** Yes — re-expansion can be exponential vs. A*.
**Cross-domain wiring:** `combinatorial-optimization/iterative-deepening`.
**Notes:** Korf 1985.

### heuristic-search-rbfs (cross-domain alias: `rbfs`, `recursive-best-first-search`)
**Domain:** Decision Logic
**Definition:** Recursive best-first with f-cost backed-up bounds; linear memory like IDA* but better behavior.
**Atom or composite:** Composite.
**Cost model:** Re-expansion possible; less than IDA* typically.
**Real wall?** Yes — same memory-time tradeoff.
**Cross-domain wiring:** `combinatorial-optimization/best-first`.
**Notes:** Korf 1993.

### weighted-a-star (cross-domain alias: `wa-star`, `weighted-a*`)
**Domain:** Decision Logic
**Definition:** A* with f = g + w·h; w>1 gives faster but suboptimal solutions bounded by w.
**Atom or composite:** Composite.
**Cost model:** Faster than A*; bounded-suboptimal.
**Real wall?** Yes — w must be chosen; bound is loose.
**Cross-domain wiring:** `combinatorial-optimization/bounded-suboptimal`.
**Notes:** Pohl 1970.

### ara-star (cross-domain alias: `ara-star`, `anytime-repairing-a*`)
**Domain:** Decision Logic
**Definition:** Anytime variant of weighted A*: starts with high w, decreases, reuses search state.
**Atom or composite:** Composite.
**Cost model:** Quick first solution; improves with time.
**Real wall?** Yes — re-evaluation cost on weight change.
**Cross-domain wiring:** `combinatorial-optimization/anytime-search`.
**Notes:** Likhachev, Gordon, Thrun 2003.

### ad-star (cross-domain alias: `ad-star`, `anytime-d-star`)
**Domain:** Decision Logic
**Definition:** Anytime D*: combines anytime suboptimality with replanning when edge costs change.
**Atom or composite:** Composite.
**Cost model:** Reuses prior search results.
**Real wall?** Yes — bookkeeping complex; pruning conservative.
**Cross-domain wiring:** `combinatorial-optimization/replanning`.
**Notes:** Likhachev et al. 2005. Robotics navigation.

### lrta-star (cross-domain alias: `lrta-star`, `learning-real-time-a*`)
**Domain:** Decision Logic
**Definition:** Real-time agent: limited look-ahead per step, updates heuristic from observed gaps.
**Atom or composite:** Composite — look-ahead + h-update.
**Cost model:** O(look-ahead) per step.
**Real wall?** Yes — early steps suboptimal; many trials to converge.
**Cross-domain wiring:** `agentic-reasoning/real-time-search`.
**Notes:** Korf 1990.

### rtaa-star (cross-domain alias: `rtaa-star`, `real-time-adaptive-a*`)
**Domain:** Decision Logic
**Definition:** Real-time A* variant: efficient heuristic updates after each move (Dijkstra-style backup).
**Atom or composite:** Composite.
**Cost model:** O(look-ahead) per step.
**Real wall?** Yes — same caveats as LRTA*.
**Cross-domain wiring:** `combinatorial-optimization/real-time-heuristic-search`.
**Notes:** Koenig & Likhachev 2006.

### heuristic-delete-relaxation (cross-domain alias: `relaxed-plan-heuristic`, `delete-relaxation`)
**Domain:** Decision Logic
**Definition:** Drop delete effects from planning problem; cost of optimal relaxed plan is heuristic estimate.
**Atom or composite:** Composite — relaxation + planning in relaxed model.
**Cost model:** Polynomial for h+; still NP-hard.
**Real wall?** Yes — relaxation loses important constraints (e.g., resource consumption).
**Cross-domain wiring:** `combinatorial-optimization/relaxation-heuristic`.
**Notes:** Foundation of h_FF, h_max, h_add.

### heuristic-landmark (cross-domain alias: `landmark-heuristic`, `lm-heuristic`)
**Domain:** Decision Logic
**Definition:** Heuristic from disjunctive action landmarks: subgoals that must be achieved on every plan.
**Atom or composite:** Composite.
**Cost model:** Polynomial extraction; landmarks reused across states.
**Real wall?** Yes — landmark discovery imperfect.
**Cross-domain wiring:** `logic-reasoning/landmark`.
**Notes:** Hoffmann, Porteous, Sebastia 2004.

### heuristic-lm-cut (cross-domain alias: `lm-cut`, `landmark-cut`)
**Domain:** Decision Logic
**Definition:** Iteratively compute justification graphs and minimum-hitting-set landmarks; admissible.
**Atom or composite:** Composite.
**Cost model:** Per-state O(planning-graph).
**Real wall?** Yes — slow per state but very informed.
**Cross-domain wiring:** `combinatorial-optimization/min-cut`.
**Notes:** Helmert & Domshlak 2009.

### heuristic-hmax (cross-domain alias: `hmax`, `h-max-heuristic`)
**Domain:** Decision Logic
**Definition:** Cost of most-expensive precondition; admissible delete-relaxation heuristic.
**Atom or composite:** Composite.
**Cost model:** Polynomial via fixpoint.
**Real wall?** Yes — uninformative (often 0 or small).
**Cross-domain wiring:** `combinatorial-optimization/max-heuristic`.
**Notes:** Bonet & Geffner 2001.

### heuristic-hadd (cross-domain alias: `hadd`, `h-additive`)
**Domain:** Decision Logic
**Definition:** Sum of precondition costs; inadmissible but informative delete-relaxation heuristic.
**Atom or composite:** Composite.
**Cost model:** Polynomial.
**Real wall?** Yes — overestimates (sums shared subgoals).
**Cross-domain wiring:** `combinatorial-optimization/additive-heuristic`.
**Notes:** Bonet & Geffner 2001.

### heuristic-hff (cross-domain alias: `hff`, `ff-heuristic`)
**Domain:** Decision Logic
**Definition:** Length of relaxed plan found by FF; inadmissible but very informative.
**Atom or composite:** Composite.
**Cost model:** Polynomial; per-state.
**Real wall?** Yes — relaxed plan can be misleading.
**Cross-domain wiring:** `combinatorial-optimization/relaxed-plan`.
**Notes:** Hoffmann 2001.

### heuristic-pattern-database (cross-domain alias: `pdb`, `pattern-database`)
**Domain:** Decision Logic
**Definition:** Precomputed table of optimal costs for abstracted subproblem; lookup as heuristic.
**Atom or composite:** Composite — pattern selection + Dijkstra in abstraction.
**Cost model:** Construction O(|pattern-states|); lookup O(1).
**Real wall?** Yes — memory-bound; pattern selection NP-hard.
**Cross-domain wiring:** `combinatorial-optimization/abstraction`.
**Notes:** Culberson & Schaeffer 1998; Korf for Rubik's cube.

### heuristic-merge-and-shrink (cross-domain alias: `m-and-s`, `merge-and-shrink-heuristic`)
**Domain:** Decision Logic
**Definition:** Build abstraction by merging transition systems and shrinking by bisimulation; admissible heuristic.
**Atom or composite:** Composite.
**Cost model:** Exponential offline; lookup O(1).
**Real wall?** Yes — abstraction-size budget hard.
**Cross-domain wiring:** `formal-verification/bisimulation`.
**Notes:** Helmert et al. 2007.

### mdp-runtime (cross-domain alias: `mdp`, `markov-decision-process`)
**Domain:** Decision Logic
**Definition:** Tuple (S, A, P, R, γ); decision-theoretic planning under stochastic actions with reward.
**Atom or composite:** Composite — formalism.
**Cost model:** Solving polynomial in |S|·|A|; large |S| intractable.
**Real wall?** Yes — full enumeration; curse of dimensionality.
**Cross-domain wiring:** `statistics-probability/markov-chain`; `ml-training/reinforcement-learning`.
**Notes:** Bellman 1957.

### pomdp-runtime (cross-domain alias: `pomdp`, `partially-observable-mdp`)
**Domain:** Decision Logic
**Definition:** MDP with hidden state and observations; belief state is policy domain.
**Atom or composite:** Composite.
**Cost model:** PSPACE-hard in finite horizon; undecidable infinite horizon.
**Real wall?** Yes — belief-space planning intractable for large |S|.
**Cross-domain wiring:** `statistics-probability/hmm`; `agentic-reasoning/partial-observability`.
**Notes:** SARSOP, POMCP, DESPOT solvers.

### belief-state-planning (cross-domain alias: `belief-mdp`, `belief-state-search`)
**Domain:** Decision Logic
**Definition:** Plan over belief states (probability distributions over states) rather than ground states.
**Atom or composite:** Composite.
**Cost model:** Belief-space exponential in |S|.
**Real wall?** Yes — must approximate.
**Cross-domain wiring:** `statistics-probability/posterior-belief`.
**Notes:** Foundation of POMDP solvers.

### belief-mdp (cross-domain alias: `b-mdp`, `belief-state-mdp`)
**Domain:** Decision Logic
**Definition:** Reformulation of POMDP as MDP over continuous belief simplex; classical MDP theory applies.
**Atom or composite:** Composite.
**Cost model:** Infinite-dimensional state; approximations needed.
**Real wall?** Yes — only computable via point-based methods.
**Cross-domain wiring:** `statistics-probability/belief-update`.
**Notes:** Kaelbling, Littman, Cassandra 1998.

### finite-state-controller (cross-domain alias: `fsc`, `policy-fsc`)
**Domain:** Decision Logic
**Definition:** Compact policy representation: FSM with action outputs and observation-driven transitions.
**Atom or composite:** Composite.
**Cost model:** Optimization NP-hard; gradient methods used.
**Real wall?** Yes — local optima; node-count tuning.
**Cross-domain wiring:** `formal-verification/fsm-controller`.
**Notes:** POMDP policy via FSC; Hansen 1998.

### value-iteration (cross-domain alias: `vi`, `bellman-value-iteration`)
**Domain:** Decision Logic
**Definition:** Iterative solver for MDP value function via Bellman backups V_{k+1}(s) = max_a [R(s,a)+γ·E[V_k]].
**Atom or composite:** Composite — iterative fixpoint.
**Cost model:** O(|S|²·|A|) per iteration; O((1-γ)^-1·log(ε^-1)) iterations.
**Real wall?** Yes — full state-sweep cost.
**Cross-domain wiring:** `control-numerical-opt/dynamic-programming`.
**Notes:** Bellman 1957.

### policy-iteration (cross-domain alias: `pi`, `howard-policy-iteration`)
**Domain:** Decision Logic
**Definition:** Alternating policy evaluation (solve linear system for V^π) and policy improvement (greedy w.r.t. V^π).
**Atom or composite:** Composite.
**Cost model:** Few iterations but each expensive (matrix solve).
**Real wall?** Yes — only for small MDPs.
**Cross-domain wiring:** `linear-algebra-matrix/linear-solve`.
**Notes:** Howard 1960.

### modified-policy-iteration (cross-domain alias: `mpi`, `partial-policy-evaluation`)
**Domain:** Decision Logic
**Definition:** Hybrid VI/PI: do k Bellman backups instead of full eval; tunable cost-accuracy.
**Atom or composite:** Composite.
**Cost model:** O(k·|S|²·|A|) per iteration.
**Real wall?** Yes — k tuning task-dependent.
**Cross-domain wiring:** `control-numerical-opt/iterative-policy-eval`.
**Notes:** Puterman & Shin 1978.

### prioritized-sweeping (cross-domain alias: `ps`, `prioritized-sweep`)
**Domain:** Decision Logic
**Definition:** Asynchronous VI prioritizing states with largest Bellman residual; faster convergence.
**Atom or composite:** Composite — priority queue + Bellman update.
**Cost model:** Per-state O(predecessors).
**Real wall?** Yes — needs reverse model.
**Cross-domain wiring:** `combinatorial-optimization/priority-queue`.
**Notes:** Moore & Atkeson 1993.

### asynchronous-dp (cross-domain alias: `async-dp`, `asynchronous-dynamic-programming`)
**Domain:** Decision Logic
**Definition:** Update arbitrary states in any order; converges under reachability assumption.
**Atom or composite:** Composite.
**Cost model:** Per-state Bellman backup.
**Real wall?** Yes — slow without good ordering.
**Cross-domain wiring:** `control-numerical-opt/async-iteration`.
**Notes:** Bertsekas. Online RL builds on this.

### lrtdp (cross-domain alias: `labeled-rtdp`, `labeled-real-time-dp`)
**Domain:** Decision Logic
**Definition:** RTDP variant that labels converged states to avoid revisits; useful for stochastic shortest path.
**Atom or composite:** Composite.
**Cost model:** Per-trial polynomial.
**Real wall?** Yes — convergence labeling needs care for cycles.
**Cross-domain wiring:** `combinatorial-optimization/stochastic-shortest-path`.
**Notes:** Bonet & Geffner 2003.

### lao-star (cross-domain alias: `lao-star`, `loop-a-o-star`)
**Domain:** Decision Logic
**Definition:** AO* extended to handle loops; computes optimal partial policies for stochastic problems.
**Atom or composite:** Composite.
**Cost model:** Iterative value propagation on AND-OR graph.
**Real wall?** Yes — graph size blow-up.
**Cross-domain wiring:** `combinatorial-optimization/ao-star`.
**Notes:** Hansen & Zilberstein 2001.

### rtdp (cross-domain alias: `rtdp`, `real-time-dynamic-programming`)
**Domain:** Decision Logic
**Definition:** Online trial-based DP: simulate trajectory, Bellman-back-up along the way, converge over many trials.
**Atom or composite:** Composite.
**Cost model:** Polynomial trials; faster than full VI for goal-reachable states.
**Real wall?** Yes — only converges in reachable region.
**Cross-domain wiring:** `agentic-reasoning/online-planning`.
**Notes:** Barto, Bradtke, Singh 1995.

### mcts-uct (cross-domain alias: `uct`, `mcts`)
**Domain:** Decision Logic
**Definition:** UCB-driven Monte Carlo tree search; balances explore/exploit at each tree node.
**Atom or composite:** Composite — selection + expand + simulate + back-up.
**Cost model:** O(simulations); asymptotically optimal.
**Real wall?** Yes — needs simulator; reward sparsity hurts.
**Cross-domain wiring:** `statistics-probability/multi-armed-bandit`.
**Notes:** Kocsis & Szepesvári 2006. AlphaGo lineage.

### mcts-puct (cross-domain alias: `puct`, `predictor-uct`)
**Domain:** Decision Logic
**Definition:** PUCT: UCB with prior P(a|s) bias from a learned policy; the AlphaZero variant.
**Atom or composite:** Composite.
**Cost model:** O(simulations) with neural net policy/value eval.
**Real wall?** Yes — neural net inference is the dominant cost.
**Cross-domain wiring:** `ml-training/policy-network`.
**Notes:** Rosin 2011; refined in AlphaZero.

### mcts-rave (cross-domain alias: `rave`, `rapid-action-value-estimation`)
**Domain:** Decision Logic
**Definition:** MCTS variant pooling action statistics across the tree (AMAF heuristic).
**Atom or composite:** Composite.
**Cost model:** Same per sim; better early-stage estimates.
**Real wall?** Yes — RAVE bias hurts late convergence; β schedule needed.
**Cross-domain wiring:** `statistics-probability/amaf`.
**Notes:** Gelly & Silver 2007 (CrazyStone, MoGo).

### mcts-duct (cross-domain alias: `duct`, `decoupled-uct`)
**Domain:** Decision Logic
**Definition:** UCT for simultaneous-move games: maintain per-player action stats; mix to compute policy.
**Atom or composite:** Composite.
**Cost model:** Same as UCT.
**Real wall?** Yes — simultaneous moves still NP-hard to optimize exactly.
**Cross-domain wiring:** `agentic-reasoning/multi-agent-game`.
**Notes:** Teytaud 2009.

### mcts-fuse (cross-domain alias: `fuse`, `feature-uct-search`)
**Domain:** Decision Logic
**Definition:** UCT variant integrating feature-based action ranking to accelerate convergence.
**Atom or composite:** Composite.
**Cost model:** Same as UCT plus feature eval.
**Real wall?** Yes — feature quality critical.
**Cross-domain wiring:** `ml-training/feature-engineering`.
**Notes:** Childs, Brodeur, Kocsis 2008.

### alphazero-planner (cross-domain alias: `alphazero`, `alpha-zero`)
**Domain:** Decision Logic
**Definition:** Self-play RL with PUCT MCTS + ResNet policy/value heads; tabula rasa learning.
**Atom or composite:** Composite — MCTS + NN + self-play loop.
**Cost model:** Massive compute; thousands of TPU-hours per game.
**Real wall?** Yes — perfect-information game assumption.
**Cross-domain wiring:** `ml-training/self-play`; `agentic-reasoning/learned-search`.
**Notes:** Silver et al. 2017. Chess/Shogi/Go.

### muzero-planner (cross-domain alias: `muzero`, `mu-zero`)
**Domain:** Decision Logic
**Definition:** Model-based RL: learn a latent dynamics model + value/policy; plan with MCTS over latent state.
**Atom or composite:** Composite — model + planner + RL trainer.
**Cost model:** Comparable to AlphaZero; works without true simulator.
**Real wall?** Yes — model errors compound in long rollouts.
**Cross-domain wiring:** `ml-training/world-model`.
**Notes:** Schrittwieser et al. 2020. Atari + board games.

### pomcp (cross-domain alias: `pomcp`, `partially-observable-mcts`)
**Domain:** Decision Logic
**Definition:** MCTS for POMDPs using particle filter belief representation.
**Atom or composite:** Composite.
**Cost model:** Per-rollout simulation cost.
**Real wall?** Yes — particle depletion in long horizons.
**Cross-domain wiring:** `statistics-probability/particle-filter`.
**Notes:** Silver & Veness 2010.

### despot-solver (cross-domain alias: `despot`, `determinized-sparse-partially-observable-tree`)
**Domain:** Decision Logic
**Definition:** POMDP online solver: builds belief tree on scenarios sampled from belief; regret-bounded.
**Atom or composite:** Composite.
**Cost model:** Sampled scenarios make tree manageable.
**Real wall?** Yes — scenario count vs. quality tradeoff.
**Cross-domain wiring:** `agentic-reasoning/online-pomdp`.
**Notes:** Somani et al. 2013.

### abt-solver (cross-domain alias: `abt`, `adaptive-belief-tree`)
**Domain:** Decision Logic
**Definition:** Online POMDP solver that adapts to belief changes between time steps for re-planning.
**Atom or composite:** Composite.
**Cost model:** Lower cost per re-plan via reuse.
**Real wall?** Yes — adaptation logic complex.
**Cross-domain wiring:** `agentic-reasoning/replan`.
**Notes:** Kurniawati et al.

### online-planning (cross-domain alias: `online-planner`, `receding-horizon`)
**Domain:** Decision Logic
**Definition:** Plan only the next few actions, execute one, re-plan; key for stochastic / partially-observable settings.
**Atom or composite:** Composite — plan + execute + observe loop.
**Cost model:** Per-step planner cost; bounded horizon.
**Real wall?** Yes — myopia; horizon depth must match problem.
**Cross-domain wiring:** `control-numerical-opt/mpc`; `agentic-reasoning/reactive-planning`.
**Notes:** MPC, RHC. Standard in robotics.

### anytime-planning (cross-domain alias: `anytime-planner`, `interruptible-planning`)
**Domain:** Decision Logic
**Definition:** Planner producing a usable plan immediately and improving it over time.
**Atom or composite:** Composite.
**Cost model:** Quality vs. time profile.
**Real wall?** Yes — must be interruptible without invariant violation.
**Cross-domain wiring:** `agentic-reasoning/anytime-algorithm`.
**Notes:** Dean & Boddy 1988.

### contingent-planning (cross-domain alias: `contingent-planner`, `branching-plan`)
**Domain:** Decision Logic
**Definition:** Plan with conditional branches based on observations; produces plan trees, not sequences.
**Atom or composite:** Composite — AND-OR plan.
**Cost model:** EXPSPACE in worst case.
**Real wall?** Yes — observation space drives plan size.
**Cross-domain wiring:** `agentic-reasoning/conditional-plan`.
**Notes:** Bonet & Geffner; PRP planner.

### conformant-planning (cross-domain alias: `conformant`, `non-observable-plan`)
**Domain:** Decision Logic
**Definition:** Plan that succeeds for all initial states without any runtime observation; sensorless.
**Atom or composite:** Composite.
**Cost model:** EXPSPACE-complete.
**Real wall?** Yes — initial uncertainty + no sensing → very restrictive.
**Cross-domain wiring:** `logic-reasoning/sensorless-planning`.
**Notes:** Goldman, Boddy 1996; T0, conformant-FF.

### replanning (cross-domain alias: `replan`, `plan-revision`)
**Domain:** Decision Logic
**Definition:** Re-invoke planner when execution diverges from plan; full or incremental replan.
**Atom or composite:** Composite — divergence detector + planner.
**Cost model:** Up to full planning cost; incremental reduces.
**Real wall?** Yes — repeated replans → thrashing.
**Cross-domain wiring:** `agentic-reasoning/online-replan`.
**Notes:** Backbone of robotic execution.

### plan-repair (cross-domain alias: `plan-repair`, `local-plan-fix`)
**Domain:** Decision Logic
**Definition:** Modify existing plan locally instead of replanning from scratch; cheaper but may degrade.
**Atom or composite:** Composite.
**Cost model:** Local search.
**Real wall?** Yes — local repair may fail; need fallback to replan.
**Cross-domain wiring:** `combinatorial-optimization/local-search`.
**Notes:** Nebel & Koehler 1995.

### plan-recognition (cross-domain alias: `plan-recognition`, `goal-recognition`)
**Domain:** Decision Logic
**Definition:** Infer agent's plan or goal from observed actions; intent inference.
**Atom or composite:** Composite — likelihood model + search.
**Cost model:** NP-hard; plan-library or planning-as-recognition methods.
**Real wall?** Yes — ambiguity inherent in observation.
**Cross-domain wiring:** `agentic-reasoning/theory-of-mind`; `statistics-probability/inverse-planning`.
**Notes:** Ramirez & Geffner 2009.

### scheduling-under-uncertainty (cross-domain alias: `stochastic-scheduling`, `robust-scheduling`)
**Domain:** Decision Logic
**Definition:** Sequence/assign tasks when durations, resources, or arrivals are stochastic.
**Atom or composite:** Composite — stochastic optimization.
**Cost model:** Stochastic programming or robust optimization; NP-hard.
**Real wall?** Yes — scenario explosion.
**Cross-domain wiring:** `combinatorial-optimization/stochastic-program`; `queueing-theory-stochastic-processes/scheduling`.
**Notes:** PROGRES, RCPSP-SU.

### rcpsp (cross-domain alias: `rcpsp`, `resource-constrained-project-scheduling`)
**Domain:** Decision Logic
**Definition:** Schedule precedence-constrained activities subject to renewable resource limits.
**Atom or composite:** Composite.
**Cost model:** NP-hard; CP/MIP/heuristic solvers.
**Real wall?** Yes — exact solvers limited to ~100 activities.
**Cross-domain wiring:** `combinatorial-optimization/scheduling`.
**Notes:** PSPLIB benchmarks.

### dispatch-rule (cross-domain alias: `priority-rule`, `dispatcher-heuristic`)
**Domain:** Decision Logic
**Definition:** Real-time scheduling heuristic: when a resource becomes free, pick task by rule (SPT, EDD, FIFO).
**Atom or composite:** Atom.
**Cost model:** O(|queue|) per dispatch.
**Real wall?** Yes — myopic; suboptimal vs. global solvers.
**Cross-domain wiring:** `operating-systems/scheduler`; `queueing-theory-stochastic-processes/dispatching`.
**Notes:** Used in manufacturing, OS schedulers.

---

## Section 6 — Temporal, Sequencing & Causal

### sequence-operator (cross-domain alias: `then`, `seq`, `sequence`)
**Domain:** Decision Logic
**Definition:** Combinator: execute A, then execute B; control flow's most basic composer.
**Atom or composite:** Atom — single sequencing operator.
**Cost model:** O(cost(A) + cost(B)).
**Real wall?** Yes — assumes A terminates; partial A blocks B.
**Cross-domain wiring:** `type-theory-programming-languages/monadic-bind`; `agentic-reasoning/sequential-action`.
**Notes:** `;` in imperative langs; BT sequence node.

### branch-operator (cross-domain alias: `if-else`, `conditional`)
**Domain:** Decision Logic
**Definition:** Predicate-driven branch: `if p then A else B`; the irreducible decision composer.
**Atom or composite:** Atom.
**Cost model:** O(p) + O(max(A,B)).
**Real wall?** Yes — predicate evaluation is the choke; side-effects in p complicate.
**Cross-domain wiring:** `logic-reasoning/material-conditional`.
**Notes:** Every language's `if`; BT selector + condition.

### loop-while (cross-domain alias: `while-loop`, `bounded-while`)
**Domain:** Decision Logic
**Definition:** Repeat body while predicate holds; the simplest unbounded iteration.
**Atom or composite:** Composite — sequence + branch + backedge.
**Cost model:** O(iterations × body).
**Real wall?** Yes — termination not guaranteed; halting problem.
**Cross-domain wiring:** `logic-reasoning/loop-invariant`.
**Notes:** Needs invariant + variant for proof.

### loop-for (cross-domain alias: `for-loop`, `bounded-for`)
**Domain:** Decision Logic
**Definition:** Bounded iteration: known count or iterator; termination structurally guaranteed.
**Atom or composite:** Composite.
**Cost model:** O(N × body); statically bounded.
**Real wall?** Yes — only as deterministic as the iterator.
**Cross-domain wiring:** `combinatorial-optimization/iteration`.
**Notes:** Range-for in modern languages; Iterator pattern.

### loop-do-while (cross-domain alias: `do-while`, `post-test-loop`)
**Domain:** Decision Logic
**Definition:** Execute body once, then while predicate holds.
**Atom or composite:** Composite.
**Cost model:** O(iterations × body), min 1 iteration.
**Real wall?** Yes — same halting concern.
**Cross-domain wiring:** `logic-reasoning/loop`.
**Notes:** Distinct from while in initial pass guarantee.

### bounded-loop (cross-domain alias: `static-bound-loop`, `for-n-times`)
**Domain:** Decision Logic
**Definition:** Loop whose iteration count is statically bounded; admits termination proofs.
**Atom or composite:** Composite.
**Cost model:** Predictable.
**Real wall?** Yes — bound must be static or proven.
**Cross-domain wiring:** `formal-verification/termination`.
**Notes:** Required for real-time systems (MISRA-C rule).

### unbounded-loop (cross-domain alias: `unbounded-while`, `open-iteration`)
**Domain:** Decision Logic
**Definition:** Loop with no static iteration bound; requires runtime check or external timeout.
**Atom or composite:** Composite.
**Cost model:** Unknown.
**Real wall?** Yes — halting-undecidable.
**Cross-domain wiring:** `logic-reasoning/halting-problem`.
**Notes:** Default in event loops, daemons.

### parallel-composition (cross-domain alias: `par`, `parallel`, `concurrent`)
**Domain:** Decision Logic
**Definition:** Execute A and B concurrently; join when both done. Result depends on interleaving / synchronization.
**Atom or composite:** Atom — single combinator.
**Cost model:** O(max(A,B)) ideally; sync overhead.
**Real wall?** Yes — races; non-determinism; needs synchronization primitives.
**Cross-domain wiring:** `distributed-systems/concurrency`; `operating-systems/fork-join`.
**Notes:** CSP `||`, π-calculus parallel.

### interrupt-operator (cross-domain alias: `interrupt`, `preempt`)
**Domain:** Decision Logic
**Definition:** Combinator: run A until event E occurs, then switch to B; preemptive composition.
**Atom or composite:** Atom.
**Cost model:** Context-switch cost.
**Real wall?** Yes — A's mid-state may be inconsistent at interrupt.
**Cross-domain wiring:** `operating-systems/preemption`.
**Notes:** Esterel signals, Lustre clock.

### race-operator (cross-domain alias: `race`, `select-first`)
**Domain:** Decision Logic
**Definition:** Run A and B in parallel, return first to finish, cancel the other.
**Atom or composite:** Atom.
**Cost model:** O(min(A,B)) + cancel overhead.
**Real wall?** Yes — cancellation safety; partial work cleanup.
**Cross-domain wiring:** `distributed-systems/speculative-execution`.
**Notes:** Go `select`, JavaScript `Promise.race`, Erlang receive.

### behavior-tree-sequence (cross-domain alias: `bt-sequence`, `bt-and-node`)
**Domain:** Decision Logic
**Definition:** BT node: tick children in order; succeed if all succeed; fail on first fail; like short-circuit AND.
**Atom or composite:** Atom — single node type.
**Cost model:** O(children) per tick.
**Real wall?** Yes — order coupling; child failure semantics.
**Cross-domain wiring:** `agentic-reasoning/behavior-tree`.
**Notes:** BehaviorTree.CPP, Unreal Behavior Tree.

### behavior-tree-selector (cross-domain alias: `bt-selector`, `bt-or-node`, `fallback`)
**Domain:** Decision Logic
**Definition:** BT node: tick children in order; succeed on first success; fail if all fail; short-circuit OR.
**Atom or composite:** Atom.
**Cost model:** O(children).
**Real wall?** Yes — exploration order is policy.
**Cross-domain wiring:** `agentic-reasoning/fallback-strategy`.
**Notes:** Foundation of BT decision-making.

### behavior-tree-decorator (cross-domain alias: `bt-decorator`, `bt-modifier`)
**Domain:** Decision Logic
**Definition:** BT node wrapping a single child to modify its return (invert, retry-N, succeed-always, timer).
**Atom or composite:** Atom.
**Cost model:** O(1) + child cost.
**Real wall?** Yes — composability bugs in retry/inverter semantics.
**Cross-domain wiring:** `type-theory-programming-languages/decorator-pattern`.
**Notes:** Common decorators: Inverter, Retry, UntilFailure.

### behavior-tree-parallel (cross-domain alias: `bt-parallel`, `bt-par-node`)
**Domain:** Decision Logic
**Definition:** BT node ticking children in parallel; succeed/fail per policy (all/any).
**Atom or composite:** Atom.
**Cost model:** O(children) per tick.
**Real wall?** Yes — children must be independent; shared state racy.
**Cross-domain wiring:** `agentic-reasoning/concurrent-task`.
**Notes:** BehaviorTree.CPP `Parallel` and `ParallelAll`.

### behavior-tree-subtree (cross-domain alias: `bt-subtree`, `bt-include`)
**Domain:** Decision Logic
**Definition:** Reusable BT fragment referenced by name from a parent tree; modularity primitive.
**Atom or composite:** Composite — named tree fragment.
**Cost model:** O(subtree) per tick.
**Real wall?** Yes — namespace/scope of blackboard variables.
**Cross-domain wiring:** `agentic-reasoning/skill-library`.
**Notes:** PyTrees, ROS2 BT XML format.

### behavior-tree-blackboard (cross-domain alias: `bt-blackboard`, `shared-bt-state`)
**Domain:** Decision Logic
**Definition:** Shared key-value store accessible by all BT nodes; the communication medium.
**Atom or composite:** Composite — typed kv store.
**Cost model:** O(1) get/set.
**Real wall?** Yes — concurrent writes need locking; tight coupling risk.
**Cross-domain wiring:** `agentic-reasoning/blackboard-architecture`.
**Notes:** Classical "blackboard system" lineage.

### statechart-harel (cross-domain alias: `statechart`, `harel-chart`)
**Domain:** Decision Logic
**Definition:** Hierarchical FSM with composite states, history, parallel regions, broadcasts.
**Atom or composite:** Composite.
**Cost model:** O(transition) per event; states scale.
**Real wall?** Yes — semantics has multiple incompatible flavors (UML, classic, SCXML).
**Cross-domain wiring:** `formal-verification/statechart`; `agentic-reasoning/hsm`.
**Notes:** Harel 1987. UML state machine basis.

### hierarchical-state-machine (cross-domain alias: `hsm`, `nested-state-machine`)
**Domain:** Decision Logic
**Definition:** FSM where states can contain sub-FSMs; handle-then-bubble event semantics.
**Atom or composite:** Composite.
**Cost model:** O(depth × handlers).
**Real wall?** Yes — semantics ambiguities in entry/exit/initial.
**Cross-domain wiring:** `operating-systems/state-pattern`.
**Notes:** Samek's QP framework; Boost.Statechart.

### history-pseudostate (cross-domain alias: `history`, `state-history`)
**Domain:** Decision Logic
**Definition:** UML pseudostate that, on re-entry to composite, restores its last active sub-state.
**Atom or composite:** Atom — single pseudostate type.
**Cost model:** O(1) entry; O(record).
**Real wall?** Yes — history persistence across restarts is implementation-dependent.
**Cross-domain wiring:** `agentic-reasoning/state-recovery`.
**Notes:** Shallow history.

### deep-history (cross-domain alias: `deep-history-pseudostate`, `recursive-history`)
**Domain:** Decision Logic
**Definition:** Like history but restores the full nested sub-state hierarchy.
**Atom or composite:** Atom.
**Cost model:** O(depth) restore.
**Real wall?** Yes — same persistence caveats.
**Cross-domain wiring:** `formal-verification/uml-statemachine`.
**Notes:** UML 2 deep-history `H*`.

### orthogonal-regions (cross-domain alias: `or-regions`, `parallel-substate`)
**Domain:** Decision Logic
**Definition:** Statechart composite with concurrent sub-state-machines, sharing events.
**Atom or composite:** Composite.
**Cost model:** O(regions × handlers).
**Real wall?** Yes — combinatorial explosion of joint state.
**Cross-domain wiring:** `distributed-systems/concurrent-fsm`.
**Notes:** Cars: ignition + audio + climate as orthogonal regions.

### uml-activity (cross-domain alias: `activity-diagram`, `uml-activity`)
**Domain:** Decision Logic
**Definition:** UML control-flow diagram with actions, decisions, forks/joins; token-flow semantics.
**Atom or composite:** Composite.
**Cost model:** Per-action execution.
**Real wall?** Yes — semantics under exceptions ill-specified.
**Cross-domain wiring:** `formal-verification/petri-net`.
**Notes:** UML 2 unified with Petri-net-flavored semantics.

### bpmn (cross-domain alias: `bpmn`, `business-process-model-notation`)
**Domain:** Decision Logic
**Definition:** OMG standard for business process models: tasks, gateways, events, swim-lanes.
**Atom or composite:** Composite — XML + execution semantics.
**Cost model:** Engine-dependent.
**Real wall?** Yes — many gateway semantics ambiguities (OR-join).
**Cross-domain wiring:** `agentic-reasoning/process-orchestration`.
**Notes:** Camunda, Activiti, Flowable engines.

### petri-net-runtime (cross-domain alias: `petri-net`, `place-transition-net`)
**Domain:** Decision Logic
**Definition:** Bipartite graph of places/transitions; tokens flow when transitions fire; concurrency-native.
**Atom or composite:** Composite.
**Cost model:** Reachability EXPSPACE; firing O(transition).
**Real wall?** Yes — reachability is EXPSPACE-complete.
**Cross-domain wiring:** `formal-verification/petri-net`.
**Notes:** Murata 1989 survey. Concurrent workflow modeling.

### colored-petri-net (cross-domain alias: `cpn`, `colored-petri-net`)
**Domain:** Decision Logic
**Definition:** Petri net where tokens carry typed values; transitions guarded by ML-style expressions.
**Atom or composite:** Composite.
**Cost model:** Same theoretical complexity; richer modeling.
**Real wall?** Yes — model size still bounded by token domains.
**Cross-domain wiring:** `type-theory-programming-languages/typed-tokens`.
**Notes:** Jensen 1992. CPN Tools.

### workflow-engine (cross-domain alias: `workflow-engine`, `process-engine`)
**Domain:** Decision Logic
**Definition:** Runtime executing process definitions (BPMN, ASL, custom) with task dispatch + persistence.
**Atom or composite:** Composite — scheduler + state store + workers.
**Cost model:** Engine-dependent; persistence dominates.
**Real wall?** Yes — at-least-once vs exactly-once delivery semantics.
**Cross-domain wiring:** `distributed-systems/orchestration`.
**Notes:** Camunda, Temporal, Cadence, Conductor.

### aws-step-functions-asl (cross-domain alias: `asl`, `amazon-states-language`)
**Domain:** Decision Logic
**Definition:** JSON state machine: Task/Choice/Wait/Parallel/Map states; AWS Step Functions runtime.
**Atom or composite:** Composite — DSL + managed runtime.
**Cost model:** Per-state transition pricing; serverless scale.
**Real wall?** Yes — state size limits; nested workflow needed for large.
**Cross-domain wiring:** `distributed-systems/serverless-orchestration`.
**Notes:** Standard and Express variants; integrates Lambda/SQS/etc.

### temporal-workflow (cross-domain alias: `temporal`, `cadence-workflow`)
**Domain:** Decision Logic
**Definition:** Code-as-workflow runtime: deterministic functions checkpointed by event-sourcing; durable execution.
**Atom or composite:** Composite — workflow code + history replay.
**Cost model:** History-replay cost on resume; pricing per action.
**Real wall?** Yes — must be deterministic; non-determinism flagged at replay.
**Cross-domain wiring:** `distributed-systems/event-sourcing`; `formal-verification/deterministic-replay`.
**Notes:** Temporal (Uber Cadence lineage). Used at Snap, Stripe.

### argo-workflows (cross-domain alias: `argo`, `argo-wf`)
**Domain:** Decision Logic
**Definition:** Kubernetes-native workflow runtime: DAG/Steps templates as CRDs, each step a container.
**Atom or composite:** Composite.
**Cost model:** Pod-startup overhead per step.
**Real wall?** Yes — pod cold-start hurts short tasks.
**Cross-domain wiring:** `distributed-systems/kubernetes`.
**Notes:** CNCF graduated. Powers Argo CD, Kubeflow Pipelines.

### airflow-dag (cross-domain alias: `airflow`, `apache-airflow-dag`)
**Domain:** Decision Logic
**Definition:** Python-defined DAG of tasks scheduled and executed by Airflow; ETL/data orchestration standard.
**Atom or composite:** Composite — Python operator graph.
**Cost model:** Scheduler latency; task isolation cost.
**Real wall?** Yes — DAG must be static (re-parsed each schedule).
**Cross-domain wiring:** `distributed-systems/etl-pipeline`.
**Notes:** Apache Airflow. Astronomer hosting.

### timer (cross-domain alias: `timer`, `delay-primitive`)
**Domain:** Decision Logic
**Definition:** Schedule a callback / state-transition at a future time.
**Atom or composite:** Atom.
**Cost model:** O(log n) heap insertion; O(1) tick comparison.
**Real wall?** Yes — clock granularity; drift; cancellation.
**Cross-domain wiring:** `operating-systems/hrtimer`.
**Notes:** Linux `hrtimer`; Go `time.After`.

### oneshot-timer (cross-domain alias: `oneshot`, `one-time-timer`)
**Domain:** Decision Logic
**Definition:** Timer that fires once and disarms.
**Atom or composite:** Atom.
**Cost model:** Same as timer.
**Real wall?** Yes — race between fire and cancel.
**Cross-domain wiring:** `operating-systems/setitimer`.
**Notes:** Most timeout patterns use one-shot.

### periodic-timer (cross-domain alias: `periodic`, `interval-timer`)
**Domain:** Decision Logic
**Definition:** Timer that re-arms at fixed period until cancelled.
**Atom or composite:** Atom.
**Cost model:** O(1) per fire.
**Real wall?** Yes — drift accumulates; tick handling latency.
**Cross-domain wiring:** `operating-systems/periodic-task`.
**Notes:** systemd OnCalendar; cron.

### monotonic-clock (cross-domain alias: `monotonic-clock`, `clock-monotonic`)
**Domain:** Decision Logic
**Definition:** Non-decreasing clock not affected by wall-clock adjustments; for measuring durations.
**Atom or composite:** Atom.
**Cost model:** O(1) read.
**Real wall?** Yes — does not relate to UTC; not for timestamps.
**Cross-domain wiring:** `operating-systems/clock_gettime`.
**Notes:** `CLOCK_MONOTONIC`, Go `time.Now().Sub`.

### deadline-scheduler (cross-domain alias: `edf`, `earliest-deadline-first-scheduler`)
**Domain:** Decision Logic
**Definition:** Scheduling policy picking task with nearest deadline; optimal for preemptive uniprocessor.
**Atom or composite:** Composite.
**Cost model:** O(log n) priority-queue insert.
**Real wall?** Yes — domino effect under overload.
**Cross-domain wiring:** `operating-systems/sched-deadline`.
**Notes:** Linux SCHED_DEADLINE.

### timeout (cross-domain alias: `timeout`, `deadline-cap`)
**Domain:** Decision Logic
**Definition:** Bound on operation duration; cancel/abort when exceeded.
**Atom or composite:** Atom — timer + cancellation.
**Cost model:** O(1) check + cancel overhead.
**Real wall?** Yes — cancellation safety; partial work cleanup.
**Cross-domain wiring:** `distributed-systems/rpc-timeout`.
**Notes:** gRPC deadlines, HTTP timeouts.

### watchdog (cross-domain alias: `watchdog-timer`, `wdt`)
**Domain:** Decision Logic
**Definition:** Hardware/software timer that resets the system if not "petted" within an interval; assumes hang.
**Atom or composite:** Atom.
**Cost model:** O(1) pet.
**Real wall?** Yes — must be petted from a reliable observer; pets-from-dead-task hide failures.
**Cross-domain wiring:** `operating-systems/watchdog`; `formal-verification/liveness`.
**Notes:** Linux watchdog API; embedded systems standard.

### deadman-switch (cross-domain alias: `dead-mans-switch`, `heartbeat-revoke`)
**Domain:** Decision Logic
**Definition:** Action triggered by absence of heartbeat; revoke permissions, alert ops, fail-over.
**Atom or composite:** Composite — heartbeat + action.
**Cost model:** O(1) per heartbeat.
**Real wall?** Yes — false positives during network blips.
**Cross-domain wiring:** `distributed-systems/heartbeat`.
**Notes:** PagerDuty deadman switches.

### retry-with-backoff (cross-domain alias: `exponential-backoff`, `retry-policy`)
**Domain:** Decision Logic
**Definition:** On failure, wait (often exponentially increasing) and retry; cap on attempts/duration.
**Atom or composite:** Composite — attempt + delay + counter.
**Cost model:** O(retries × delay).
**Real wall?** Yes — synchronized retries cause thundering herd.
**Cross-domain wiring:** `distributed-systems/retry`.
**Notes:** AWS SDK default backoff; gRPC retry.

### jitter (cross-domain alias: `random-jitter`, `decorrelated-jitter`)
**Domain:** Decision Logic
**Definition:** Random component added to retry/backoff to decorrelate clients; "full jitter" / "decorrelated jitter".
**Atom or composite:** Atom.
**Cost model:** O(1) RNG.
**Real wall?** Yes — too much jitter masks signal; too little keeps herd.
**Cross-domain wiring:** `statistics-probability/uniform-sample`.
**Notes:** AWS Architecture blog: full vs equal vs decorrelated.

### retry-budget (cross-domain alias: `retry-budget`, `budgeted-retries`)
**Domain:** Decision Logic
**Definition:** Cap retries as a fraction of base request rate to avoid retry storms in failure scenarios.
**Atom or composite:** Composite — token bucket scoped to retries.
**Cost model:** O(1) take + refill.
**Real wall?** Yes — under outage, budget exhaustion delays recovery.
**Cross-domain wiring:** `distributed-systems/rate-limit`.
**Notes:** Envoy retry budget, gRPC retry throttling.

### circuit-breaker (cross-domain alias: `cb`, `circuit-breaker-pattern`)
**Domain:** Decision Logic
**Definition:** Three-state machine (closed/open/half-open) gating calls to a failing dependency.
**Atom or composite:** Composite — state machine + counters.
**Cost model:** O(1) per call check.
**Real wall?** Yes — tuning thresholds vs false-trip is hard.
**Cross-domain wiring:** `distributed-systems/circuit-breaker`.
**Notes:** Hystrix, resilience4j, Polly.

### bulkhead-pattern (cross-domain alias: `bulkhead`, `pool-isolation`)
**Domain:** Decision Logic
**Definition:** Partition resources (threads, connections) by client/dependency so one failure can't drain others.
**Atom or composite:** Composite — per-tenant pool.
**Cost model:** O(per-partition resource).
**Real wall?** Yes — under-allocated bulkheads block fairness.
**Cross-domain wiring:** `distributed-systems/resource-isolation`; `operating-systems/cgroup`.
**Notes:** Hystrix bulkheads; resilience4j.

### causal-chain (cross-domain alias: `causal-chain`, `cause-effect-chain`)
**Domain:** Decision Logic
**Definition:** Ordered sequence of events where each causally precedes the next; basis of audit traces.
**Atom or composite:** Composite.
**Cost model:** O(chain-length) traversal.
**Real wall?** Yes — distinguishing causation from correlation requires interventions.
**Cross-domain wiring:** `causal-inference/causal-chain`; `agentic-reasoning/causal-trace`.
**Notes:** Pearl's causal DAGs.

### happens-before (cross-domain alias: `hb`, `partial-order-of-events`)
**Domain:** Decision Logic
**Definition:** Lamport's relation: e1 → e2 if e1 causally precedes e2 in some process/communication.
**Atom or composite:** Atom — irreducible event relation.
**Cost model:** O(1) compare per pair given timestamps.
**Real wall?** Yes — only partial order; concurrent events unordered.
**Cross-domain wiring:** `distributed-systems/lamport-clock`.
**Notes:** Lamport 1978. Foundation of distributed-systems theory.

### vector-clock-decision (cross-domain alias: `vector-clock`, `vc-decision`)
**Domain:** Decision Logic
**Definition:** Vector clock used to decide ordering / conflict resolution among events.
**Atom or composite:** Composite — vector + compare.
**Cost model:** O(N) per compare, N=replicas.
**Real wall?** Yes — vector grows with cluster; pruning required.
**Cross-domain wiring:** `distributed-systems/vector-clock`.
**Notes:** Riak, Voldemort.

### lamport-clock (cross-domain alias: `lamport-timestamp`, `logical-clock`)
**Domain:** Decision Logic
**Definition:** Scalar logical clock incremented on local events and synced with received messages; consistent with happens-before.
**Atom or composite:** Atom.
**Cost model:** O(1).
**Real wall?** Yes — doesn't capture causality directly; only consistent.
**Cross-domain wiring:** `distributed-systems/lamport-clock`.
**Notes:** Lamport 1978.

### hybrid-logical-clock (cross-domain alias: `hlc`, `hybrid-clock`)
**Domain:** Decision Logic
**Definition:** Combine physical and logical clocks; bounded skew + causal consistency.
**Atom or composite:** Atom.
**Cost model:** O(1).
**Real wall?** Yes — relies on bounded clock skew.
**Cross-domain wiring:** `distributed-systems/hlc`.
**Notes:** CockroachDB, YugabyteDB.

### ltl-always (cross-domain alias: `globally`, `box-operator`)
**Domain:** Decision Logic
**Definition:** LTL `□φ` — φ holds at every future state; the safety/invariant operator.
**Atom or composite:** Atom — single LTL operator.
**Cost model:** Per-step φ-check.
**Real wall?** Yes — only refutable on prefix.
**Cross-domain wiring:** `logic-reasoning/ltl-globally`.
**Notes:** Pnueli 1977.

### ltl-eventually (cross-domain alias: `eventually`, `diamond-operator`)
**Domain:** Decision Logic
**Definition:** LTL `◇φ` — φ holds at some future state; the liveness operator.
**Atom or composite:** Atom.
**Cost model:** Cannot be confirmed on finite prefix.
**Real wall?** Yes — only finitely refutable with deadline.
**Cross-domain wiring:** `logic-reasoning/ltl-eventually`.
**Notes:** Pnueli 1977.

### ltl-until (cross-domain alias: `until`, `u-operator`)
**Domain:** Decision Logic
**Definition:** LTL `φ U ψ` — φ holds until ψ; ψ must eventually hold.
**Atom or composite:** Atom.
**Cost model:** Step automaton.
**Real wall?** Yes — partial verdict on prefix.
**Cross-domain wiring:** `logic-reasoning/ltl-until`.
**Notes:** Standard LTL operator.

### ltl-next (cross-domain alias: `next`, `x-operator`)
**Domain:** Decision Logic
**Definition:** LTL `Xφ` — φ holds at the next state.
**Atom or composite:** Atom.
**Cost model:** O(1) per step.
**Real wall?** Yes — depends on step granularity.
**Cross-domain wiring:** `logic-reasoning/ltl-next`.
**Notes:** Step semantics is the fine print.

### ltl-release (cross-domain alias: `release`, `r-operator`)
**Domain:** Decision Logic
**Definition:** LTL `φ R ψ` — ψ holds until and including the point φ becomes true (or forever).
**Atom or composite:** Atom.
**Cost model:** Step automaton.
**Real wall?** Yes — finite-trace semantics is subtle.
**Cross-domain wiring:** `logic-reasoning/ltl-release`.
**Notes:** Dual of Until.

### ltl-weak-until (cross-domain alias: `weak-until`, `w-operator`)
**Domain:** Decision Logic
**Definition:** LTL `φ W ψ` — φ holds until ψ, but ψ may never hold.
**Atom or composite:** Atom.
**Cost model:** Step automaton.
**Real wall?** Yes — relates Until + Globally.
**Cross-domain wiring:** `logic-reasoning/ltl-weak-until`.
**Notes:** Useful for safety with optional progress.

### state-chart-broadcast (cross-domain alias: `broadcast-event`, `chart-broadcast`)
**Domain:** Decision Logic
**Definition:** Event sent to all states/regions of a statechart simultaneously; foundation of Harel charts.
**Atom or composite:** Atom.
**Cost model:** O(active states).
**Real wall?** Yes — fan-out can be huge; bounded by spec.
**Cross-domain wiring:** `formal-verification/statechart-broadcast`.
**Notes:** Harel statecharts; Esterel signals.

### saga-pattern (cross-domain alias: `saga`, `compensating-transaction`)
**Domain:** Decision Logic
**Definition:** Long-running transaction as sequence of local transactions + compensations on failure.
**Atom or composite:** Composite — sequence + compensations.
**Cost model:** O(steps × local-cost); compensations on failure.
**Real wall?** Yes — semantic compensation is hard; not all actions reversible.
**Cross-domain wiring:** `distributed-systems/saga`; `databases/long-running-transaction`.
**Notes:** Garcia-Molina & Salem 1987. Temporal/Camunda support.
