# Operating Systems — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Execution & Scheduling

### thread-task-struct (cross-domain alias: `kernel-thread`, `lwp`, `light-weight-process`)
**Domain:** Operating Systems
**Definition:** The kernel's schedulable unit, represented in Linux by `struct task_struct` (~10 KB), carrying registers, stack pointer, scheduling class, credentials, and namespace pointers.
**Atom or composite:** Atom — the irreducible dispatch unit; every higher abstraction (process, goroutine, fiber pool) is a composite over threads.
**Cost model:** ~5–10 µs creation via `clone()`, ~1–2 µs context switch with cache cold, 8–16 KB kernel stack each.
**Real wall?** Yes — kernel memory per thread, TLB pressure under thousands of threads.
**Cross-domain wiring:** Maps to **distributed-systems/actor**, **queueing-theory/server**, **ml-training/worker**; the universal "execution token" passed by the scheduler.
**Notes:** Linux ditched 1:1 LinuxThreads for NPTL (Drepper, 2003). Plan 9 used `rfork` precursor; FreeBSD `kse` was M:N (Anderson scheduler activations).

### process-mm-struct (cross-domain alias: `address-space-container`, `pid-bag`)
**Domain:** Operating Systems
**Definition:** A protection-domain wrapper grouping one or more threads sharing a `mm_struct` (page tables), file descriptor table, signal disposition, and credentials.
**Atom or composite:** Composite — process = threads + address space + fd table + creds + cgroup membership.
**Cost model:** `fork()` ~50–200 µs with COW page table copy; `posix_spawn` ~10× cheaper for exec-then-discard patterns.
**Real wall?** Yes — PID namespace is 32-bit (PID_MAX_LIMIT = 4 194 304), kernel struct overhead.
**Cross-domain wiring:** Mirrors **distributed-systems/replica-group** at the host level; the unit of accounting for **cgroup v2** quotas.
**Notes:** Ritchie & Thompson (1969); `fork()` is the only Unix syscall that returns twice. Plan 9 unified everything to `rfork`.

### task-job-group (cross-domain alias: `tgid`, `thread-group`, `posix-process`)
**Domain:** Operating Systems
**Definition:** A POSIX process viewed as a TGID (thread group id) containing N tasks sharing PID-from-the-outside but distinct TIDs internally.
**Atom or composite:** Composite over `task_struct`s sharing `mm_struct` + `files_struct`.
**Cost model:** Signal delivery O(threads), `tgkill` O(1).
**Real wall?** Yes — `signalfd` and `prctl(PR_SET_PDEATHSIG)` interact with TGID semantics.
**Cross-domain wiring:** Aggregation parallel to **distributed-systems/quorum**; signal fan-out to TGID = broadcast in **networking/multicast**.
**Notes:** TGID/TID split arrived with NPTL; `getpid()` returns TGID, `gettid()` returns TID.

### fiber-ucontext (cross-domain alias: `green-thread`, `stackful-coroutine`)
**Domain:** Operating Systems
**Definition:** A cooperatively scheduled user-space execution context with its own stack, swapped via `swapcontext()` / `setjmp`+`longjmp` or assembly trampolines; the kernel sees a single thread.
**Atom or composite:** Atom at the user-runtime layer; invisible to the kernel scheduler.
**Cost model:** ~50–200 ns swap (no syscall, just register save/restore), 64 KB–8 MB stack each (often `mmap`'d with guard page).
**Real wall?** Yes — stack memory and guard-page count cap fiber count.
**Cross-domain wiring:** Foundation of **ml-training/async-pipeline**, **networking/connection-per-fiber** servers (Erlang BEAM, Go runtime conceptually).
**Notes:** Windows `CreateFiber` (1996); Boost.Context, libco, libmill; Go's goroutine is a hybrid (M:N over fibers).

### coroutine-stackless (cross-domain alias: `async-await`, `generator`, `cps-rewrite`)
**Domain:** Operating Systems
**Definition:** A compiler-transformed function whose state is reified into a heap object; resumption is a function call into the state machine.
**Atom or composite:** Composite — produced by CPS / state-machine transform over plain functions.
**Cost model:** O(captured-state) bytes, ~5–20 ns resume; no stack switch.
**Real wall?** No (pure compile-time construct), but memory for the frame is real.
**Cross-domain wiring:** Same trick as **type-theory-programming-languages/monadic-bind**; Rust `Future`, C++20 `co_await`, Python `async def`.
**Notes:** Roots in Conway (1963); Knuth treated coroutines as primitive equal to subroutines.

### priority-nice-prio (cross-domain alias: `urgency`, `weight`, `share`)
**Domain:** Operating Systems
**Definition:** A scalar that biases the scheduler's choice — Linux `nice` ∈ [-20, 19], static priority for RT classes ∈ [1, 99], CFS weight = nice_to_weight[nice+20].
**Atom or composite:** Atom — a single integer per task, but used as composite weight in fair-share trees.
**Cost model:** O(1) per scheduling decision in EEVDF; weight enters `vruntime` divisor.
**Real wall?** Soft — RT priorities are 99, beyond that the kernel won't preempt itself.
**Cross-domain wiring:** Maps to **queueing-theory/priority-discipline**, **networking/dscp**, **ml-training/sample-weight**.
**Notes:** UNIX `nice` is from V6 (1975). Linux exposed `setpriority(2)` and `sched_setattr(2)` (3.14+).

### quantum-time-slice (cross-domain alias: `slice`, `tick`, `sched-period`)
**Domain:** Operating Systems
**Definition:** The maximum continuous CPU time a runnable task may consume before the scheduler reconsiders; CFS computes it as `sched_period / weight_share`.
**Atom or composite:** Atom — derived quantity but treated as a single budget number per dispatch.
**Cost model:** Tick HZ (100/250/1000) bounds resolution; hrtimer-driven dispatch is ~µs precision.
**Real wall?** Yes — timer interrupt rate vs. power consumption trade-off (NO_HZ_FULL).
**Cross-domain wiring:** Analogous to **queueing-theory/processor-sharing-quantum**, **networking/qdisc-quantum** (DRR).
**Notes:** CTSS (1961) introduced time-slicing. Linux moved from O(1) (fixed slices) to CFS (slice = function of load).

### deadline-sched-deadline (cross-domain alias: `edf-deadline`, `relative-deadline`)
**Domain:** Operating Systems
**Definition:** A `(runtime, deadline, period)` tuple driving SCHED_DEADLINE; the task must receive `runtime` ns within each `period` ns, ordered by absolute deadline (EDF + CBS).
**Atom or composite:** Composite — three timing constants forming a sporadic task model.
**Cost model:** Admission test is O(n) over deadline tasks (UB ≤ 1); runtime accounting per tick.
**Real wall?** Yes — admission control rejects new tasks if utilization exceeds CPU bandwidth.
**Cross-domain wiring:** Direct port of **control-numerical-opt/edf** scheduling, **queueing-theory/cbs** constant-bandwidth server.
**Notes:** Liu & Layland (1973). Linux merged SCHED_DEADLINE in 3.14 (Lelli, Faggioli, Trimarchi).

### scheduling-class-fifo-rr-other (cross-domain alias: `sched-policy`, `dispatcher-discipline`)
**Domain:** Operating Systems
**Definition:** A pluggable policy module (`sched_class`) with `enqueue_task`, `dequeue_task`, `pick_next_task`; Linux ships stop, dl, rt, fair (CFS/EEVDF), idle.
**Atom or composite:** Composite — vtable of ~15 callbacks per class; classes chained in priority order.
**Cost model:** `pick_next_task` ~50–200 ns; class lookup is a static chain walk.
**Real wall?** No (software abstraction) but each class adds runqueue memory.
**Cross-domain wiring:** Parallel to **networking/qdisc-pluggable**, **database-streaming-sketching/operator-vtable**.
**Notes:** Solaris pioneered pluggable classes (TS, IA, RT, SYS, FX, FSS); Linux adopted in 2.6.23.

### eevdf-scheduler (cross-domain alias: `earliest-eligible-virtual-deadline-first`, `cfs-replacement`)
**Domain:** Operating Systems
**Definition:** Linux 6.6+ fair-class scheduler replacing CFS; orders by virtual deadline `vd = ve + slice/weight`, picks earliest eligible; supports per-task latency-nice hints.
**Atom or composite:** Composite — virtual-time tree (RB-tree on `vd`) + eligibility predicate (`ve ≤ V`).
**Cost model:** O(log N) insert/pick via RB-tree; constant per-tick virtual-time advance.
**Real wall?** Yes — virtual-time wraparound at 2^63 ns ≈ 292 years (effectively none).
**Cross-domain wiring:** Identical to **queueing-theory/eevdf-fair-queue**, the same algorithm Bennett & Zhang used for network packets (1996).
**Notes:** Bennett-Zhang 1996; Linux port by Peter Zijlstra, merged 6.6 (2023).

### preemption-points (cross-domain alias: `preempt-disable`, `voluntary-yield`)
**Domain:** Operating Systems
**Definition:** Kernel locations where a higher-priority task may displace the current; controlled by `preempt_count`, `cond_resched()`, full PREEMPT_RT inversions.
**Atom or composite:** Atom — single boolean (enabled/disabled) per CPU, but composed into preemption models (none, voluntary, full, RT).
**Cost model:** `preempt_disable` is one atomic inc on the current task; preemption itself costs a context switch (~1 µs).
**Real wall?** Yes — RCU read sections, spinlocks, and IRQ handlers must run preempt-off.
**Cross-domain wiring:** Same idea as **distributed-systems/lease** windows; bounded non-preemptible regions = bounded blocking term.
**Notes:** Linux gained PREEMPT in 2.6.0 (Robert Love); PREEMPT_RT merged piecewise, completed 2024.

### priority-inheritance-pi-mutex (cross-domain alias: `pip`, `priority-boost`, `ceiling-protocol`)
**Domain:** Operating Systems
**Definition:** When a high-priority task blocks on a lock held by a lower-priority task, the holder transiently inherits the waiter's priority; bounds blocking to a single critical section.
**Atom or composite:** Composite — protocol over (mutex, blocked-tasks, holder).
**Cost model:** O(chain-depth) for transitive boosts; rt_mutex chain walk.
**Real wall?** Yes — unbounded chains can still occur without ceiling protocols; PREEMPT_RT uses rtmutex everywhere.
**Cross-domain wiring:** Maps to **distributed-systems/priority-escalation** in distributed locking; **control-numerical-opt/wcet-analysis** uses PIP bounds.
**Notes:** Sha, Rajkumar, Lehoczky (1990); cause of the famous Mars Pathfinder 1997 reset.

### round-robin-rr (cross-domain alias: `circular-fifo`, `sched-rr`)
**Domain:** Operating Systems
**Definition:** Equal-quantum cyclic dispatch among same-priority runnables; `SCHED_RR` time-slice default 100 ms (Linux), tunable via `/proc/sys/kernel/sched_rr_timeslice_ms`.
**Atom or composite:** Atom — simplest fair-share discipline.
**Cost model:** O(1) per dispatch; timer interrupt costs the quantum boundary.
**Real wall?** No, but uniform quantum is starvation-free only within a priority level.
**Cross-domain wiring:** **queueing-theory/round-robin**, **networking/wrr** weighted round-robin packet scheduler.
**Notes:** Used in Multics, every Unix variant. Linux RT class only.

### sporadic-server-cbs (cross-domain alias: `constant-bandwidth-server`, `aperiodic-handler`)
**Domain:** Operating Systems
**Definition:** A server task that admits aperiodic jobs into an EDF schedule with bounded bandwidth `Q/T`; SCHED_DEADLINE implements CBS to throttle overruns.
**Atom or composite:** Composite — (budget Q, period T, deadline) triple wrapping aperiodic arrivals.
**Cost model:** O(1) budget accounting per tick; admission control O(n).
**Real wall?** Yes — total bandwidth must satisfy `Σ Q_i/T_i ≤ 1` per CPU.
**Cross-domain wiring:** **queueing-theory/token-bucket**, **networking/policer-tbf**.
**Notes:** Abeni & Buttazzo (1998); Sprunt-Sha-Lehoczky sporadic server (1989).

### work-stealing-deque (cross-domain alias: `cilk-deque`, `wsq`, `lifo-bottom-fifo-top`)
**Domain:** Operating Systems
**Definition:** Per-worker double-ended queue; owner pushes/pops at the bottom (LIFO, cache-warm), thieves steal from the top (FIFO, oldest); Chase-Lev or THE protocol.
**Atom or composite:** Composite — circular array + bottom/top atomics + ABA-safe reads.
**Cost model:** O(1) local push/pop, O(1) amortized steal; Chase-Lev uses a single CAS per steal.
**Real wall?** Yes — false sharing of top/bottom forces cache-line padding.
**Cross-domain wiring:** Backbone of **ml-training/data-parallel-runtime**, Go scheduler, Rust Rayon, Java ForkJoinPool.
**Notes:** Blumofe-Leiserson Cilk (1995); Chase-Lev (2005); Lê-Pop-Cohen-Nardelli (2013) for ARMv8 memory model.

### work-donating (cross-domain alias: `helping-mechanism`, `obstruction-free-aid`)
**Domain:** Operating Systems
**Definition:** Dual of work stealing — when a worker enters a critical section, idle workers donate cycles by finishing the slow operation on its behalf (e.g., lock-free helping in concurrent data structures).
**Atom or composite:** Composite — over (descriptor, helper-loop, completion-CAS).
**Cost model:** Worst-case O(threads) helpers per operation, amortized O(1).
**Real wall?** Yes — descriptor allocation dominates.
**Cross-domain wiring:** Lock-free **distributed-systems/cooperative-progress**, RCU's grace-period helpers.
**Notes:** Herlihy-Shavit (2008); foundation of wait-free transformations.

### futex-fast-userspace-mutex (cross-domain alias: `futex2`, `userspace-park`)
**Domain:** Operating Systems
**Definition:** Linux syscall `futex(2)` that parks a thread on a userspace 32-bit word until another thread wakes it; uncontended path stays in userspace with a single atomic.
**Atom or composite:** Atom — single hashed wait queue, single op (FUTEX_WAIT/WAKE/CMP_REQUEUE/PI/WAKE_OP).
**Cost model:** Uncontended lock = ~10 ns (1 CAS); contended = ~1–2 µs (syscall + wakeup).
**Real wall?** Yes — kernel hash table size, futex hash collisions; futex2 (5.16+) adds per-address queues.
**Cross-domain wiring:** Building block under **queueing-theory/blocking-queue**, glibc `pthread_mutex_t`, Rust `parking_lot`.
**Notes:** Franke-Russell-Kirkwood (2002); FUTEX_PI added by Molnar/Rostedt.

### runqueue-rb-tree (cross-domain alias: `rq`, `cfs-rq`, `eevdf-tree`)
**Domain:** Operating Systems
**Definition:** Per-CPU red-black tree ordering runnable tasks by `vruntime` (CFS) or virtual deadline (EEVDF); leftmost = next-to-run.
**Atom or composite:** Composite — RB-tree + leftmost cache + nr_running counter + load-weight sum.
**Cost model:** O(log N) insert/extract; leftmost cached for O(1) `pick_next_task`.
**Real wall?** Yes — RB-tree rebalancing dominates on high-fanout fork bombs.
**Cross-domain wiring:** Same structure as **database-streaming-sketching/order-by-index**, **queueing-theory/sorted-runqueue**.
**Notes:** Ingo Molnar's CFS (2.6.23, 2007). EEVDF reuses the tree but reorders by `vd`.

### cpu-affinity-pinning (cross-domain alias: `taskset`, `sched-setaffinity`, `pinning-mask`)
**Domain:** Operating Systems
**Definition:** Bitmask restricting a task to a subset of CPUs; `sched_setaffinity(2)` and `taskset(1)`; respected by every scheduler class.
**Atom or composite:** Atom — bitmask per task.
**Cost model:** Zero runtime cost once set; enforced at every dispatch decision.
**Real wall?** Yes — pinned tasks may cause load imbalance and idle siblings.
**Cross-domain wiring:** Direct knob for **ml-training/numa-aware-placement**, **networking/rps-rfs** packet steering.
**Notes:** Linux 2.5.8 added `sched_setaffinity`; `cset` and `tuned-profiles` automate it.

### numa-balancing-autonuma (cross-domain alias: `automatic-page-migration`, `numa-hinting`)
**Domain:** Operating Systems
**Definition:** Kernel feature that periodically marks pages NUMA-protected (no-access) so faults reveal access locality, then migrates pages or tasks closer to the accessor.
**Atom or composite:** Composite — sampling timer + protnone faults + migration kthread.
**Cost model:** ~1–5 % overhead from extra page faults; migration is ~µs per 4 KB page.
**Real wall?** Yes — bandwidth between NUMA nodes (UPI/Infinity Fabric) is finite.
**Cross-domain wiring:** Same problem as **distributed-systems/data-locality**, **ml-training/gpu-placement**.
**Notes:** Mel Gorman, Linux 3.8 (2013); inspired by AutoNUMA (Andrea Arcangeli) and `sched/numa` (Peter Zijlstra).

### energy-aware-scheduling-eas (cross-domain alias: `eas-cpu-capacity`, `power-aware-dispatch`)
**Domain:** Operating Systems
**Definition:** Fair-class augmentation that consults a per-CPU energy model and selects the CPU minimizing `Δenergy` per unit utilization; targets big.LITTLE.
**Atom or composite:** Composite — utilization PELT signal + energy model table + frequency domains.
**Cost model:** O(perf-domains) per wakeup; bounded ≤ 16 domains in practice.
**Real wall?** Yes — energy model accuracy and capacity asymmetry detection.
**Cross-domain wiring:** **control-numerical-opt/lyapunov-frequency-control**, **signal-processing-rf/power-curve**.
**Notes:** ARM contributed EAS; merged Linux 5.0 (2018). Requires `CONFIG_ENERGY_MODEL`.

### kthread-workqueue (cross-domain alias: `kworker`, `bound-unbound-wq`, `cmwq`)
**Domain:** Operating Systems
**Definition:** Per-CPU or unbound pool of kernel threads executing deferred `struct work_struct` items; replaces 2.x keventd.
**Atom or composite:** Composite — pool of kthreads + work list + flush/drain primitives.
**Cost model:** Enqueue is O(1); idle kworkers consume zero CPU (parked).
**Real wall?** Yes — total kworker count is bounded; long-running work can stall others.
**Cross-domain wiring:** Same pattern as **distributed-systems/task-queue**, Celery, Sidekiq.
**Notes:** Concurrency Managed Workqueue (cmwq) by Tejun Heo, 2.6.36 (2010).

### softirq-bottom-half (cross-domain alias: `softirq`, `tasklet`, `bh`)
**Domain:** Operating Systems
**Definition:** Deferred interrupt processing context — softirqs (NET_RX, NET_TX, TIMER, BLOCK, RCU, SCHED, HRTIMER, TASKLET) run with interrupts on, preemption off, on the CPU that raised them.
**Atom or composite:** Atom — 10 hardcoded softirq vectors; tasklets are a composite atop TASKLET softirq.
**Cost model:** `__do_softirq()` loops up to 10 passes / 2 ms before deferring to ksoftirqd.
**Real wall?** Yes — softirq starvation under high packet rate is the classic NAPI motivation.
**Cross-domain wiring:** Same deferred-work pattern as **networking/napi-poll**, **distributed-systems/batched-callback**.
**Notes:** Replaced 2.2 bottom halves; Rusty Russell, Andrea Arcangeli, Ingo Molnar (1999–2003).

### rt-throttling-bandwidth (cross-domain alias: `sched-rt-runtime`, `rt-bandwidth-cap`)
**Domain:** Operating Systems
**Definition:** Kernel-enforced limit (default 950 ms per 1000 ms) on aggregate RT-class CPU time, preventing a runaway SCHED_FIFO task from starving the kernel.
**Atom or composite:** Atom — pair `(rt_runtime_us, rt_period_us)` per cgroup or global.
**Cost model:** O(1) accounting per tick.
**Real wall?** Yes — disabling throttling (`rt_runtime_us = -1`) is the only way to get hard real-time, at the cost of safety.
**Cross-domain wiring:** **queueing-theory/leaky-bucket** applied to CPU class.
**Notes:** Linux 2.6.25; rejected by hard-RT camps, kept by general-purpose distros.

### gang-coscheduling (cross-domain alias: `gang-scheduling`, `coordinated-dispatch`)
**Domain:** Operating Systems
**Definition:** Scheduler discipline that runs a group of communicating tasks simultaneously across N CPUs, eliminating spin-then-block latency in synchronizing workloads.
**Atom or composite:** Composite — group descriptor + N-way barrier at dispatch time.
**Cost model:** O(N) dispatch fan-out; idle CPUs while waiting for last group member.
**Real wall?** Yes — fragmentation of idle CPUs ("holes") under bursty groups.
**Cross-domain wiring:** **ml-training/all-reduce-coordination**, HPC MPI runs.
**Notes:** Ousterhout (1982). Linux experimental `core scheduling` (5.14) is a security-driven variant.

### lottery-scheduling (cross-domain alias: `proportional-share-tickets`, `stride`)
**Domain:** Operating Systems
**Definition:** Each task holds N "tickets"; dispatcher draws a uniform-random ticket per quantum, giving expected share N_i/ΣN.
**Atom or composite:** Composite — ticket counts + PRNG + winner-lookup tree.
**Cost model:** O(log N_tasks) per draw with a Fenwick tree; O(1) for stride deterministic variant.
**Real wall?** Stochastic guarantees only — variance ~1/√samples.
**Cross-domain wiring:** **statistics-probability/weighted-sampling**, **networking/probabilistic-wrr**.
**Notes:** Waldspurger & Weihl (1994). Stride scheduling (1995) made it deterministic.

### sparta-bpf-ghost-scheduler (cross-domain alias: `ghost-google`, `userspace-scheduler-bpf`)
**Domain:** Operating Systems
**Definition:** Framework allowing scheduling policy to live in userspace (or BPF) while the kernel exposes per-task callbacks; ghOSt (Google, 2021) and `sched_ext` (Linux 6.12) productize the idea.
**Atom or composite:** Composite — kernel hooks + BPF dispatch program + queue maps.
**Cost model:** ~50–200 ns per hook call; BPF JIT keeps it near native.
**Real wall?** Yes — userspace scheduler must complete dispatch within preemption budget or kernel falls back.
**Cross-domain wiring:** Same idea as **networking/xdp-bpf-dispatch**, lets ML and DB workloads ship custom policies.
**Notes:** ghOSt (Humphries et al., SOSP 2021); `sched_ext` by Tejun Heo, Linux 6.12 (2024).

---

## Process Management

### pid-tid-pgid-sid (cross-domain alias: `process-id-quad`, `session-tuple`)
**Domain:** Operating Systems
**Definition:** Four 32-bit namespaced identifiers — PID (thread group), TID (thread), PGID (process group, signal target), SID (session, terminal scope).
**Atom or composite:** Composite — four fields in `task_struct`, each with its own namespace mapping.
**Cost model:** O(1) lookup via per-namespace IDR (radix tree).
**Real wall?** Yes — PID exhaustion (PID_MAX_LIMIT = 2^22) bounds active tasks.
**Cross-domain wiring:** Same naming concept as **distributed-systems/node-id** with hierarchical scoping.
**Notes:** SVR4 added sessions; namespaces (Bivens, Pavlovicz, 2006–2013) made all four per-container.

### fork-clone-vfork (cross-domain alias: `process-creation-syscalls`, `clone3`)
**Domain:** Operating Systems
**Definition:** Family of syscalls creating a new task; `fork()` duplicates everything COW, `vfork()` shares VM until exec, `clone()`/`clone3()` selects per-resource sharing via `CLONE_*` flags.
**Atom or composite:** Composite — single syscall with bitmask of resource-sharing flags.
**Cost model:** `fork` ~50–200 µs; `clone3(CLONE_VM|CLONE_FS|...)` for threads ~5–10 µs.
**Real wall?** Yes — page table COW setup cost dominates large processes.
**Cross-domain wiring:** Source of **distributed-systems/process-replication** semantics; Plan 9 `rfork` is the canonical fine-grained ancestor.
**Notes:** `clone3` (5.3) added for io_uring and PID file descriptors (`pidfd`).

### execve-binfmt-loader (cross-domain alias: `program-load`, `binfmt-elf`, `binfmt-misc`)
**Domain:** Operating Systems
**Definition:** Syscall replacing the calling process's address space with a new program image; binfmt dispatchers (ELF, script, misc/wasm) decide the loader.
**Atom or composite:** Composite — VFS lookup + binfmt match + ELF segment mmap + interpreter (ld.so).
**Cost model:** ~1–10 ms for dynamic ELF (interpreter resolution + relocations); `posix_spawn` short-circuits fork+exec.
**Real wall?** Yes — ASLR randomization, COW page-fault-in dominates startup.
**Cross-domain wiring:** Linker/loader interplay touches **type-theory-programming-languages/dynamic-linking**.
**Notes:** `binfmt_misc` lets you register `.jar`, `.wasm`, `.py` shebangs system-wide.

### posix-spawn (cross-domain alias: `vfork-exec-fast-path`, `psx_spawn`)
**Domain:** Operating Systems
**Definition:** POSIX call combining fork+exec into one operation with `posix_spawn_file_actions_t` and `posix_spawnattr_t` describing fd manipulation and signal masks.
**Atom or composite:** Composite — fork+exec compressed, avoids page-table copy entirely on Linux (uses `CLONE_VM|CLONE_VFORK`).
**Cost model:** ~10× faster than fork+exec for large processes; saves COW page-table copy.
**Real wall?** Yes — limited to operations expressible in `file_actions`; complex setup still needs fork.
**Cross-domain wiring:** Same fast-spawn idea as **distributed-systems/process-pool-warm-start**.
**Notes:** glibc 2.24+ implements via vfork+exec; macOS/FreeBSD have it as primary mechanism.

### wait-waitpid-waitid-pidfd (cross-domain alias: `process-reap`, `child-status`)
**Domain:** Operating Systems
**Definition:** Family of syscalls that block until a child changes state, returning exit status; `waitid(P_PIDFD, ...)` and `pidfd_send_signal` modernize the API to be race-free.
**Atom or composite:** Atom — single syscall family over (selector, status, options).
**Cost model:** O(1) blocking wait; PID racy without pidfd.
**Real wall?** Yes — without pidfd, PID reuse races allow signaling the wrong process.
**Cross-domain wiring:** **distributed-systems/leader-detection** at host scale; `pidfd` is the host's `epoll`-able process handle.
**Notes:** `pidfd` Christian Brauner, 5.2 (2019); fixes a 50-year PID-race CVE class.

### zombie-orphan-init-reaper (cross-domain alias: `defunct-process`, `subreaper`)
**Domain:** Operating Systems
**Definition:** Zombie = exited child with unreaped status; orphan = child whose parent died and is reparented to `init` (or nearest `PR_SET_CHILD_SUBREAPER`); reaper collects.
**Atom or composite:** Composite — task in `EXIT_ZOMBIE` state + parent pointer + subreaper chain.
**Cost model:** O(1) per child reap; unreaped zombies consume one task_struct slot.
**Real wall?** Yes — runaway zombie creation can hit PID limit.
**Cross-domain wiring:** Same orphan-adoption pattern as **distributed-systems/leader-takeover**.
**Notes:** `PR_SET_CHILD_SUBREAPER` (3.4) lets containers like systemd-nspawn act as init within a tree.

### daemon-double-fork (cross-domain alias: `setsid-fork`, `background-process`)
**Domain:** Operating Systems
**Definition:** Idiom: `fork` → `setsid` → `fork` again to detach from controlling terminal and prevent reacquisition; the grandchild becomes a session-less daemon.
**Atom or composite:** Composite — sequence of syscalls; systemd's `Type=forking` parses this.
**Cost model:** Two forks ≈ 100–400 µs; one-time.
**Real wall?** Yes — semantic, not physical; modern services use `Type=simple` and let systemd manage.
**Cross-domain wiring:** Same daemon pattern as **distributed-systems/long-lived-worker**.
**Notes:** Stevens APUE chapter 13; obsolete under systemd / launchd / Windows services.

### copy-on-write-cow-fork (cross-domain alias: `cow-page-share`, `lazy-duplication`)
**Domain:** Operating Systems
**Definition:** On fork, parent and child share all pages read-only with `_PAGE_RW` cleared; first write triggers a fault that allocates a fresh page and breaks sharing.
**Atom or composite:** Atom — per-page semantic, but composes into entire address-space duplication.
**Cost model:** Fork is O(page-table-entries), ~µs per GB; first-write fault is ~1–5 µs each.
**Real wall?** Yes — large RSS + heavy writes after fork can quadruple memory transiently.
**Cross-domain wiring:** Same algorithm as **database-streaming-sketching/cow-tree**, **graphics-rendering-lod/cow-mesh**.
**Notes:** SunOS 4 (1988); essential for `fork+exec` and Redis BGSAVE.

### exit-code-coredump-signal-disposition (cross-domain alias: `wstatus`, `signal-on-death`)
**Domain:** Operating Systems
**Definition:** Encoded 16-bit status combining `exit(value)`, killing signal, core-dump bit; decoded via `WIFEXITED`, `WEXITSTATUS`, `WTERMSIG`, `WCOREDUMP`.
**Atom or composite:** Atom — single packed int.
**Cost model:** O(1) generation; core dump itself can be GB of disk I/O.
**Real wall?** Yes — `/proc/sys/kernel/core_pattern` can pipe dumps to a helper (systemd-coredump, abrt).
**Cross-domain wiring:** Same death-record idea as **distributed-systems/tombstone**.
**Notes:** POSIX-mandated encoding; `core_pattern %P` (PID) helps disambiguate concurrent crashes.

### signalfd-eventfd-timerfd (cross-domain alias: `fd-ification`, `poll-friendly-events`)
**Domain:** Operating Systems
**Definition:** Trio of syscalls that expose signals, counters, and timers as readable file descriptors — eliminating async-signal-safety constraints by integrating with `epoll`/`io_uring`.
**Atom or composite:** Composite — three siblings sharing the "make X an fd" pattern.
**Cost model:** ~µs read latency; fd integration cost = epoll add (~50 ns amortized).
**Real wall?** Yes — `signalfd` only sees signals that would otherwise be delivered; defaults stay disposition-aware.
**Cross-domain wiring:** Foundation of **networking/event-loop-uniform-dispatch**, libevent, libuv.
**Notes:** Davide Libenzi (2.6.22, 2007); preserves Davie Stevens' "everything is a fd" doctrine.

### prctl-process-control (cross-domain alias: `prctl`, `pr-set-name`, `pr-set-dumpable`)
**Domain:** Operating Systems
**Definition:** Catch-all syscall (`prctl(2)`) setting per-task attributes: thread name, dumpable, no_new_privs, seccomp mode, MDWE, capability bound, child subreaper, tagged address ABI.
**Atom or composite:** Composite — single syscall multiplexing ~70 options.
**Cost model:** O(1) per option.
**Real wall?** Yes — irreversible toggles (`NO_NEW_PRIVS`, `SET_DUMPABLE 0`) are one-way switches.
**Cross-domain wiring:** Per-task feature gates akin to **distributed-systems/per-node-flags**.
**Notes:** SVR4 origin; Linux added seccomp (3.5), MDWE (memory-deny-write-execute, 6.3).

---

## Virtual Memory & MMU

### page-frame (cross-domain alias: `pfn`, `physical-page`, `struct-page`)
**Domain:** Operating Systems
**Definition:** Fixed-size (4 KB on x86, 16 KB Apple Silicon, 64 KB POWER) physical-memory unit, tracked by `struct page` (~64 B) in `mem_map[]`.
**Atom or composite:** Atom — the universal MMU allocation quantum.
**Cost model:** 64 B per 4 KB → 1.5 % memory overhead just for `struct page` array.
**Real wall?** Yes — page size is hardware-fixed per ISA; TLB coverage = entries × page size.
**Cross-domain wiring:** Same fixed-quantum model as **networking/mtu-frame**, **database-streaming-sketching/disk-page**.
**Notes:** Atlas (1962) introduced paging. `struct page` shrinking is a perpetual Linux memory-saving project (folios, 5.16+).

### virtual-address-space (cross-domain alias: `mm-struct`, `vas`, `process-vm`)
**Domain:** Operating Systems
**Definition:** Per-process linear name space mapped by a page table; 48 or 57 bits (LA57) on x86_64, 48 on ARMv8.4-TTL.
**Atom or composite:** Composite — set of `vm_area_struct`s + page table tree + ASID.
**Cost model:** O(log N_VMAs) lookup via maple tree (replaced rbtree+linked-list in 6.1).
**Real wall?** Yes — bits of VA; canonical-form requirement forces high-half kernel split.
**Cross-domain wiring:** Same naming abstraction as **distributed-systems/key-range**.
**Notes:** Liam Howlett's maple tree (Linux 6.1, 2022) ended the dual rbtree/linked-list era.

### page-table-radix-tree (cross-domain alias: `pgd-p4d-pud-pmd-pte`, `multi-level-page-table`)
**Domain:** Operating Systems
**Definition:** Hierarchical hardware-walked structure mapping VA→PA; x86_64 uses 4-level (PML4) or 5-level (LA57); each level indexes 9 bits.
**Atom or composite:** Composite — tree of physical pages each containing 512 64-bit entries.
**Cost model:** TLB miss = 4 (or 5) memory loads, ~50–100 ns each cold; nested virt doubles it.
**Real wall?** Yes — hardware-defined format; software cannot change PTE bits arbitrarily.
**Cross-domain wiring:** Same indexing as **retrieval-search/radix-trie**.
**Notes:** Intel SDM Vol 3A §4; LA57 adds bit 56 for 57-bit VA (Linux 4.14+).

### vma-vm-area-struct (cross-domain alias: `mmap-region`, `memory-region`)
**Domain:** Operating Systems
**Definition:** Description of one contiguous virtual range with uniform `prot`/`flags`/backing file; linked via maple tree per `mm_struct`.
**Atom or composite:** Composite — `(vm_start, vm_end, vm_flags, vm_file, vm_ops)`.
**Cost model:** ~200 B per VMA; processes commonly have 100s.
**Real wall?** Yes — `/proc/sys/vm/max_map_count` default 65530 caps VMA count (raised for JVMs, Elasticsearch).
**Cross-domain wiring:** Mirrors **database-streaming-sketching/extent-list**.
**Notes:** Maple-tree replacement reduces lock contention vs old `mmap_sem`/`mmap_lock`.

### tlb-translation-lookaside-buffer (cross-domain alias: `address-translation-cache`, `i-tlb-d-tlb`)
**Domain:** Operating Systems
**Definition:** Hardware cache (typically 64–2048 entries, split L1 i-TLB/d-TLB plus unified L2) of VA→PA translations to avoid walking the page table.
**Atom or composite:** Atom — fixed-size CAM lookup per memory access.
**Cost model:** Hit = ~1 cycle; miss = full page walk (~20 cycles best, hundreds with nested).
**Real wall?** Yes — TLB size × page size = "TLB reach"; the dominant ceiling on large-working-set workloads.
**Cross-domain wiring:** Generic translation cache parallel to **networking/arp-cache**, **retrieval-search/lookup-cache**.
**Notes:** GE 645 / Multics (1969) introduced "associative memory"; INVPCID (Haswell) lets you flush per-ASID.

### tlb-shootdown-ipi (cross-domain alias: `cross-cpu-flush`, `tlb-invalidate-ipi`)
**Domain:** Operating Systems
**Definition:** Inter-processor interrupt sent to all CPUs sharing an address space when PTEs change, forcing each to flush its TLB; ~µs per shootdown.
**Atom or composite:** Composite — IPI + per-CPU flush handler + completion barrier.
**Cost model:** O(N_cpus) IPI + ~1 µs ack; modern x86 amortizes via `INVLPGB` (Zen 5, future).
**Real wall?** Yes — fundamental cost of shared address spaces; bottleneck for `munmap`-heavy servers.
**Cross-domain wiring:** Same coherence-broadcast pattern as **distributed-systems/cache-invalidation**.
**Notes:** Black 1989; mitigated by ASID/PCID and lazy flushing.

### asid-pcid-tag (cross-domain alias: `address-space-id`, `tagged-tlb`)
**Domain:** Operating Systems
**Definition:** Per-address-space tag in each TLB entry, allowing context switches without full flush; x86 PCID = 12 bits, ARM ASID = 8/16 bits.
**Atom or composite:** Atom — single tag value per `mm_struct` per CPU.
**Cost model:** Zero on hit; PCID rollover triggers flush.
**Real wall?** Yes — limited bits force rollover and forced flush.
**Cross-domain wiring:** Same tag-multiplexing idea as **networking/vlan-tag**.
**Notes:** Linux uses PCID since 4.14 (KPTI motivation); each `mm_struct` gets a 12-bit slot.

### iommu-smmu (cross-domain alias: `dma-translation`, `vt-d`, `amd-vi`)
**Domain:** Operating Systems
**Definition:** Bus-side MMU translating device-issued DMA addresses into host physical addresses, enabling isolation, SR-IOV, and userspace drivers (VFIO).
**Atom or composite:** Composite — root table + context table + IO page table + IOTLB.
**Cost model:** Setup ~ms per device; per-DMA translation hits IOTLB (~ns).
**Real wall?** Yes — IOMMU page-table walks; pinned-page memory for DMA mappings.
**Cross-domain wiring:** Same isolation idea as **cryptography-hashing/sandboxed-co-processor**; required for **distributed-systems/rdma-multi-tenant**.
**Notes:** Intel VT-d, AMD-Vi, ARM SMMU; VFIO exposes to userspace (DPDK, SPDK).

### page-fault-major-minor (cross-domain alias: `fault-handler`, `do-page-fault`)
**Domain:** Operating Systems
**Definition:** CPU exception when a PTE is missing/invalid; kernel resolves by allocating, COW-breaking, swap-in, or signaling SIGSEGV.
**Atom or composite:** Atom — single trap vector (#PF on x86, data abort on ARM).
**Cost model:** Minor (anon zero-fill, COW) ~1 µs; major (swap-in, file read) ms.
**Real wall?** Yes — major-fault count dominates RT jitter; mlock pins pages to eliminate.
**Cross-domain wiring:** Trap-and-emulate pattern shared with **graphics-rendering-lod/demand-load-textures**.
**Notes:** Demand paging traces to Atlas (1962); userfaultfd (4.3) lets userspace handle faults.

### copy-on-write-page-cow (cross-domain alias: `cow-fault`, `wp-fault`)
**Domain:** Operating Systems
**Definition:** Page-level COW: shared read-only pages between processes; write triggers `do_wp_page` to allocate, copy, and remap private.
**Atom or composite:** Atom — per-page event during fork or mmap MAP_PRIVATE.
**Cost model:** ~5 µs per broken page (alloc + memcpy + PTE update + TLB flush).
**Real wall?** Yes — write-amplification after fork; transparent-huge-page COW is 2 MB at a time.
**Cross-domain wiring:** Same CoW pattern as **database-streaming-sketching/cow-btree**.
**Notes:** Foundation for SunOS vfork-free fork; ZFS and btrfs lift CoW to filesystem.

### swap-anon-lru-kswapd (cross-domain alias: `page-reclaim`, `mglru`, `swappiness`)
**Domain:** Operating Systems
**Definition:** Demand-driven eviction of anon pages to swap and file pages back to disk; managed by `kswapd` per NUMA node, scanning LRU lists.
**Atom or composite:** Composite — anon-LRU + file-LRU + kswapd kthread + reclaim policy.
**Cost model:** Page-out is ms (disk I/O); MGLRU (5.18) generationally batches scans.
**Real wall?** Yes — swap disk bandwidth; thrashing risk under memory pressure.
**Cross-domain wiring:** Same eviction problem as **retrieval-search/lru-cache**, **database-streaming-sketching/buffer-pool**.
**Notes:** MGLRU by Yu Zhao (Google, 5.18, 2022) is the first major LRU rewrite in 20 years.

### oom-killer-score (cross-domain alias: `out-of-memory-handler`, `oom-score-adj`)
**Domain:** Operating Systems
**Definition:** Last-resort kernel handler that selects a victim task by `oom_score = rss + oom_score_adj` and sends SIGKILL when reclaim fails.
**Atom or composite:** Composite — scan all tasks, compute score, kill max.
**Cost model:** O(tasks) per invocation; once per OOM event.
**Real wall?** Yes — kernel must not deadlock under OOM, hence forced kill.
**Cross-domain wiring:** **distributed-systems/back-pressure-shedding** at the host level.
**Notes:** `oom_score_adj` ∈ [-1000, 1000]; -1000 = immune. systemd-oomd uses PSI to act earlier.

### mlock-mlockall-pin (cross-domain alias: `wired-memory`, `lock-pages`)
**Domain:** Operating Systems
**Definition:** Syscalls preventing pages from being paged out; required for RT and security-sensitive (key material) workloads.
**Atom or composite:** Atom — set/clear of `VM_LOCKED` on VMA.
**Cost model:** O(pages) at mlock time (fault them in); zero ongoing.
**Real wall?** Yes — `RLIMIT_MEMLOCK` and global `vm.max_user_locked_kbytes` caps.
**Cross-domain wiring:** Same residency-guarantee pattern as **ml-training/pinned-host-memory** for GPU DMA.
**Notes:** POSIX `mlock(2)`; `mlock2(MLOCK_ONFAULT)` (4.4) defers fault but locks once present.

### mmap-mremap-mprotect-madvise (cross-domain alias: `vm-syscall-quad`, `address-space-edits`)
**Domain:** Operating Systems
**Definition:** Four syscalls editing the VMA tree — `mmap` creates, `mremap` resizes/relocates, `mprotect` changes flags, `madvise` hints behavior.
**Atom or composite:** Composite — each operates on (addr, len) ranges, may split/merge VMAs.
**Cost model:** ~µs–ms depending on length and TLB-flush scope.
**Real wall?** Yes — VMA-count limit + `mmap_lock` contention historically; per-VMA-lock (6.4) helps.
**Cross-domain wiring:** Same "edit a range" abstraction as **database-streaming-sketching/interval-update**.
**Notes:** `madvise(MADV_HUGEPAGE/COLD/PAGEOUT/COLLAPSE)` extends hints continuously; `mremap` is Linux-unique among Unixes.

### userfaultfd-handler (cross-domain alias: `uffd`, `userspace-page-fault`)
**Domain:** Operating Systems
**Definition:** File descriptor that delivers page-fault events to userspace, letting a handler resolve via `UFFDIO_COPY`/`UFFDIO_ZEROPAGE`/`UFFDIO_CONTINUE`.
**Atom or composite:** Composite — fd + event queue + ioctl resolution.
**Cost model:** ~10–50 µs per fault (extra syscall + wakeup).
**Real wall?** Yes — privilege gate (`vm.unprivileged_userfaultfd=0` by default since 5.11).
**Cross-domain wiring:** Foundation of **distributed-systems/post-copy-vm-migration** (QEMU live migration), CRIU.
**Notes:** Andrea Arcangeli (4.3, 2015); used in QEMU/KVM post-copy, distributed shared memory.

### hugepage-2m-1g-thp (cross-domain alias: `hugetlb`, `transparent-huge-page`)
**Domain:** Operating Systems
**Definition:** Page-size selections larger than 4 KB — 2 MB and 1 GB on x86, 32 MB/2 MB on ARM; reduces TLB miss rate proportionally to size ratio.
**Atom or composite:** Atom at hardware level (single PTE points to a large frame); composite at THP level (khugepaged collapses anon ranges).
**Cost model:** 1 GB pages must be reserved at boot; 2 MB THP allocation can stall under fragmentation.
**Real wall?** Yes — physical contiguity; compaction work scales superlinearly.
**Cross-domain wiring:** Same coarsening idea as **database-streaming-sketching/page-size-tuning**, **ml-training/large-pages-for-tensors**.
**Notes:** `hugetlbfs` for explicit, `transparent_hugepage=madvise` for opportunistic; THP merged 2.6.38 (Andrea Arcangeli).

### khugepaged-collapse (cross-domain alias: `huge-page-promotion`, `thp-collapse`)
**Domain:** Operating Systems
**Definition:** Background kthread that periodically scans anon VMAs and promotes contiguous 4 KB runs into 2 MB huge pages via in-place copy.
**Atom or composite:** Composite — kthread + scan cursor + collapse work.
**Cost model:** Scans `khugepaged_pages_to_scan` per pass, sleeps `khugepaged_scan_sleep_millisecs`.
**Real wall?** Yes — collapse stalls allocating thread briefly; defrag tied to compaction.
**Cross-domain wiring:** Same online-defrag pattern as **database-streaming-sketching/lsm-compaction**.
**Notes:** Tunables in `/sys/kernel/mm/transparent_hugepage`; `MADV_COLLAPSE` (6.1) lets userspace request synchronous collapse.

### kswapd-direct-reclaim (cross-domain alias: `background-reclaim`, `pf-reclaim`)
**Domain:** Operating Systems
**Definition:** Per-NUMA-node kthread that wakes when watermarks (`min`, `low`, `high`) trigger; direct reclaim is when an allocating thread reclaims synchronously.
**Atom or composite:** Composite — async kthread + sync fallback.
**Cost model:** Direct reclaim adds latency to alloc, multi-ms.
**Real wall?** Yes — reclaim must run fast enough to keep up with allocation rate, else OOM.
**Cross-domain wiring:** **queueing-theory/back-pressure-loop** with watermarks as set-points.
**Notes:** Watermarks tunable via `min_free_kbytes`; `vm.watermark_scale_factor` (4.6+) finer control.

### page-cache-dirty-writeback (cross-domain alias: `pdflush`, `bdi-writeback`, `dirty-ratio`)
**Domain:** Operating Systems
**Definition:** Address-space cache of file data; dirty pages flushed by per-BDI flusher threads under deadline (`dirty_expire_centisecs`) or pressure (`dirty_ratio`).
**Atom or composite:** Composite — radix/xarray of pages + dirty list + writeback work.
**Cost model:** Writeback bandwidth ≈ device throughput; `fsync` forces sync flush.
**Real wall?** Yes — `dirty_bytes` accumulation pauses writers under "stall-on-dirty".
**Cross-domain wiring:** Same buffer-and-flush pattern as **database-streaming-sketching/wal-buffer**.
**Notes:** Per-BDI writeback by Jens Axboe (2.6.32, 2009); xarray (4.20) replaced page-cache radix tree.

### slab-slub-slob-allocator (cross-domain alias: `kmem-cache`, `kmalloc-backend`)
**Domain:** Operating Systems
**Definition:** Kernel object cache — slab (original, per-CPU array), SLUB (default, simpler), SLOB (tiny, embedded); fronts `kmalloc`.
**Atom or composite:** Composite — per-cache freelist + per-CPU partial slabs + node lists.
**Cost model:** ~30 ns hot-path alloc; cold pulls a new slab from buddy.
**Real wall?** Yes — fragmentation; SLUB merges similar caches by default.
**Cross-domain wiring:** Same per-class freelist idea as **distributed-systems/typed-object-pool**.
**Notes:** Bonwick slab (1994); Christoph Lameter SLUB (2007); SLOB removed 6.4 (2023).

### buddy-page-allocator (cross-domain alias: `__alloc_pages`, `order-N-alloc`)
**Domain:** Operating Systems
**Definition:** Power-of-two physical-page allocator; maintains free lists per order 0–10 (4 KB to 4 MB).
**Atom or composite:** Composite — N free lists + split/merge primitive.
**Cost model:** O(MAX_ORDER) split cost; per-CPU pageset caches order-0.
**Real wall?** Yes — fragmentation produces "external" failures at high orders.
**Cross-domain wiring:** Classic memory allocator pattern, parallel to **distributed-systems/range-allocator**.
**Notes:** Knuth TAOCP §2.5; Linux's per-CPU pageset (Hugh Dickins) for order-0 amortization.

### zone-dma-normal-highmem (cross-domain alias: `numa-zone`, `pgdat`)
**Domain:** Operating Systems
**Definition:** Per-NUMA-node partition of physical memory by addressing capability — DMA (<16 MB ISA), DMA32 (<4 GB), normal, movable, device.
**Atom or composite:** Atom — per-zone freelist root.
**Cost model:** Zone fallback chain (`zonelist`) walked when preferred zone is empty.
**Real wall?** Yes — legacy ISA-DMA constraint and 32-bit device addressing.
**Cross-domain wiring:** Same tiered-storage idea as **database-streaming-sketching/tiered-cache**.
**Notes:** ZONE_MOVABLE (2.6.23) supports memory hotplug; ZONE_DEVICE for pmem/HMM.

### compaction-fragmentation (cross-domain alias: `memory-defrag`, `kcompactd`)
**Domain:** Operating Systems
**Definition:** Background defragmentation moving movable pages to coalesce free runs into high-order blocks for THP/jumbo allocations.
**Atom or composite:** Composite — kthread + migrate_pages + isolation lists.
**Cost model:** Per-page migration ~5–10 µs; full compaction multi-ms.
**Real wall?** Yes — non-movable allocations (kernel internal) pin and block compaction.
**Cross-domain wiring:** Same online-compaction idea as **database-streaming-sketching/lsm-major-compaction**.
**Notes:** Mel Gorman (2.6.35, 2010); proactive compaction `vm.compaction_proactiveness` (5.14).

### ksm-kernel-same-page-merging (cross-domain alias: `ksm`, `dedup-anon-pages`)
**Domain:** Operating Systems
**Definition:** Kthread that scans `madvise(MADV_MERGEABLE)` regions, computes a hash, and merges identical pages into a single COW-shared page.
**Atom or composite:** Composite — scan kthread + stable/unstable RB-trees + checksum.
**Cost model:** O(pages) scan; merge breaks on first write (COW).
**Real wall?** Yes — side channel allowing cross-VM information leaks (CVE-2015-2877 ilk).
**Cross-domain wiring:** Same content-addressed dedup as **distributed-systems/dedup-store**, **cryptography-hashing/content-hash**.
**Notes:** Izik Eidus, Red Hat (2.6.32, 2009); used heavily in KVM hosts.

### damon-data-access-monitor (cross-domain alias: `data-access-monitoring`, `proactive-reclaim`)
**Domain:** Operating Systems
**Definition:** Kernel framework that samples region-level access frequency using `vmstat`-like region buckets and triggers policy actions (reclaim, promote).
**Atom or composite:** Composite — region tree + sampler + policy modules (LRU_SORT, RECLAIM, MIGRATE).
**Cost model:** Configurable sampling overhead (default ~0.5 %); region adjustment is amortized.
**Real wall?** No — pure observation, but actions touch real pages.
**Cross-domain wiring:** Same access-tracking idea as **database-streaming-sketching/heat-map**.
**Notes:** SeongJae Park (5.15, 2021); productized by Meta and AWS.

### memory-tier-cxl-cpus-vs-cxl-memory (cross-domain alias: `memory-tiering`, `cxl-memory-expander`)
**Domain:** Operating Systems
**Definition:** Multi-tier memory (DRAM + CXL.mem + persistent memory) with NUMA-distance-based promotion/demotion managed by kernel `page_demotion`.
**Atom or composite:** Composite — NUMA nodes flagged by tier + autonuma promotion + page_demotion under pressure.
**Cost model:** CXL.mem ~150–300 ns vs DRAM ~80 ns; 2–4× bandwidth penalty.
**Real wall?** Yes — physical PCIe lane bandwidth and CXL switch hops.
**Cross-domain wiring:** Same hot/cold tiering as **database-streaming-sketching/heat-tiers**, **distributed-systems/storage-tiers**.
**Notes:** CXL 3.0 spec; Linux memory tiering merged piecewise 5.15–6.x.

### mmio-memory-mapped-io (cross-domain alias: `mmio-bar`, `device-register`)
**Domain:** Operating Systems
**Definition:** Device registers mapped into the CPU address space, accessed via load/store; PCIe BARs configured at enumeration time.
**Atom or composite:** Atom — single load/store touching a non-cacheable mapping.
**Cost model:** ~50–200 ns per access (uncached, posted writes faster).
**Real wall?** Yes — write combining, read serializing semantics dictate driver patterns.
**Cross-domain wiring:** Same memory-as-control plane as **electromagnetics-antennas/register-map**, **signal-processing-rf/codec-config**.
**Notes:** `ioremap()` family; `__iomem` annotation, `READ_ONCE`/`WRITE_ONCE` for ordering.

### memory-pool-mempool (cross-domain alias: `mempool-t`, `reserve-pool`)
**Domain:** Operating Systems
**Definition:** Pre-allocated kernel object pool guaranteeing forward progress in I/O paths under memory pressure (used by block layer, MD/DM).
**Atom or composite:** Composite — backing allocator + min-reserve list + alloc/free hooks.
**Cost model:** O(1) alloc; refill from main allocator when not depleted.
**Real wall?** Yes — pool depth determines max in-flight I/O without deadlock.
**Cross-domain wiring:** Same reservation idea as **networking/skb-pool**, **distributed-systems/connection-pool**.
**Notes:** David Howells; essential for writeback paths that must not block on allocation.

---

## File Systems

### inode-struct (cross-domain alias: `vfs-inode`, `metadata-record`)
**Domain:** Operating Systems
**Definition:** Per-file metadata record (mode, uid, gid, size, atime/mtime/ctime, block pointers, xattrs); 256–4096 B on disk, ~600 B in memory.
**Atom or composite:** Composite — fixed-size record + indirect block pointers.
**Cost model:** O(1) by inum, O(depth) data-block lookup through indirect blocks (ext4) or extent tree (XFS/ext4).
**Real wall?** Yes — inode count is fixed at FS creation (ext) or dynamic (XFS); exhaustion fails creates with ENOSPC.
**Cross-domain wiring:** Same record-by-id idea as **database-streaming-sketching/heap-tuple-id**.
**Notes:** Thompson Unix V1; Ritchie's "inode" name from "index node" or "i-node".

### dentry-dcache (cross-domain alias: `directory-entry`, `name-cache`)
**Domain:** Operating Systems
**Definition:** In-memory name→inode binding; hashed by `(parent, hash(name))`; supports negative entries (NXDOMAIN-style).
**Atom or composite:** Composite — hash chain entry + ref counts + LRU.
**Cost model:** O(1) lookup hit, ~5–10 ns; miss = full path walk + inode load.
**Real wall?** Yes — dcache size bounded by memory; pruning via shrinker under pressure.
**Cross-domain wiring:** Same NS cache as **networking/dns-cache**, **distributed-systems/name-resolution-cache**.
**Notes:** Linus's RCU-walk path lookup (2.6.38, 2011) made dcache walks lockless.

### superblock-fs-mount (cross-domain alias: `vfs-superblock`, `mount-instance`)
**Domain:** Operating Systems
**Definition:** Per-mounted-filesystem control record holding block size, root inode, mount flags, journal pointer, and FS-specific operations vtable.
**Atom or composite:** Composite — `struct super_block` + `super_operations`.
**Cost model:** One per mount; lookup via `mnt_namespace`.
**Real wall?** Yes — fsuuid uniqueness; mount-table size.
**Cross-domain wiring:** Same root-of-name-space pattern as **database-streaming-sketching/catalog-root**.
**Notes:** VFS by Sun (1986); Linux adopted, generalized to plug arbitrary filesystems.

### vfs-pluggable-filesystem (cross-domain alias: `virtual-fs`, `fs-vtable`)
**Domain:** Operating Systems
**Definition:** Abstract layer presenting `inode`, `dentry`, `file`, `super_block` with vtables; concrete FSes (ext4, xfs, btrfs, fuse) implement the operations.
**Atom or composite:** Composite — five vtables and four objects unified across all filesystems.
**Cost model:** Indirect call per VFS op; LTO inlining helps minimally.
**Real wall?** No (abstraction), but every new FS pays the VFS contract.
**Cross-domain wiring:** Classic adapter pattern, like **networking/socket-protocol-ops**.
**Notes:** Sun SunOS 2.0 (1986); Linux adopted at 0.99.

### ext4-extent-fs (cross-domain alias: `ext4`, `htree`, `extent-tree`)
**Domain:** Operating Systems
**Definition:** Linux's default journaling FS; extents (4-tuple `(logical, length, physical)`), HTree directories, 48-bit block addresses, ≤1 EB volume.
**Atom or composite:** Composite — extent tree + HTree dir + journal (JBD2).
**Cost model:** O(log) extent lookup; tail-packed dirent stride.
**Real wall?** Yes — block-group locality bounds inode density.
**Cross-domain wiring:** Layout parallels **database-streaming-sketching/btree-on-disk**.
**Notes:** Theodore Ts'o, Mingming Cao (2.6.28, 2008); descendant of ext2/ext3.

### xfs-allocation-groups (cross-domain alias: `xfs`, `ag`, `b-plus-tree-fs`)
**Domain:** Operating Systems
**Definition:** SGI's 64-bit journaling FS; AG-sharded for parallelism, BTree-of-everything (free space, inode, refcount, rmap), delayed allocation.
**Atom or composite:** Composite — per-AG btrees + log + realtime subvolume.
**Cost model:** Per-AG locks scale linearly with AG count; deferred ops batch metadata.
**Real wall?** Yes — log size and AG count are mkfs-time decisions.
**Cross-domain wiring:** Same shard-per-region idea as **distributed-systems/range-shard**.
**Notes:** Originated IRIX 5.3 (1994); Linux port by SGI (2001); reflinks and self-describing metadata since 3.16.

### btrfs-cow-fs (cross-domain alias: `btrfs`, `b-tree-fs`, `chunk-tree`)
**Domain:** Operating Systems
**Definition:** Copy-on-write FS with snapshots, subvolumes, checksumming, RAID0/1/10/5/6, send/receive; everything stored in copy-on-write B-trees.
**Atom or composite:** Composite — chunk + extent + checksum + root trees.
**Cost model:** CoW write amplification ~2×; scrub is O(disk) background.
**Real wall?** Yes — RAID5/6 has known write-hole issues; metadata fragmentation.
**Cross-domain wiring:** Same CoW root as **database-streaming-sketching/cow-btree**; checksums = **cryptography-hashing/crc32c**.
**Notes:** Chris Mason at Oracle (2007); mainlined 2.6.29.

### zfs-arc-zil (cross-domain alias: `zfs`, `dnode`, `dmu`)
**Domain:** Operating Systems
**Definition:** OpenZFS — pooled storage, CoW, integrity-checked merkle blocks, ARC (adaptive replacement cache), ZIL (intent log) for sync writes.
**Atom or composite:** Composite — DMU + ARC + ZIL + spa pool.
**Cost model:** ARC overhead ~1 GB per TB; dedup tables can blow up RAM.
**Real wall?** Yes — RAM-for-dedup, ARC pressure; not GPL (CDDL) so kernel out-of-tree.
**Cross-domain wiring:** ARC is the canonical **retrieval-search/2q-cache** generalization.
**Notes:** Sun Solaris (2005); now OpenZFS umbrella (Linux, FreeBSD, illumos, macOS).

### f2fs-flash-fs (cross-domain alias: `f2fs`, `log-structured-fs`)
**Domain:** Operating Systems
**Definition:** Samsung's log-structured FS for flash — multi-head logging, hot/cold separation, NAT (node address table) virtualizes inode numbers.
**Atom or composite:** Composite — segments + NAT + SIT + SSA + checkpoint.
**Cost model:** Sequential writes ideal for SSD GC; checkpoint stalls under heavy fsync.
**Real wall?** Yes — NAND erase-block alignment; over-provisioning improves wear.
**Cross-domain wiring:** Same log-structured pattern as **database-streaming-sketching/lsm-tree**.
**Notes:** Jaegeuk Kim (3.8, 2012); default on many Android devices.

### overlayfs-union (cross-domain alias: `overlay`, `upper-lower-dir`)
**Domain:** Operating Systems
**Definition:** Stacking FS presenting a writable upperdir merged over read-only lowerdirs; copy-up on write; used by every container runtime.
**Atom or composite:** Composite — N lowerdirs + upperdir + workdir + merged view.
**Cost model:** First write triggers copy-up (file copy); subsequent zero.
**Real wall?** Yes — copy-up cost for large files; whiteouts implemented as 0:0 char devices.
**Cross-domain wiring:** Same layered immutability as **graphics-rendering-lod/layered-textures**, **type-theory-programming-languages/scope-shadowing**.
**Notes:** Miklos Szeredi (3.18, 2014); replaces aufs/unionfs for most uses.

### tmpfs-ramfs (cross-domain alias: `shmfs`, `in-memory-fs`)
**Domain:** Operating Systems
**Definition:** Page-cache-backed FS with no persistence; tmpfs supports swap-out, ramfs does not; size-limited via mount options.
**Atom or composite:** Composite — vfs over the page cache without backing storage.
**Cost model:** Read/write at memory speed; tmpfs swappable, ramfs OOMs.
**Real wall?** Yes — bounded by RAM (+ swap for tmpfs).
**Cross-domain wiring:** Same in-memory backing as **database-streaming-sketching/in-memory-table**.
**Notes:** Linus added ramfs (2.4); Hugh Dickins extended to tmpfs (2.4 → 2.6).

### procfs-sysfs-debugfs-configfs (cross-domain alias: `pseudo-fs-quartet`, `kernel-introspection-fs`)
**Domain:** Operating Systems
**Definition:** Four kernel-exported FSes — procfs (process info), sysfs (device model), debugfs (debug), configfs (userspace-driven object creation).
**Atom or composite:** Composite — four FSes sharing the "kernel-state-as-files" doctrine.
**Cost model:** Reads trigger on-demand serialization; sysfs writes drive driver callbacks.
**Real wall?** Yes — procfs lacks stable ABI; sysfs is the documented one.
**Cross-domain wiring:** Same "control plane as files" as **networking/netlink-as-fs**.
**Notes:** procfs from Plan 9 (Pike); sysfs by Pat Mochel (2.6, 2003).

### fuse-userspace-filesystem (cross-domain alias: `fuse`, `cuse`, `virtio-fs`)
**Domain:** Operating Systems
**Definition:** Kernel module forwarding VFS calls to a userspace daemon over `/dev/fuse`; lets filesystems live outside the kernel.
**Atom or composite:** Composite — kernel proxy + userspace daemon + req/reply queues.
**Cost model:** 2× syscall + context switch per op; FUSE2 ~µs latency, FUSE3 batches.
**Real wall?** Yes — context-switch overhead bounds throughput; `passthrough_hp` in 6.9+ removes some.
**Cross-domain wiring:** Same userspace-driver pattern as **networking/dpdk-userspace-stack**.
**Notes:** Miklos Szeredi (2.6.14, 2005); virtio-fs by Stefan Hajnoczi (5.4) for VMs.

### journal-jbd2-write-ahead (cross-domain alias: `fs-journal`, `jbd2`)
**Domain:** Operating Systems
**Definition:** Ordered log of metadata (and optionally data) changes ensuring crash consistency via WAL discipline; ext4 uses JBD2.
**Atom or composite:** Composite — log buffer + commit + checkpoint thread.
**Cost model:** Each metadata op doubles writes; group commit amortizes.
**Real wall?** Yes — journal device bandwidth; commit interval (default 5 s) bounds data loss.
**Cross-domain wiring:** Same WAL as **database-streaming-sketching/wal**, **distributed-systems/raft-log**.
**Notes:** Stephen Tweedie ext3 journal (2001); JBD2 generalized for 48-bit blocks.

### journal-mode-writeback-ordered (cross-domain alias: `data-journal-modes`, `data=journal`)
**Domain:** Operating Systems
**Definition:** ext3/4 mount option choosing whether data is journaled (journal), only metadata after data writeback (ordered, default), or unordered (writeback).
**Atom or composite:** Atom — one of three discrete modes.
**Cost model:** journal mode = 2× write; writeback fastest, weakest guarantee.
**Real wall?** Yes — `data=journal` is the only mode that survives full ordering on overwrite-in-place.
**Cross-domain wiring:** Choice maps to **database-streaming-sketching/wal-vs-redo**.
**Notes:** Documented in `Documentation/filesystems/ext4.rst`; default since 2.6.30 is `ordered`.

### reflink-copy-on-write-clone (cross-domain alias: `cp-reflink`, `clonerange`)
**Domain:** Operating Systems
**Definition:** FS operation creating a new file/range that shares blocks with the source via CoW until either side is modified; FICLONE/FICLONERANGE ioctls.
**Atom or composite:** Composite — extent reference count + CoW break.
**Cost model:** O(extents) reference bump; near-instant regardless of size.
**Real wall?** Yes — refcount table must persist; XFS and btrfs support, ext4 does not.
**Cross-domain wiring:** Same dedup primitive as **distributed-systems/snapshot-pointer**.
**Notes:** XFS reflinks (4.9, 2016); btrfs since day one; `cp --reflink=auto`.

### snapshot-read-only-writable (cross-domain alias: `fs-snapshot`, `subvol-snapshot`)
**Domain:** Operating Systems
**Definition:** Point-in-time FS image, instantly created via CoW; reads see old data, writes diverge into a new subvolume tree.
**Atom or composite:** Composite — root pointer copy + per-block CoW.
**Cost model:** O(1) create; storage grows with divergence.
**Real wall?** Yes — snapshot count bounded by metadata overhead; refcount blow-up under churn.
**Cross-domain wiring:** Same MVCC pattern as **database-streaming-sketching/mvcc-version**.
**Notes:** Hammer (DragonFly) and ZFS pioneered; btrfs and dm-thin productized on Linux.

### fsync-fdatasync-syncfs (cross-domain alias: `durability-barrier`, `flush-sync`)
**Domain:** Operating Systems
**Definition:** Syscalls forcing buffered writes (and metadata) to stable storage; `fdatasync` skips non-essential metadata; `syncfs` flushes a whole mount.
**Atom or composite:** Atom — single block-layer barrier.
**Cost model:** Bound by device flush — 1–10 ms HDD, 50 µs–1 ms NVMe, ~µs NVMe with FUA.
**Real wall?** Yes — power-loss durability requires actual platter/NAND commit; FUA bit on each write avoids cache.
**Cross-domain wiring:** Same durability boundary as **distributed-systems/fsync-checkpoint**, **database-streaming-sketching/group-commit-fsync**.
**Notes:** PostgreSQL fsync gate (2018 lore) shaped Linux's "fsync may report errors only once" semantics.

### io-uring-async-io (cross-domain alias: `iouring`, `sqe-cqe`, `sqpoll`)
**Domain:** Operating Systems
**Definition:** Pair of shared SQ/CQ rings between user and kernel allowing async submission of arbitrary syscalls with optional poll-mode (SQPOLL) and registered FDs.
**Atom or composite:** Composite — SQ ring + CQ ring + submission descriptor + io_uring_setup syscall.
**Cost model:** Amortized zero syscalls per op with SQPOLL; ~100 ns op enqueue.
**Real wall?** Yes — submission ring depth; security mitigations (kernel.io_uring_disabled) lock down attack surface.
**Cross-domain wiring:** Same async dispatch as **networking/rdma-qp**, **ml-training/cuda-stream**.
**Notes:** Jens Axboe (5.1, 2019); >100 op types by 6.x; foundation of modern async runtimes (tokio-uring, rustix-aio).

### epoll-edge-level (cross-domain alias: `epoll-ctl`, `epoll-wait`, `ET-LT`)
**Domain:** Operating Systems
**Definition:** Linux event multiplexer: `epoll_create1`, `epoll_ctl` for fd registration, `epoll_wait` returns ready set; edge-triggered (one event per transition) or level-triggered.
**Atom or composite:** Composite — interest set (RB-tree) + ready list.
**Cost model:** O(ready) on wait (vs O(N) for poll/select); ~50 ns per ready event.
**Real wall?** Yes — fd count bounded by `RLIMIT_NOFILE`; not portable beyond Linux.
**Cross-domain wiring:** Generic reactor pattern; cf. **networking/event-loop-pattern**.
**Notes:** Davide Libenzi (2.5.45, 2002); BSD kqueue equivalent (Lemon, 2000).

### fanotify-inotify (cross-domain alias: `fs-watch`, `file-change-notify`)
**Domain:** Operating Systems
**Definition:** Filesystem change notifications — inotify is per-inode/file with rich event mask; fanotify is per-mount/sb with permission control (used by antivirus).
**Atom or composite:** Composite — fd + event mask + watch descriptor.
**Cost model:** O(1) per event; watch count limited by `fs.inotify.max_user_watches`.
**Real wall?** Yes — watch descriptors are kernel memory; large directory trees exhaust default limits.
**Cross-domain wiring:** Same notify pattern as **distributed-systems/watch-event-stream**, **database-streaming-sketching/change-data-capture**.
**Notes:** inotify by John McCutchan (2.6.13); fanotify by Eric Paris (2.6.36); fanotify FAN_REPORT_FID (5.1).

### extended-attributes-xattr (cross-domain alias: `xattrs`, `posix-1e-acl-storage`)
**Domain:** Operating Systems
**Definition:** Name/value pairs attached to inodes; namespaces `user.`, `trusted.`, `security.`, `system.`; used by SELinux, ACLs, capabilities.
**Atom or composite:** Composite — keyed map per inode.
**Cost model:** Inline in inode if small, indirect block if large; lookup ~µs.
**Real wall?** Yes — per-FS xattr block size limit (ext4: 64 KB block).
**Cross-domain wiring:** Same key-value-on-record as **database-streaming-sketching/jsonb-column**.
**Notes:** POSIX.1e draft (withdrawn); Linux added 2.6.

### sendfile-splice-tee-vmsplice (cross-domain alias: `zero-copy-syscalls`, `pipe-splice`)
**Domain:** Operating Systems
**Definition:** Family of syscalls moving data fd→fd without traversing userspace — `sendfile` (file→socket), `splice` (any fd↔pipe), `tee` (pipe→pipe duplicate), `vmsplice` (userpages→pipe).
**Atom or composite:** Composite — kernel pipe buffers as zero-copy intermediary.
**Cost model:** Saves 1–2 copies vs read+write; bound by device/network.
**Real wall?** Yes — page-aligned, pipe-buffer-limited, no transformation in the middle.
**Cross-domain wiring:** Same zero-copy pattern as **networking/zero-copy-tx**, **distributed-systems/rdma-send**.
**Notes:** `sendfile` (2.2); `splice` by Jens Axboe (2.6.17, 2006); io_uring is the modern superset.

### posix-fadvise-madvise (cross-domain alias: `access-pattern-hint`, `dontneed-willneed`)
**Domain:** Operating Systems
**Definition:** Syscalls hinting expected access pattern (SEQUENTIAL, RANDOM, WILLNEED, DONTNEED, NOREUSE) to optimize prefetch/eviction.
**Atom or composite:** Composite — hint enum + (offset, len) range.
**Cost model:** O(pages-touched) for WILLNEED (faulting in), instant for hints.
**Real wall?** Soft — kernel may ignore.
**Cross-domain wiring:** Same hint mechanism as **database-streaming-sketching/access-pattern-pragma**.
**Notes:** POSIX 2001; `madvise(MADV_COLD/PAGEOUT)` (5.4) added explicit eviction.

### fallocate-punch-hole (cross-domain alias: `preallocate`, `falloc-punch`)
**Domain:** Operating Systems
**Definition:** Syscall manipulating file allocation — reserve blocks without writing (FALLOC_DEFAULT), zero-fill (FALLOC_ZERO_RANGE), or punch a hole (FALLOC_PUNCH_HOLE).
**Atom or composite:** Composite — operation flag + (offset, len).
**Cost model:** Reservation is O(1) metadata; punch-hole O(extents-affected).
**Real wall?** Yes — sparse-file support per FS varies; XFS/ext4/btrfs full, FAT none.
**Cross-domain wiring:** Same range-edit as **database-streaming-sketching/range-delete**.
**Notes:** Mike Snitzer (2.6.23); essential for VM images, DB autogrow.

### direct-io-o-direct (cross-domain alias: `o_direct`, `unbuffered-io`)
**Domain:** Operating Systems
**Definition:** `open(..., O_DIRECT)` bypasses page cache, requiring aligned buffers/sizes (typically 4 KB or 512 B); used by databases.
**Atom or composite:** Atom — flag on open.
**Cost model:** Avoids double-buffering; alignment overhead if app mis-sized.
**Real wall?** Yes — Linus famously called it "deranged" (2007), but DBs depend on it.
**Cross-domain wiring:** Same bypass pattern as **database-streaming-sketching/buffer-pool-owns-cache**.
**Notes:** PostgreSQL eventually opted in (16+); MySQL/InnoDB always.

### readv-writev-preadv2-pwritev2 (cross-domain alias: `scatter-gather-io`, `vectorized-syscalls`)
**Domain:** Operating Systems
**Definition:** Vectored I/O syscalls — single call transfers between multiple userspace buffers and one fd; pwritev2/preadv2 add per-call flags (RWF_DSYNC, RWF_HIPRI).
**Atom or composite:** Composite — iov array + fd + offset + flags.
**Cost model:** One syscall for N segments; saves N-1 syscalls.
**Real wall?** Yes — IOV_MAX (default 1024) caps segments.
**Cross-domain wiring:** Same gather/scatter as **networking/scatter-gather-dma**.
**Notes:** `readv/writev` SVR4; `preadv2/pwritev2` Linux 4.6 (2016).

---

## Device & I/O

### character-device (cross-domain alias: `cdev`, `chrdev`)
**Domain:** Operating Systems
**Definition:** Device accessed via byte-stream interface (`/dev/tty*`, `/dev/null`, `/dev/random`); driver registers `file_operations` for read/write/ioctl/mmap.
**Atom or composite:** Composite — (major, minor) + cdev + fops table.
**Cost model:** Per-op fops dispatch; no inherent buffering by kernel.
**Real wall?** Yes — (major, minor) namespace bounded at 32 bits.
**Cross-domain wiring:** Same stream abstraction as **networking/byte-stream-socket**.
**Notes:** SVR4 split major→drivers, minor→instances; dynamic majors since 2.6.

### block-device (cross-domain alias: `bdev`, `blkdev`, `block-layer`)
**Domain:** Operating Systems
**Definition:** Fixed-size-sector device (512 B/4 KB) abstracted by the block layer; supports request merging, scheduling (mq-deadline, bfq, kyber, none).
**Atom or composite:** Composite — gendisk + request_queue + I/O scheduler.
**Cost model:** Per-bio ~µs overhead; scheduling can add ms under contention.
**Real wall?** Yes — physical seek (HDD) or NAND program (SSD) time.
**Cross-domain wiring:** Same packetized I/O as **networking/skb-tx-queue**.
**Notes:** Multi-queue block layer by Jens Axboe (3.13, 2014) replaced single request_queue.

### udev-devtmpfs (cross-domain alias: `udev`, `device-naming`)
**Domain:** Operating Systems
**Definition:** Userspace daemon (`udevd`) that listens to uevents from sysfs and creates `/dev` nodes with rules-based names; devtmpfs auto-creates basic nodes.
**Atom or composite:** Composite — netlink listener + rule engine + symlink farm.
**Cost model:** Per-uevent ~ms latency.
**Real wall?** Yes — rule complexity; race-prone for hotplug.
**Cross-domain wiring:** Same naming/discovery pattern as **distributed-systems/service-discovery**.
**Notes:** Greg KH (2003); merged into systemd in 2012.

### ioctl-syscall (cross-domain alias: `ioctl`, `device-control-cmd`)
**Domain:** Operating Systems
**Definition:** Catch-all syscall (`ioctl(fd, cmd, arg)`) for device-specific control; cmd encodes direction/size/type/nr via `_IO`/`_IOR`/`_IOW`/`_IOWR` macros.
**Atom or composite:** Composite — magic number + driver dispatch.
**Cost model:** Trap + driver-specific handler.
**Real wall?** Yes — type-unsafe; major source of CVEs; replaced piecewise by netlink/sysfs/io_uring.
**Cross-domain wiring:** Same "do anything by number" as **networking/setsockopt**.
**Notes:** Thompson Unix V4 (1973); refused to die.

### dma-buf-shared-buffer (cross-domain alias: `dma-buf`, `cross-driver-buffer`)
**Domain:** Operating Systems
**Definition:** Kernel mechanism letting drivers share DMA-capable buffers across subsystems (GPU↔V4L2↔DRM); fd-based handles enable userspace passing.
**Atom or composite:** Composite — dma_buf object + ops vtable + import/export.
**Cost model:** Zero-copy across drivers; fence/sync overhead ~µs.
**Real wall?** Yes — coherence between devices and CPU caches.
**Cross-domain wiring:** Same shared-DMA buffer as **ml-training/gpu-direct**, **graphics-rendering-lod/zero-copy-texture**.
**Notes:** Sumit Semwal (3.3, 2012); foundational for camera→encoder→display pipelines.

### nvme-queue (cross-domain alias: `nvme-sq-cq`, `nvm-express`)
**Domain:** Operating Systems
**Definition:** NVMe device exposes up to 64 K submission/completion queue pairs over PCIe, doorbell-rung, polling-or-MSI-X serviced.
**Atom or composite:** Composite — SQ + CQ + doorbell registers + MSI-X vector.
**Cost model:** Submit ~100 ns; completion ~µs; >1 M IOPS per drive achievable.
**Real wall?** Yes — PCIe lanes, controller queue depth (typically 1024 per queue).
**Cross-domain wiring:** Same ring-pair as **networking/virtio-ring**, **distributed-systems/rdma-qp**.
**Notes:** NVMe spec 1.0 (2011); Linux nvme driver by Matthew Wilcox.

### virtio-vhost (cross-domain alias: `virtio-pci`, `vhost-user`, `vhost-net`)
**Domain:** Operating Systems
**Definition:** Paravirtual device standard with split-ring queues; vhost moves data path into the host kernel; vhost-user moves it to a userspace daemon (DPDK).
**Atom or composite:** Composite — config space + virtqueues + feature bits.
**Cost model:** Notification cost (eventfd) dominates small I/O; batching crucial.
**Real wall?** Yes — host CPU bandwidth for vring processing.
**Cross-domain wiring:** Same paravirt-ring as **networking/dpdk-vhost-user**, **distributed-systems/qemu-data-path**.
**Notes:** Rusty Russell (2008); OASIS standard since 2016; virtio-net, blk, scsi, fs, gpu, crypto…

### sr-iov-vf-pf (cross-domain alias: `sr-iov`, `virtual-function`)
**Domain:** Operating Systems
**Definition:** PCIe feature exposing physical-function-backed virtual functions, each assignable to a VM or namespace; bypasses host driver.
**Atom or composite:** Composite — PF config space + N VFs + IOMMU isolation.
**Cost model:** Wire-speed I/O (no host involvement); VF count vendor-bounded (32–256).
**Real wall?** Yes — silicon resources; per-VF queue counts.
**Cross-domain wiring:** Same VF concept as **networking/sr-iov-nic**, GPU vGPU.
**Notes:** PCI-SIG SR-IOV spec (2007); Linux VFIO is the userspace assignment path.

### msi-x-irq-affinity (cross-domain alias: `msi-x`, `interrupt-steering`)
**Domain:** Operating Systems
**Definition:** Per-vector message-signaled interrupts (up to 2048 per device); each vector can be pinned to a specific CPU via `/proc/irq/N/smp_affinity`.
**Atom or composite:** Composite — MSI-X table + per-vector affinity.
**Cost model:** ~µs per IRQ; affinity miss causes IPI to actual handler CPU.
**Real wall?** Yes — vector count caps per-device queue parallelism.
**Cross-domain wiring:** Same hash-to-core pattern as **networking/rss-flow-steering**.
**Notes:** PCI 3.0; threaded IRQs (Thomas Gleixner, PREEMPT_RT) added priority.

### irqbalance-cpu-mask (cross-domain alias: `irqbalance`, `auto-irq-distribute`)
**Domain:** Operating Systems
**Definition:** Userspace daemon adjusting IRQ affinity masks based on load and topology; respects cache/NUMA/power domains.
**Atom or composite:** Composite — policy daemon over per-irq affinity.
**Cost model:** Periodic (10 s default) re-evaluation; negligible runtime cost.
**Real wall?** Soft — daemon decisions can conflict with RPS/RFS manual tuning.
**Cross-domain wiring:** Same load-balancing idea as **queueing-theory/jsq-policy**.
**Notes:** Arjan van de Ven (2003); often disabled in low-latency tuning.

### pcie-enumeration-bus-walk (cross-domain alias: `pci-probe`, `bdf`)
**Domain:** Operating Systems
**Definition:** Recursive scan of PCI/PCIe topology populating `pci_dev` for each (bus, device, function); reads config space (0x00–0xFFF for PCIe).
**Atom or composite:** Composite — DFS over bridges + capability list parse.
**Cost model:** O(devices) at boot; ~ms each.
**Real wall?** Yes — config space format is hardware-fixed.
**Cross-domain wiring:** Same discovery pattern as **distributed-systems/topology-discovery**.
**Notes:** PCI Local Bus Spec; hotplug via ACPI/SHPC adds devices at runtime.

### poll-select-legacy (cross-domain alias: `poll`, `select`, `pollfd`)
**Domain:** Operating Systems
**Definition:** POSIX/4.2BSD wait-for-multiple-fds; poll uses `pollfd` array (no fd-set limit), select uses bitmasks capped at FD_SETSIZE.
**Atom or composite:** Composite — fd array + event mask + timeout.
**Cost model:** O(N) scan every call; allocations per call.
**Real wall?** Yes — quadratic per-event under many idle fds (vs O(ready) for epoll).
**Cross-domain wiring:** Same multiplex pattern, predecessor of all modern reactors.
**Notes:** 4.2BSD select (1983); POSIX poll (1988). Still used in portable code.

### kqueue-bsd (cross-domain alias: `kqueue`, `kevent`)
**Domain:** Operating Systems
**Definition:** BSD/macOS event multiplexer; supports fd, signal, timer, file, process, user filters in a unified `kevent` API.
**Atom or composite:** Composite — kqueue fd + N filter types.
**Cost model:** O(ready) wait; richer than epoll (covers signals/timers natively).
**Real wall?** Not on Linux (use epoll+eventfd+signalfd as polyfill).
**Cross-domain wiring:** Same reactor pattern as epoll; both descend from /dev/poll.
**Notes:** Jonathan Lemon (FreeBSD 4.1, 2000); the design epoll borrowed from.

### iocp-windows (cross-domain alias: `i-o-completion-port`)
**Domain:** Operating Systems
**Definition:** Windows kernel completion port — async I/O fires `GetQueuedCompletionStatus`; thread pool model with controlled concurrency.
**Atom or composite:** Composite — completion port handle + N worker threads.
**Cost model:** Designed for high-concurrency servers; ~µs per completion.
**Real wall?** Yes — Windows-only; concurrency bounded by configured threads.
**Cross-domain wiring:** Same proactor pattern as io_uring; conceptually inverse of epoll's reactor.
**Notes:** Windows NT 3.5; documented in MSDN since 1996.

---

## Networking-OS-side

### netlink-socket (cross-domain alias: `af-netlink`, `nl-msg`)
**Domain:** Operating Systems
**Definition:** Datagram socket family for kernel↔user message-passing over typed channels (NETLINK_ROUTE, NETLINK_GENERIC, NETLINK_AUDIT, NETLINK_KOBJECT_UEVENT).
**Atom or composite:** Composite — protocol family + 32 typed buses + TLV attribute encoding.
**Cost model:** ~µs per message; unicast and multicast supported.
**Real wall?** Yes — netlink buffers are kernel memory, rate-limited under flood.
**Cross-domain wiring:** Same typed bus as **distributed-systems/dbus**, **networking/control-plane-channel**.
**Notes:** Alexey Kuznetsov (2.1.x); generic netlink (Jamal Hadi Salim, 2.6.15) for new subsystems.

### rtnetlink-route-config (cross-domain alias: `iproute2-rtnetlink`, `RTM_NEWROUTE`)
**Domain:** Operating Systems
**Definition:** NETLINK_ROUTE protocol for FIB/neigh/link/addr management; backend of `ip(8)` from iproute2.
**Atom or composite:** Composite — RTM_* message types + nested attributes.
**Cost model:** Per-op kernel time + RCU FIB update.
**Real wall?** Yes — FIB scale (millions of routes) drives lookup-algorithm choice.
**Cross-domain wiring:** Same routing-table-edit as **networking/bgp-rib**.
**Notes:** Replaced ioctl-based ifconfig/route circa 2002.

### nftables-iptables (cross-domain alias: `netfilter-tables`, `nft`)
**Domain:** Operating Systems
**Definition:** Kernel packet-filtering frameworks — iptables uses fixed chains/tables, nftables compiles rules to a VM-style bytecode interpreted in netfilter hooks.
**Atom or composite:** Composite — hook + chain + rule (match + verdict) + set/map.
**Cost model:** O(rules) classic iptables; nftables sets give O(1) prefix/range/IP lookups.
**Real wall?** Yes — per-packet rule eval bounds throughput.
**Cross-domain wiring:** Same packet classifier as **networking/firewall-classifier**, **database-streaming-sketching/filter-pushdown**.
**Notes:** Pablo Neira Ayuso nftables (3.13, 2014); successor to iptables/ipset/arptables/ebtables.

### xdp-express-data-path (cross-domain alias: `xdp`, `bpf-xdp`)
**Domain:** Operating Systems
**Definition:** Hook in the driver poll path where a BPF program processes raw `xdp_buff`s before `sk_buff` allocation; verdicts PASS/DROP/TX/REDIRECT/ABORTED.
**Atom or composite:** Composite — hook + bpf prog + per-queue context.
**Cost model:** ~50–100 ns per packet; full line rate at 100 Gbps achievable.
**Real wall?** Yes — driver support; some NICs offload to hardware (XDP-HW).
**Cross-domain wiring:** Same fast-path filter as **networking/dpdk-rx**, but in-kernel.
**Notes:** Brenden Blanco (4.8, 2016); foundation of Cilium, Katran.

### af-xdp-zero-copy-socket (cross-domain alias: `af-xdp`, `umem-tx-rx`)
**Domain:** Operating Systems
**Definition:** Socket family providing userspace direct access to NIC rings via shared UMEM region; zero-copy bypass with XDP_REDIRECT.
**Atom or composite:** Composite — UMEM + fill ring + completion ring + RX/TX rings.
**Cost model:** DPDK-class throughput; userspace busy-poll.
**Real wall?** Yes — driver support + huge-page UMEM allocation.
**Cross-domain wiring:** Kernel analogue of **networking/dpdk-userspace-rx**.
**Notes:** Magnus Karlsson (4.18, 2018); Intel ice/i40e/mlx5 best-supported.

### af-packet-raw-socket (cross-domain alias: `af-packet`, `pf-packet`)
**Domain:** Operating Systems
**Definition:** Socket family delivering raw Ethernet frames to userspace, supports `PACKET_MMAP` ring buffers, `PACKET_FANOUT` load distribution.
**Atom or composite:** Composite — socket + mmap ring + fanout policy.
**Cost model:** mmap ring saves syscalls; ~µs per packet.
**Real wall?** Yes — `CAP_NET_RAW` privilege; not as fast as AF_XDP.
**Cross-domain wiring:** Foundation of `tcpdump`, libpcap; cf. **signal-processing-rf/iq-capture**.
**Notes:** Linux 2.0; `TPACKET_V3` (3.2) added per-block timestamps.

### traffic-control-tc-qdisc (cross-domain alias: `tc-qdisc-class-filter`, `htb-fq-codel`)
**Domain:** Operating Systems
**Definition:** Per-NIC queueing-discipline tree shaping/policing egress (and ingress via mirror); qdiscs include pfifo_fast, htb, fq_codel, fq, cake, mq, taprio.
**Atom or composite:** Composite — qdisc tree + classifiers + actions.
**Cost model:** Per-packet enqueue + classifier eval; htb O(1), cake more.
**Real wall?** Yes — wire rate; soft IRQ context bounds dequeue.
**Cross-domain wiring:** Direct port of **queueing-theory/policers-shapers**, **networking/dscp-qos**.
**Notes:** Alexey Kuznetsov (1999); fq_codel default since 3.6 in many distros.

### rps-rfs-receive-flow-steering (cross-domain alias: `rps`, `rfs`, `accelerated-rfs`)
**Domain:** Operating Systems
**Definition:** Software hash steering — RPS distributes RX softirq to CPUs by hash; RFS additionally biases toward CPU where the app last ran.
**Atom or composite:** Composite — hash-to-cpu map + per-flow expected-cpu table.
**Cost model:** Adds IPI cost; reduces L1/L2 cache misses on small flows.
**Real wall?** Yes — diminishing returns above ~10 Gbps; RSS hardware steering preferred.
**Cross-domain wiring:** Same load-spreading pattern as **networking/rss-toeplitz**.
**Notes:** Tom Herbert (2.6.35, 2010); ARFS leverages NIC hash entries.

### gro-gso-tso-lro (cross-domain alias: `offload-segmentation`, `large-receive-offload`)
**Domain:** Operating Systems
**Definition:** Segmentation/reassembly offloads — GSO defers TCP segmentation to driver/HW, TSO is HW-only, GRO merges incoming segments on RX, LRO is HW version.
**Atom or composite:** Composite — sk_buff frag list + offload bits.
**Cost model:** Reduces per-packet header processing by 10×+ for bulk streams.
**Real wall?** Yes — middleboxes may break TSO/GSO; LRO disables TCP CWR.
**Cross-domain wiring:** Same batched-frame idea as **networking/jumbo-frames**, **distributed-systems/coalesced-rpc**.
**Notes:** Herbert Xu GSO/GRO (2.6.18+); LRO Linux-only, deprecated for routers.

### sk-buff-skb (cross-domain alias: `skb`, `socket-buffer`)
**Domain:** Operating Systems
**Definition:** Linux's universal packet descriptor (~232 B); carries metadata + linear data area + scatter-gather frags + chained fragments.
**Atom or composite:** Composite — fixed-size header + variadic payload + frag list.
**Cost model:** Allocation ~µs (kmem_cache); largest source of TX latency.
**Real wall?** Yes — skb size on stack and per-packet overhead drove TSO/GRO adoption.
**Cross-domain wiring:** Universal packet abstraction; cf. **networking/mbuf-bsd**.
**Notes:** Alan Cox; one of the oldest Linux net structures.

### napi-new-api (cross-domain alias: `napi-poll`, `gro-poll`)
**Domain:** Operating Systems
**Definition:** Interrupt-mitigation framework — driver fires IRQ once, NAPI polls until budget exhausted; switches to poll mode under load.
**Atom or composite:** Composite — napi_struct + poll callback + budget counter.
**Cost model:** Amortizes IRQ cost over batch (default 64 packets/poll).
**Real wall?** Yes — softirq budget; busy CPU starvation under DOS.
**Cross-domain wiring:** Same hybrid IRQ/poll as **storage/nvme-poll**, **distributed-systems/adaptive-batching**.
**Notes:** Jamal Hadi Salim (2.4.20, 2002); foundation for high-rate NIC drivers.

### network-namespace-netns (cross-domain alias: `netns`, `network-isolation`)
**Domain:** Operating Systems
**Definition:** Per-namespace network stack — separate routing, firewall, sockets, ports; pivotal for containers and VRFs.
**Atom or composite:** Composite — namespace ref + all net subsystems vnet-aware.
**Cost model:** ~MB memory per netns; some per-skb dispatch overhead.
**Real wall?** Yes — interface count, port-range scaling.
**Cross-domain wiring:** Container substrate; pairs with **distributed-systems/tenant-isolation**.
**Notes:** Eric Biederman, Pavel Emelyanov (2.6.24, 2008).

### veth-pair (cross-domain alias: `veth`, `pipe-pair-nic`)
**Domain:** Operating Systems
**Definition:** Pair of virtual Ethernet devices, packets sent on one appear on the other; canonical container ↔ host networking link.
**Atom or composite:** Composite — two `net_device`s + xmit_one cross-namespace.
**Cost model:** ~µs per cross-NS packet; one softirq context switch.
**Real wall?** Yes — softirq cost limits per-container throughput.
**Cross-domain wiring:** Same point-to-point virtual link as **networking/tap-tun**.
**Notes:** Pavel Emelyanov (2.6.24); pair semantics like Plan 9 pipefs.

### macvlan-ipvlan (cross-domain alias: `macvlan`, `ipvlan`)
**Domain:** Operating Systems
**Definition:** Sub-interface drivers — macvlan assigns unique MAC per slave (L2), ipvlan shares parent MAC with separate IPs (L3); avoid bridge overhead.
**Atom or composite:** Composite — parent dev + N slaves + mode (bridge/private/vepa/passthru).
**Cost model:** Single-copy delivery; cheaper than bridge for many slaves.
**Real wall?** Yes — switch port may reject multiple MACs (port-security).
**Cross-domain wiring:** Same multi-tenant L2 idea as **networking/sr-iov-vf-mac**.
**Notes:** Eric Biederman macvlan (2.6.27); Mahesh Bandewar ipvlan (3.19).

### bridge-fdb (cross-domain alias: `linux-bridge`, `mac-learning`)
**Domain:** Operating Systems
**Definition:** Software Ethernet bridge with MAC learning, STP, multicast snooping; classic container/VM L2 fabric.
**Atom or composite:** Composite — port list + FDB hash + STP state.
**Cost model:** FDB lookup ~50 ns; STP convergence seconds.
**Real wall?** Yes — flooding under MAC churn; FDB size.
**Cross-domain wiring:** Software switch parallel to **networking/ovs-bridge**.
**Notes:** Lennert Buytenhek (2.4); replaced by OVS/eBPF in many large deployments.

### vxlan-geneve-overlay (cross-domain alias: `vxlan`, `geneve`, `vtep`)
**Domain:** Operating Systems
**Definition:** UDP-encapsulated L2-over-L3 tunnels — VXLAN (24-bit VNI), Geneve (variable TLV options); used for multi-tenant clouds.
**Atom or composite:** Composite — encap header + VTEP socket + FDB-by-VNI.
**Cost model:** ~50–100 ns encap; checksum offload critical.
**Real wall?** Yes — MTU reduction (54 B VXLAN, 78 B Geneve); fragmentation hurts.
**Cross-domain wiring:** Direct **networking/tunneling-protocol** primitive used by Kubernetes CNI plugins.
**Notes:** Stephen Hemminger vxlan (3.7, 2012); Geneve (RFC 8926, 2020).

### wireguard-kernel (cross-domain alias: `wg-kmod`, `noise-ik`)
**Domain:** Operating Systems
**Definition:** UDP-based VPN in the kernel using Noise IK handshake (Curve25519, ChaCha20-Poly1305, BLAKE2s); ~4000 LoC.
**Atom or composite:** Composite — peer table + noise state machine + crypto kthread.
**Cost model:** Line-rate at 10 Gbps; ~1 µs per packet (with AES-NI/AVX2 fallbacks).
**Real wall?** Yes — single CPU core saturates ~5 Gbps; multi-queue helps.
**Cross-domain wiring:** Composes **cryptography-hashing/chacha20-poly1305**, **cryptography-advanced/noise-protocol**.
**Notes:** Jason Donenfeld (5.6, 2020); designed for minimal LoC.

### xfrm-ipsec-framework (cross-domain alias: `xfrm`, `transform-framework`)
**Domain:** Operating Systems
**Definition:** Linux's IPsec policy/SA database; transforms (`xfrm_state`) applied per direction by `xfrm_policy` lookup.
**Atom or composite:** Composite — policy DB (rule list) + SA DB (lookup table).
**Cost model:** Per-packet policy match + SA-applied transform.
**Real wall?** Yes — replay-window state per SA; large peer counts hit DB scale.
**Cross-domain wiring:** Same SA model as **networking/ipsec-rfc4301**, **cryptography-hashing/aead-suite**.
**Notes:** Alexey Kuznetsov + USAGI; foundation for strongSwan, libreswan.

### so-reuseport-load-balance (cross-domain alias: `so-reuseport`, `port-balanced-listen`)
**Domain:** Operating Systems
**Definition:** Socket option allowing multiple listeners on the same (addr, port); kernel hashes incoming connections across them.
**Atom or composite:** Composite — reuseport group + 5-tuple hash + bpf attach point.
**Cost model:** O(1) selection; eliminates accept() lock contention.
**Real wall?** Yes — works for TCP/UDP; UDP groups can lose packets on rebalance.
**Cross-domain wiring:** Software-level **networking/equal-cost-multipath** for connections.
**Notes:** Tom Herbert (3.9 TCP, 3.9 UDP, 2013); SO_REUSEPORT BPF prog attach 4.5.

### tcp-bbr-cubic-reno (cross-domain alias: `cca-pluggable`, `tcp-congestion-control`)
**Domain:** Operating Systems
**Definition:** Modular pluggable congestion-control algorithms; Linux selects per socket via `setsockopt(TCP_CONGESTION)`.
**Atom or composite:** Composite — `tcp_congestion_ops` vtable + ACK callback + cwnd state.
**Cost model:** Per-ACK callback; small additive overhead.
**Real wall?** Yes — bandwidth-delay product (Linux RWND auto-tunes).
**Cross-domain wiring:** Same pluggable algorithm seam as **networking/cca-research**.
**Notes:** Stephen Hemminger pluggable framework (2.6.13); BBR by Google (4.9, 2016).

### multipath-tcp-mptcp (cross-domain alias: `mptcp`, `multipath-subflows`)
**Domain:** Operating Systems
**Definition:** TCP extension carrying a single byte stream across multiple subflows over different paths; in mainline kernel since 5.6.
**Atom or composite:** Composite — meta-socket + N subflow sockets + scheduler.
**Cost model:** Per-subflow state; meta-level reordering buffer.
**Real wall?** Yes — middlebox compatibility, ordering buffer memory.
**Cross-domain wiring:** Application of **networking/multipath-routing**, **distributed-systems/connection-aggregation**.
**Notes:** RFC 8684; Linux MPTCP fork merged piecemeal 5.6+.

### so-zerocopy-msg-zerocopy (cross-domain alias: `msg-zerocopy`, `tx-zerocopy`)
**Domain:** Operating Systems
**Definition:** Socket option allowing `send(2)` to pin user pages instead of copying; completion notifications via error queue.
**Atom or composite:** Composite — flag + completion ring + page pinning.
**Cost model:** Saves the copy on >10 KB sends; setup overhead.
**Real wall?** Yes — pinned-page accounting; per-page reference holds back free.
**Cross-domain wiring:** Same zero-copy as **networking/rdma-send**.
**Notes:** Willem de Bruijn (4.14, 2017); used by Envoy, NGINX.

### tcp-tfo-fastopen (cross-domain alias: `tfo`, `tcp-fast-open`)
**Domain:** Operating Systems
**Definition:** TCP option carrying SYN-cookie-like fastopen cookie that lets clients send data in the SYN, eliminating a round trip on repeat connects.
**Atom or composite:** Composite — cookie cache + SYN data + verify.
**Cost model:** Saves 1 RTT per repeat connect; negligible kernel overhead.
**Real wall?** Yes — middleboxes drop unknown TCP options; cookie blacklist.
**Cross-domain wiring:** Same 0-RTT idea as **cryptography-hashing/tls-1.3-0rtt**, **distributed-systems/keepalive-resume**.
**Notes:** Yuchung Cheng et al. (RFC 7413, Linux 3.7); used by Chrome.

### tcp-nodelay-cork-quickack (cross-domain alias: `nagle-off`, `tcp-cork`)
**Domain:** Operating Systems
**Definition:** Per-socket options disabling Nagle (NODELAY), buffering until full (CORK), or forcing immediate ACK (QUICKACK).
**Atom or composite:** Atom — individual TCP option flags.
**Cost model:** Latency/throughput trade-off; CORK reduces packets-per-byte.
**Real wall?** Yes — Nagle/delayed-ACK interaction causes ~40 ms stalls (the classic small-message bug).
**Cross-domain wiring:** Same buffering knob as **networking/coalescing-toggle**.
**Notes:** Nagle (RFC 896, 1984); TCP_CORK Linux-specific.

---

## Containers & Isolation

### namespace-pid-mnt-net-ipc-uts-user-cgroup-time (cross-domain alias: `ns-types-eight`, `linux-namespaces`)
**Domain:** Operating Systems
**Definition:** Per-task pointers to eight namespace types; `unshare(2)` or `clone3(CLONE_NEW*)` creates a new instance; `setns(2)` joins existing.
**Atom or composite:** Composite — eight independent flavors of "what's visible to this task".
**Cost model:** Per-namespace struct + per-resource bookkeeping; ~MB total per container.
**Real wall?** Yes — per-ns scaling (PID, ports, mounts) is bounded.
**Cross-domain wiring:** Foundation for **distributed-systems/multi-tenant-isolation**.
**Notes:** PID/UTS/IPC (2.4.19–2.6.19), net (2.6.24), user (3.8), cgroup (4.6), time (5.6).

### cgroup-v2-unified-hierarchy (cross-domain alias: `cgroup2`, `unified-hierarchy`)
**Domain:** Operating Systems
**Definition:** Single per-mount hierarchy where each cgroup applies a set of enabled controllers (cpu, memory, io, pids, rdma, hugetlb, misc).
**Atom or composite:** Composite — directory tree + controller files + delegation rules.
**Cost model:** Per-task cgroup pointer; controller cost varies (memory ~1 % overhead).
**Real wall?** Yes — controllers see whole subtree; nested overhead.
**Cross-domain wiring:** Resource governance layer atop **queueing-theory/per-class-fair-share**.
**Notes:** Tejun Heo (4.5, 2016); replaces dozen-mountpoint v1; `cgroup.controllers` enables.

### cpu-cgroup-bandwidth (cross-domain alias: `cpu-cfs-quota`, `cpu-max`)
**Domain:** Operating Systems
**Definition:** Per-cgroup `quota/period` limit on CFS runtime; aggregates across CPUs; throttles tasks exceeding budget.
**Atom or composite:** Composite — runtime accounting + throttling timer.
**Cost model:** O(1) accounting per tick; throttle adds wakeup latency.
**Real wall?** Yes — burst handling pre-5.14 caused tail-latency spikes (the "CPU throttling problem").
**Cross-domain wiring:** Direct **queueing-theory/token-bucket** on CPU time.
**Notes:** Paul Turner (3.2, 2011); `cpu.max.burst` (5.14) added burst credit.

### memory-cgroup-controller (cross-domain alias: `memcg`, `memory.max`)
**Domain:** Operating Systems
**Definition:** Per-cgroup limit on anon + page-cache + kernel mem; on overshoot, reclaim within memcg, then OOM-kill within memcg.
**Atom or composite:** Composite — usage counters + per-memcg LRU + memcg OOM killer.
**Cost model:** Per-page charge ~30 ns; per-memcg LRU adds reclaim cost.
**Real wall?** Yes — global vs memcg reclaim interactions; charge overhead at scale.
**Cross-domain wiring:** Same per-tenant quota as **distributed-systems/quota-per-namespace**.
**Notes:** Pavel Emelyanov, KAMEZAWA Hiroyuki (2.6.25); rewrote for v2 (4.5).

### io-cgroup-blkio (cross-domain alias: `io.max`, `io.weight`, `bfq`)
**Domain:** Operating Systems
**Definition:** Per-cgroup I/O limits (bps, iops) and fair-share weights; v2 controller works with bfq, mq-deadline, kyber.
**Atom or composite:** Composite — per-cgroup bytes/iops counters + throttle dispatch.
**Cost model:** Per-bio counter inc; throttle adds queueing latency.
**Real wall?** Yes — buffered writeback charged to wrong cgroup historically; fixed in v2.
**Cross-domain wiring:** **queueing-theory/proportional-share** applied to I/O.
**Notes:** Vivek Goyal blkio v1 (2.6.33); cgroup-v2 io rewrite by Tejun Heo.

### pids-cgroup-pid-cap (cross-domain alias: `pids.max`, `fork-bomb-cap`)
**Domain:** Operating Systems
**Definition:** Per-cgroup cap on number of forked tasks; defends against fork bombs in multi-tenant clusters.
**Atom or composite:** Atom — single integer counter + max.
**Cost model:** Atomic inc on fork; O(1).
**Real wall?** Yes — only barrier to fork-bomb DoS in shared kernels.
**Cross-domain wiring:** **distributed-systems/concurrency-cap** at host level.
**Notes:** Aleksa Sarai (4.3, 2015).

### oci-runc-crun-runtime (cross-domain alias: `oci-runtime`, `runc`, `crun`)
**Domain:** Operating Systems
**Definition:** OCI Runtime Spec implementations creating a process with namespaces/cgroups/caps/seccomp/AppArmor/LSM from a JSON `config.json`.
**Atom or composite:** Composite — JSON spec → series of `clone3` + setns + prctl + mount.
**Cost model:** Container start ~100 ms (runc), ~30 ms (crun in C).
**Real wall?** Yes — startup latency hit by cgroup creation, seccomp filter compile.
**Cross-domain wiring:** Bridge between **distributed-systems/orchestrator** (k8s) and host kernel features.
**Notes:** Docker libcontainer → runc (2015); crun by Giuseppe Scrivano (C, faster).

### containerd-cri-o-shim (cross-domain alias: `containerd`, `cri-o`, `shim-v2`)
**Domain:** Operating Systems
**Definition:** Container runtime daemons implementing Kubernetes CRI; they own image lifecycle, snapshotters, and spawn OCI runtimes via per-container shims.
**Atom or composite:** Composite — gRPC server + snapshotter + shim process + runtime.
**Cost model:** Image pull dominates; shim startup ~10 ms each.
**Real wall?** Yes — per-pod shim memory; concurrent-pull bottlenecks.
**Cross-domain wiring:** Same daemon-managed-process pattern as **distributed-systems/agent-manager**.
**Notes:** containerd extracted from Docker (2017); cri-o by Red Hat for OpenShift.

### linux-capabilities-cap-sys-admin (cross-domain alias: `caps`, `cap-bounding-set`)
**Domain:** Operating Systems
**Definition:** 41 bits dividing root's powers (CAP_SYS_ADMIN, CAP_NET_ADMIN, …) stored as bounding/effective/permitted/inheritable/ambient sets per task.
**Atom or composite:** Composite — five 64-bit bitmasks per task.
**Cost model:** O(1) per check; overhead negligible.
**Real wall?** Yes — CAP_SYS_ADMIN is "the new root" — too coarse.
**Cross-domain wiring:** Same fine-grained authority idea as **cryptography-hashing/capability-token**.
**Notes:** POSIX.1e draft; Linux 2.2. Ambient set (4.3) lets non-root binaries inherit caps.

### no-new-privs-nnp (cross-domain alias: `nnp`, `pr-set-no-new-privs`)
**Domain:** Operating Systems
**Definition:** One-way prctl flag preventing further-execve privilege escalation (setuid, file caps, LSM transitions); required for unprivileged seccomp.
**Atom or composite:** Atom — single bit in task struct.
**Cost model:** Zero ongoing; checked on execve.
**Real wall?** Yes — irreversible per task and descendants.
**Cross-domain wiring:** **distributed-systems/privilege-monotone-shrink**.
**Notes:** Andy Lutomirski (3.5, 2012); enabler for sandboxing.

### pivot-root-chroot (cross-domain alias: `pivot_root`, `chroot-2`)
**Domain:** Operating Systems
**Definition:** Syscalls reshaping a mount namespace's root — `chroot` makes a subtree the root (defeatable by CAP_SYS_CHROOT escapes), `pivot_root` swaps with the old root for containers.
**Atom or composite:** Atom — root-pointer swap in mount namespace.
**Cost model:** O(1).
**Real wall?** Yes — chroot is not security boundary on its own; combine with NS+caps.
**Cross-domain wiring:** Same root-rebind as **distributed-systems/chrooted-tenant**.
**Notes:** chroot V7 Unix; pivot_root by Werner Almesberger (2.4) for initramfs.

### user-namespace-uid-mapping (cross-domain alias: `user-ns`, `subuid-subgid`)
**Domain:** Operating Systems
**Definition:** Namespace mapping container UIDs/GIDs to host ranges via `/proc/<pid>/uid_map`; unprivileged users can become "root" inside their NS.
**Atom or composite:** Composite — UID map + GID map + capability transition.
**Cost model:** Map lookup ~ns per `getuid` call.
**Real wall?** Yes — historically large CVE surface; many distros restrict via sysctl.
**Cross-domain wiring:** Same identity-translation as **distributed-systems/principal-mapping**.
**Notes:** Eric Biederman (3.8, 2013); shifts the trust boundary inward.

### kubernetes-pod-csi-cni (cross-domain alias: `pod-spec`, `csi-driver`, `cni-plugin`)
**Domain:** Operating Systems
**Definition:** Kubernetes pod = group of co-scheduled containers sharing net/IPC NS; CSI provides storage volumes via gRPC, CNI configures networking via exec plugins.
**Atom or composite:** Composite — pod sandbox + CSI mounts + CNI veth + N containers.
**Cost model:** Pod startup ~seconds; CNI setup dominant for cold pods.
**Real wall?** Yes — IP exhaustion under high pod density; CSI/CNI driver maturity.
**Cross-domain wiring:** OS-side substrate for **distributed-systems/orchestration**.
**Notes:** Borg → Kubernetes (Google, 2014); CRI/CSI/CNI standardize plug-ins.

---

## Security — MAC & LSM

### lsm-linux-security-module (cross-domain alias: `lsm-hook`, `security-vtable`)
**Domain:** Operating Systems
**Definition:** Hook framework with ~250 inline `security_*` calls; modules (SELinux, AppArmor, SMACK, Tomoyo, Yama, LoadPin, SafeSetID, Lockdown, BPF-LSM, Landlock) register implementations.
**Atom or composite:** Composite — static_call hooks + per-module struct of callbacks.
**Cost model:** ~1 ns/static-key hook when no module; module cost varies.
**Real wall?** Yes — stacking only since 5.1; some hooks "major" (only one) vs "minor" (many).
**Cross-domain wiring:** Same plug-in security pattern as **distributed-systems/policy-engine**.
**Notes:** Crispin Cowan, Wirex (2.6.0, 2003); rejected MAC monoculture by being pluggable.

### selinux-type-enforcement (cross-domain alias: `selinux`, `type-enforcement`, `policy-binary`)
**Domain:** Operating Systems
**Definition:** NSA-originated MAC system labeling every subject/object with `user:role:type:level`; policy rules `allow source target:class perms;` enforced by AVC.
**Atom or composite:** Composite — labels (xattrs) + binary policy + access vector cache.
**Cost model:** AVC hit ~50 ns; miss ~µs (policy walk).
**Real wall?** Yes — policy size (Fedora ~5 MB) and label persistence.
**Cross-domain wiring:** Same MAC formal model as **formal-verification/lattice-policy**.
**Notes:** NSA, 1998; Linux LSM merge 2.6.0; Bell-LaPadula + Biba composed via TE.

### apparmor-path-based (cross-domain alias: `apparmor`, `pathname-mac`)
**Domain:** Operating Systems
**Definition:** Per-program profile listing allowed paths and capabilities; pathname-based (not label-based), easier ops trade-off vs SELinux precision.
**Atom or composite:** Composite — profile file + DFA-compiled match engine.
**Cost model:** DFA match per access ~µs.
**Real wall?** Yes — pathname spoofing risks; mount-NS interactions.
**Cross-domain wiring:** Same per-binary policy as **distributed-systems/per-service-policy**.
**Notes:** Crispin Cowan (Immunix); merged 2.6.36; Ubuntu/Debian default.

### smack-simplified-mac (cross-domain alias: `smack`, `simplified-mandatory-access`)
**Domain:** Operating Systems
**Definition:** Label-based MAC with simpler ruleset than SELinux; default in Tizen (Samsung); labels are arbitrary short strings.
**Atom or composite:** Composite — label xattr + rule list (subject, object, mode).
**Cost model:** Rule lookup O(rules) on miss.
**Real wall?** Yes — coarser policy; relies on careful labeling.
**Cross-domain wiring:** Same MAC family; lighter operational load.
**Notes:** Casey Schaufler (2.6.25); Tizen IoT.

### tomoyo-learning-mac (cross-domain alias: `tomoyo`, `pathname-learning`)
**Domain:** Operating Systems
**Definition:** Path-based MAC with learning mode that builds policy from observed program behavior; tree of "domains" per process ancestry.
**Atom or composite:** Composite — domain tree + per-domain access list.
**Cost model:** Policy walk per access; learning mode logs to /sys.
**Real wall?** Yes — generated policy may be over-permissive.
**Cross-domain wiring:** Profile-by-observation like **ml-training/behavioral-clustering**.
**Notes:** Toshiharu Harada NTT (2.6.30); embedded use.

### landlock-stacked-sandbox (cross-domain alias: `landlock`, `unprivileged-sandbox`)
**Domain:** Operating Systems
**Definition:** Unprivileged LSM allowing any process to ratchet down its own filesystem (and network in 6.7+) access via stackable rulesets.
**Atom or composite:** Composite — ruleset fd + path beneath + restrict_self syscall.
**Cost model:** Per-access O(rulesets); negligible for short stacks.
**Real wall?** Yes — coarse capability classes; growing per kernel release.
**Cross-domain wiring:** Same user-driven sandbox as OpenBSD `pledge`/`unveil`.
**Notes:** Mickaël Salaün (5.13, 2021); ratcheting only — irreversible.

### yama-ptrace-scope (cross-domain alias: `yama`, `ptrace_scope`)
**Domain:** Operating Systems
**Definition:** Tiny LSM restricting `ptrace` attachment: 0 classic, 1 only parent or `prctl(PR_SET_PTRACER)`, 2 admin-only, 3 disabled.
**Atom or composite:** Atom — single integer sysctl.
**Cost model:** O(1) per ptrace attempt.
**Real wall?** Yes — debuggers hit it constantly; common cause of "cannot attach".
**Cross-domain wiring:** Same family as **distributed-systems/no-cross-tenant-debug**.
**Notes:** Kees Cook ChromeOS (3.4, 2012); default scope 1 on Ubuntu since 10.10.

### seccomp-bpf (cross-domain alias: `seccomp`, `syscall-filter`, `prctl-pr-set-seccomp`)
**Domain:** Operating Systems
**Definition:** Per-task BPF filter evaluated on every syscall; verdicts ALLOW, KILL_PROCESS, KILL_THREAD, TRAP, ERRNO, USER_NOTIF, LOG, TRACE.
**Atom or composite:** Composite — cBPF program + per-task filter chain.
**Cost model:** ~50–100 ns per syscall; programs cap at 4096 insns.
**Real wall?** Yes — filter sees only register-passed args (no deref); cannot make policy decisions on string paths without USER_NOTIF.
**Cross-domain wiring:** Same kernel-side policy compile as **networking/tc-bpf**.
**Notes:** Will Drewry, Google ChromeOS (3.5, 2012); Docker/runc/crun lean on it.

### lockdown-mode (cross-domain alias: `kernel-lockdown`, `integrity-mode`)
**Domain:** Operating Systems
**Definition:** Kernel feature blocking even-root operations that could violate kernel integrity (e.g., kexec to unsigned, `/dev/mem`); two levels: integrity, confidentiality.
**Atom or composite:** Atom — enum boot/runtime flag.
**Cost model:** Negligible per-check.
**Real wall?** Yes — entered via UEFI Secure Boot or `kernel.lockdown=`; cannot exit.
**Cross-domain wiring:** Same "even-admin-can't" as **cryptography-hashing/tee-attestation**.
**Notes:** Matthew Garrett (5.4, 2019); ties to Secure Boot certificate chain.

### keyring-kernel-keys (cross-domain alias: `keyring`, `keyctl`)
**Domain:** Operating Systems
**Definition:** Kernel-managed secrets store with types (user, big_key, asymmetric, encrypted, trusted) and per-key ACL; used by NFSv4 idmap, eCryptfs, dm-crypt LUKS, IMA.
**Atom or composite:** Composite — keyring (set) + key (typed object) + ACL.
**Cost model:** O(log N) key lookup; cryptographic ops dominate.
**Real wall?** Yes — kernel-memory-resident; quotas via `keys.maxkeys`.
**Cross-domain wiring:** Same secrets-vault primitive as **cryptography-hashing/key-store**.
**Notes:** David Howells RHEL (2.6.10); session/user/process/thread keyring scoping.

---

## Security — Hardware & Memory Safety

### cheri-capability-machine (cross-domain alias: `cheri`, `morello`, `capability-pointer`)
**Domain:** Operating Systems
**Definition:** Hardware-enforced fat pointer carrying (base, length, permissions, tag) — 128-bit + 1-bit tag; dereference outside bounds traps.
**Atom or composite:** Atom — single capability register; cannot be forged because of the tag bit.
**Cost model:** 2× pointer size (128 b → 16 B + tag); negligible perf overhead on ARM Morello.
**Real wall?** Yes — requires new ISA; software must be recompiled "pure-cap" or "hybrid".
**Cross-domain wiring:** Same unforgeable-token idea as **cryptography-hashing/capability**, **type-theory-programming-languages/ownership**.
**Notes:** Cambridge / SRI CHERI (Watson, Neumann, Woodruff, 2010+); ARM Morello board (2022).

### mpk-pkru-protection-keys (cross-domain alias: `mpk`, `pku`, `pkey-mprotect`)
**Domain:** Operating Systems
**Definition:** Intel/ARM userspace mechanism associating each page with a 4-bit pkey; the `PKRU` register gates read/write per key without TLB flush.
**Atom or composite:** Composite — per-page 4-bit field + per-thread PKRU register.
**Cost model:** WRPKRU ~5 ns; no syscall, no TLB shootdown.
**Real wall?** Yes — only 16 keys (15 usable); userspace only; PKRU is unprivileged.
**Cross-domain wiring:** Same intra-process compartment as **distributed-systems/in-process-tenant**, V8 sandbox.
**Notes:** Intel Skylake-X (2017); Linux `pkey_mprotect` 4.6.

### mte-arm-memory-tagging (cross-domain alias: `mte`, `tagged-pointers`, `arm-mte`)
**Domain:** Operating Systems
**Definition:** ARMv8.5 hardware tags 4-bit color on each 16-byte allocation granule and pointer; mismatch traps. Sync (kuasan) and async modes.
**Atom or composite:** Composite — pointer tag + memory tag + comparator on access.
**Cost model:** ~2–5 % perf overhead at sync; <1 % async.
**Real wall?** Yes — 4-bit collision (1/16) means probabilistic, not deterministic, safety.
**Cross-domain wiring:** Same hardware bounds-check as CHERI but coarser; complement to **ml-training/asan**.
**Notes:** ARMv8.5 (2019); Pixel 8 ships userspace MTE 2023.

### pac-pointer-authentication (cross-domain alias: `pac`, `arm-pa`)
**Domain:** Operating Systems
**Definition:** ARMv8.3 instructions sign return addresses / data pointers with HMAC of (pointer, context, key); verification before use traps if forged.
**Atom or composite:** Atom — PAC bits in upper unused VA bits.
**Cost model:** ~1–2 % perf overhead; AUTIA/AUTIB instructions.
**Real wall?** Yes — collision: ~64-N key bits per pointer; counter-based attacks exist.
**Cross-domain wiring:** Same MAC-as-defense as **cryptography-hashing/hmac**.
**Notes:** ARMv8.3-A (2017); macOS arm64e ships, Linux kernel uses for return addresses 5.7+.

### cet-shadow-stack-ibt (cross-domain alias: `cet`, `shadow-stack`, `ibt-endbr`)
**Domain:** Operating Systems
**Definition:** Intel CET — shadow stack (independent return-address copy) + IBT (every indirect branch must land at ENDBR); enforces control-flow integrity.
**Atom or composite:** Composite — SS register + per-thread SS pages + ENDBR insertion.
**Cost model:** Shadow stack ~1 % overhead; IBT compile-time.
**Real wall?** Yes — requires both CPU (Tiger Lake+) and compiler/loader support.
**Cross-domain wiring:** Same CFI idea as ARM BTI, RISC-V Zicfilp/Zicfiss.
**Notes:** Intel SDM Vol 1 §17; Linux 6.6 enabled shadow stack for userspace.

### kaslr-randomization (cross-domain alias: `kaslr`, `address-space-layout-randomization`)
**Domain:** Operating Systems
**Definition:** Boot-time random offset for kernel image (and per-CPU sections via FGKASLR); ASLR for userspace mmap/stack.
**Atom or composite:** Atom — random offset added at load.
**Cost model:** Boot-time only; runtime perf unchanged.
**Real wall?** Yes — defeats by side channels (KAISER, Meltdown bypassed it before mitigations).
**Cross-domain wiring:** Same statistical-defense idea as **statistics-probability/random-perturbation**.
**Notes:** Kees Cook (3.14, 2014); FGKASLR for per-function (5.10).

### kpti-page-table-isolation (cross-domain alias: `kpti`, `kaiser`)
**Domain:** Operating Systems
**Definition:** Separate kernel page-table per userspace process (user-mode visible page tables exclude kernel mappings) to mitigate Meltdown CPU side channel.
**Atom or composite:** Composite — kernel PGD + user PGD + entry trampoline.
**Cost model:** 5–30 % syscall slowdown pre-PCID; ~1 % with PCID.
**Real wall?** Yes — TLB pressure doubles without PCID/ASID.
**Cross-domain wiring:** Same shadow-mapping pattern as **distributed-systems/security-boundary**.
**Notes:** Gruss-Lipp-Schwarz et al. KAISER (2017); Linux KPTI 4.15 (2018).

### spectre-mitigations-retpoline-ibrs (cross-domain alias: `retpoline`, `ibrs-ibpb-stibp`)
**Domain:** Operating Systems
**Definition:** Compiler/runtime mitigations against speculative-execution side channels — retpoline replaces indirect call with trapping ret, IBRS restricts speculation predictor.
**Atom or composite:** Composite — codegen pattern + MSR writes at boundaries.
**Cost model:** 1–10 % perf hit depending on workload.
**Real wall?** Yes — silicon root cause; mitigations are workarounds with cost.
**Cross-domain wiring:** Same speculative-leak class as **cryptography-hashing/cache-timing**.
**Notes:** Spectre v2 (Kocher, Horn, 2018); Linux ships dozen "mitigations=" toggles.

### tpm-measured-boot (cross-domain alias: `tpm`, `pcr-extend`)
**Domain:** Operating Systems
**Definition:** TPM 2.0 device records boot-chain hashes into PCRs via `extend(pcr_i, hash(measurement))`; remote attestation reveals chain.
**Atom or composite:** Composite — PCR bank + extend op + quote signature.
**Cost model:** ~ms per extend (SHA1/256 over 1 KB of measurement).
**Real wall?** Yes — physical TPM bandwidth; firmware-TPM (fTPM) faster but weaker.
**Cross-domain wiring:** **cryptography-hashing/merkle-chain** with hardware root of trust.
**Notes:** TCG TPM 2.0 (2014); Linux trusted-keys uses TPM for sealing.

### sgx-tdx-sev-realm (cross-domain alias: `tee`, `confidential-computing`, `enclave`)
**Domain:** Operating Systems
**Definition:** CPU features creating memory regions opaque even to the OS — Intel SGX (per-process enclave, ≤256 MB), TDX (whole VM), AMD SEV-SNP (whole VM with encryption + integrity), ARM CCA Realms.
**Atom or composite:** Composite — TEE create + attestation report + sealed memory.
**Cost model:** Entry/exit ~µs (SGX), full VM exit (TDX/SEV); memory encryption ~5 % BW.
**Real wall?** Yes — silicon-level isolation; side channels (Foreshadow, Crosstalk) keep emerging.
**Cross-domain wiring:** Same trust-anchor as **distributed-systems/attested-compute**, **cryptography-advanced/remote-attestation**.
**Notes:** SGX (Skylake, 2015); SEV (Naples, 2017); TDX (Sapphire Rapids, 2023); CCA (Arm v9, 2024).

### ima-evm-fs-verity (cross-domain alias: `ima`, `evm`, `fs-verity`, `dm-verity`)
**Domain:** Operating Systems
**Definition:** Integrity subsystem — IMA records file hashes, EVM signs xattrs, fs-verity merkle-verifies on read, dm-verity verifies block devices (Android verified-boot).
**Atom or composite:** Composite — measurement list + signature verification + appraisal policy.
**Cost model:** Hash on first read; verify on every block read (dm-verity) or page-cache fill (fs-verity).
**Real wall?** Yes — performance vs security knob; fs-verity is mmap-friendly merkle.
**Cross-domain wiring:** Hardware-rooted **cryptography-hashing/merkle-tree** on filesystems.
**Notes:** IMA by Mimi Zohar (2.6.30); fs-verity by Eric Biggers (5.4).

---

## Synchronization Primitives

### mutex-pthread (cross-domain alias: `posix-mutex`, `pthread_mutex_t`)
**Domain:** Operating Systems
**Definition:** Sleeping lock with futex fast path; supports `PTHREAD_MUTEX_NORMAL/ERRORCHECK/RECURSIVE/PI/ROBUST`.
**Atom or composite:** Composite — atomic word + futex parking.
**Cost model:** Uncontended ~15 ns; contended ~1–2 µs (futex sleep).
**Real wall?** Yes — robust + PI is the only sound RT-safe shape; otherwise unbounded waits.
**Cross-domain wiring:** Same critical-section primitive across every OS.
**Notes:** POSIX 1003.1c (1995); NPTL on Linux uses futex.

### semaphore-system-v-posix (cross-domain alias: `sem-t`, `semget`)
**Domain:** Operating Systems
**Definition:** Counting synchronization object — SysV `semop` (named by key, with arrays), POSIX named (`sem_open`) and unnamed (`sem_init`).
**Atom or composite:** Atom — single integer counter + wait queue.
**Cost model:** ~100 ns futex-backed; SysV adds IPC namespace bookkeeping.
**Real wall?** Yes — SysV ipcs limits (`/proc/sys/kernel/sem`).
**Cross-domain wiring:** Same counting-resource pool as **queueing-theory/permit-token**.
**Notes:** Dijkstra (1962); SysV semop allows multi-op atomic sequences.

### condition-variable-cv (cross-domain alias: `pthread_cond_t`, `wait-signal`)
**Domain:** Operating Systems
**Definition:** Wait/notify primitive paired with a mutex; `cond_wait` releases lock and parks, `cond_signal/broadcast` wakes one/all.
**Atom or composite:** Composite — futex + sequence counter + waiter list.
**Cost model:** Wakeup ~µs; spurious wakeups require predicate loop.
**Real wall?** Yes — historical Linux NPTL had thundering-herd; current uses FUTEX_REQUEUE.
**Cross-domain wiring:** Same monitor pattern as **type-theory-programming-languages/hoare-monitor**.
**Notes:** Hoare/Brinch Hansen monitors (1974); POSIX added; Hoare vs Mesa semantics matter.

### barrier-pthread-barrier (cross-domain alias: `barrier`, `phase-sync`)
**Domain:** Operating Systems
**Definition:** N-way synchronization point; all N threads block at `pthread_barrier_wait` until the Nth arrives, then all proceed.
**Atom or composite:** Composite — counter + generation + futex.
**Cost model:** O(1) atomic + futex wake.
**Real wall?** Yes — slowest thread sets the rate (Amdahl).
**Cross-domain wiring:** Same fan-in pattern as **distributed-systems/quorum-barrier**, MPI_Barrier.
**Notes:** POSIX 1003.1j (2000); used heavily in OpenMP `#pragma omp barrier`.

### spinlock-ticket-mcs-qspinlock (cross-domain alias: `spinlock`, `qspinlock`)
**Domain:** Operating Systems
**Definition:** Busy-wait lock; Linux uses qspinlock (Peter Zijlstra) — 4-byte lock word with per-CPU MCS nodes for queueing under contention.
**Atom or composite:** Composite — atomic word + per-CPU MCS chain on overflow.
**Cost model:** Uncontended ~5 ns; contended scales linearly with contender count, no cache-line ping (MCS).
**Real wall?** Yes — must be held with preemption off; cannot block.
**Cross-domain wiring:** Same lock primitive across all kernels; MCS lock idea (Mellor-Crummey & Scott).
**Notes:** Linux qspinlock 4.2 (2015); paravirt variants for KVM/Xen.

### rwlock-rw-semaphore (cross-domain alias: `rwlock`, `rw_semaphore`, `percpu-rwsem`)
**Domain:** Operating Systems
**Definition:** Reader/writer locks — multiple readers or single writer; Linux `rw_semaphore`, `percpu_rw_semaphore` (per-CPU read-fast, slow write).
**Atom or composite:** Composite — atomic counters + waiter queue.
**Cost model:** Reader fast path ~10 ns; writer fast path ~20 ns; percpu_rwsem read = no atomic.
**Real wall?** Yes — writer starvation if readers dominate; per-cpu version inverts.
**Cross-domain wiring:** Same shared/exclusive pattern as **database-streaming-sketching/shared-row-lock**.
**Notes:** Linux rwsem rewrite by Waiman Long (4.9, 2016) for fairness.

### rcu-read-copy-update (cross-domain alias: `rcu`, `srcu`, `tasks-rcu`)
**Domain:** Operating Systems
**Definition:** Synchronization framework where readers never block; writers publish new copy then wait for "grace period" until all pre-existing readers have left; multiple flavors (classic, SRCU, Tasks).
**Atom or composite:** Composite — quiescent-state tracking + callback list + grace-period kthreads.
**Cost model:** Reader = compiler barrier (~ns); grace period ~ms.
**Real wall?** Yes — memory reclamation latency = grace period; not for short-lived objects.
**Cross-domain wiring:** Same epoch-based reclamation as **distributed-systems/hazard-epoch**, lock-free DB.
**Notes:** Paul McKenney (2.5, 2002); the foundation of scalable kernel structures.

### atomic-types-atomic-t (cross-domain alias: `atomic_t`, `compare-exchange`, `xchg`)
**Domain:** Operating Systems
**Definition:** Lock-free integer operations (add, sub, inc, dec, cmpxchg, xchg) translating to single hardware instructions on supported widths.
**Atom or composite:** Atom — single primitive per op.
**Cost model:** ~5–20 ns each; contended cache line bounces.
**Real wall?** Yes — hardware doesn't offer wider than ~128-bit atomic; bigger ops need RCU/locks.
**Cross-domain wiring:** Same lock-free building block as **distributed-systems/cas-loop**, **database-streaming-sketching/atomic-counter**.
**Notes:** Linux `atomic_t` since 1.x; `atomic_long_t` and `refcount_t` added later.

### memory-barrier-fence-smp-mb (cross-domain alias: `mb-rmb-wmb`, `acquire-release-fence`)
**Domain:** Operating Systems
**Definition:** Hardware ordering instructions — `smp_mb` (full), `smp_rmb`, `smp_wmb`, plus acquire/release pairs and `READ_ONCE/WRITE_ONCE` for compiler barriers.
**Atom or composite:** Atom — single fence/instruction emitted by the compiler.
**Cost model:** ~5–20 ns each; acquire/release cheaper than full fence on weak-ordered (ARM).
**Real wall?** Yes — hardware memory model (TSO on x86, RVWMO on RISC-V, weak on ARM).
**Cross-domain wiring:** Same fence as in **distributed-systems/consistency-fence**, language MM specs.
**Notes:** Linux memory model formalized 2018 by McKenney et al.; "kernel.org/doc/Documentation/memory-barriers.txt".

### completion-wait-queue (cross-domain alias: `struct-completion`, `wait_event`)
**Domain:** Operating Systems
**Definition:** One-shot wait primitive — `wait_for_completion` blocks until `complete` is called; built on wait_queue_head + simple done counter.
**Atom or composite:** Composite — done counter + wait queue.
**Cost model:** ~µs wakeup; interruptible variants honor signals.
**Real wall?** Yes — interrupted waits return -ERESTARTSYS; drivers must handle.
**Cross-domain wiring:** Same future/promise primitive as **type-theory-programming-languages/promise**, libuv async-handle.
**Notes:** Linus's pattern, popularized 2.6.

### lockdep-deadlock-detector (cross-domain alias: `lockdep`, `lock-class-graph`)
**Domain:** Operating Systems
**Definition:** Runtime deadlock detector tracking lock-acquisition order across lock classes; reports unsafe orderings before they cause real deadlock.
**Atom or composite:** Composite — lock class hash + dependency graph + IRQ-state vector.
**Cost model:** ~3–10× lock overhead with CONFIG_PROVE_LOCKING; off in production.
**Real wall?** Yes — class-count limit (`MAX_LOCKDEP_KEYS`) and graph memory.
**Cross-domain wiring:** Same wait-for-graph idea as **distributed-systems/deadlock-detector**.
**Notes:** Ingo Molnar (2.6.18, 2006); single most-effective kernel-debug feature.

### seqlock-seqcount (cross-domain alias: `seqlock_t`, `seqcount`)
**Domain:** Operating Systems
**Definition:** Optimistic reader pattern — reader snapshots an even counter, reads, re-reads counter; if odd or changed, retry. Writer bumps counter around the write.
**Atom or composite:** Composite — counter + writer spinlock.
**Cost model:** Reader = 2 reads (no atomic if uncontended), writer ~50 ns.
**Real wall?** Yes — writes must complete fast or readers livelock; not safe for pointer-chasing.
**Cross-domain wiring:** Same optimistic-versioning pattern as **database-streaming-sketching/mvcc-snapshot**.
**Notes:** Stephen Hemminger (2.5, 2002); used for jiffies, timekeeping.

### ww-mutex-wound-wait (cross-domain alias: `ww-mutex`, `wound-wait-mutex`)
**Domain:** Operating Systems
**Definition:** Mutex variant supporting deadlock-free acquisition of multiple locks by transaction — younger transaction "wounded" gives up.
**Atom or composite:** Composite — base mutex + ww_class context + timestamp.
**Cost model:** Slightly more than mutex; rollback cost on wound.
**Real wall?** Yes — caller must be prepared to release/retry on -EDEADLK.
**Cross-domain wiring:** Same wound-wait scheme as **database-streaming-sketching/deadlock-prevention-ts**.
**Notes:** Maarten Lankhorst (3.13, 2014) for GPU drivers handling multi-buffer lock.

### percpu-counter-refcount (cross-domain alias: `percpu-counter`, `local_t`)
**Domain:** Operating Systems
**Definition:** Counter sharded per CPU to avoid cache-line ping; periodic sync to global; `refcount_t` adds saturation arithmetic to prevent over/underflow CVEs.
**Atom or composite:** Composite — per-CPU local + global sum + batch threshold.
**Cost model:** Counter increment ~ns per CPU; global read is O(NCPUs).
**Real wall?** Yes — read-after-write returns approximate value.
**Cross-domain wiring:** Same sharded-counter as **distributed-systems/sharded-stat**, **database-streaming-sketching/cms-sketch**.
**Notes:** Andrew Morton percpu_counter (2.6); `refcount_t` 4.13 (Kees Cook).

### rseq-restartable-sequences (cross-domain alias: `rseq`, `userspace-restartable-critical-section`)
**Domain:** Operating Systems
**Definition:** Per-thread userspace critical section that kernel rolls back if interrupted; enables fast per-CPU operations without locks.
**Atom or composite:** Composite — abort handler PC + critical section descriptor.
**Cost model:** Zero on uncontested path; rare restart on preempt/migrate.
**Real wall?** Yes — handler must be properly aligned; debugger interaction tricky.
**Cross-domain wiring:** Foundation of tcmalloc-style per-CPU caches, Rust crossbeam.
**Notes:** Mathieu Desnoyers (4.18, 2018); used by glibc malloc on Linux.

### urcu-userspace-rcu (cross-domain alias: `urcu`, `liburcu`)
**Domain:** Operating Systems
**Definition:** Userspace RCU library porting kernel RCU patterns; flavors (mb, signal, qsbr, bp) trade cost between reader and writer.
**Atom or composite:** Composite — per-thread reader counter + grace-period polling.
**Cost model:** Reader 0–10 ns; grace period via memory barrier broadcast or polling.
**Real wall?** Yes — QSBR requires explicit quiescent state calls.
**Cross-domain wiring:** Foundation for LTTng, DPDK, ScyllaDB.
**Notes:** Mathieu Desnoyers, McKenney (~2009); userspace counterpart of kernel RCU.

### htm-rtm-tsx-tme (cross-domain alias: `hardware-transactional-memory`, `tsx`, `tme`)
**Domain:** Operating Systems
**Definition:** Hardware-supported optimistic critical sections — Intel TSX (XBEGIN/XEND/XABORT), ARM TME; conflicts trigger transparent rollback.
**Atom or composite:** Composite — speculative execution buffer + abort handlers.
**Cost model:** Best-case faster than lock; abort fallback is the worst case.
**Real wall?** Yes — Intel TSX disabled on most CPUs post-TAA (2019); capacity limits (~32 cache lines).
**Cross-domain wiring:** Same STM/HTM family as **database-streaming-sketching/optimistic-concurrency**.
**Notes:** Sun Rock first commercial (2009, cancelled); Intel TSX (Haswell, 2013, troubled history).

### atomic128-cmpxchg16b (cross-domain alias: `cmpxchg16b`, `casp`, `lse`)
**Domain:** Operating Systems
**Definition:** 16-byte atomic compare-and-swap — x86 `CMPXCHG16B`, ARM `CASP` (LSE); enables wider lock-free structures (tagged pointers).
**Atom or composite:** Atom — single 128-bit RMW.
**Cost model:** ~10–20 ns; contended cache-line bounce dominates.
**Real wall?** Yes — alignment to 16 B; LSE atomics added in ARMv8.1.
**Cross-domain wiring:** Same primitive underpins **distributed-systems/hazard-pointer**, lock-free queues.
**Notes:** Linux `cmpxchg_double` macro; many lock-free MPMC queues require it.

---

## IPC

### pipe-pipefs (cross-domain alias: `pipe`, `fifo`, `splice-buffer`)
**Domain:** Operating Systems
**Definition:** Kernel byte-stream FIFO between fds; backing is `struct pipe_inode_info` with a circular buffer of page references (default 16 pages = 64 KB).
**Atom or composite:** Composite — pipe inode + page ring + waiter queue.
**Cost model:** Per-write ~µs; resize via `F_SETPIPE_SZ`.
**Real wall?** Yes — pipe size capped by `/proc/sys/fs/pipe-max-size`; per-user limit.
**Cross-domain wiring:** Same producer-consumer channel as **distributed-systems/queue**.
**Notes:** Doug McIlroy concept (1964), Thompson V3 implementation; the original Unix pipeline.

### socket-af-unix (cross-domain alias: `unix-domain-socket`, `af_unix`, `seqpacket`)
**Domain:** Operating Systems
**Definition:** Same-host socket family; SOCK_STREAM, SOCK_DGRAM, SOCK_SEQPACKET; supports path-bound, abstract (`\0`-prefixed), and anonymous (socketpair).
**Atom or composite:** Composite — socket + sk_buff queue.
**Cost model:** ~µs RTT; faster than TCP localhost (no TCP/IP stack).
**Real wall?** Yes — credential-passing limited to local; abstract NS escapes filesystem.
**Cross-domain wiring:** Same local-IPC pattern as **networking/loopback** but cheaper.
**Notes:** 4.2BSD (1983); abstract namespace is Linux extension.

### scm-rights-fd-passing (cross-domain alias: `scm_rights`, `fd-over-socket`)
**Domain:** Operating Systems
**Definition:** Ancillary data carrying file descriptors over AF_UNIX via `sendmsg(SCM_RIGHTS)`; receiver gets a duplicate fd referring to same struct file.
**Atom or composite:** Composite — cmsg header + int array of fds.
**Cost model:** O(N) fd installs in receiver; per-fd ref bump.
**Real wall?** Yes — receiver `RLIMIT_NOFILE` enforcement; garbage collection for cyclic graphs (CVE history).
**Cross-domain wiring:** Same capability-handoff as **cryptography-hashing/capability-token-pass**.
**Notes:** 4.3BSD; foundation of systemd activation, Wayland.

### posix-message-queue (cross-domain alias: `mq-overview`, `mq_open`)
**Domain:** Operating Systems
**Definition:** Named, priority-ordered datagram queue with `mq_open`/`mq_send`/`mq_receive`; lives under `/dev/mqueue` in `mqueuefs`.
**Atom or composite:** Composite — named inode + priority heap.
**Cost model:** ~µs send/receive; priority insertion O(log N).
**Real wall?** Yes — `/proc/sys/fs/mqueue/msg_max` and msgsize_max caps.
**Cross-domain wiring:** Same priority channel as **queueing-theory/priority-queue**.
**Notes:** POSIX 1003.1b (1993); Linux 2.6.6.

### sysv-msg-sem-shm (cross-domain alias: `sysv-ipc`, `ipcs`)
**Domain:** Operating Systems
**Definition:** Three SysV IPC families — message queues, semaphores, shared memory; keyed by `key_t`, listed via `ipcs(1)`.
**Atom or composite:** Composite — three subsystems sharing key/ID model.
**Cost model:** Older interface; per-op syscall.
**Real wall?** Yes — global namespace before NS-awareness (added in 2.6.30).
**Cross-domain wiring:** Pre-POSIX template; cf. **distributed-systems/named-channel**.
**Notes:** SVR2 (1984); Stevens APUE chapter 14 documents extensively.

### posix-shm-shm-open (cross-domain alias: `shm_open`, `shm-tmpfs`)
**Domain:** Operating Systems
**Definition:** Shared memory via `shm_open` (returns fd from `/dev/shm` tmpfs), `ftruncate`, `mmap`; named, fd-passable, lifetime tied to last unlink.
**Atom or composite:** Composite — tmpfs-backed file + mmap MAP_SHARED.
**Cost model:** Setup ~µs; access at memory speed.
**Real wall?** Yes — `/dev/shm` tmpfs limit (usually 50 % RAM).
**Cross-domain wiring:** Same shared-region as **distributed-systems/shared-buffer-rdma**.
**Notes:** POSIX 1003.1b; Linux tmpfs-backed since 2.4.

### memfd-sealed-memfd (cross-domain alias: `memfd_create`, `memfd-seal`)
**Domain:** Operating Systems
**Definition:** Anonymous file in memory created via `memfd_create`; supports seals (F_SEAL_SHRINK, GROW, WRITE, FUTURE_WRITE, EXEC) committing immutability.
**Atom or composite:** Composite — anon file + per-seal bit.
**Cost model:** Setup ~µs; access at memory speed.
**Real wall?** Yes — once sealed, irreversible; F_SEAL_EXEC (6.3) blocks PROT_EXEC mmap.
**Cross-domain wiring:** Foundation of secure GPU buffer sharing, sandbox program loading.
**Notes:** David Herrmann (3.17, 2014); used by Wayland, Chromium, Vulkan.

### binder-android-ipc (cross-domain alias: `binder`, `android-binder`)
**Domain:** Operating Systems
**Definition:** Android's IPC mechanism — `/dev/binder` driver, ServiceManager, parcels, oneway/twoway, fd passing; transactions are RPC-like.
**Atom or composite:** Composite — driver + parcel marshaling + thread pool.
**Cost model:** ~10 µs per transaction; thread-pool sized at process registration.
**Real wall?** Yes — single-threaded per binder lookup; Project Treble split context to vendor/system.
**Cross-domain wiring:** Same RPC-over-shmem pattern as **distributed-systems/local-grpc**.
**Notes:** OpenBinder (Be Inc → Palm → Android, 2008); Linux mainline 3.19.

### dbus-message-bus (cross-domain alias: `dbus`, `org.freedesktop.dbus`)
**Domain:** Operating Systems
**Definition:** Userspace message bus with system and session instances; methods, signals, properties over AF_UNIX with `dbus-daemon` routing.
**Atom or composite:** Composite — daemon + per-process bus connections + introspection XML.
**Cost model:** ~30 µs round-trip; daemon is bottleneck.
**Real wall?** Yes — single daemon, latency; kdbus and bus1 attempts to kernelize were rejected.
**Cross-domain wiring:** Same RPC bus as **distributed-systems/rpc-bus**, GNOME and KDE substrate.
**Notes:** Havoc Pennington (2002); systemd dbus-broker is a faster userspace implementation.

### mach-port-rights (cross-domain alias: `mach-port`, `port-send-receive-right`)
**Domain:** Operating Systems
**Definition:** macOS/Darwin IPC — port name table per task with send, receive, send-once, port-set, dead-name rights; `mach_msg` carries them.
**Atom or composite:** Composite — port name + right type + protected by capability semantics.
**Cost model:** ~µs msg send; kernel maintains right ownership invariants.
**Real wall?** Yes — port table size; dead-name notifications under crash.
**Cross-domain wiring:** Same capability IPC as **cryptography-hashing/capability**; ancestor of Fuchsia Zircon.
**Notes:** Mach 3 (CMU, 1986); macOS XNU still uses internally.

### zircon-channel-fuchsia (cross-domain alias: `zircon-channel`, `fidl-message`)
**Domain:** Operating Systems
**Definition:** Fuchsia OS IPC — handle-based bidirectional channel carrying typed FIDL messages (with handles inline); `zx_channel_call` is two-way.
**Atom or composite:** Composite — channel handle + 64 KB message + 64 handle limit.
**Cost model:** ~µs per call; designed for microkernel IPC fast path.
**Real wall?** Yes — handle table per process; channel queue depth.
**Cross-domain wiring:** Same typed channel as **distributed-systems/grpc-stream** but in-kernel.
**Notes:** Google Fuchsia Zircon (2017); descendant of Magenta which descended from LK.

### sel4-endpoint-notification (cross-domain alias: `sel4-ep`, `sel4-notification`)
**Domain:** Operating Systems
**Definition:** seL4 IPC primitives — Endpoint (synchronous rendezvous), Notification (asynchronous signal-set); both are capability-protected objects.
**Atom or composite:** Composite — kernel object + capability + queue.
**Cost model:** ~1 µs IPC; formally verified for correctness on ARM/x86/RISC-V.
**Real wall?** Yes — synchronous IPC; no buffering in Endpoint.
**Cross-domain wiring:** Reference design for **distributed-systems/verified-rpc**.
**Notes:** Liedtke L4 lineage; Klein et al. seL4 (2009, SOSP); first OS kernel proven correct.

### l4-ipc-microkernel (cross-domain alias: `l4-ipc`, `liedtke-ipc`)
**Domain:** Operating Systems
**Definition:** Synchronous register-only message passing — Liedtke's L4 fits typical message in registers, avoiding TLB and cache pollution; round-trip ~200 cycles.
**Atom or composite:** Composite — sender + receiver + register payload + optional memory items.
**Cost model:** ~100–200 ns IPC on modern CPUs; the original µkernel performance breakthrough.
**Real wall?** Yes — synchronous (no buffering); requires careful API design.
**Cross-domain wiring:** Direct ancestor of Fiasco, NOVA, Pistachio, seL4, Genode.
**Notes:** Jochen Liedtke (1993); proved that microkernel IPC can be cheap.

### doors-solaris (cross-domain alias: `solaris-doors`, `door-call`)
**Domain:** Operating Systems
**Definition:** Solaris RPC primitive where client `door_call` traps into kernel and resumes on a server thread that immediately returns, no scheduling decision.
**Atom or composite:** Composite — door fd + server thread pool.
**Cost model:** ~µs; cheapest local RPC at the time.
**Real wall?** Yes — Solaris-only (Linux never adopted); thread pool sizing.
**Cross-domain wiring:** Same migrating-thread RPC as **distributed-systems/colo-rpc**.
**Notes:** Sun Solaris 2.5 (1995); Spring OS research; named for the "door" you walk through.

### signal-posix-rt-signal (cross-domain alias: `signal`, `sigaction`, `siginfo`)
**Domain:** Operating Systems
**Definition:** Asynchronous notifications delivered to a process/thread; 32 standard + 32 RT signals; handlers run with restricted async-signal-safety.
**Atom or composite:** Composite — pending bitmap + queue (for RT) + per-signal disposition.
**Cost model:** Delivery ~µs (interrupt-like); signal handler must save/restore.
**Real wall?** Yes — handler can use only async-signal-safe syscalls (write, _exit, …).
**Cross-domain wiring:** Same async-event idea as **distributed-systems/wakeup-callback**.
**Notes:** V7 Unix signals; POSIX `sigaction` (1990); RT signals queueable.

### pidfd-process-fd (cross-domain alias: `pidfd`, `pidfd_send_signal`, `pidfd_open`)
**Domain:** Operating Systems
**Definition:** File descriptor referring to a specific process; survives PID reuse, signal targeting via `pidfd_send_signal`, EOF-on-exit polling via epoll.
**Atom or composite:** Atom — fd wrapping a `struct pid`.
**Cost model:** ~ns lookup; replaces racy PID arithmetic.
**Real wall?** Yes — must be created before the target dies for guarantee.
**Cross-domain wiring:** Same handle-based identity as **distributed-systems/lease-handle**.
**Notes:** Christian Brauner (5.3, 2019); pidfd_getfd (5.6) lets you steal fds.

### eventfd-semaphore (cross-domain alias: `eventfd`, `efd_semaphore`)
**Domain:** Operating Systems
**Definition:** 64-bit counter exposed as fd; write adds to counter, read returns and resets (or decrements in semaphore mode); pollable.
**Atom or composite:** Atom — single u64 in kernel + waitqueue.
**Cost model:** ~50 ns read/write; epoll-friendly.
**Real wall?** No — pure userspace coordination.
**Cross-domain wiring:** Same coordination handle as **distributed-systems/local-semaphore-fd**.
**Notes:** Davide Libenzi (2.6.22, 2007); used heavily by io_uring, KVM, libuv.

---

## Real-time

### preempt-rt-fully-preemptible (cross-domain alias: `preempt-rt`, `prt-patch-set`)
**Domain:** Operating Systems
**Definition:** Linux configuration making the kernel fully preemptible — converts spinlocks to rtmutex (PI), threaded IRQs, hrtimer-driven, eliminates non-preemptible paths.
**Atom or composite:** Composite — set of patches (now ~99 % mainlined as of 6.12) enabling `PREEMPT_RT`.
**Cost model:** ~5–15 % throughput hit for sub-100 µs wakeup-latency guarantee.
**Real wall?** Yes — physical interrupt path latency; SMI-storms still tank determinism.
**Cross-domain wiring:** Foundation for **control-numerical-opt/closed-loop**, robotics, audio.
**Notes:** Ingo Molnar, Thomas Gleixner, Steven Rostedt (2004+); mainline merge complete 2024.

### hrtimer-high-resolution-timer (cross-domain alias: `hrtimer`, `ktime`)
**Domain:** Operating Systems
**Definition:** Nanosecond-resolution timer backed by per-CPU `ktime` source; supports HRTIMER_NORESTART/RESTART, multiple clock bases.
**Atom or composite:** Composite — RB-tree per CPU + per-base clock source.
**Cost model:** ~µs precision typical; ~100 ns scheduler wakeup.
**Real wall?** Yes — hardware timer resolution; TSC drift.
**Cross-domain wiring:** Same scheduling primitive as **control-numerical-opt/deadline-timer**.
**Notes:** Thomas Gleixner (2.6.16, 2006); replaces low-res jiffy timers.

### clock-gettime-realtime-monotonic (cross-domain alias: `clock_gettime`, `clock_id`)
**Domain:** Operating Systems
**Definition:** Family of clocks — REALTIME (wall, settable), MONOTONIC (forward-only since boot), MONOTONIC_RAW (no NTP slew), BOOTTIME (includes suspend), PROCESS/THREAD_CPUTIME.
**Atom or composite:** Composite — `clock_id` selector + vDSO fast read.
**Cost model:** vDSO call ~20 ns (no syscall); kernel fallback ~µs.
**Real wall?** Yes — REALTIME jumps backward on adjtime; use MONOTONIC for elapsed measurement.
**Cross-domain wiring:** Foundation for **distributed-systems/hybrid-logical-clock**, **statistics-probability/temporal-event**.
**Notes:** POSIX 1003.1b; vDSO since 2.6.22.

### clock-nanosleep-absolute (cross-domain alias: `nanosleep`, `TIMER_ABSTIME`)
**Domain:** Operating Systems
**Definition:** Sleep until absolute or relative `timespec`; with `TIMER_ABSTIME` avoids drift; pairs with hrtimer.
**Atom or composite:** Atom — single syscall.
**Cost model:** Wake-up latency ~µs–ms depending on PREEMPT_RT.
**Real wall?** Yes — interrupted sleeps return remainder; signal handling complicates loops.
**Cross-domain wiring:** Same time-driven dispatch as **control-numerical-opt/sample-loop**.
**Notes:** POSIX 1003.1b; cited as the only POSIX sleep API safe under NTP slew.

### ptp-precision-time-protocol (cross-domain alias: `ptp`, `ieee-1588`, `phc`)
**Domain:** Operating Systems
**Definition:** Linux PTP Hardware Clock subsystem — `/dev/ptpN` exposing NIC PHC; `linuxptp` synchronizes via IEEE 1588 to sub-µs.
**Atom or composite:** Composite — PHC + user daemon + cross-timestamp ioctl.
**Cost model:** Hardware timestamping eliminates kernel jitter; software fallback worse.
**Real wall?** Yes — switch transparency; cable asymmetry bounds accuracy.
**Cross-domain wiring:** Backbone of **distributed-systems/spanner-truetime**-style sync.
**Notes:** Richard Cochran (3.0, 2011); essential for finance, 5G, audio.

### isolcpus-nohz-full (cross-domain alias: `isolcpus`, `nohz_full`, `rcu_nocbs`)
**Domain:** Operating Systems
**Definition:** Boot params reserving CPUs from scheduler/timer/RCU for RT or DPDK; `isolcpus=` removes from balancing, `nohz_full=` disables timer tick when only one runnable task, `rcu_nocbs=` offloads RCU callbacks.
**Atom or composite:** Composite — cpuset + tick subsystem + RCU offload.
**Cost model:** Per-CPU dedication; "free" once configured.
**Real wall?** Yes — must combine all three for true tickless; remaining tick from `task_isolation` work.
**Cross-domain wiring:** Same CPU partitioning as **distributed-systems/dedicated-shard**.
**Notes:** Frederic Weisbecker NO_HZ_FULL (3.10+); CPU isolation v2 (cpuset) preferred over isolcpus.

### threaded-irq-rt-priority (cross-domain alias: `threaded-irq`, `request_threaded_irq`)
**Domain:** Operating Systems
**Definition:** IRQ handlers split into a hard-IRQ stub and a kthread doing the bulk work; under PREEMPT_RT every IRQ becomes threaded so RT tasks can preempt it.
**Atom or composite:** Composite — hard handler + irq_thread + rt priority.
**Cost model:** ~µs extra wakeup but bounded; enables priority inheritance over IRQ.
**Real wall?** Yes — some critical IRQs (NMI) cannot be threaded.
**Cross-domain wiring:** Same defer-to-kthread as workqueue but with rt prio.
**Notes:** Thomas Gleixner; mainlined incrementally; default for many drivers since 5.x.

### priority-ceiling-protocol-pcp (cross-domain alias: `priority-ceiling`, `ipcp`)
**Domain:** Operating Systems
**Definition:** Each resource has a ceiling = highest priority of any task that may lock it; an acquirer is raised to ceiling — bounds blocking to one critical section transitively.
**Atom or composite:** Composite — resource ceiling + per-task active priority.
**Cost model:** O(1) ceiling lookup; no chain walking.
**Real wall?** Yes — needs static analysis of who locks what.
**Cross-domain wiring:** Same RT-bound idea as **control-numerical-opt/wcet-analysis**.
**Notes:** Sha, Rajkumar, Lehoczky (1990); deadlock-free, single-blocking-bound; complement to PIP.

### wcet-worst-case-execution-time (cross-domain alias: `wcet`, `bound-blocking`)
**Domain:** Operating Systems
**Definition:** Upper bound on a code path's execution time including pipeline, cache, and DMA contention; required input to RT schedulability analysis.
**Atom or composite:** Composite — measured + static analysis + safety margin.
**Cost model:** Analysis is offline; runtime cost zero.
**Real wall?** Yes — modern CPUs (speculation, OoO) make tight WCETs nearly impossible.
**Cross-domain wiring:** Same upper-bound proof as **formal-verification/cost-bound**.
**Notes:** Foundation of rate-monotonic and EDF feasibility tests.

### sched-deadline-cbs-edf (cross-domain alias: `sched_deadline`, `setattr-deadline`)
**Domain:** Operating Systems
**Definition:** Linux SCHED_DEADLINE: EDF + CBS; `sched_setattr` with `(runtime, deadline, period)`; admission control via global utilization check.
**Atom or composite:** Composite — per-task triple + RB-tree by deadline + CBS budget.
**Cost model:** O(log N_dl) dispatch; admission O(N) approximation.
**Real wall?** Yes — admission rejects beyond UB ≤ 1 per CPU.
**Cross-domain wiring:** Same EDF as **queueing-theory/earliest-deadline-first**.
**Notes:** Lelli, Faggioli, Trimarchi (3.14, 2014); rich `chrt --deadline` interface.

### task-isolation-full-quiet (cross-domain alias: `task-isolation`, `quiescent-cpu`)
**Domain:** Operating Systems
**Definition:** Kernel feature ensuring a userspace task on a dedicated CPU receives no kernel interruptions (no IPIs, no timers, no scheduler tick).
**Atom or composite:** Composite — prctl + nohz_full + RCU offload + IPI tracker.
**Cost model:** Setup cost; "free" steady state.
**Real wall?** Yes — debug interruptions, ARM still has gtod refresh.
**Cross-domain wiring:** Same dedicated-core pattern as **networking/dpdk-pinned-poller**.
**Notes:** Christoph Lameter, Frederic Weisbecker; partially mainlined.

---

## Persistence & PMEM

### file-vfs-file-struct (cross-domain alias: `file*`, `open-file-handle`)
**Domain:** Operating Systems
**Definition:** Kernel struct representing an open file — pointer to inode, current offset (`f_pos`), flags, fops vtable; multiple `struct file` per inode possible.
**Atom or composite:** Composite — inode ref + position + fops + per-fd flags.
**Cost model:** ~256 B per open file; refcount-managed.
**Real wall?** Yes — `RLIMIT_NOFILE` per-process, `/proc/sys/fs/file-max` global.
**Cross-domain wiring:** Same handle-to-resource as **distributed-systems/session-handle**.
**Notes:** SVR4 file table descendant; per-fd vs shared offset matters for dup/fork.

### block-sector-512n-4kn (cross-domain alias: `lba`, `sector-size`, `512-emulation`)
**Domain:** Operating Systems
**Definition:** Smallest addressable disk unit — historic 512 B, modern 4 KB native; many SSDs emulate 512 over 4 KB internals.
**Atom or composite:** Atom — single LBA addressing unit.
**Cost model:** Misaligned 512-on-4K incurs read-modify-write on writes.
**Real wall?** Yes — silicon-level; partition table must align to 4 KB.
**Cross-domain wiring:** Same atomic-unit as **database-streaming-sketching/disk-page**.
**Notes:** Advanced Format (T10/T13, 2010); GPT and modern mkfs align by default.

### log-structured-write (cross-domain alias: `log-structured-fs`, `append-only-log`)
**Domain:** Operating Systems
**Definition:** Filesystem/storage design where all writes are sequential to a log; reads use index/CoW tree; GC compacts.
**Atom or composite:** Composite — log + index + GC.
**Cost model:** Sequential writes ideal for HDD/SSD; GC amplification can degrade.
**Real wall?** Yes — capacity overhead for GC; tail-write contention.
**Cross-domain wiring:** Same shape as **database-streaming-sketching/lsm-tree**, **distributed-systems/raft-log**.
**Notes:** Rosenblum & Ousterhout LFS (1992); F2FS, NILFS2, btrfs descendants.

### journal-write-ahead-wal (cross-domain alias: `wal`, `redo-log`, `journal-mode-data`)
**Domain:** Operating Systems
**Definition:** Append-only log of mutations applied before in-place changes; recovery replays log; FS journals (JBD2) and DB WAL share the discipline.
**Atom or composite:** Composite — log records + commit marker + checkpoint.
**Cost model:** 2× writes; group commit amortizes.
**Real wall?** Yes — fsync of log defines durability boundary.
**Cross-domain wiring:** Universal pattern across **database-streaming-sketching/wal**, **distributed-systems/raft-log**.
**Notes:** Earliest in System R (Gray, 1981); foundation of crash consistency.

### checkpoint-shadowing (cross-domain alias: `checkpoint`, `consistent-snapshot`)
**Domain:** Operating Systems
**Definition:** Periodic capture of a consistent on-disk image; WAL can then be truncated up to the checkpoint LSN.
**Atom or composite:** Composite — quiesce + flush + log truncate.
**Cost model:** Background but pauses writers briefly.
**Real wall?** Yes — checkpoint frequency vs recovery time trade-off.
**Cross-domain wiring:** Same recovery boundary as **database-streaming-sketching/checkpoint**.
**Notes:** ARIES (Mohan et al., 1992); F2FS checkpoint; ext4 commit interval.

### nvdimm-pmem-dax (cross-domain alias: `pmem`, `dax`, `fsdax`, `devdax`)
**Domain:** Operating Systems
**Definition:** Persistent memory exposed as byte-addressable; FS DAX maps file pages directly (no page cache), Dev DAX exposes raw region.
**Atom or composite:** Composite — pmem driver + fsdax mount option + MAP_SYNC.
**Cost model:** ~80 ns read (DRAM speed); persists across power-loss.
**Real wall?** Yes — Intel Optane DCPMM discontinued (2022); CXL pmem successor.
**Cross-domain wiring:** Same byte-addressable storage as **distributed-systems/persistent-shared-memory**.
**Notes:** Linux pmem driver, libpmem, libpmemobj (SNIA NVM Programming Model).

### map-sync-dax-msync (cross-domain alias: `MAP_SYNC`, `fsdax-flush`)
**Domain:** Operating Systems
**Definition:** `mmap` flag (with MAP_SHARED_VALIDATE) guaranteeing that page-fault-installed mappings are durable without subsequent `fsync`; requires fsdax + filesystem support.
**Atom or composite:** Atom — single mmap flag.
**Cost model:** Eliminates fsync syscall; persist via CLWB/CLFLUSHOPT + SFENCE.
**Real wall?** Yes — only available on fsdax mounts of ext4/xfs.
**Cross-domain wiring:** Same direct-durability as **database-streaming-sketching/persistent-mapped-file**.
**Notes:** Dan Williams (4.15, 2018); enables zero-syscall durability.

### zoned-storage-zns (cross-domain alias: `zoned-block-device`, `zns-ssd`, `host-managed`)
**Domain:** Operating Systems
**Definition:** Device divided into append-only zones; host must reset zone to overwrite; eliminates SSD internal GC.
**Atom or composite:** Composite — zone metadata + per-zone write pointer.
**Cost model:** Sequential writes only; resets are O(zone-size).
**Real wall?** Yes — application must enforce append-only.
**Cross-domain wiring:** Same append-only model as **database-streaming-sketching/segment-store**.
**Notes:** SMR HDDs first; NVMe ZNS spec (2020); F2FS, btrfs zoned support, zonefs.

### zfs-arc-l2arc-zil (cross-domain alias: `arc`, `l2arc-flash-cache`, `zil-sync-log`)
**Domain:** Operating Systems
**Definition:** ZFS caching hierarchy — ARC (RAM, ARC = adaptive replacement cache), L2ARC (flash victim cache), ZIL (synchronous-write intent log, can be on SLOG SSD).
**Atom or composite:** Composite — three tiered caches + persistence policy.
**Cost model:** ARC ~10 % of memory; L2ARC index in RAM too; ZIL only on `O_SYNC`.
**Real wall?** Yes — L2ARC index RAM cost famously dominates large flash caches.
**Cross-domain wiring:** Same tiered cache as **distributed-systems/multi-tier-storage**.
**Notes:** Megiddo & Modha ARC (2003); ZFS by Bonwick et al. (2005).

### btrfs-send-receive (cross-domain alias: `btrfs-send`, `incremental-snapshot-stream`)
**Domain:** Operating Systems
**Definition:** Btrfs facility producing a byte-stream representing the diff between two snapshots, replayable by `btrfs receive`; underlies replication.
**Atom or composite:** Composite — snapshot pair + change record stream.
**Cost model:** O(changed-extents) compute; transfer size = delta.
**Real wall?** Yes — must keep both snapshots; receive applies CoW-safe.
**Cross-domain wiring:** Same incremental replication as **distributed-systems/snapshot-shipping**, ZFS send/recv.
**Notes:** Alexander Block (3.6, 2012).

### dm-crypt-luks (cross-domain alias: `dm-crypt`, `luks2`, `cryptsetup`)
**Domain:** Operating Systems
**Definition:** Device-mapper target XTS-encrypting block I/O; LUKS metadata header stores anti-forensic split, key-derivation (Argon2id LUKS2), multiple keyslots.
**Atom or composite:** Composite — DM target + AEAD/XTS engine + LUKS header.
**Cost model:** ~1–2 GB/s per core with AES-NI; sector-aligned IO.
**Real wall?** Yes — XTS is per-sector; no integrity unless dm-integrity stacked.
**Cross-domain wiring:** Stack with **cryptography-hashing/argon2id**, **cryptography-hashing/aes-xts**.
**Notes:** Christophe Saout dm-crypt (2.6.4); LUKS by Clemens Fruhwirth.

### fscrypt-file-encryption (cross-domain alias: `fscrypt`, `ext4-encryption`)
**Domain:** Operating Systems
**Definition:** Per-file/directory encryption with master keys in keyring; ext4/f2fs/ubifs support; filenames encrypted too via SipHash-padded blocks.
**Atom or composite:** Composite — per-inode key + master-key keyring + filename encryption.
**Cost model:** AES-XTS for content, AES-CBC-CTS or HCTR2 for filenames.
**Real wall?** Yes — directory entries longer encrypted than plaintext.
**Cross-domain wiring:** Same per-record encryption as **database-streaming-sketching/transparent-data-encryption**.
**Notes:** Theodore Ts'o ext4 encryption (4.1, 2015); v2 with HCTR2 added 5.18.

---

## Observability & Tracing

### perf-perf_event_open (cross-domain alias: `perf`, `pmu-counter`, `perf-record`)
**Domain:** Operating Systems
**Definition:** Linux performance-counters subsystem — `perf_event_open` syscall opens HW/SW/tracepoint counters; `perf(1)` is the userspace driver.
**Atom or composite:** Composite — `perf_event_attr` + ring buffer + sampling logic.
**Cost model:** PMU read ~ns; sampling rate impacts overhead.
**Real wall?** Yes — PMU counter count (4–8 generic on x86); skid (sample lands few insns past trigger).
**Cross-domain wiring:** Same instrumentation surface as **ml-training/profiler-trace**.
**Notes:** Peter Zijlstra, Ingo Molnar (2.6.31, 2009); reference profiler.

### ftrace-function-graph (cross-domain alias: `ftrace`, `function-graph-tracer`)
**Domain:** Operating Systems
**Definition:** In-kernel function-call tracer using mcount/`-pg` insertion or static_call; outputs to per-CPU ring buffers via `/sys/kernel/tracing`.
**Atom or composite:** Composite — instrumented function entries + ring buffer + parser.
**Cost model:** ~50 ns per traced call; off by default.
**Real wall?** Yes — buffer size bounds retention; per-CPU storage.
**Cross-domain wiring:** Same function tracing as **distributed-systems/distributed-tracing-span**.
**Notes:** Steven Rostedt (2.6.27, 2008); `set_ftrace_filter` for selective tracing.

### kprobes-uprobes (cross-domain alias: `kprobe`, `uprobe`, `dynamic-tracing`)
**Domain:** Operating Systems
**Definition:** Dynamic instrumentation — patch a kernel (kprobe) or userspace (uprobe) instruction with INT3/breakpoint; on hit, invoke handler then single-step.
**Atom or composite:** Composite — probe + handler + per-CPU trampoline.
**Cost model:** ~µs per hit; many concurrent kprobes use optimized variants.
**Real wall?** Yes — cannot kprobe certain core code (notify_die path); uprobe requires copy-on-write.
**Cross-domain wiring:** Generic instrumentation foundation; cf. **distributed-systems/agent-injection**.
**Notes:** IBM Linux Tech Center (2.6.9 kprobe); uprobe by Srikar Dronamraju (3.5).

### tracepoint-static (cross-domain alias: `tracepoint`, `TRACE_EVENT`)
**Domain:** Operating Systems
**Definition:** Source-level instrumentation macro emitting a hook at compile time; near-zero cost when off (jump-label-controlled), structured fields when on.
**Atom or composite:** Composite — declaration + jump-label patchpoint + binary format.
**Cost model:** Zero when disabled; ~50 ns when enabled.
**Real wall?** Yes — must be declared up front (vs probes); ABI-stable across versions.
**Cross-domain wiring:** Same compile-time hooks as **database-streaming-sketching/instrumented-operator**.
**Notes:** Mathieu Desnoyers (2.6.28, 2008); ~1500 tracepoints in current kernel.

### ebpf-program (cross-domain alias: `ebpf`, `bpf-program`, `verifier`)
**Domain:** Operating Systems
**Definition:** Restricted VM with 11 64-bit registers, 512-byte stack, helper functions, and a verifier proving termination + safety; JIT-compiled to native.
**Atom or composite:** Composite — bytecode + verifier + JIT + helpers + maps.
**Cost model:** JITed runs at near-native speed; verification offline.
**Real wall?** Yes — verifier limits (1 M insns since 5.2); helper allowlist per prog type.
**Cross-domain wiring:** Universal kernel-side compute fabric; bridge to **networking/xdp**, **ml-training/in-kernel-sampler**.
**Notes:** Alexei Starovoitov (3.18, 2014); foundation of Cilium, Falco, Pixie, Katran.

### bpf-map (cross-domain alias: `bpf-map`, `hashmap`, `array-map`, `ringbuf-map`)
**Domain:** Operating Systems
**Definition:** Kernel data structure shared between BPF programs and userspace; types: hash, array, percpu, lpm-trie, ringbuf, perf event array, stack, queue, hashmap-of-maps, sockmap.
**Atom or composite:** Composite — type + key/value sizes + max entries + flags.
**Cost model:** O(1) hash; O(k) LPM; ringbuf reservation atomic.
**Real wall?** Yes — kernel memory (memlock); max_entries.
**Cross-domain wiring:** Same kernel↔user shared structures as **distributed-systems/shared-state**.
**Notes:** Alexei Starovoitov; ringbuf (5.8) by Andrii Nakryiko.

### bpf-prog-types (cross-domain alias: `bpf-attach-type`, `xdp-kprobe-cgroup-skb`)
**Domain:** Operating Systems
**Definition:** ~30 BPF attach points — XDP, sched_cls, kprobe, uprobe, tracepoint, perf_event, cgroup_skb/sock/sock_addr, sk_msg, sk_skb, sock_ops, lsm, fentry/fexit, struct_ops, freplace, lirc.
**Atom or composite:** Composite — N hooks, each with prog type and helper set.
**Cost model:** Each type has tailored helpers/context; verifier rules differ.
**Real wall?** Yes — helpers restricted per type to maintain safety.
**Cross-domain wiring:** Same composable attach as plugin architecture.
**Notes:** Expanded continuously since 3.18; struct_ops (5.6) lets BPF implement kernel vtables.

### bpf-trampoline-fentry-fexit (cross-domain alias: `bpf-trampoline`, `fentry`, `bpf-lsm`)
**Domain:** Operating Systems
**Definition:** Ftrace-based trampoline replacing function entry/exit with BPF dispatch; lower overhead than kprobes, used by fentry/fexit/lsm BPF.
**Atom or composite:** Composite — trampoline page + BPF prog + return handler.
**Cost model:** ~10 ns per call (vs ~µs for kprobe).
**Real wall?** Yes — function must be BTF-attachable; not all are.
**Cross-domain wiring:** Same lightweight-instrumentation as **distributed-systems/lazy-attach-tracer**.
**Notes:** Alexei Starovoitov (5.5, 2020); foundation of BPF-LSM and Tetragon.

### bpf-co-re-vmlinux-btf (cross-domain alias: `co-re`, `compile-once-run-everywhere`, `btf`)
**Domain:** Operating Systems
**Definition:** Mechanism letting a single BPF binary work across kernel versions — BTF (BPF Type Format) gives field offsets at load time, libbpf relocates.
**Atom or composite:** Composite — BTF blob + libbpf relocation engine.
**Cost model:** Load-time relocation; runtime identical.
**Real wall?** Yes — kernel must export BTF (default since 5.5); pre-5.x needs BTFHub.
**Cross-domain wiring:** Same "portable across runtimes" as JVM bytecode.
**Notes:** Andrii Nakryiko (2019); transformed eBPF deployment story.

### usdt-userspace-probe (cross-domain alias: `usdt`, `dtrace-user-probe`)
**Domain:** Operating Systems
**Definition:** Userspace statically-defined probe — DTRACE_PROBE macro emits an ELF note marking a NOP that can be patched by uprobe.
**Atom or composite:** Composite — ELF note + NOP + uprobe attachment.
**Cost model:** Zero when not attached; uprobe cost when attached.
**Real wall?** No (just NOP); requires recompilation with `sys/sdt.h`.
**Cross-domain wiring:** Same compile-time hook as kernel tracepoints; used by JVM, PostgreSQL, libc.
**Notes:** Originated DTrace (Solaris, 2003); ported to Linux uprobes.

### lttng-userspace-tracer (cross-domain alias: `lttng`, `ust`)
**Domain:** Operating Systems
**Definition:** Userspace tracer with shared-memory ring buffers and zero-copy from app to per-CPU buffers; works alongside kernel LTTng.
**Atom or composite:** Composite — per-CPU buffer + sessiond + relay/consumer.
**Cost model:** ~100 ns per event; designed for production tracing.
**Real wall?** Yes — buffer size, consumer keeping up.
**Cross-domain wiring:** Same trace pipeline as **distributed-systems/structured-logging**.
**Notes:** Mathieu Desnoyers (Polytechnique Montréal); CTF (Common Trace Format) output.

### blktrace-block-tracer (cross-domain alias: `blktrace`, `blk-mq-trace`)
**Domain:** Operating Systems
**Definition:** Per-bio event stream — issue, completion, merge, requeue; visualized via btt/iowatcher.
**Atom or composite:** Composite — kernel tracepoints + relay channel + userspace tool.
**Cost model:** ~µs per event; buffer overflow loses data.
**Real wall?** Yes — IO-bound tracing; relay buffer sizing.
**Cross-domain wiring:** Same per-request trace as **database-streaming-sketching/query-trace**.
**Notes:** Jens Axboe (2.6.16); replaced by bpftrace recipes for many uses.

### proc-pid-task-stats (cross-domain alias: `/proc`, `pidstat`, `taskstats`)
**Domain:** Operating Systems
**Definition:** Per-process counters exposed under `/proc/<pid>` plus binary `taskstats` netlink interface (delay accounting, IO accounting).
**Atom or composite:** Composite — kernel per-task accumulators + on-demand serialization.
**Cost model:** Read cost ~µs per file.
**Real wall?** Yes — /proc format unstable; taskstats netlink-bound.
**Cross-domain wiring:** Same metric-export pattern as **distributed-systems/prometheus-text-exposition**.
**Notes:** Plan 9 origin; Linux taskstats by Jay Lan (2.6.18).

### psi-pressure-stall-information (cross-domain alias: `psi`, `cpu-mem-io-pressure`)
**Domain:** Operating Systems
**Definition:** `/proc/pressure/{cpu,memory,io}` exposing percent of time tasks stalled on each resource (some/full); ewma-smoothed.
**Atom or composite:** Composite — stall accounting + ewma + per-cgroup.
**Cost model:** Hot-path accounting on task state transitions ~ns.
**Real wall?** No (observation only).
**Cross-domain wiring:** Same congestion signal as **queueing-theory/utilization-vs-queue-length**.
**Notes:** Johannes Weiner Meta (4.20, 2019); used by systemd-oomd, k8s SchedulingFramework.

### kallsyms-kcore-stack-unwinder (cross-domain alias: `kallsyms`, `orc-unwinder`, `frame-pointer`)
**Domain:** Operating Systems
**Definition:** `/proc/kallsyms` exports kernel symbol table; `/proc/kcore` exposes kernel memory; ORC (5.x) is Linux's compact stack unwinder.
**Atom or composite:** Composite — symbol table + section addresses + ORC unwind tables.
**Cost model:** Sort by addr O(log N) lookup; ORC ~ns per frame.
**Real wall?** Yes — `kptr_restrict` hides symbols from non-root.
**Cross-domain wiring:** Same symbolication backbone as **ml-training/sample-stack-trace**.
**Notes:** ORC by Josh Poimboeuf (4.14, 2017); replaced DWARF for kernel unwinding.

---

## Boot & Init

### uefi-firmware-runtime (cross-domain alias: `uefi`, `efi-rs`, `secure-boot`)
**Domain:** Operating Systems
**Definition:** Firmware spec for boot — boot services (memory, image load), runtime services (variable, time, capsule), Secure Boot key DB (PK, KEK, db, dbx).
**Atom or composite:** Composite — protocols + variables + boot loader chain.
**Cost model:** Boot services free up to ExitBootServices; runtime services callable at kernel time.
**Real wall?** Yes — firmware bugs are physical reality; capsule updates require reboot.
**Cross-domain wiring:** Root of **cryptography-hashing/secure-boot-chain**.
**Notes:** Intel EFI (1998) → UEFI Forum; required for modern x86 PCs.

### grub-systemd-boot-bootloader (cross-domain alias: `grub2`, `systemd-boot`, `syslinux`)
**Domain:** Operating Systems
**Definition:** Second-stage loader presenting menus, loading kernel + initramfs, passing command-line; UEFI-aware variants are EFI applications.
**Atom or composite:** Composite — config parser + filesystem drivers + boot protocol.
**Cost model:** ~1 s boot menu + load.
**Real wall?** Yes — must support each FS at boot time; signed binaries for Secure Boot.
**Cross-domain wiring:** Same staged-loader pattern as **distributed-systems/multi-stage-init**.
**Notes:** GRUB by Erich Boleyn (1999); GRUB 2 rewrite; systemd-boot (gummiboot) for UEFI-only.

### initramfs-initrd (cross-domain alias: `initrd`, `initramfs`, `dracut`)
**Domain:** Operating Systems
**Definition:** Compressed cpio archive unpacked into rootfs early; runs `/init` to mount the real root; dracut/mkinitcpio build it.
**Atom or composite:** Composite — cpio + early userspace + udev rules.
**Cost model:** ~100 MB compressed typically; unpack + execve add seconds.
**Real wall?** Yes — must contain modules for storage controllers, encryption, network.
**Cross-domain wiring:** Same minimal-root pattern as **distributed-systems/bootstrap-image**.
**Notes:** Linus initramfs (2.5.45, 2002); replaces initrd ramdisk.

### acpi-aml-tables (cross-domain alias: `acpi`, `dsdt`, `aml-vm`)
**Domain:** Operating Systems
**Definition:** ACPI tables (DSDT, SSDT) carry AML bytecode interpreted by kernel `acpica` for power and platform control; events delivered via SCI.
**Atom or composite:** Composite — table set + interpreter + GPE/SCI delivery.
**Cost model:** AML calls can be slow (ms); kernel mitigates via caching.
**Real wall?** Yes — buggy AML = kernel hangs; common BIOS-bug source.
**Cross-domain wiring:** Platform-side coordination layer; integration with **distributed-systems/power-policy**.
**Notes:** Intel ACPICA (open-sourced 1999); cross-OS interpreter.

### kernel-module-modprobe (cross-domain alias: `kmod`, `lsmod`, `module-signing`)
**Domain:** Operating Systems
**Definition:** Dynamically loaded ELF objects extending the kernel; `modprobe` resolves dependencies via `modules.dep`; signing via X.509 enforced under lockdown.
**Atom or composite:** Composite — ELF + module init/exit + symbol exports.
**Cost model:** Load ~ms; some modules have heavy probe routines.
**Real wall?** Yes — module signing under lockdown; out-of-tree modules taint kernel.
**Cross-domain wiring:** Same plug-in pattern as **distributed-systems/plugin-loader**.
**Notes:** Linus modules 0.99 (1992); kmod by Lucas De Marchi modernized userspace.

### systemd-pid1-init (cross-domain alias: `systemd`, `unit-files`, `socket-activation`)
**Domain:** Operating Systems
**Definition:** Modern Linux init system — declarative unit files (service, socket, mount, timer, slice), socket activation, cgroup-tracked services.
**Atom or composite:** Composite — pid 1 + unit graph + dbus interface + journald.
**Cost model:** Boot time often <2 s with parallel start.
**Real wall?** Yes — high blast radius (single dependency graph); contested design.
**Cross-domain wiring:** Same orchestration pattern as **distributed-systems/local-service-manager**.
**Notes:** Lennart Poettering, Kay Sievers (2010); de facto default across major distros.

### launchd-macos (cross-domain alias: `launchd`, `plist-launch-agents`)
**Domain:** Operating Systems
**Definition:** macOS pid-1 daemon — XPC services, launch agents/daemons, socket activation; .plist describes triggers.
**Atom or composite:** Composite — pid 1 + per-user agents + Mach-port routed XPC.
**Cost model:** Lazy launch on first request; ~ms overhead.
**Real wall?** Yes — macOS-only; integrates tightly with Mach.
**Cross-domain wiring:** Conceptual ancestor of systemd's socket activation.
**Notes:** Dave Zarzycki Apple (Tiger 10.4, 2005).

### efi-runtime-services-rtc (cross-domain alias: `efi-rs`, `set-virtual-address-map`)
**Domain:** Operating Systems
**Definition:** Subset of UEFI services available after `ExitBootServices` — variable get/set, time get/set, ResetSystem, capsule update.
**Atom or composite:** Composite — fixed-virtual-address mapping + SetVirtualAddressMap.
**Cost model:** Each call traps into firmware; may take ms.
**Real wall?** Yes — firmware bugs serialize calls or hang; Linux uses workqueue.
**Cross-domain wiring:** Same firmware-callback substrate as **distributed-systems/bmc-redfish**.
**Notes:** Linux EFI runtime by Matt Fleming, Ard Biesheuvel.

### kexec-fast-reboot (cross-domain alias: `kexec`, `kdump`)
**Domain:** Operating Systems
**Definition:** Syscall loading a new kernel image into reserved memory and jumping into it, bypassing firmware; underpins kdump (boot a crash kernel from a dying one).
**Atom or composite:** Composite — kimage + segments + purgatory.
**Cost model:** Skips firmware ~5–60 s of POST.
**Real wall?** Yes — devices must quiesce; new kernel re-init from scratch.
**Cross-domain wiring:** Same live-handoff as **distributed-systems/live-upgrade**.
**Notes:** Eric Biederman (2.6.13, 2005); kdump exclusive of Secure Boot until 5.x.

### suspend-hibernate-s0-s3-s4 (cross-domain alias: `suspend-to-ram`, `hibernate-to-disk`, `s0ix`)
**Domain:** Operating Systems
**Definition:** ACPI sleep states — S0 (working), modern S0ix (low power idle), S3 (suspend to RAM), S4 (hibernate to disk), S5 (off).
**Atom or composite:** Composite — driver `pm_ops` (prepare/suspend/resume) + ACPI state transition.
**Cost model:** Suspend ~seconds; resume ~seconds.
**Real wall?** Yes — every driver must implement quiesce/restore; flaky drivers cause "won't wake" bugs.
**Cross-domain wiring:** Same checkpoint/restore family as CRIU; **distributed-systems/quiescent-restart**.
**Notes:** Rafael Wysocki maintainer; S0ix replaces S3 on modern Intel laptops.

---

## Power & Thermal

### cpufreq-governor-schedutil (cross-domain alias: `cpufreq`, `schedutil`, `intel_pstate`)
**Domain:** Operating Systems
**Definition:** Per-CPU P-state selector — governors: performance, powersave, ondemand, conservative, schedutil (scheduler-coupled), userspace; intel_pstate is its own driver.
**Atom or composite:** Composite — utilization signal + frequency table + transition latency.
**Cost model:** Frequency transition ~µs to ms; governor decision ~ms.
**Real wall?** Yes — silicon-defined P-states; HWP (hardware P-state, Skylake+) bypasses governor.
**Cross-domain wiring:** Same control loop as **control-numerical-opt/dvfs-controller**.
**Notes:** schedutil by Rafael Wysocki, Steven Rostedt (4.7, 2016) — first governor in scheduler.

### cpuidle-c-states (cross-domain alias: `cpuidle`, `c-state`, `mwait`)
**Domain:** Operating Systems
**Definition:** Per-CPU idle-state selector choosing C-state (C0 active, C1, C1E, C2, C6, C7, …) on idle; each saves more power, costs more wake latency.
**Atom or composite:** Composite — predictor + state table + governor.
**Cost model:** Deeper C-states wake in µs–ms; predictor (TEO, menu) chooses.
**Real wall?** Yes — silicon-defined C-states; deep states flush L1/L2.
**Cross-domain wiring:** Same sleep-state decision as **distributed-systems/quiescent-cache**.
**Notes:** Venki Pallipadi (2.6.21); TEO governor (5.0) by Rafael Wysocki.

### thermal-zone-trip-point (cross-domain alias: `thermal-zone`, `cooling-device`)
**Domain:** Operating Systems
**Definition:** Kernel framework with zones (sensors) and cooling devices (fans, frequency, idle injection); trip points trigger policies.
**Atom or composite:** Composite — zone + sensor + cooling device + governor (step_wise, user_space, power_allocator).
**Cost model:** Periodic polling ~ms; throttling reduces clock.
**Real wall?** Yes — physical TDP, fan RPM.
**Cross-domain wiring:** Same set-point control as **control-numerical-opt/pid-controller**, **electromagnetics-antennas/heat-flux**.
**Notes:** Zhang Rui Intel (3.0+); IPA governor (3.18) uses power model.

### rapl-running-average-power-limit (cross-domain alias: `rapl`, `intel-rapl`)
**Domain:** Operating Systems
**Definition:** Intel/AMD MSR-based interface reporting and limiting CPU/DRAM package power; energy counters in µJ.
**Atom or composite:** Composite — MSRs (POWER_LIMIT, ENERGY_STATUS) + powercap framework.
**Cost model:** MSR read ~ns; sampling at ms cadence.
**Real wall?** Yes — silicon power limits; thermal headroom.
**Cross-domain wiring:** Same energy meter as **ml-training/wattmeter**, **information-theory-coding/landauer**.
**Notes:** Intel SDM Vol 3; powercap framework by Srinivas Pandruvada.

### runtime-pm-autosuspend (cross-domain alias: `runtime-pm`, `dev_pm_runtime`)
**Domain:** Operating Systems
**Definition:** Per-device autosuspend framework — driver marks idle via `pm_runtime_put`, kernel suspends after timeout, resumes on next access.
**Atom or composite:** Composite — usage counter + delay + suspend/resume callbacks.
**Cost model:** Suspend/resume per-device ms; saves mW–W idle.
**Real wall?** Yes — wakeup latency for sub-systems; PCIe ASPM interactions.
**Cross-domain wiring:** Same idle-then-wake as **distributed-systems/cold-start**.
**Notes:** Rafael Wysocki (2.6.32); kbluetooth, usb, audio benefit.

### powercap-framework (cross-domain alias: `powercap`, `domain-policy`)
**Domain:** Operating Systems
**Definition:** Generic sysfs framework for power-capping domains (RAPL, dtpm, idle injection); exposes `power_limit_uw`, `energy_uj`.
**Atom or composite:** Composite — domain + constraint + governor.
**Cost model:** Sample at ms cadence.
**Real wall?** Yes — physical limits; over-aggressive throttle reduces throughput.
**Cross-domain wiring:** Same cap mechanism as **distributed-systems/budget-controller**.
**Notes:** Jacob Pan Intel (3.13, 2014); foundation of Intel SGX RAPL bypass research.

### intel-pstate-driver (cross-domain alias: `intel_pstate`, `hwp`)
**Domain:** Operating Systems
**Definition:** Intel-specific cpufreq driver that consults HWP autonomous controller (Skylake+); governor "powersave" and "performance" map to HWP hints.
**Atom or composite:** Composite — HWP MSR control + load-tracking + epp (energy perf preference).
**Cost model:** Hardware does the actual P-state hunting; software only sets hints.
**Real wall?** Yes — HWP is silicon; OS can only suggest.
**Cross-domain wiring:** Same offloaded-control as **distributed-systems/policy-pushdown**.
**Notes:** Dirk Brandewie Intel (3.9, 2013); HWP since 4.2.

### dtpm-dynamic-thermal-power (cross-domain alias: `dtpm`, `dynamic-thermal-pm`)
**Domain:** Operating Systems
**Definition:** Hierarchical power-budget tree where parent limit is split among children; integrates with cpufreq and devfreq.
**Atom or composite:** Composite — DTPM nodes + powercap interface.
**Cost model:** Tree rebalance on update; near-zero steady state.
**Real wall?** Yes — depends on accurate power-cost estimation per device.
**Cross-domain wiring:** Same hierarchical resource model as **distributed-systems/hierarchical-quota**.
**Notes:** Daniel Lezcano (5.14, 2021); foundation of arm energy-aware policy.

---

## OS Architecture & Virtualization

### monolithic-kernel (cross-domain alias: `monolithic`, `unified-address-space`)
**Domain:** Operating Systems
**Definition:** All OS subsystems (scheduler, VFS, network stack, drivers) share one kernel address space; calls between them are function calls.
**Atom or composite:** Composite — kernel image + modules sharing global address space.
**Cost model:** ~ns inter-subsystem calls; large attack surface.
**Real wall?** Yes — single bug in any module is kernel-fatal.
**Cross-domain wiring:** Architecturally opposite of **distributed-systems/microservices**.
**Notes:** Linux, FreeBSD, Solaris, AIX; vs. microkernel debate (Tanenbaum-Torvalds, 1992).

### microkernel-l4-sel4 (cross-domain alias: `microkernel`, `l4-family`)
**Domain:** Operating Systems
**Definition:** Tiny kernel (5–10 K LoC) providing only address spaces, threads, IPC, capabilities; drivers and FS run in userspace.
**Atom or composite:** Composite — IPC + address-space + thread + scheduler primitives.
**Cost model:** IPC ~200 ns (modern L4); each cross-process call adds context switch.
**Real wall?** Yes — IPC speed historically the gating factor for microkernels.
**Cross-domain wiring:** Reference design for **formal-verification/seL4-proof**.
**Notes:** Liedtke L4 (1993); seL4 (2009) first formally verified.

### exokernel-libos (cross-domain alias: `exokernel`, `library-os`)
**Domain:** Operating Systems
**Definition:** Kernel exposes raw hardware resources via capabilities; applications link a "library OS" implementing abstractions; MIT XOK, Nemesis, Aegis.
**Atom or composite:** Composite — protection-only kernel + per-app libOS.
**Cost model:** Eliminates abstraction tax; security relies on download-and-verify of code.
**Real wall?** Yes — every app must reimplement OS services unless library-shared.
**Cross-domain wiring:** Same hardware-as-API idea as **networking/dpdk**, **distributed-systems/unikernel**.
**Notes:** Engler, Kaashoek MIT (1995); reborn as unikernels.

### unikernel-mirage-includeos (cross-domain alias: `unikernel`, `library-os-vm`)
**Domain:** Operating Systems
**Definition:** Single-address-space single-process VM combining app + libOS in one binary; boots in ms; deploys as VM.
**Atom or composite:** Composite — type-safe runtime + bare-metal drivers + app.
**Cost model:** ~10–100 ms boot; minimal RAM footprint.
**Real wall?** Yes — debugging is harder; ecosystem narrow.
**Cross-domain wiring:** Same single-binary VM as **distributed-systems/serverless-snapshot**.
**Notes:** MirageOS (OCaml, Madhavapeddy et al., 2010); IncludeOS (C++), Hermitcore.

### kvm-hypervisor (cross-domain alias: `kvm`, `vmx-svm`, `vcpu-ioctl`)
**Domain:** Operating Systems
**Definition:** Linux kernel module turning the host kernel into a Type-2 hypervisor using VT-x/SVM; user QEMU drives via `/dev/kvm` ioctls.
**Atom or composite:** Composite — KVM module + VMCS/VMCB + EPT/NPT + irqchip.
**Cost model:** VM exits ~1–5 µs; hot exits hardware-eliminated (vAPIC, posted IRQ).
**Real wall?** Yes — VM-exit reasons defined by CPU; nested EPT cost.
**Cross-domain wiring:** Hardware-rooted virtualization parallel to **distributed-systems/sandbox-vm**.
**Notes:** Avi Kivity Qumranet (2.6.20, 2007); now Red Hat / IBM driven.

### xen-paravirt (cross-domain alias: `xen`, `pv`, `dom0`)
**Domain:** Operating Systems
**Definition:** Type-1 hypervisor running directly on hardware; Dom0 manages, DomU guests run paravirtual or HVM; predates HW virtualization.
**Atom or composite:** Composite — hypervisor + Dom0 + per-domain memory.
**Cost model:** PV hypercalls ~µs; HVM uses VT-x.
**Real wall?** Yes — Dom0 SPoF; PV requires guest cooperation.
**Cross-domain wiring:** Same hypervisor model as ESXi, Hyper-V; powers AWS EC2 (XenServer→Nitro).
**Notes:** Barham et al. Cambridge (SOSP 2003).

### hyper-v-nt-hypervisor (cross-domain alias: `hyper-v`, `wsl2-vm`)
**Domain:** Operating Systems
**Definition:** Microsoft Type-1 hypervisor — parent partition (Windows), child partitions; enlightenments via VMBus; backs WSL2, Hyper-V VM, Hyper-V Server.
**Atom or composite:** Composite — hvix64.exe + VMBus + synthetic devices.
**Cost model:** VMBus IPC ~µs; full HVM exits via hardware.
**Real wall?** Yes — Windows-licensed; integrates with VBS (Virtualization-Based Security).
**Cross-domain wiring:** Same Type-1 model as Xen; foundational for VBS-protected code integrity.
**Notes:** Win Server 2008; default hypervisor on Win 11 (HVCI).

### virtio-vhost-user-vsock (cross-domain alias: `virtio-paravirt`, `vsock`)
**Domain:** Operating Systems
**Definition:** Paravirtualized device family (net, blk, scsi, fs, gpu, crypto, balloon, console, vsock); host/guest share queues for low overhead.
**Atom or composite:** Composite — virtqueues + feature bits + transport (PCI, MMIO, channel).
**Cost model:** ~µs per request with notification batching.
**Real wall?** Yes — split between host driver and VMM (vhost-user moves to userspace).
**Cross-domain wiring:** Same shared-ring abstraction as **networking/dpdk-vhost-user**.
**Notes:** virtio 1.0 (OASIS, 2016); vsock (4.8) provides host-guest socket.

### nested-virtualization (cross-domain alias: `nested-virt`, `l1-l2`)
**Domain:** Operating Systems
**Definition:** L1 hypervisor running inside L0 hypervisor's guest; Intel VMCS shadowing, AMD nested NPT enable.
**Atom or composite:** Composite — shadowed control structures + nested page tables.
**Cost model:** L2 exit cost ~10× L1 exit; mitigated by hardware (Intel VMCS shadowing).
**Real wall?** Yes — silicon assists, but density and bug surface grow.
**Cross-domain wiring:** Same recursive isolation as **distributed-systems/nested-sandbox**.
**Notes:** Linux KVM nested (3.1, 2011); essential for cloud-in-cloud and CI.

### dpdk-userspace-stack (cross-domain alias: `dpdk`, `pmd-poll-mode`)
**Domain:** Operating Systems
**Definition:** Userspace networking framework bypassing kernel via VFIO + uio + huge pages + poll-mode drivers; lock-free hashes and rings.
**Atom or composite:** Composite — EAL + PMD + rte_ring + rte_hash.
**Cost model:** ~80 ns per packet at line rate; 100 Gbps single-core feasible.
**Real wall?** Yes — dedicated cores; complete bypass of kernel features.
**Cross-domain wiring:** Same kernel-bypass class as **networking/spdk-storage-bypass**, **distributed-systems/rdma**.
**Notes:** Intel (2010); now LF project; used by OVS-DPDK, Vector Packet Processor.

### spdk-userspace-nvme (cross-domain alias: `spdk`, `userspace-nvme-driver`)
**Domain:** Operating Systems
**Definition:** Storage-side counterpart of DPDK — userspace NVMe driver, NVMe-oF target, lightweight async I/O framework.
**Atom or composite:** Composite — uio/vfio-pci + libc-free libraries + reactor threads.
**Cost model:** ~µs per IO at 10 M IOPS scale.
**Real wall?** Yes — dedicated cores; bypasses kernel filesystem.
**Cross-domain wiring:** Same kernel-bypass as DPDK; pairs in NVMe-oF setups.
**Notes:** Intel (2015); now under SNIA/LF.

### rdma-verbs-qp (cross-domain alias: `rdma`, `ibv-verbs`, `queue-pair`)
**Domain:** Operating Systems
**Definition:** OS-bypass network — userspace posts WRs to QP, NIC DMAs directly to/from registered memory; SEND/RECV/WRITE/READ verbs.
**Atom or composite:** Composite — QP + MR (memory region) + CQ + PD.
**Cost model:** ~1 µs RDMA RTT; CPU bypass.
**Real wall?** Yes — pinned memory + IB/RoCE NIC; congestion control still maturing.
**Cross-domain wiring:** Same OS-bypass as **networking/iwarp**, **distributed-systems/rdma-rpc** (eRPC, FaRM).
**Notes:** InfiniBand verbs (Mellanox, 2002); Linux rdma-core.

### vfio-userspace-driver (cross-domain alias: `vfio`, `pci-passthrough`)
**Domain:** Operating Systems
**Definition:** Framework letting userspace own PCIe devices safely via IOMMU isolation; foundation of GPU/NIC passthrough and DPDK/SPDK.
**Atom or composite:** Composite — group + container + IOMMU domain.
**Cost model:** Userspace I/O = kernel speed minus context switches.
**Real wall?** Yes — IOMMU group atomicity (all-or-nothing).
**Cross-domain wiring:** Bridge between kernel isolation and **distributed-systems/zero-trust-device**.
**Notes:** Alex Williamson Red Hat (3.6, 2012).

### sgx-enclave (cross-domain alias: `sgx`, `enclave-ecall`, `epc`)
**Domain:** Operating Systems
**Definition:** Intel SGX creates an encrypted enclave in the Enclave Page Cache (EPC); ECALL transitions into untrusted entry, ENCLU instructions inside.
**Atom or composite:** Composite — EPC + ECREATE/EADD/EEXTEND/EINIT lifecycle + attestation.
**Cost model:** Enclave entry ~µs; memory-encryption ~5 % BW.
**Real wall?** Yes — EPC size (~256 MB), discontinued in 12th-gen consumer CPUs.
**Cross-domain wiring:** Foundation of **cryptography-advanced/remote-attestation**, Open Enclave SDK.
**Notes:** Skylake (2015); Linux SGX driver mainlined 5.11 (2021).

### tdx-trust-domain (cross-domain alias: `tdx`, `intel-tdx`, `trust-domain-extension`)
**Domain:** Operating Systems
**Definition:** Intel CPU feature creating a confidential VM whose memory is encrypted with per-VM keys and verified via integrity tree; OS cannot read guest memory.
**Atom or composite:** Composite — TD module + private-memory + MIGTD migration agent.
**Cost model:** ~5–10 % BW overhead; SEAM enter/exit per hypercall.
**Real wall?** Yes — Sapphire Rapids+ silicon; SEAM verification.
**Cross-domain wiring:** Same confidential VM as AMD SEV-SNP, ARM CCA.
**Notes:** Intel (2020 announcement, Sapphire Rapids 2023); KVM TDX support 6.x.

### sev-snp-confidential-vm (cross-domain alias: `sev-snp`, `amd-sev`, `vmsa-encryption`)
**Domain:** Operating Systems
**Definition:** AMD SEV-SNP — per-VM memory encryption + Reverse Map Table (RMP) integrity; VM measures itself via attestation report.
**Atom or composite:** Composite — RMP + VMPLs + AEAD encryption.
**Cost model:** ~5 % BW overhead; VMSA save/restore on exit.
**Real wall?** Yes — silicon-bound; firmware-mediated attestation.
**Cross-domain wiring:** Same Confidential Compute pillar as TDX, CCA.
**Notes:** AMD EPYC Milan (2021); Linux KVM/QEMU support 5.19+.

### arm-cca-realm (cross-domain alias: `arm-cca`, `realm-management-monitor`)
**Domain:** Operating Systems
**Definition:** ARM Confidential Compute Architecture — Realm world (orthogonal to Secure/Normal), Realm Management Monitor mediates attestation and memory.
**Atom or composite:** Composite — RMM + Granule Protection Tables.
**Cost model:** Hypercalls (RMI/RSI) take µs.
**Real wall?** Yes — ARMv9.2+ silicon, Neoverse RMM firmware.
**Cross-domain wiring:** Same confidential-VM family as SEV/TDX.
**Notes:** ARM v9 (announced 2021); Linux kernel guest 6.7+, host 6.10+.

---

## Research-grade & Frontier Primitives

### ghost-userspace-scheduler (cross-domain alias: `ghost-google`, `sched-bpf-userspace`)
**Domain:** Operating Systems
**Definition:** Google scheduler-offload framework — kernel exposes per-task callbacks via shared region/eBPF, userspace process makes dispatch decisions.
**Atom or composite:** Composite — kernel transport + userspace policy + fallback.
**Cost model:** Adds ~µs to wakeup vs in-kernel; flexibility wins.
**Real wall?** Yes — userspace must keep up; kernel preempts on timeout.
**Cross-domain wiring:** Same userspace-policy escape as eBPF struct_ops.
**Notes:** Humphries, Kaffes, Mazières, Kozyrakis (SOSP 2021).

### sched-ext-linux-pluggable (cross-domain alias: `sched_ext`, `bpf-scheduler-class`)
**Domain:** Operating Systems
**Definition:** Linux 6.12+ pluggable scheduler class implemented as a BPF struct_ops; user attaches a BPF program implementing the scheduler vtable.
**Atom or composite:** Composite — BPF struct_ops + kernel hooks + safety fallback.
**Cost model:** ~10–20 % overhead vs native CFS; flexibility unmatched.
**Real wall?** Yes — BPF verifier limits + fallback timeouts.
**Cross-domain wiring:** Userspace-defined scheduling at last; foundation of scx_lavd, scx_rusty.
**Notes:** Tejun Heo Meta (6.12, 2024); first BPF program type that affects scheduling.

### rendezvous-channel-cso (cross-domain alias: `rendezvous`, `csp-channel`, `ada-rendezvous`)
**Domain:** Operating Systems
**Definition:** Synchronous unbuffered channel where sender and receiver must both be ready; from Hoare CSP and Ada language.
**Atom or composite:** Atom — pair of slots with handshake.
**Cost model:** ~µs per exchange; blocking semantics.
**Real wall?** Yes — both parties must arrive; deadlock-prone without timeouts.
**Cross-domain wiring:** Foundation of **type-theory-programming-languages/csp**, Go's unbuffered chan.
**Notes:** Hoare CSP (1978); Ada 83 rendezvous; Go (Pike, Cox) chose CSP roots.

### stigmergy-trace (cross-domain alias: `stigmergy`, `pheromone-trace`, `indirect-coordination`)
**Domain:** Operating Systems
**Definition:** Coordination through environmental modification — agents leave traces (e.g., file timestamps, shared counters) that other agents read; no direct messaging.
**Atom or composite:** Composite — shared substrate + read/write protocol.
**Cost model:** O(reads) on substrate; eventual consistency.
**Real wall?** Yes — substrate persistence; eventual-consistency window.
**Cross-domain wiring:** Direct port of **biology-bioinformatics/ant-pheromone**, used in **distributed-systems/gossip-coordination**.
**Notes:** Grassé 1959 (ants); Theraulaz & Bonabeau formalized for swarm robotics; applied to OS as filesystem-mediated build systems (Bazel).

### chronos-clock-hybrid-logical (cross-domain alias: `chronos`, `hlc`, `hybrid-logical-clock`)
**Domain:** Operating Systems
**Definition:** Composite clock combining wall-clock (real time) and logical counter; advances `max(physical, last_hlc + 1)`, providing causality + bounded drift.
**Atom or composite:** Composite — 64-bit physical + 16-bit logical part.
**Cost model:** ~ns per tick; one CAS for monotonicity.
**Real wall?** Yes — physical-clock drift bounds correctness.
**Cross-domain wiring:** Same hybrid timing used by **distributed-systems/cockroachdb-hlc**, **distributed-systems/spanner-truetime**.
**Notes:** Kulkarni, Demirbas, Madappa (HLC 2014); building block for CockroachDB, YugabyteDB.

### hydra-consistency-policy-overlay (cross-domain alias: `hydra-consistency`, `multi-head-coherence`)
**Domain:** Operating Systems
**Definition:** Research-grade composite policy where multiple consistency heads (strong, causal, eventual) operate on the same dataset selected per operation; OS exposes via flag.
**Atom or composite:** Composite — N consistency engines + per-op policy selector.
**Cost model:** Per-op routing ~ns; underlying cost varies by chosen head.
**Real wall?** Yes — composing heads requires careful invariant proofs.
**Cross-domain wiring:** Generalization of **distributed-systems/red-blue-consistency**, **database-streaming-sketching/multiversion**.
**Notes:** Hybrid models per Lloyd et al. (COPS, 2011); productized in Cassandra-LWT, AWS DynamoDB GSI.

### logos-meta-governor (cross-domain alias: `logos-policy-governor`, `meta-scheduling`)
**Domain:** Operating Systems
**Definition:** Research-grade higher-order scheduler that selects among scheduling-class policies per workload phase using feedback signals (PSI, perf counters).
**Atom or composite:** Composite — phase detector + policy library + transition logic.
**Cost model:** ~ms decision cadence; per-tick steady state cheap.
**Real wall?** Yes — phase-misdetection costs latency or throughput.
**Cross-domain wiring:** Same meta-controller as **control-numerical-opt/gain-scheduling**.
**Notes:** Conceptual lineage: Sotomayor MIT, ghOSt; productizable via sched_ext.

### atlas-memory-tier-orchestrator (cross-domain alias: `atlas-memory`, `cross-node-tier-policy`)
**Domain:** Operating Systems
**Definition:** Memory orchestrator combining DAMON + memory tiering + CXL hot-page migration across a fleet; policies driven by per-app SLOs.
**Atom or composite:** Composite — DAMON regions + tier table + migration kernel + fleet coordinator.
**Cost model:** Sampling + migration ~µs–ms per page.
**Real wall?** Yes — interconnect bandwidth; page-table edit cost.
**Cross-domain wiring:** Same tiering as **distributed-systems/cross-host-paging**, Meta Transparent Memory Offloading (TMO).
**Notes:** TMO (Weiner et al., ASPLOS 2022); generalized in research as fleet-wide policy.

### capability-machine-cheri (cross-domain alias: `cheri-capability`, `pure-cap-abi`)
**Domain:** Operating Systems
**Definition:** ISA-level fat pointer carrying bounds and permissions; every load/store checked; capability-based pointer arithmetic only.
**Atom or composite:** Atom — single 129-bit register (128 bits + 1 tag).
**Cost model:** ~1–2 % runtime overhead; doubles pointer size.
**Real wall?** Yes — requires recompile to pure-cap ABI; hybrid mode mixes ints+caps.
**Cross-domain wiring:** Same unforgeable token as **cryptography-hashing/macaroon**.
**Notes:** Cambridge/SRI (Watson, Neumann, 2010+); ARM Morello SoC (2022); Microsoft CHERIoT.

### memory-protection-keys-mpk (cross-domain alias: `mpk-pkru`, `pkey`)
**Domain:** Operating Systems
**Definition:** Per-page 4-bit pkey + per-thread `PKRU` register gating R/W access; userspace toggles in ~5 ns without syscall.
**Atom or composite:** Composite — PTE field + PKRU register + `pkey_mprotect`.
**Cost model:** WRPKRU ~5 ns; check is parallel to TLB.
**Real wall?** Yes — only 16 keys; userspace-only on Intel.
**Cross-domain wiring:** Foundation of **distributed-systems/in-process-compartment**, V8 sandbox, Erim, libmpk.
**Notes:** Intel Skylake-X; kernel pkeys (5.x) for kernel-side use; ARM has equivalent (PAN+).

### ebpf-cilium-style-networking (cross-domain alias: `ebpf-networking`, `cilium-bpf`)
**Domain:** Operating Systems
**Definition:** XDP/TC-BPF programs implementing k8s service load balancer, network policy, observability without iptables; per-pod identity, hash maps for endpoints.
**Atom or composite:** Composite — XDP + tc-bpf + cgroup-skb + maps + sockmap.
**Cost model:** ~100 ns per packet; eliminates iptables linear walk.
**Real wall?** Yes — verifier limits; kernel feature spread.
**Cross-domain wiring:** Same approach as **networking/sw-load-balancer-bpf**.
**Notes:** Isovalent Cilium (2017); replaced kube-proxy in many deployments.

### unified-namespace-zircon (cross-domain alias: `process-local-namespace`, `fuchsia-local-namespace`)
**Domain:** Operating Systems
**Definition:** Fuchsia's per-component local namespace mapping path prefixes to capabilities (channels); no global filesystem root.
**Atom or composite:** Composite — path table + channel handles.
**Cost model:** Path lookup involves channel routing.
**Real wall?** Yes — paradigm shift from global VFS.
**Cross-domain wiring:** Same locally-scoped naming as **distributed-systems/per-tenant-fs**.
**Notes:** Fuchsia documentation; descendant of Plan 9 per-process namespace.

### scheduler-activations (cross-domain alias: `scheduler-activation`, `mn-threading`)
**Domain:** Operating Systems
**Definition:** Kernel notifies userspace runtime when blocking events occur, letting the runtime decide M:N thread multiplexing; replaces kernel-managed threads.
**Atom or composite:** Composite — upcall + per-process runtime + per-CPU virtual processors.
**Cost model:** Reduces kernel-thread count; runtime complexity high.
**Real wall?** Yes — historical; abandoned by FreeBSD KSE in favor of 1:1 NPTL.
**Cross-domain wiring:** Direct ancestor of Go's goroutine runtime; **type-theory-programming-languages/effects-system**.
**Notes:** Anderson, Bershad, Lazowska, Levy (1991); Solaris LWP and FreeBSD KSE.

### io-uring-sqpoll-iopoll (cross-domain alias: `sqpoll`, `iopoll`)
**Domain:** Operating Systems
**Definition:** io_uring modes — SQPOLL has a kernel thread polling the SQ ring (no syscall on submit); IOPOLL polls block driver for completions (no IRQ).
**Atom or composite:** Composite — flag at setup + dedicated kthread for SQPOLL.
**Cost model:** SQPOLL: zero syscall, one core 100 %; IOPOLL: zero IRQ, app polls.
**Real wall?** Yes — burns a CPU; only worth it under sustained load.
**Cross-domain wiring:** Same busy-poll trade-off as **networking/dpdk**, **distributed-systems/poll-mode-driver**.
**Notes:** Jens Axboe (5.1+); used by Ceph BlueStore, ScyllaDB Seastar.

### async-page-faults (cross-domain alias: `async-pf`, `kvm-apf`)
**Domain:** Operating Systems
**Definition:** KVM feature where guest page faults (e.g., swapped-out host memory) deliver an event to the guest letting it schedule other tasks while resolving.
**Atom or composite:** Composite — apf token + guest interrupt + host completion.
**Cost model:** Hides multi-ms host I/O behind guest scheduling.
**Real wall?** Yes — requires guest kernel cooperation.
**Cross-domain wiring:** Same async-fault as **distributed-systems/remote-paging**, FaRM.
**Notes:** Gleb Natapov (2.6.38, 2011); revisited in disaggregated-memory research.

### scheduler-coscheduling-core-sched (cross-domain alias: `core-scheduling`, `cs-cookie`)
**Domain:** Operating Systems
**Definition:** Linux core scheduling — only tasks with the same "cookie" run together on hyperthread siblings; defenses against MDS-class side channels.
**Atom or composite:** Composite — per-task cookie + sibling match logic.
**Cost model:** May leave siblings idle if no match; ~5–10 % overhead.
**Real wall?** Yes — security-driven, costs throughput.
**Cross-domain wiring:** Same isolation idea as **distributed-systems/tenant-coscheduling**.
**Notes:** Vineeth Pillai, Joel Fernandes (5.14, 2021); used by ChromeOS for site isolation.

### restartable-sequences-glibc (cross-domain alias: `rseq-glibc`, `per-cpu-malloc`)
**Domain:** Operating Systems
**Definition:** glibc 2.35+ uses rseq for per-CPU malloc freelists, allowing single-instruction-restartable atomics without LOCK prefix.
**Atom or composite:** Composite — rseq descriptor + per-CPU arena + restart on migrate.
**Cost model:** Eliminates atomic LOCK; ~5× faster small allocs in some workloads.
**Real wall?** Yes — rseq must be enabled; signal handling care.
**Cross-domain wiring:** Same per-CPU shortcut as **distributed-systems/sharded-counter**.
**Notes:** Florian Weimer glibc 2.35 (2022); enabled by default 2.40.

---

## Diagnostics & Fault Tolerance

### ecc-memory-scrubbing (cross-domain alias: `ecc`, `mca-scrub`)
**Domain:** Operating Systems
**Definition:** ECC DIMMs detect/correct single-bit errors; OS periodically scrubs (reads-with-correct-writeback) to prevent silent accumulation.
**Atom or composite:** Composite — ECC syndrome + scrubber daemon + MCE event.
**Cost model:** ~µs per uncorrected event; periodic scrub at ~MB/s.
**Real wall?** Yes — DRAM cell error rate; SECDED only corrects single-bit.
**Cross-domain wiring:** Same error-correction class as **information-theory-coding/secded**.
**Notes:** Hamming SECDED (1950); EDAC framework Linux 2.6.

### mce-machine-check-exception (cross-domain alias: `mce`, `cper-error-record`)
**Domain:** Operating Systems
**Definition:** Hardware-fatal-error reporting interrupt; `mcelog` (or in-kernel `mce_decode`) parses banks and offlines bad pages/CPUs.
**Atom or composite:** Composite — MCA bank registers + handler + policy.
**Cost model:** ~µs handler; offline-page may be expensive.
**Real wall?** Yes — uncorrected errors can panic; SRAO (Software Recoverable Action Optional) vs SRAR (Required).
**Cross-domain wiring:** Same fault-event pattern as **distributed-systems/health-alert**.
**Notes:** Intel SDM Vol 3 Ch 16; Andi Kleen mcelog (2007).

### kdump-kexec-crash (cross-domain alias: `kdump`, `crash-kernel`)
**Domain:** Operating Systems
**Definition:** Reserve memory at boot; on panic, kexec into a dump-capture kernel that saves `/proc/vmcore` for post-mortem.
**Atom or composite:** Composite — crashkernel reservation + capture kernel + makedumpfile.
**Cost model:** Setup ~MB memory reserved; dump time depends on RAM size.
**Real wall?** Yes — reserved memory unavailable to main kernel.
**Cross-domain wiring:** Same post-mortem pattern as **distributed-systems/last-will-checkpoint**.
**Notes:** Vivek Goyal (2.6.13, 2005).

### panic-oops-warn (cross-domain alias: `kernel-panic`, `oops`, `bug-on`)
**Domain:** Operating Systems
**Definition:** Severity tiers — WARN logs and continues; oops kills task and lives on; panic halts the system; `BUG_ON` is panic-grade assert.
**Atom or composite:** Composite — assertion family + die-notifier chain.
**Cost model:** Logs stack; panic kicks kdump.
**Real wall?** Yes — `panic_on_oops=1` for production servers; trades availability for evidence.
**Cross-domain wiring:** Same fail-fast doctrine as **distributed-systems/crash-only-software**.
**Notes:** Linus's `BUG_ON`; `panic` from V5 Unix.

### softlockup-hardlockup-detector (cross-domain alias: `lockup-detector`, `nmi-watchdog`)
**Domain:** Operating Systems
**Definition:** Watchdog — soft lockup if a task hogs CPU > 20 s without scheduling; hard lockup if no interrupts in 10 s, detected via NMI/perf.
**Atom or composite:** Composite — per-CPU timer + perf NMI event.
**Cost model:** Negligible; one NMI per ~4 s.
**Real wall?** Yes — false positives in heavy CPU work; tunable thresholds.
**Cross-domain wiring:** Same liveness-detector as **distributed-systems/heartbeat**.
**Notes:** Ingo Molnar (2.6.18); Don Zickus modernized via perf (2.6.36).

### rcu-stall-detector (cross-domain alias: `rcu-stall`, `gp-stuck`)
**Domain:** Operating Systems
**Definition:** Background detector printing warnings when a grace period hasn't ended within configured timeout; identifies stuck preempt-off code.
**Atom or composite:** Composite — timer + per-CPU quiescent-state probes.
**Cost model:** Polls at GP duration; no overhead steady state.
**Real wall?** Yes — false positives under PREEMPT_RT and heavy idle.
**Cross-domain wiring:** Same stuck-task diagnostic as **distributed-systems/consensus-stall**.
**Notes:** Paul McKenney (3.x); `rcu_cpu_stall_timeout` tunable.

### hung-task-detector (cross-domain alias: `hung-task`, `khungtaskd`)
**Domain:** Operating Systems
**Definition:** `khungtaskd` periodically scans tasks in TASK_UNINTERRUPTIBLE; if any spent > `hung_task_timeout_secs` (default 120) in D state, log + optional panic.
**Atom or composite:** Composite — periodic scan + state timestamp.
**Cost model:** O(tasks) per scan; cheap.
**Real wall?** Yes — slow disks naturally produce D-state; false positives common.
**Cross-domain wiring:** Same liveness pattern as **distributed-systems/zombie-task-detector**.
**Notes:** Ingo Molnar (2.6.30, 2009); essential for diagnosing storage hangs.

### fault-injection-failslab-failpage (cross-domain alias: `fault-injection`, `failslab`, `should_fail_bio`)
**Domain:** Operating Systems
**Definition:** Kernel `fault_injection` framework probabilistically failing kmalloc, alloc_page, bio submission, etc., to surface error-path bugs.
**Atom or composite:** Composite — probability + verbosity + scope filter.
**Cost model:** Cheap when off; chosen probability when on.
**Real wall?** No (test-only).
**Cross-domain wiring:** Same chaos-engineering pattern as **distributed-systems/chaos-monkey**.
**Notes:** Akinobu Mita (2.6.20, 2007).

### kasan-kmsan-ubsan (cross-domain alias: `kasan`, `kmsan`, `ubsan`, `kfence`)
**Domain:** Operating Systems
**Definition:** Compile-time sanitizers — KASAN (out-of-bounds + use-after-free via shadow memory), KMSAN (uninitialized reads), UBSAN (undefined behavior), KFENCE (probabilistic sampling).
**Atom or composite:** Composite — compiler instrumentation + per-tool runtime.
**Cost model:** 2–4× slowdown KASAN, ~1× KFENCE (sampling), 0.1× UBSAN.
**Real wall?** Yes — runtime overhead; mostly dev/CI use.
**Cross-domain wiring:** Same dynamic-checker family as ASAN/MSAN/TSAN in userspace.
**Notes:** Google KASAN (Andrey Konovalov, 4.0); KFENCE (5.12) for production sampling.

### kcsan-data-race-detector (cross-domain alias: `kcsan`, `kernel-thread-sanitizer`)
**Domain:** Operating Systems
**Definition:** Sampling data-race detector — when an access is instrumented, briefly pause and check for concurrent unsynchronized access on the same address.
**Atom or composite:** Composite — compiler hooks + watchpoint logic.
**Cost model:** ~1 % overhead at typical sample rate.
**Real wall?** Yes — false negatives proportional to sample rate.
**Cross-domain wiring:** Same race-detection class as TSAN/Helgrind.
**Notes:** Marco Elver Google (5.8, 2020); used heavily in Linux RCU development.

### lockdep-prove-locking (cross-domain alias: `lockdep`, `prove-locking`)
**Domain:** Operating Systems
**Definition:** As above (synchronization section) — runtime detector that builds a wait-for graph of lock classes and reports cycles before they deadlock.
**Atom or composite:** Composite — class hash + dependency graph.
**Cost model:** 3–10× lock cost with CONFIG_PROVE_LOCKING.
**Real wall?** Yes — class-count limit.
**Cross-domain wiring:** Compile-time-disabled prod parity with **formal-verification/lock-order-proof**.
**Notes:** Ingo Molnar (2.6.18); the single most influential kernel debug tool.

### crash-utility-vmcore (cross-domain alias: `crash`, `vmcore-analysis`)
**Domain:** Operating Systems
**Definition:** Userspace tool (crash + drgn + makedumpfile) reading vmcore + vmlinux to inspect kernel state, list tasks, walk lists, print stacks.
**Atom or composite:** Composite — symbol table + memory dump + scripted commands.
**Cost model:** Disk-bound load; analysis cheap.
**Real wall?** Yes — bitwidth/abi must match.
**Cross-domain wiring:** Same offline analysis as **distributed-systems/postmortem-report**.
**Notes:** Dave Anderson (Red Hat); drgn by Omar Sandoval modernizes via Python.

### nmi-watchdog (cross-domain alias: `nmi-wd`, `perf-watchdog`)
**Domain:** Operating Systems
**Definition:** Periodic NMI from perf counter checks that local-timer interrupts are firing; hard-lockup detector built atop.
**Atom or composite:** Composite — perf-counter setup + NMI handler.
**Cost model:** ~1 NMI / 4 s; negligible.
**Real wall?** Yes — consumes a perf counter slot.
**Cross-domain wiring:** Same independent watchdog as **distributed-systems/external-monitor**.
**Notes:** Don Zickus (2.6.36); essential for diagnosing infinite IRQ-off loops.

### watchdog-hardware-iwd (cross-domain alias: `hw-watchdog`, `iTCO`, `wdt`)
**Domain:** Operating Systems
**Definition:** Hardware timer that resets the system if not pinged within deadline; `/dev/watchdog` exposed to userspace daemons.
**Atom or composite:** Composite — chip register + driver + userland daemon.
**Cost model:** ~ms ping interval.
**Real wall?** Yes — guaranteed reset even when CPU is wedged.
**Cross-domain wiring:** Same liveness contract as **distributed-systems/external-watchdog**.
**Notes:** Many chipsets (iTCO, ipmi_watchdog, sbsa_gwdt); systemd integrates as service.

### lockup-stop-machine (cross-domain alias: `stop-machine`, `cpu-pause-all`)
**Domain:** Operating Systems
**Definition:** Mechanism halting all CPUs except the caller; used for module loading/unloading, ftrace text patching, hibernate.
**Atom or composite:** Composite — IPI to every CPU + spinwait barrier.
**Cost model:** ~ms global pause; intensifies tail latency.
**Real wall?** Yes — pauses real-time tasks; only emergency use.
**Cross-domain wiring:** Same global-quiesce as **distributed-systems/quiescent-checkpoint**.
**Notes:** Rusty Russell (2.6.27); minimized over time via text_poke alternatives.

### crash-only-software (cross-domain alias: `crash-only`, `fail-fast-restart`)
**Domain:** Operating Systems
**Definition:** Software design that only "crashes" — no graceful shutdown path; restart is the standard recovery, persistence handled by WAL.
**Atom or composite:** Composite — invariant: no shutdown handler + idempotent restart.
**Cost model:** Cheap shutdown; recovery dominates restart time.
**Real wall?** No (design pattern).
**Cross-domain wiring:** Same doctrine as **distributed-systems/fast-fail-restart**; ChromeOS, Erlang's "let it crash".
**Notes:** Candea & Fox (HotOS 2003); informs systemd's restart=on-failure.

### memory-failure-recovery (cross-domain alias: `mce-hwpoison`, `MADV_HWPOISON`)
**Domain:** Operating Systems
**Definition:** Linux mechanism to remove a physical page from use after an uncorrectable ECC error; if anon, kill owners; if file, drop from cache.
**Atom or composite:** Composite — hwpoison page flag + walk-mappings + kill.
**Cost model:** ~µs per page; runtime cost zero after marked.
**Real wall?** Yes — page lost from address space; large workloads may need restart.
**Cross-domain wiring:** Same isolate-and-evict as **distributed-systems/bad-node-quarantine**.
**Notes:** Andi Kleen (2.6.32, 2009).

### slab-debug-poison-redzone (cross-domain alias: `slab-debug`, `slub-debug`)
**Domain:** Operating Systems
**Definition:** SLUB debug poisons freed memory (0x6b), marks redzones at object boundaries, traces last alloc/free; surfaces use-after-free and overruns.
**Atom or composite:** Composite — sentinel bytes + per-object stack trace.
**Cost model:** ~10–20 % overhead; off in prod.
**Real wall?** Yes — extra memory per object.
**Cross-domain wiring:** Same canary pattern as ASAN, electric fence.
**Notes:** Christoph Lameter; activated by `slub_debug=` kernel cmdline.

---

## Additional Topics & Bridge Primitives

### vdso-fast-syscall (cross-domain alias: `vdso`, `vsyscall`, `linux-gate`)
**Domain:** Operating Systems
**Definition:** Per-process mmap of a shared object exposing fast read-only syscalls (`__vdso_clock_gettime`, `gettimeofday`, `getcpu`) executed without kernel transition.
**Atom or composite:** Composite — kernel-provided shared object + per-cpu data page.
**Cost model:** ~20 ns vs ~200 ns full syscall.
**Real wall?** Yes — limited to time/cpu reads; signed and ASLRed.
**Cross-domain wiring:** Same userspace acceleration as **networking/userland-stack**.
**Notes:** Andi Kleen (2.5.x); now mandatory on x86_64.

### syscall-entry-exit-trampoline (cross-domain alias: `syscall-entry`, `int80-syscall-sysenter`)
**Domain:** Operating Systems
**Definition:** Architecture-specific entry/exit paths from user to kernel; SYSCALL/SYSRET on x86_64, SVC on ARM; saves user registers, switches stacks.
**Atom or composite:** Composite — ISA instruction + entry asm + per-CPU stack.
**Cost model:** ~50–100 ns trap; KPTI doubles cost.
**Real wall?** Yes — silicon-defined; ARM/x86 mitigations bloat path.
**Cross-domain wiring:** Same boundary as **type-theory-programming-languages/foreign-function-call**.
**Notes:** SYSCALL added Pentium II; x86 SYSENTER preceded.

### vsyscall-vsyscall-page (cross-domain alias: `vsyscall-page`, `legacy-vsyscall`)
**Domain:** Operating Systems
**Definition:** Legacy fixed-address page at 0xffffffff... hosting `gettimeofday` and `time`; superseded by vDSO; now mostly emulated (vsyscall=xonly).
**Atom or composite:** Composite — fixed-address mapping + emulator.
**Cost model:** Slow due to emulation; kept for compatibility.
**Real wall?** Yes — predictable address = ROP gadget source.
**Cross-domain wiring:** Historical pre-vDSO mechanism.
**Notes:** Replaced piecewise; `vsyscall=none` recommended.

### futex2-improved-futex (cross-domain alias: `futex2`, `futex_waitv`)
**Domain:** Operating Systems
**Definition:** New futex syscalls (`futex_waitv`, `futex_wake`, `futex_requeue`) supporting wait-on-multiple, larger key sizes, NUMA-aware queues.
**Atom or composite:** Composite — extended op set + per-address queue.
**Cost model:** Comparable to futex; multi-wait saves syscalls.
**Real wall?** Yes — kernel hash table sizing; PI mode complexity.
**Cross-domain wiring:** Same primitive but better multiplexed; used by Wine, glibc.
**Notes:** André Almeida, Peter Zijlstra (5.16, 2022); Steam Proton motivator.

### perf-pebs-precise-event (cross-domain alias: `pebs`, `precise-event-based-sampling`)
**Domain:** Operating Systems
**Definition:** Intel PEBS — hardware writes register state into a buffer on counter overflow, eliminating skid for precise sampling.
**Atom or composite:** Composite — PEBS buffer + DS area + perf integration.
**Cost model:** Buffer fill at sample rate; analysis offline.
**Real wall?** Yes — only certain events PEBS-capable; ARM SPE is the equivalent.
**Cross-domain wiring:** Same precise sampling as **ml-training/precise-profiling**.
**Notes:** Intel PEBS since Nehalem; perf-pebs (2.6.32).

### perf-intel-pt-arm-spe (cross-domain alias: `intel-pt`, `arm-spe`, `cs-etm`)
**Domain:** Operating Systems
**Definition:** Hardware processor-trace mechanisms — Intel Processor Trace, ARM Statistical Profiling Extension, ARM CoreSight ETM — emit branch/sample streams to RAM.
**Atom or composite:** Composite — tracer hardware + per-thread buffer + perf decoder.
**Cost model:** ~1–5 % overhead; massive data rate (GB/s).
**Real wall?** Yes — storage bandwidth dominates; postprocessing complex.
**Cross-domain wiring:** Same trace primitive as **signal-processing-rf/iq-capture-buffer**.
**Notes:** Intel PT (Broadwell+); ARM SPE (Neoverse); perf intel-pt by Adrian Hunter.

### bpf-iter-kernel-iterator (cross-domain alias: `bpf-iter`, `dump-kernel-objects`)
**Domain:** Operating Systems
**Definition:** BPF program iterating kernel objects (task, sock, inode, vma, cgroup) via seq_file-style callbacks; produces customized dumps efficiently.
**Atom or composite:** Composite — iterator type + BPF prog + seq_file.
**Cost model:** Sequential walk; faster than /proc parse.
**Real wall?** Yes — types added piecewise.
**Cross-domain wiring:** Same kernel-walking as **distributed-systems/runtime-introspection**.
**Notes:** Yonghong Song Facebook (5.8, 2020).

### kernel-keyring-asymmetric-key (cross-domain alias: `asymmetric-keys`, `pkcs11-bridge`)
**Domain:** Operating Systems
**Definition:** Keyring type holding RSA/ECDSA/EdDSA public keys; used by IMA/EVM for signature verification, modsign.
**Atom or composite:** Composite — key blob + algorithm + verify op.
**Cost model:** Verify time per signature (~ms RSA, ~µs Ed25519).
**Real wall?** Yes — algorithm-specific; HW-backed via TPM/PKCS#11 module.
**Cross-domain wiring:** Same key store as **cryptography-hashing/public-key-infrastructure**.
**Notes:** David Howells (3.7); foundation of module signature, IMA appraisal.

### crypto-api-skcipher-aead (cross-domain alias: `crypto-api`, `tfm`, `akcipher`)
**Domain:** Operating Systems
**Definition:** Kernel crypto framework — `tfm` handles symmetric (skcipher), AEAD, hash (shash, ahash), KDF, asymmetric; HW accelerators register here.
**Atom or composite:** Composite — algorithm registry + per-tfm context + scatterlist I/O.
**Cost model:** Async (ahash) hides HW latency; sync (shash) faster for small.
**Real wall?** Yes — algorithm coverage; NEON/AES-NI selection per CPU.
**Cross-domain wiring:** Bridge layer to **cryptography-hashing/symmetric-suite**.
**Notes:** Herbert Xu maintainer; FIPS mode toggle.

### linux-rng-getrandom (cross-domain alias: `random-pool`, `getrandom`, `chacha-prng`)
**Domain:** Operating Systems
**Definition:** Kernel CSPRNG seeded from entropy sources (HW RNG, interrupt timing), reseeds periodically; `getrandom(2)` blocks until initialized.
**Atom or composite:** Composite — entropy pool + extract + ChaCha20 PRF.
**Cost model:** ~ns per byte once seeded.
**Real wall?** Yes — initial seeding latency at early boot; CRNG state transitions.
**Cross-domain wiring:** Same CSPRNG as **cryptography-hashing/fortuna**, **statistics-probability/cryptographic-rng**.
**Notes:** Theodore Ts'o (1.x); Jason Donenfeld rewrite (5.17, 2022) — pure ChaCha20.

### vsock-host-guest (cross-domain alias: `vsock`, `af_vsock`)
**Domain:** Operating Systems
**Definition:** Socket family for VM↔host communication with (CID, port) addressing; no network stack involved.
**Atom or composite:** Composite — virtio-vsock transport + AF_VSOCK socket.
**Cost model:** ~µs per packet; faster than virtio-net to host.
**Real wall?** Yes — transport-specific; not designed for routing.
**Cross-domain wiring:** Same host-guest channel as Hyper-V VMBus.
**Notes:** Stefan Hajnoczi (4.8, 2016); used by Firecracker, Kata Containers.

### kdbus-bus1-defunct (cross-domain alias: `kdbus`, `bus1`)
**Domain:** Operating Systems
**Definition:** Two attempts to put dbus into the kernel — kdbus (2014, rejected) and bus1 (2017, abandoned); ideas inform later eBPF and io_uring designs.
**Atom or composite:** Historical/composite.
**Cost model:** Aimed at sub-µs RPC; never merged.
**Real wall?** Political/political — design unification on existing primitives preferred.
**Cross-domain wiring:** Lessons-learned input to current IPC research.
**Notes:** Greg KH and David Herrmann; merit revisited piecewise.

### syscall-table-x86_64 (cross-domain alias: `syscall-nr`, `syscall-table`)
**Domain:** Operating Systems
**Definition:** Numbered table mapping syscall numbers to handlers; ~450 syscalls on x86_64; per-architecture; auditing tools inspect.
**Atom or composite:** Atom — array of function pointers.
**Cost model:** O(1) dispatch; modern entry uses `sys_call_table[nr]`.
**Real wall?** Yes — ABI compatibility constraints; rare to remove.
**Cross-domain wiring:** Same dispatch table as VM bytecode handlers.
**Notes:** Linus's original syscall design (0.01); auto-generated since 5.6 (Christoph Hellwig).

### audit-subsystem (cross-domain alias: `auditd`, `audit-rule`)
**Domain:** Operating Systems
**Definition:** Linux audit framework — kernel emits records on syscall/file/process events to userspace `auditd` via netlink; rules in `auditctl`.
**Atom or composite:** Composite — kernel emitter + rule engine + log file.
**Cost model:** ~µs per audited event; overhead with deep rules.
**Real wall?** Yes — log volume; netlink backpressure can lose events.
**Cross-domain wiring:** Same audit-trail as **distributed-systems/audit-log**.
**Notes:** Steve Grubb Red Hat (2.6.6); required for Common Criteria / STIG compliance.

### loadpin-lsm (cross-domain alias: `loadpin`, `secureboot-pin`)
**Domain:** Operating Systems
**Definition:** LSM that pins all kernel-loadable files (modules, firmware) to one filesystem; defeats arbitrary path-based load attacks.
**Atom or composite:** Composite — recorded sb + per-load check.
**Cost model:** O(1) check per load.
**Real wall?** Yes — must start with the right filesystem; once-only set.
**Cross-domain wiring:** Same single-source-of-truth as **distributed-systems/pinned-image-registry**.
**Notes:** Kees Cook ChromeOS (4.7, 2016).

### safesetid-lsm (cross-domain alias: `safesetid`, `setid-policy`)
**Domain:** Operating Systems
**Definition:** LSM restricting which setuid/setgid transitions are allowed via per-UID rules; replaces ad-hoc CAP_SETUID checks.
**Atom or composite:** Composite — rule policy + hook in `__set_*uid`.
**Cost model:** O(rules) per setid; cached lookup.
**Real wall?** Yes — policy must be carefully composed not to break daemons.
**Cross-domain wiring:** Same setid-restriction as **distributed-systems/principal-policy**.
**Notes:** Micah Morton Google (5.1, 2019).

### pmsg-pstore-ramoops (cross-domain alias: `pstore`, `ramoops`, `pmsg`)
**Domain:** Operating Systems
**Definition:** Persistent-store framework recording kernel logs to NVRAM/EFI variables/RAM region surviving reboot; ramoops uses a fixed RAM region.
**Atom or composite:** Composite — backend (ram, efi, acpi-pmem) + frontends (oops, panic, console, ftrace).
**Cost model:** Per-write cost of underlying medium.
**Real wall?** Yes — fixed region size; oldest records dropped.
**Cross-domain wiring:** Same post-crash store as **distributed-systems/black-box-recorder**.
**Notes:** Tony Luck (2.6.39); ramoops standard on Android.

### bpf-link-attach-lifecycle (cross-domain alias: `bpf-link`, `pinned-bpf`)
**Domain:** Operating Systems
**Definition:** BPF object representing a long-lived attachment (kprobe, tc, xdp) with explicit lifetime separate from fd-holding process; pinnable in bpffs.
**Atom or composite:** Composite — link fd + program + attachment context.
**Cost model:** ~negligible bookkeeping.
**Real wall?** Yes — pinned-link reference must be cleaned.
**Cross-domain wiring:** Same attached-resource handle as **distributed-systems/lease-handle**.
**Notes:** Andrii Nakryiko (5.7+, 2020); modernized loose-attach model.

### proc-self-fdinfo (cross-domain alias: `fdinfo`, `pos-flags`)
**Domain:** Operating Systems
**Definition:** Per-fd info exposed under `/proc/<pid>/fdinfo/<fd>` — current offset, flags, mountid, and per-type extensions (epoll list, fanotify, eventpoll).
**Atom or composite:** Composite — per-fd text exposed via seq_file.
**Cost model:** Read on demand.
**Real wall?** Yes — text parsing; format growth.
**Cross-domain wiring:** Same introspection surface as **distributed-systems/process-state-dump**.
**Notes:** Foundation for CRIU's fd-state collection.

### criu-checkpoint-restore (cross-domain alias: `criu`, `checkpoint-restore-userspace`)
**Domain:** Operating Systems
**Definition:** Userspace tool that captures every kernel-resident state of a process tree (mem, fds, sockets, namespaces, timers) and recreates it via setns/userfaultfd/clone3/pidfd.
**Atom or composite:** Composite — many kernel APIs orchestrated by userspace.
**Cost model:** Dump time ∝ memory; restore re-warms caches.
**Real wall?** Yes — every new kernel feature must be CRIU-aware.
**Cross-domain wiring:** Foundation of **distributed-systems/process-migration**, OpenVZ → LXC.
**Notes:** Pavel Emelyanov Parallels/Virtuozzo (2011); Google KataOS/Borg uses.

### prctl-pdeathsig-parent-death (cross-domain alias: `pdeathsig`, `parent-death-signal`)
**Domain:** Operating Systems
**Definition:** `prctl(PR_SET_PDEATHSIG, sig)` causes the kernel to send `sig` to the calling task when the parent dies — useful for cleanup.
**Atom or composite:** Atom — single per-task signal field.
**Cost model:** O(1) on parent exit.
**Real wall?** Yes — reset on setuid; raceable.
**Cross-domain wiring:** Same lifecycle-tied notification as **distributed-systems/heartbeat-on-death**.
**Notes:** Linux extension; not POSIX.

### tunable-procfs-sysctl (cross-domain alias: `sysctl`, `proc-sys`, `kernel-knob`)
**Domain:** Operating Systems
**Definition:** Hierarchical key-value space at `/proc/sys` (and `sysctl(8)`) exposing thousands of kernel tunables — `vm.swappiness`, `net.core.somaxconn`, `kernel.panic`.
**Atom or composite:** Composite — N typed knobs + R/W permissions.
**Cost model:** Read/write at user request; no runtime cost.
**Real wall?** Yes — wrong setting can degrade or crash; many are global.
**Cross-domain wiring:** Same config-knob substrate as **distributed-systems/dynamic-config**.
**Notes:** SVR4 sysctl; Linux extended massively.

### namespace-time-vdso (cross-domain alias: `time-ns`, `boottime-offset`)
**Domain:** Operating Systems
**Definition:** Time namespace lets a container see a shifted CLOCK_BOOTTIME / CLOCK_MONOTONIC; vDSO must consult offset.
**Atom or composite:** Composite — offset pair + vDSO path.
**Cost model:** ~ns extra in vDSO.
**Real wall?** Yes — wall-clock (REALTIME) not virtualized.
**Cross-domain wiring:** Foundation of **distributed-systems/checkpoint-restore-time-jump**.
**Notes:** Andrei Vagin (5.6, 2020); enables CRIU restore preserving uptime.

### vsyscall-vdso-clock-tai (cross-domain alias: `clock_tai`, `tai-utc-offset`)
**Domain:** Operating Systems
**Definition:** `CLOCK_TAI` — international atomic time, monotonic and ahead of UTC by leap-second count; preferred by precision-time apps.
**Atom or composite:** Atom — one of `clock_id` values.
**Cost model:** Same as other vDSO clocks.
**Real wall?** Yes — TAI–UTC offset must be NTP-supplied.
**Cross-domain wiring:** Same time-base choice as **distributed-systems/leap-smear-vs-tai**.
**Notes:** Linux 3.10 (2013).

### proc-net-tcp-conntrack (cross-domain alias: `conntrack`, `nf_conntrack`)
**Domain:** Operating Systems
**Definition:** Netfilter connection tracking — per-flow state for stateful firewall and NAT; entries in `/proc/net/nf_conntrack`.
**Atom or composite:** Composite — flow tuple + state + timeout.
**Cost model:** O(1) hash lookup; bucket-resize under load.
**Real wall?** Yes — `nf_conntrack_max` cap; full table drops packets.
**Cross-domain wiring:** Same flow state as **networking/stateful-fw**.
**Notes:** Rusty Russell, Harald Welte (netfilter 2.4).

### ebpf-cgroup-skb (cross-domain alias: `cgroup-skb`, `cgroup-bpf-network`)
**Domain:** Operating Systems
**Definition:** BPF programs attached to cgroup v2 filtering ingress/egress packets per cgroup; basis for k8s NetworkPolicy in Cilium.
**Atom or composite:** Composite — cgroup + attach point + BPF prog.
**Cost model:** ~100 ns per packet.
**Real wall?** Yes — pre-routing position; cannot route, only filter.
**Cross-domain wiring:** Same per-namespace policy as **networking/per-tenant-firewall**.
**Notes:** Daniel Mack (4.10, 2017).

### ebpf-sockmap-redirect (cross-domain alias: `sockmap`, `sk_msg`)
**Domain:** Operating Systems
**Definition:** BPF map of TCP sockets allowing in-kernel splice between sockets via `bpf_sk_redirect_map`; sidesteps userspace for proxies.
**Atom or composite:** Composite — sockmap + sk_msg/sk_skb BPF prog + redirect helper.
**Cost model:** Eliminates user-kernel copies in service mesh.
**Real wall?** Yes — limited to TCP currently.
**Cross-domain wiring:** Foundation of **networking/service-mesh-bypass**.
**Notes:** John Fastabend (4.14, 2017).

### kernel-keyring-trusted-tpm (cross-domain alias: `trusted-keys`, `tpm-sealed`)
**Domain:** Operating Systems
**Definition:** Keyring of TPM-sealed symmetric keys; root key in TPM, sealed against PCR state; unseal fails if boot chain differs.
**Atom or composite:** Composite — TPM key blob + sealing policy.
**Cost model:** Seal/unseal ~ms (TPM bandwidth).
**Real wall?** Yes — TPM-only; physical access defeats sealing minus measured boot.
**Cross-domain wiring:** Hardware-rooted variant of **cryptography-hashing/sealed-secret**.
**Notes:** Mimi Zohar, David Safford IBM (2.6.39, 2011).

### ima-policy-appraisal (cross-domain alias: `ima-appraise`, `ima-policy`)
**Domain:** Operating Systems
**Definition:** Integrity Measurement Architecture in appraise mode — verifies file signatures (stored in xattr security.ima) on access; denies if mismatch.
**Atom or composite:** Composite — policy file + per-file signature + verify.
**Cost model:** Hash + signature verify on first access; ~ms per binary.
**Real wall?** Yes — signed-binary chain; firstboot bootstrap.
**Cross-domain wiring:** Same per-file attestation as **cryptography-hashing/code-signing**.
**Notes:** Mimi Zohar; foundation of certified Linux deployments.

### efi-secure-boot-shim (cross-domain alias: `shim-efi`, `mok-manager`)
**Domain:** Operating Systems
**Definition:** Shim loader signed by Microsoft (or vendor) that verifies grub/kernel via its own MOK (Machine Owner Key) DB; enables Linux on Secure Boot PCs.
**Atom or composite:** Composite — shim binary + MOK store + revocation list.
**Cost model:** One-time at boot.
**Real wall?** Yes — Microsoft signing service; revocation broadcasts.
**Cross-domain wiring:** Same trust-chain delegation as **cryptography-hashing/x509-pkix**.
**Notes:** Matthew Garrett (2012); now part of every major distro.

### sysfs-uevent-hotplug (cross-domain alias: `uevent`, `kobject-event`)
**Domain:** Operating Systems
**Definition:** Kernel sends netlink/multicast events on device add/remove/change; userspace udev applies rules.
**Atom or composite:** Composite — kobject + uevent + netlink delivery.
**Cost model:** ~µs per event.
**Real wall?** Yes — best-effort delivery; race-prone for fast plug cycles.
**Cross-domain wiring:** Same device-discovery as **distributed-systems/topology-change-event**.
**Notes:** Greg KH (2.6); replaces hotplug script chain.

### kernel-livepatch (cross-domain alias: `livepatch`, `kpatch-kgraft`)
**Domain:** Operating Systems
**Definition:** Hot-patches kernel functions without reboot via ftrace mechanism; consistency model tracks task migration to ensure no stack uses old code.
**Atom or composite:** Composite — replacement function + ftrace hook + consistency model.
**Cost model:** Each call goes through ftrace ~10 ns; patch apply ~ms.
**Real wall?** Yes — only function-granularity; data structure changes hard.
**Cross-domain wiring:** Same hot-swap pattern as **distributed-systems/rolling-upgrade**, Erlang hot code load.
**Notes:** kpatch (Red Hat) + kGraft (SUSE) merged as livepatch (4.0, 2015).

### bpftime-userspace-bpf (cross-domain alias: `bpftime`, `userspace-bpf`)
**Domain:** Operating Systems
**Definition:** Research-grade project running eBPF programs in userspace (uprobe attach, syscall hook) via LLVM JIT; same programs as kernel BPF.
**Atom or composite:** Composite — userspace verifier + JIT + helper shim.
**Cost model:** ~ns per probe hit, ~10× faster than uprobe-based.
**Real wall?** No — pure userspace.
**Cross-domain wiring:** Brings eBPF model to non-kernel contexts.
**Notes:** Yusheng Zheng et al. (2023, SOSP); shows BPF is becoming a portable VM.

### overcommit-memory (cross-domain alias: `overcommit`, `vm.overcommit_memory`)
**Domain:** Operating Systems
**Definition:** Policy controlling whether `mmap`/`brk` can succeed beyond available RAM+swap — 0 heuristic, 1 always, 2 never (strict).
**Atom or composite:** Atom — sysctl with three values.
**Cost model:** No runtime cost; OOM probability shifts.
**Real wall?** Yes — mode 2 makes fork-then-exec hard for big processes.
**Cross-domain wiring:** Same admission-control choice as **distributed-systems/admission-vs-on-overload**.
**Notes:** Linux mode 0 default; Redis advises mode 1 for save fork.

### transparent-hugepage-defrag (cross-domain alias: `thp-defrag`, `defer+madvise`)
**Domain:** Operating Systems
**Definition:** `/sys/kernel/mm/transparent_hugepage/defrag` controls how aggressively compaction runs to satisfy a 2 MB allocation — always, defer, defer+madvise, madvise, never.
**Atom or composite:** Atom — one of 5 strings.
**Cost model:** `always` can stall allocators; `madvise` cheapest.
**Real wall?** Yes — defrag work blocks the faulting thread.
**Cross-domain wiring:** Same tunable as **database-streaming-sketching/compaction-scheduler**.
**Notes:** Mel Gorman tuned to `madvise` default for many distros after PostgreSQL stalls.

### linux-ftrace-trampoline (cross-domain alias: `ftrace-trampoline`, `mcount-pg`)
**Domain:** Operating Systems
**Definition:** Per-function trampoline patched at runtime to call ftrace, kprobe, or bpf fentry handlers; uses `mcount`/`-fpatchable-function-entry`.
**Atom or composite:** Composite — patch site + trampoline + handler list.
**Cost model:** ~10 ns when active.
**Real wall?** Yes — text_poke must be atomic; stop_machine fallback historically.
**Cross-domain wiring:** Foundation of all dynamic kernel instrumentation.
**Notes:** Steven Rostedt (2.6.27).

### linux-static-key-jump-label (cross-domain alias: `static-key`, `jump-label`)
**Domain:** Operating Systems
**Definition:** Branch whose `nop`/`jmp` instruction is patched at runtime; tracepoints, schedstats, and feature flags use it to avoid runtime checks.
**Atom or composite:** Composite — key + branch site + patch routine.
**Cost model:** Zero when off; ~5 ns when toggling.
**Real wall?** Yes — patching requires text-poke serialization.
**Cross-domain wiring:** Same zero-cost feature flag as **distributed-systems/dynamic-feature-flag**.
**Notes:** Jason Baron Red Hat (2.6.37, 2011); foundation of jump_label.

### ebpf-spinlock-helper (cross-domain alias: `bpf-spin-lock`, `map-locking`)
**Domain:** Operating Systems
**Definition:** BPF helper providing a 4-byte spinlock embedded inside map values; programs can atomically RMW shared state.
**Atom or composite:** Composite — `struct bpf_spin_lock` + helpers.
**Cost model:** ~10 ns uncontested.
**Real wall?** Yes — verifier requires balanced acquire/release; no nested locks.
**Cross-domain wiring:** Same per-key lock as **database-streaming-sketching/row-lock**.
**Notes:** Alexei Starovoitov (5.1, 2019).

### ebpf-ringbuf-mpsc (cross-domain alias: `bpf-ringbuf`, `mpsc-ringbuf`)
**Domain:** Operating Systems
**Definition:** Lock-free MPSC ring buffer for BPF→user pipeline; supports reserve/submit and per-CPU reservations; ordering preserved.
**Atom or composite:** Composite — shared ring + mmap-able + epoll-able.
**Cost model:** ~50 ns reserve+submit; avoids perf event array per-CPU duplication.
**Real wall?** Yes — single consumer; size at create.
**Cross-domain wiring:** Same MPSC pattern as **distributed-systems/log-shipping-ring**.
**Notes:** Andrii Nakryiko (5.8, 2020).

### linux-kernel-module-namespace (cross-domain alias: `kmod-version-magic`, `module-info`)
**Domain:** Operating Systems
**Definition:** Modules tag with `vermagic` (kernel version, SMP, preempt, GCC version) to refuse loading on mismatched kernel.
**Atom or composite:** Atom — string in `.modinfo` section.
**Cost model:** O(1) check at load.
**Real wall?** Yes — strictly enforced; out-of-tree modules must rebuild per kernel.
**Cross-domain wiring:** Same ABI-tag protection as **distributed-systems/version-handshake**.
**Notes:** 2.4+; foundational for DKMS workflow.

### linux-tasks-rcu (cross-domain alias: `tasks-rcu`, `srcu-tasks`)
**Domain:** Operating Systems
**Definition:** RCU variant whose grace periods wait for every task to voluntarily schedule; required for safe trampoline patching where preempt-disabled regions don't help.
**Atom or composite:** Composite — task-list scan + voluntary checkpoint.
**Cost model:** Grace period ~ms–seconds (waits for sleeping tasks).
**Real wall?** Yes — slow grace periods bound patch latency.
**Cross-domain wiring:** Same epoch-wait as **distributed-systems/version-quiesce**.
**Notes:** Paul McKenney (4.6); Tasks-Trace RCU (5.7) for sleepable BPF.

### perf-bpf-program-loaded (cross-domain alias: `perf-bpf`, `bpf-prog-perf`)
**Domain:** Operating Systems
**Definition:** perf can run a BPF program at sample points to filter/aggregate before delivery; reduces sample bandwidth.
**Atom or composite:** Composite — perf event + BPF prog + helper return.
**Cost model:** ~50 ns extra per sample, but cuts data rate massively.
**Real wall?** Yes — verifier limits filter complexity.
**Cross-domain wiring:** Same in-kernel aggregation as **database-streaming-sketching/pushdown-filter**.
**Notes:** Wang Nan Huawei (4.4, 2015).

### oom-killer-cgroup-aware (cross-domain alias: `memcg-oom`, `oom-group`)
**Domain:** Operating Systems
**Definition:** Memcg-level OOM killer plus `memory.oom.group` killing all tasks in cgroup atomically; prevents partial-kill resurrection.
**Atom or composite:** Composite — memcg OOM + group flag.
**Cost model:** O(tasks in memcg).
**Real wall?** Yes — task killed at memcg granularity for stability.
**Cross-domain wiring:** Same group-fail as **distributed-systems/atomic-tenant-eviction**.
**Notes:** Roman Gushchin (4.19, 2018).

### cgroup-systemd-slices (cross-domain alias: `slice-unit`, `systemd-cgls`)
**Domain:** Operating Systems
**Definition:** systemd organizes cgroup v2 hierarchy as `.slice` → `.service`/`.scope` units; delegate=yes lets a child manage its own subtree.
**Atom or composite:** Composite — unit file + cgroup path + delegated controllers.
**Cost model:** Per-unit start ~ms.
**Real wall?** Yes — single PID 1; subtree delegation respects v2 rules.
**Cross-domain wiring:** Same hierarchical-quota as **distributed-systems/multi-tenant-hierarchy**.
**Notes:** systemd 232+; kubelet (cgroupfs vs systemd) interaction.

### kernel-page-table-isolation-pti-arm (cross-domain alias: `kpti-arm`, `arm-ttbr-switch`)
**Domain:** Operating Systems
**Definition:** ARM equivalent of x86 KPTI — TTBR0_EL1 and TTBR1_EL1 split so userspace TLB doesn't cache kernel mappings; mitigates Meltdown-class.
**Atom or composite:** Composite — TTBR switching + entry trampoline.
**Cost model:** ~1–5 % syscall overhead; mitigated by ASID.
**Real wall?** Yes — hardware ASID slots.
**Cross-domain wiring:** Same isolation as x86 KPTI.
**Notes:** Will Deacon (4.16, 2018).

### perf-stat-counters (cross-domain alias: `perf-stat`, `cycles-instructions-cache-misses`)
**Domain:** Operating Systems
**Definition:** Wrapper exposing PMU counters as aggregate stats over a workload — cycles, instructions, cache-misses, branch-misses, stalled cycles.
**Atom or composite:** Composite — perf_event_open + sum + report.
**Cost model:** Counter read ~ns; multiplexing if events > slots.
**Real wall?** Yes — PMU slot count limits simultaneous events.
**Cross-domain wiring:** Same metric-gather as **ml-training/training-throughput-metrics**.
**Notes:** Ingo Molnar (2.6.31).

### perf-record-flamegraph (cross-domain alias: `perf-record`, `flamegraph`)
**Domain:** Operating Systems
**Definition:** Sampling profile of stack traces; Brendan Gregg's flamegraph SVG renders aggregated samples as a tree of width-proportional rectangles.
**Atom or composite:** Composite — sampler + symbolizer + folding script.
**Cost model:** ~1 % overhead at default rate.
**Real wall?** Yes — symbolization quality drives interpretability.
**Cross-domain wiring:** Same hierarchical-aggregation viz as **distributed-systems/distributed-trace-flamegraph**.
**Notes:** Brendan Gregg flamegraph (2011); now ubiquitous.

### inotify-watch-descriptor (cross-domain alias: `inotify-fd`, `wd`)
**Domain:** Operating Systems
**Definition:** Per-watch descriptor returned by `inotify_add_watch`; reads from inotify fd yield event records referencing wd.
**Atom or composite:** Composite — fd + watch table + event queue.
**Cost model:** O(1) per event; watch count bounded by `max_user_watches`.
**Real wall?** Yes — kernel memory per watch; many editors flood watches.
**Cross-domain wiring:** Same file-change feed as **distributed-systems/cdc-source**.
**Notes:** Robert Love + John McCutchan (2.6.13, 2005).

### shadow-stack-userspace (cross-domain alias: `userspace-shadow-stack`, `cet-ss`)
**Domain:** Operating Systems
**Definition:** Per-thread parallel stack of return addresses maintained by CPU; CALL pushes to both, RET pops both and compares.
**Atom or composite:** Composite — SS pages + mmap with `MAP_ABOVE4G`-style flag.
**Cost model:** Negligible runtime cost; ~1 % perf hit.
**Real wall?** Yes — needs CPU + compiler + glibc + kernel support.
**Cross-domain wiring:** Same CFI as ARM PAC return signing.
**Notes:** Intel CET (Tiger Lake, 2020); Linux glibc support 2.39 (2024).

### kernel-page-ownership-tracker (cross-domain alias: `page-owner`, `kpageflags`)
**Domain:** Operating Systems
**Definition:** Debug feature recording the stack trace that allocated each page; viewable via `/sys/kernel/debug/page_owner`.
**Atom or composite:** Composite — per-page extension + stack trace.
**Cost model:** Extra `struct page_ext` ~128 B per page; ~3 % memory overhead.
**Real wall?** Yes — memory cost limits production use.
**Cross-domain wiring:** Same allocator-debug as **ml-training/memory-attribution**.
**Notes:** Joonsoo Kim (3.16, 2014).

### perf-cgroup-mode (cross-domain alias: `perf-cgroup`, `--cgroup`)
**Domain:** Operating Systems
**Definition:** perf events scoped to a cgroup; counters fire only when a task in that cgroup is on-CPU.
**Atom or composite:** Composite — perf_event + cgroup attach.
**Cost model:** Per-context-switch toggle.
**Real wall?** Yes — context-switch overhead; cgroup v2 simplified.
**Cross-domain wiring:** Same tenant-scoped metric as **distributed-systems/per-tenant-metrics**.
**Notes:** Stephane Eranian (2.6.39, 2011).

### io-uring-registered-files-buffers (cross-domain alias: `registered-fd`, `registered-buf`)
**Domain:** Operating Systems
**Definition:** io_uring fast paths registering fds and userspace buffers up-front so each op uses an index instead of pointer + fd_get.
**Atom or composite:** Composite — registration tables + index-based references.
**Cost model:** Saves ~100 ns per op vs unregistered.
**Real wall?** Yes — registration is a setup cost.
**Cross-domain wiring:** Same handle-cache as **networking/dpdk-mempool**.
**Notes:** Jens Axboe (5.1+, expanded through 5.7).

### linux-net-namespace-l4-port-isolation (cross-domain alias: `port-namespace`, `netns-port-bind`)
**Domain:** Operating Systems
**Definition:** Each netns has its own TCP/UDP port table — port 80 is fine to bind in 100 containers simultaneously.
**Atom or composite:** Composite — per-netns hash tables.
**Cost model:** Lookups identical to non-namespaced; per-ns memory.
**Real wall?** Yes — IP/port scaling at container density.
**Cross-domain wiring:** Foundation of Kubernetes Service IP-per-pod model.
**Notes:** Builds on netns; predates IPVS but supersedes its isolation.

### ksm-shared-zero-page (cross-domain alias: `zero-page`, `huge-zero-page`)
**Domain:** Operating Systems
**Definition:** Single global zero-filled page (and 2 MB huge variant) mapped read-only into every COW-zero VA; saves memory and TLB.
**Atom or composite:** Atom — single physical page.
**Cost model:** Zero allocation cost; fault writes break to private page.
**Real wall?** Yes — first-write COW pays normal cost.
**Cross-domain wiring:** Same singleton-dedup as **distributed-systems/null-object-pool**.
**Notes:** Andrea Arcangeli (huge zero page 3.10).

### autonuma-fault-balancing (cross-domain alias: `autonuma`, `numa-hinting-faults`)
**Domain:** Operating Systems
**Definition:** Periodically marks pages with PROT_NONE so faults reveal accessor CPU; balancer migrates pages or tasks toward locality.
**Atom or composite:** Composite — sampler timer + protnone faults + task NUMA preference.
**Cost model:** ~1–5 % overhead.
**Real wall?** Yes — bandwidth between NUMA nodes.
**Cross-domain wiring:** Same placement optimizer as **ml-training/numa-aware-tensor-placement**.
**Notes:** Mel Gorman (3.8); Peter Zijlstra task NUMA preference.

### memfd-secret-secret-memory (cross-domain alias: `memfd-secret`, `secretmem`)
**Domain:** Operating Systems
**Definition:** memfd variant whose pages are removed from the kernel direct map; even root cannot read them via `/proc/kcore`.
**Atom or composite:** Composite — anon file + direct-map removal.
**Cost model:** TLB pressure (always unmapped from kernel); setup cost.
**Real wall?** Yes — limited to small secrets due to direct-map fragmentation.
**Cross-domain wiring:** Same kernel-opaque memory as TEE pages.
**Notes:** Mike Rapoport IBM (5.14, 2021).

### perf-bpf-output-event (cross-domain alias: `bpf-output`, `perf-bpf-output-channel`)
**Domain:** Operating Systems
**Definition:** Special perf event type letting BPF programs emit custom records into perf ring buffers; precursor to BPF ringbuf.
**Atom or composite:** Composite — perf event channel + BPF emit helper.
**Cost model:** ~100 ns per emit.
**Real wall?** Yes — per-CPU buffers; ringbuf superseded for new code.
**Cross-domain wiring:** Same custom-event pattern as USDT.
**Notes:** Wang Nan (4.4); de-facto standard before BPF ringbuf.

### ksched-priority-boost-on-wakeup (cross-domain alias: `latency-nice`, `wakeup-preemption`)
**Domain:** Operating Systems
**Definition:** EEVDF's `latency-nice` per-task hint biases virtual-deadline tightness — lower-latency tasks get shorter slices and earlier deadlines.
**Atom or composite:** Atom — single integer per task.
**Cost model:** Adjusts vd calc; no extra per-tick cost.
**Real wall?** Yes — at most -20..+19 range.
**Cross-domain wiring:** Same QoS hint as **networking/dscp**.
**Notes:** Peter Zijlstra EEVDF; first scheduler letting userspace tune wake-up latency directly.

### plimit-rlimit (cross-domain alias: `rlimit`, `prlimit`, `ulimit`)
**Domain:** Operating Systems
**Definition:** Per-process resource limits — NOFILE, NPROC, AS, STACK, CORE, CPU, MEMLOCK, RTPRIO, NICE; soft and hard caps.
**Atom or composite:** Composite — array of `struct rlimit` per task.
**Cost model:** O(1) check at relevant syscall.
**Real wall?** Yes — once hard reduced, cannot raise without CAP_SYS_RESOURCE.
**Cross-domain wiring:** Same per-task quota as **distributed-systems/per-job-limits**.
**Notes:** POSIX; `prlimit(2)` (2.6.36) lets one process set another's.

### cgroup-rdma-controller (cross-domain alias: `rdma-cgroup`, `hca-handle-cap`)
**Domain:** Operating Systems
**Definition:** Per-cgroup limit on RDMA verbs resources (HCA handles, MR count) to prevent runaway QP creation in multi-tenant.
**Atom or composite:** Composite — per-controller counters + max.
**Cost model:** O(1) per resource acquire.
**Real wall?** Yes — HCA hardware tables.
**Cross-domain wiring:** Same quota model as **distributed-systems/per-tenant-resource-limit**.
**Notes:** Parav Pandit Mellanox (4.11, 2017).

### linux-net-bonding-teaming (cross-domain alias: `bonding`, `team`, `lacp`)
**Domain:** Operating Systems
**Definition:** Aggregate multiple physical NICs into one virtual NIC for HA/throughput; modes: balance-rr, active-backup, balance-xor, broadcast, 802.3ad (LACP), balance-tlb, balance-alb.
**Atom or composite:** Composite — master + N slaves + scheduler.
**Cost model:** Negligible per-packet overhead.
**Real wall?** Yes — switch-side LACP capability; some modes work only on directly-connected switch.
**Cross-domain wiring:** Same link aggregation as **networking/lacp-bond**.
**Notes:** Donald Becker (2.0); `teamd` userspace alternative.

### linux-conntrack-zones (cross-domain alias: `conntrack-zone`, `multi-tenant-nat`)
**Domain:** Operating Systems
**Definition:** Per-zone conntrack tables allowing identical 5-tuples to coexist (e.g., overlapping container IPs); set via `--zone` in nft/iptables.
**Atom or composite:** Composite — zone id + per-zone hash table.
**Cost model:** Same as conntrack; multiplies memory by zone count.
**Real wall?** Yes — total table cap aggregate.
**Cross-domain wiring:** Same per-tenant flow table as **networking/vrf**.
**Notes:** Patrick McHardy (3.7, 2012).

### syscall-table-removed-rename (cross-domain alias: `removed-syscall`, `obsolete-call`)
**Domain:** Operating Systems
**Definition:** Linux nearly never removes a syscall; obsolete ones return -ENOSYS at runtime to preserve ABI; new versions get new names (epoll_create1, accept4, openat2).
**Atom or composite:** Atom — versioning convention.
**Cost model:** Zero.
**Real wall?** Yes — Linus's "we don't break userspace" rule.
**Cross-domain wiring:** Same ABI evolution as **type-theory-programming-languages/protocol-versioning**.
**Notes:** Linus Torvalds repeatedly; foundational for binary stability.

### bpf-sleepable-program (cross-domain alias: `sleepable-bpf`, `bpf-prog-sleepable`)
**Domain:** Operating Systems
**Definition:** BPF programs flagged sleepable can call helpers that may block (e.g., `bpf_copy_from_user`); attached at fentry/lsm.
**Atom or composite:** Composite — flag + Tasks-Trace RCU + restricted contexts.
**Cost model:** Tasks-Trace RCU grace periods slower.
**Real wall?** Yes — only certain attach points safe.
**Cross-domain wiring:** Same sleepable-callback as **distributed-systems/coroutine-callback**.
**Notes:** Alexei Starovoitov (5.7, 2020).

### kernel-page-cache-readahead (cross-domain alias: `readahead`, `mm-readahead`)
**Domain:** Operating Systems
**Definition:** Heuristic prefetcher that, on sequential reads, asynchronously fetches the next window; tuned via `read_ahead_kb` per BDI.
**Atom or composite:** Composite — read pattern detector + async fetch.
**Cost model:** Saves miss latency; wastes BW if pattern misread.
**Real wall?** Yes — BDI read-ahead size bounded.
**Cross-domain wiring:** Same prefetch as **database-streaming-sketching/buffer-pool-prefetch**.
**Notes:** Linux `mm/readahead.c`; Matthew Wilcox refactor (5.4).

### lru-mglru-multi-gen (cross-domain alias: `mglru`, `multi-generational-lru`)
**Domain:** Operating Systems
**Definition:** Multi-generational LRU replacing classic active/inactive lists with N "generations"; promotes pages by recency, batches eviction.
**Atom or composite:** Composite — per-memcg gen lists + per-generation accounting.
**Cost model:** Per-page promote ~ns; full scan O(pages/gens).
**Real wall?** Yes — generation count and memory overhead.
**Cross-domain wiring:** Same generational pattern as **graphics-rendering-lod/generational-mesh-cache**.
**Notes:** Yu Zhao Google (5.18, 2022); first major LRU rewrite in 20 years.

### kernel-cma-contiguous-memory-allocator (cross-domain alias: `cma`, `dma-coherent-pool`)
**Domain:** Operating Systems
**Definition:** Reserves a contiguous physical region at boot, movable until a driver needs it (cameras, GPUs, codecs).
**Atom or composite:** Composite — reservation + migration on alloc.
**Cost model:** Migration of pages in CMA region under demand.
**Real wall?** Yes — fragmentation if CMA too small.
**Cross-domain wiring:** Same physical-contig pool as embedded GPU pre-allocs.
**Notes:** Marek Szyprowski (3.5, 2012); critical for ARM SoCs.

### kernel-zswap-zram (cross-domain alias: `zswap`, `zram`, `compressed-swap`)
**Domain:** Operating Systems
**Definition:** zswap compresses pages destined for swap-in-memory before they reach disk; zram is a block device backed by compressed RAM.
**Atom or composite:** Composite — compression engine (lzo, zstd) + pool allocator (zsmalloc).
**Cost model:** ~µs compress/decompress; 2–4× compression ratio typical.
**Real wall?** Yes — CPU cost of compression; pool fragmentation.
**Cross-domain wiring:** Same compressed-cache as **distributed-systems/in-memory-compressed-page-cache**.
**Notes:** Seth Jennings (3.11, 2013); ChromeOS default.

### kernel-tasks-trace-rcu (cross-domain alias: `tasks-trace-rcu`, `srcu-trace`)
**Domain:** Operating Systems
**Definition:** RCU flavor whose grace period explicitly waits for sleepable readers (BPF sleepable programs); requires reader-id tracking.
**Atom or composite:** Composite — reader ID table + GP scanner.
**Cost model:** Slower GP than classic RCU.
**Real wall?** Yes — adds reader-side bookkeeping.
**Cross-domain wiring:** Same sleepable-reader reclamation as **distributed-systems/online-epoch-with-sleep**.
**Notes:** Paul McKenney (5.7, 2020); pairs with sleepable BPF.

### linux-net-tcp-tlp-rack (cross-domain alias: `tlp`, `rack-tlp`, `loss-recovery`)
**Domain:** Operating Systems
**Definition:** Tail Loss Probe + Recent ACKnowledgment — TCP loss-recovery heuristics replacing RTO-based timeouts with finer-grained signals.
**Atom or composite:** Composite — RACK reordering window + TLP timer.
**Cost model:** Trims tail latency dramatically on lossy paths.
**Real wall?** Yes — heuristic tuning; RFC 8985.
**Cross-domain wiring:** Same loss-detection improvement as **networking/quic-loss-detection**.
**Notes:** Yuchung Cheng Google (TLP 3.5, RACK 4.4).

### linux-tcp-ecn-l4s (cross-domain alias: `ecn`, `l4s-ect1`)
**Domain:** Operating Systems
**Definition:** Explicit Congestion Notification — IP/TCP signal congestion without dropping; L4S uses ECT(1) with scalable congestion controls.
**Atom or composite:** Composite — ECN bits + TCP CWR/ECE handshake.
**Cost model:** Saves head-of-line drops; backwards-compat with non-ECN.
**Real wall?** Yes — bleaching by middleboxes; deployment trip wires.
**Cross-domain wiring:** Same fast-feedback signal as **queueing-theory/dctcp-marking**.
**Notes:** Floyd RFC 3168; DCTCP for data centers; L4S 9330 (2023).

### linux-bpf-struct-ops (cross-domain alias: `struct-ops`, `bpf-vtable`)
**Domain:** Operating Systems
**Definition:** Mechanism letting a BPF program implement a kernel struct-of-function-pointers (tcp_congestion_ops, sched_ext_ops); typed and verified.
**Atom or composite:** Composite — kernel vtable + BPF prog + verifier.
**Cost model:** JITed indirect call ~ns.
**Real wall?** Yes — limited to opted-in subsystems.
**Cross-domain wiring:** Same plug-in pattern as **distributed-systems/userspace-extension-vtable**.
**Notes:** Martin KaFai Lau Facebook (5.6, 2020); enabled BPF-TCP-CC, sched_ext.
