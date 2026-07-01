# Networking Domain Primitives
> Cross-domain wiring: TCP congestion = fold-window, routing = shortest-path, HTTP = state-machine,
> QUIC = stream-multiplex, network-coding = linear-coding over GF(2^n)

## 1. Physical / Signal Layer

### [PRIM-001] photon-arrival
- **Atom/Composite:** Primitive
- **Definition:** Detection of a single photon or optical quantum at a photodetector.
- **Cost Model:** O(1) per arrival event; bandwidth = photon flux × quantum efficiency.
- **Real Wall:** Dark count rate, APD gain drift, timing jitter (>10 ps degrades correlation resolution).
- **Cross-Domain Aliases:** fold-collapse (signal-processing-rf), spike-detection (neuroscience).

### [PRIM-002] signal-attenuation
- **Atom/Composite:** Primitive
- **Definition:** Exponential decay of signal power: P(z) = P₀·e^(-αz).
- **Cost Model:** O(1) to evaluate; power budget = P_tx - path_loss - receiver_sensitivity.
- **Real Wall:** α depends on wavelength (fiber: 0.2 dB/km at 1550 nm), medium (free-space: inverse-square + atmospheric extinction).
- **Cross-Domain Aliases:** fold-fade (signal-processing-rf), diffusion-loss (physics-diffusion).

### [PRIM-003] chromatic-dispersion
- **Atom/Composite:** Primitive
- **Definition:** Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
- **Cost Model:** O(N) to simulate N-channel WDM; compensation requires DSP O(N log N).
- **Real Wall:** D (ps/nm/km) varies with fiber type; uncompensated dispersion limits bit rate × distance product.
- **Cross-Domain Aliases:** inter-symbol-interference (signal-processing-rf), convolution-spread (linear-algebra-matrix).

### [PRIM-004] polarization-mode-dispersion
- **Atom/Composite:** Primitive
- **Definition:** Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
- **Cost Model:** O(1) to compute DGD; accumulates as √L (not linearly).
- **Real Wall:** In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems.
- **Cross-Domain Aliases:** stochastic-delay (control-numerical-opt), random-phase-kick (quantum-computing).

### [PRIM-005] optical-snr-measurement
- **Atom/Composite:** Primitive
- **Definition:** OSNR = P_signal / (N_ASE · Δf_ref). Reference bandwidth = 0.1 nm (12.5 GHz at 1550 nm).
- **Cost Model:** O(1) with optical spectrum analyzer; O(N) for swept-wavelength.
- **Real Wall:** ASE noise from EDFAs; OSNR < ~15 dB for 100 Gbps PM-QPSK causes BER floor.
- **Cross-Domain Aliases:** snr-ratio (signal-processing-rf), fisher-information (information-theory-coding).

### [PRIM-006] coherent-detection
- **Atom/Composite:** Primitive
- **Definition:** Mix received optical signal with local oscillator (homodyne/heterodyne), recovering full amplitude + phase.
- **Cost Model:** DSP O(N log N) for polarization-diversity receiver; ADC at ≥2× symbol rate; ENOB >7 bits for 64-QAM.
- **Real Wall:** LO phase noise (laser linewidth); hardware AES-NI equivalent for optical DSP power.
- **Cross-Domain Aliases:** matched-filter (signal-processing-rf), phase-recovery (quantum-computing).

### [PRIM-007] wavelength-division-multiplex
- **Atom/Composite:** Composite (multiplexing + filtering)
- **Definition:** N wavelength channels transmitted simultaneously over one fiber.
- **Cost Model:** MUX/DMUX insertion loss ≈ 3–8 dB; non-ideal filter roll-off causes inter-channel crosstalk.
- **Real Wall:** EDFA gain flatness (±1.5 dB over C-band limits usable channels); fiber nonlinearity (SPM, XPM, FWM) increases with total power.
- **Cross-Domain Aliases:** fdma-channelization (signal-processing-rf), dimension-projection (linear-algebra-matrix).

### [PRIM-008] fiber-nonlinearity
- **Atom/Composite:** Primitive
- **Definition:** Kerr nonlinearity: n = n₀ + n₂·|E|². Generates SPM, XPM, FWM.
- **Cost Model:** O(N²) to compute inter-channel FWM in N-channel WDM; split-step Fourier method for accurate simulation.
- **Real Wall:** n₂ ≈ 2.6×10⁻²⁰ m²/W for silica; self-steepening at ps pulses.
- **Cross-Domain Aliases:** nonlinear-phase-shift (quantum-computing), interaction-term (physics-diffusion).

## 2. MAC / Link Layer

### [PRIM-009] csma-ca-channel-access
- **Atom/Composite:** Primitive
- **Definition:** Carrier Sense Multiple Access with Collision Avoidance: listen before transmit, random backoff on busy channel.
- **Cost Model:** Expected throughput S = 1/(1+2E); backoff window 2^k·slot_time.
- **Real Wall:** Hidden node problem; capture effect in 802.11 (~6–10 dB capture ratio).
- **Cross-Domain Aliases:** contention-resolution (database-streaming), slotted-aloha (information-theory-coding).

### [PRIM-010] arq-retransmission
- **Atom/Composite:** Composite
- **Definition:** Automatic Repeat reQuest: sender transmits, receiver ACKs/NACKs, sender retransmits on failure. Variants: Stop-and-Wait, Go-Back-N, Selective Repeat.
- **Cost Model:** Throughput = RTT / (RTT + retransmit_delay); efficiency = 1/(1+2p) for Stop-and-Wait (p = error probability).
- **Real Wall:** Timeout too short → spurious retransmissions; ACK/NACK loss doubles latency.
- **Cross-Domain Aliases:** error-correction-retry (cryptography-hashing), loop-iteration (ml-training).

### [PRIM-011] forward-error-correction
- **Atom/Composite:** Composite
- **Definition:** Structured redundancy (RS, LDPC, Turbo, Polar) so receiver corrects errors without retransmission.
- **Cost Model:** Coding gain (dB); FEC overhead = (n-k)/k; latency increases with codeword length n.
- **Real Wall:** Shannon limit: minimum E_b/N₀ = log₂(M) / (2^code_rate - 1) asymptotically.
- **Cross-Domain Aliases:** channel-coding (information-theory-coding), redundancy-insertion (cryptography-hashing).

### [PRIM-012] crc-checksum
- **Atom/Composite:** Primitive
- **Definition:** Cyclic Redundancy Check: data divided by polynomial generator, remainder appended as check bits.
- **Cost Model:** O(N) per frame (hardware: linear feedback shift register); undetected error probability = 2^(-r).
- **Real Wall:** Poly selection: CRC-32 (Ethernet), CRC-16 (USB); not纠错 — only detects.
- **Cross-Domain Aliases:** hash-verify (cryptography-hashing), syndrome-decode (information-theory-coding).

### [PRIM-013] mac-address-filtering
- **Atom/Composite:** Primitive
- **Definition:** Switch learns (MAC, port) bindings, forwards frames only to destination port; unknown destinations flooded.
- **Cost Model:** O(1) CAM/TCAM lookup; aging time (default 300 s) evicts stale entries.
- **Real Wall:** MAC flooding attack (fills table, causes broadcast); 48-bit address space.
- **Cross-Domain Aliases:** bloom-filter (database-streaming), routing-table-lookup (networking).

### [PRIM-014] vlan-tagging
- **Atom/Composite:** Composite
- **Definition:** 802.1Q: 4-byte VLAN tag (12-bit VLAN ID, 3-bit priority code point) added to Ethernet frame.
- **Cost Model:** VLAN table lookup; QinQ (stacked VLANs) doubles tag overhead; max 4096 VLANs (12-bit space).
- **Real Wall:** VLAN hopping (switch spoofing, double-tagging); inter-VLAN routing requires L3.
- **Cross-Domain Aliases:** namespace-isolation (distributed-systems), priority-queue (control-numerical-opt).

### [PRIM-015] link-aggregation
- **Atom/Composite:** Composite
- **Definition:** Bundling N physical links into one logical LAG (LACP 802.3ad): increases bandwidth + failover.
- **Cost Model:** Per-flow hash (L2/L3/L4 5-tuple); effective throughput ≤ N × slowest link.
- **Real Wall:** Hash collision (two large flows → same member); MLAG requires consistent hashing across two switches.
- **Cross-Domain Aliases:** parallel-channels (distributed-systems), beamforming (signal-processing-rf).

### [PRIM-016] spanning-tree-protocol
- **Atom/Composite:** Composite
- **Definition:** STP (802.1D): loop-free L2 topology by blocking redundant ports. RSTP (802.1w) <1 s convergence; MSTP (802.1s) groups VLANs into instances.
- **Cost Model:** Convergence: classic STP ~50 s (too slow); RSTP <1 s; port roles: root/designated/alternate/blocking.
- **Real Wall:** Single root = single point of failure; topology changes flush MAC tables.
- **Cross-Domain Aliases:** minimum-spanning-tree (linear-algebra-matrix), cycle-elimination (distributed-systems).

### [PRIM-017] qos-queuing
- **Atom/Composite:** Composite
- **Definition:** Per-interface multiple queues with scheduling: SP (strict priority), WRR, DRR (deficit round-robin), CB-WFQ.
- **Cost Model:** Queue depth → buffer bloat; scheduling: O(1) for SP, O(N) for WRR; jitter = queue_depth / service_rate.
- **Real Wall:** Head-of-line blocking; priority inversion; buffer starvation for low-priority queues.
- **Cross-Domain Aliases:** priority-queue (control-numerical-opt), packet-classification (signal-processing-rf).

### [PRIM-018] traffic-shaping-policing
- **Atom/Composite:** Composite
- **Definition:** Shaper (token bucket) limits average + peak rate; policer drops non-conforming traffic; dual token bucket (CIR + EIR) separates conformant vs. excess.
- **Cost Model:** Token bucket: tokens += rate·Δt, capped at burst_size; packet admitted if tokens ≥ packet_size.
- **Real Wall:** Burst tolerance vs. sustained rate trade-off; policing with no buffer → TCP retransmit storm.
- **Cross-Domain Aliases:** rate-limiter (distributed-systems), token-bucket (information-theory-coding).

### [PRIM-019] mac-learning-table
- **Atom/Composite:** Primitive
- **Definition:** Switch dynamic MAC table: source MAC of received frame → port; aging evicts inactive entries.
- **Cost Model:** O(1) CAM/TCAM lookup; MAC flapping (same MAC on multiple ports) = L2 loop indicator.
- **Real Wall:** MAC move rate = churn indicator; hardware TCAM for 4096–65536 entries.
- **Cross-Domain Aliases:** hash-table-lookup (cryptography-hashing), cache-invalidation (distributed-systems).

### [PRIM-020] ethernet-flow-control
- **Atom/Composite:** Primitive
- **Definition:** 802.3x PAUSE frames halt transmission. PFC (802.1Qbb) applies per-priority-class PAUSE.
- **Cost Model:** PFC storm: switch A pauses B → B pauses C → cascade of pauses (deadlock risk).
- **Real Wall:** Used in lossless DCB (Data Center Bridging) fabrics; DCBX (802.1Qaz) negotiates PFC parameters.
- **Cross-Domain Aliases:** backpressure (control-numerical-opt), tcp-window-flow-control (networking).

## 3. Network / Internet Layer

### [PRIM-021] ip-routing-table-lookup
- **Atom/Composite:** Primitive
- **Definition:** Longest prefix match (LPM) on destination IP: find most specific (longest mask) matching route.
- **Cost Model:** O(1) with TCAM; O(log N) with binary trie (depth = 32 for IPv4, 128 for IPv6); N ≈ 900K global IPv4 prefixes.
- **Real Wall:** TCAM power consumption (8W per chip); software trie is power-efficient but slower.
- **Cross-Domain Aliases:** longest-prefix-match (distributed-systems), trie-lookup (information-theory-coding).

### [PRIM-022] bgp-path-vector
- **Atom/Composite:** Composite
- **Definition:** BGP carries AS path as ordered AS number list; best path: LOCAL_PREF > AS_PATH length > MED > IGP cost > Router-ID.
- **Cost Model:** BGP convergence depends on MRAI (default 30 s); full table ~900K prefixes; session keepalive (90 s).
- **Real Wall:** AS_PATH prepending manipulation; route flap damping; BGP best-path not always globally optimal.
- **Cross-Domain Aliases:** policy-propagation (distributed-systems), shortest-path-with-attributes (linear-algebra-matrix).

### [PRIM-023] ospf-link-state
- **Atom/Composite:** Composite
- **Definition:** OSPF floods LSAs to all routers in area; each router builds identical LSDB; SPF (Dijkstra) computes shortest path tree.
- **Cost Model:** SPF complexity O(L + N log N) per LSA change; runs on every LSA change (μs to ms).
- **Real Wall:** Large LSDB → OSPF area hierarchy (ABR/Area 0); LSA flooding storm after link flap.
- **Cross-Domain Aliases:** dijkstra-shortest-path (linear-algebra-matrix), gossips-protocol (distributed-systems).

### [PRIM-024] is-is-spf
- **Atom/Composite:** Composite
- **Definition:** IS-IS: link-state IGP (operates at Layer 2, CLNS); wide metrics (sub-TLVs) for TE extensions; TLV extensibility without protocol change.
- **Cost Model:** Same SPF complexity as OSPF; used by most Tier-1 ISPs; supports segment routing.
- **Real Wall:** Runs directly over Layer 2 (no IP); multi-topology IS-IS for IPv6.
- **Cross-Domain Aliases:** dijkstra-shortest-path (linear-algebra-matrix), link-state-flooding (distributed-systems).

### [PRIM-025] rip-distance-vector
- **Atom/Composite:** Composite
- **Definition:** Each router advertises hop count to each destination (max 15 hops); periodic full-table advertisement (30 s); Bellman-Ford distance vector exchange.
- **Cost Model:** O(N²) worst-case; converges slowly (minutes); rarely used in production.
- **Real Wall:** 15-hop limit; routing loops (count-to-infinity); replaced by OSPF/BGP.
- **Cross-Domain Aliases:** bellman-ford (linear-algebra-matrix), periodic-beacon (distributed-systems).

### [PRIM-026] icmp-echo-probe
- **Atom/Composite:** Primitive
- **Definition:** ICMP Echo Request/Reply (ping): send packet with ID + sequence number, record RTT, compute loss rate.
- **Cost Model:** O(N) packets for N-probe measurement; RTT resolution = OS timer resolution (typically 1 ms).
- **Real Wall:** ICMP rate limiting by devices; firewalls block ICMP; path MTU discovery uses ICMP "packet too big" (often blocked).
- **Cross-Domain Aliases:** round-trip-measurement (distributed-systems), latency-sampling (signal-processing-rf).

### [PRIM-027] traceroute-hop-enumeration
- **Atom/Composite:** Composite
- **Definition:** Send packets with increasing TTL; each router returns ICMP Time Exceeded; final destination returns ICMP Echo Reply or TCP SYN/ACK.
- **Cost Model:** N hops × probes_per_hop × timeout; load-balanced paths → multiple IPs per hop.
- **Real Wall:** NAT devices overwrite source IP; IPv6 traceroute uses ICMPv6 type 2 (packet too big).
- **Cross-Domain Aliases:** path-enumeration (distributed-systems), reverse-trace (control-numerical-opt).

### [PRIM-028] nat-address-translation
- **Atom/Composite:** Composite
- **Definition:** Map (src_IP, src_port) ↔ (public_IP, mapped_port); symmetric NAT vs. port-restricted NAT; cone vs. restricted NAT.
- **Cost Model:** Connection tracking table: O(N) state; ICMP query NAT requires special handling (ICMP ID → transport ID).
- **Real Wall:** Symmetric NAT breaks P2P protocols (STUN fails); IPv4 exhaustion; CGNAT shared among thousands of subscribers.
- **Cross-Domain Aliases:** address-space-translation (distributed-systems), coordinate-transform (linear-algebra-matrix).

### [PRIM-029] ipv6-nd-neighbor-discovery
- **Atom/Composite:** Composite
- **Definition:** RS/RA (router discovery), NS/NA (neighbor discovery), DAD (Duplicate Address Detection). NUD (Neighbor Unreachability Detection) probes stale entries.
- **Cost Model:** DAD: probe own address with NS; wait RANDOM_DELAY (0–1 s); NA confirms address.
- **Real Wall:** ND cache poisoning; RA guard blocks rogue RAs; SLAAC without DHCPv6 = no DNS info (use RDNSS in RA).
- **Cross-Domain Aliases:** arp-resolution (networking), address-discovery (distributed-systems).

### [PRIM-030] ip-fragmentation-reassembly
- **Atom/Composite:** Composite
- **Definition:** Path MTU discovery → sender fragments if packet > MTU; receiver reassembles (60 s timeout); overlapping fragments = security risk (teardrop attack).
- **Cost Model:** Fragment overhead: each has 20-byte IP header; tunnel MTU = 1480 (standard for GRE over Internet).
- **Real Wall:** PMTU discovery uses ICMP type 2 (blocked by many firewalls); IPv6 fragments only at source.
- **Cross-Domain Aliases:** packet-segmentation (signal-processing-rf), data-reassembly (database-streaming).

### [PRIM-031] mpls-label-switching
- **Atom/Composite:** Composite
- **Definition:** 20-bit MPLS label (shim header) determines forwarding path; LDP or RSVP-TE distributes labels; penultimate hop popping (PHP) avoids final label lookup.
- **Cost Model:** Label lookup O(1) (hardware TCAM); LDP convergence slow (minutes); RSVP-TE scales poorly (per-flow state).
- **Real Wall:** LDP sync with IGP; segment routing (SR-MPLS, SRv6) addresses state explosion.
- **Cross-Domain Aliases:** label-projection (linear-algebra-matrix), table-lookup-forwarding (distributed-systems).

### [PRIM-032] segment-routing
- **Atom/Composite:** Composite
- **Definition:** SR-MPLS: source encodes path as stack of MPLS labels (SID); SRv6: SID = 128-bit IPv6 address encoding behavior + locator. No per-flow state at transit.
- **Cost Model:** Forwarding entries proportional to SID count, not path count; TI-LFA (topology-independent loop-free alternates) for FRR.
- **Real Wall:** Large SID tables in SRv6; ECMP handled natively; Microloop avoidance.
- **Cross-Domain Aliases:** source-routing (distributed-systems), label-stack (linear-algebra-matrix).

### [PRIM-033] vxlan-overlay
- **Atom/Composite:** Composite
- **Definition:** 24-bit VNI (Virtual Network Identifier); MAC-in-UDP encapsulation; VTEP at hypervisor; control plane via multicast or EVPN.
- **Cost Model:** VTEP encapsulation overhead: +50 bytes (VXLAN + UDP + IP + Ethernet); MTU must be ≥1600.
- **Real Wall:** VXLAN underlay requires IP multicast (for BUM flooding) or EVPN control plane.
- **Cross-Domain Aliases:** network-virtualization (distributed-systems), tunnel-encapsulation (cryptography-hashing).

### [PRIM-034] geneve-tunnel
- **Atom/Composite:** Composite
- **Definition:** Geneve: extensible tunnel with TLV metadata (variable-length options), MAC-in-UDP, service function chaining and NFV.
- **Cost Model:** Options parsing O(N) where N = option count; OVS-DPDK integration for vSwitch offload.
- **Real Wall:** TLV processing in fast path requires hardware support (e.g., Tofino switch ASICs).
- **Cross-Domain Aliases:** vxlan-overlay (networking), protocol-extension (distributed-systems).

### [PRIM-035] gre-tunnel
- **Atom/Composite:** Composite
- **Definition:** Generic Routing Encapsulation: encapsulates any L3 protocol inside IP (protocol type field); optional key field for demultiplexing.
- **Cost Model:** GRE header = 4 bytes min (no key), 8 bytes (with key + sequence); fragmentation if encapsulated packet exceeds MTU.
- **Real Wall:** No intrinsic encryption; GRE + IPsec = double encapsulation overhead.
- **Cross-Domain Aliases:** tunnel-proxification (distributed-systems), protocol-wrapping (information-theory-coding).

### [PRIM-036] ipsec-tunnel
- **Atom/Composite:** Composite
- **Definition:** IPsec: AH (integrity only) or ESP (confidentiality + integrity); tunnel mode (full IP header replaced) vs. transport (preserved).
- **Cost Model:** AES-GCM-256 (authenticated encryption); hardware offload in NIC; AES-NI CPU instruction set.
- **Real Wall:** NAT traversal (NAT-T, UDP port 4500); perfect forward secrecy (DH exchange); post-quantum hybrid key exchange.
- **Cross-Domain Aliases:** encryption-channel (cryptography-hashing), secure-channel (distributed-systems).

### [PRIM-037] ipsec-nat-traversal
- **Atom/Composite:** Composite
- **Definition:** NAT-T: encapsulate ESP in UDP (port 4500) for NAT traversal; NAT-T detection via IKE port 500 → 4500 transition; keepalive every 5–20 s.
- **Cost Model:** UDP encapsulation overhead: 8-byte UDP + 4-byte non-ESP marker.
- **Real Wall:** NAT device UDP port reuse timeout (30 s to 5 min); symmetric NAT issues.
- **Cross-Domain Aliases:** nat-address-translation (networking), tunnel-through-NAT (distributed-systems).

### [PRIM-038] dscp-qos-marking
- **Atom/Composite:** Primitive
- **Definition:** DSCP (6-bit ToS field): EF = Expedited Forwarding, AFxy = Assured Forwarding (x=class 1-4, y=drop precedence 1-3), BE = Best Effort (0).
- **Cost Model:** PHB (Per-Hop Behavior) at each router: queuing priority based on DSCP.
- **Real Wall:** DSCP mutation by middleboxes; inter-AS DSCP preservation not guaranteed.
- **Cross-Domain Aliases:** qos-queue (networking), traffic-priority (control-numerical-opt).

### [PRIM-039] reverse-path-filtering
- **Atom/Composite:** Primitive
- **Definition:** uRPF (Unicast Reverse Path Forwarding): check source IP has valid return path via arrival interface; strict vs. loose mode.
- **Cost Model:** O(1) RIB/FIB lookup for source address; TCAM-based uRPF for line-rate enforcement.
- **Real Wall:** Asymmetric routing (legitimate packets dropped by strict uRPF); loose mode needed for multi-homed sites.
- **Cross-Domain Aliases:** anti-spoof-filter (distributed-systems), route-validity-check (networking).

### [PRIM-040] anycast-routing
- **Atom/Composite:** Composite
- **Definition:** Multiple nodes advertise same IP prefix via BGP; clients' traffic routes to nearest (lowest-metric) node.
- **Cost Model:** Traffic distribution depends on AS path + IGP metrics; no true load balancing per session.
- **Real Wall:** No connection persistence; health checks must withdraw prefix on failure.
- **Cross-Domain Aliases:** geographic-routing (distributed-systems), multi-destination-unreliable (information-theory-coding).

## 4. Transport / Reliability

### [PRIM-041] tcp-congestion-window
- **Atom/Composite:** Primitive
- **Definition:** cwnd: sender's estimate of bytes in flight; grows via AIMD (additive increase, multiplicative decrease).
- **Cost Model:** Throughput ≈ cwnd / RTT; effective cwnd = min(cwnd, receive_window); cwnd in bytes (Linux 2.6+).
- **Real Wall:** BBR vs. CUBIC vs. DCTCP have different growth models; pacing rate ≠ cwnd alone.
- **Cross-Domain Aliases:** fold-window (information-theory-coding), capacity-estimator (control-numerical-opt).

### [PRIM-042] tcp-slow-start
- **Atom/Composite:** Primitive
- **Definition:** Exponential cwnd growth from IW (10 MSS) until first loss or ssthresh. Each ACK → cwnd += MSS; effectively doubles per RTT.
- **Cost Model:** RTTs to fill pipe = log₂(capacity / IW); slow start overshoot causes large queueing spikes.
- **Real Wall:** ACK compression (burst ACKs → cwnd spikes); IW too large causes bufferbloat.
- **Cross-Domain Aliases:** exponential-growth (ml-training), boot-strap (distributed-systems).

### [PRIM-043] tcp-congestion-avoidance
- **Atom/Composite:** Primitive
- **Definition:** After slow start, cwnd increases by 1 MSS per RTT (linear); multiplicative decrease on loss (halve cwnd).
- **Cost Model:** Throughput = (cwnd·MSS) / RTT; bottleneck link utilization maximized near equilibrium.
- **Real Wall:** CUBIC uses cubic function of time since last reduction (better for high-BDP); DCTCP uses ECN marks for byte-granular response.
- **Cross-Domain Aliases:** aimd-rate-control (control-numerical-opt), additive-increase (ml-training).

### [PRIM-044] tcp-fast-retransmit
- **Atom/Composite:** Primitive
- **Definition:** On 3 duplicate ACKs, retransmit lost segment without waiting for RTO. Spurious retransmit on reordered packets.
- **Cost Model:** DUPACK threshold = 3; fast retransmit avoids RTO wait (typically 1–3×RTT saved).
- **Real Wall:** Reordering triggers spurious retransmit; FACK (Forward ACK) tracks outstanding packets for better recovery.
- **Cross-Domain Aliases:** early-retransmit (database-streaming), speculative-retry (ml-training).

### [PRIM-045] tcp-recovery-sack
- **Atom/Composite:** Composite
- **Definition:** TCP SACK (Selective Acknowledgment) allows receiver to report non-contiguous blocks received, enabling efficient selective retransmit.
- **Cost Model:** SACK option = up to 4 blocks; RENO without SACK = Go-Back-N retransmit (inefficient for large pipes).
- **Real Wall:** SACK-induced fragmentation; SACK negotiation fails on some middleboxes.
- **Cross-Domain Aliases:** selective-repeat-ack (database-streaming), partial-acknowledgment (information-theory-coding).

### [PRIM-046] tcp-timeout-retransmission
- **Atom/Composite:** Primitive
- **Definition:** RTO (Retransmission Timeout): if ACK not received within RTO, resend all unacknowledged data.
- **Cost Model:** RTO = SRTT + 4·RTTVAR (Jacobson/Karels algorithm); minimum RTO (Linux: 200 ms, can be lower with TCP timestamps).
- **Real Wall:** Ambiguous ACK (can't distinguish loss from delay); coarse-grained timers in older systems cause unnecessary retransmits.
- **Cross-Domain Aliases:** timeout-backoff (distributed-systems), exponential-backoff (control-numerical-opt).

### [PRIM-047] tcp-bbr-congestion-control
- **Atom/Composite:** Primitive
- **Definition:** BBR (Bottleneck Bandwidth and RTT): model-based, estimates bandwidth and RTprop to operate at optimal operating point (bandwidth = BDIMax, pacing = BDP).
- **Cost Model:** State machine: Startup → Drain → ProbeBW → ProbeRTT; periodic pacing gain cycling for throughput probing.
- **Real Wall:** BBRv1 vs. v2 vs. v3 have different gain cycling and loss handling; incompatible with L4S (Low Latency, Low Loss, Scalable Throughput).
- **Cross-Domain Aliases:** model-pacing (control-numerical-opt), adaptive-rate (ml-training).

### [PRIM-048] tcp-cubic
- **Atom/Composite:** Primitive
- **Definition:** CUBIC: cwnd growth as cubic function of time since last congestion event: W(t) = C·(t-K)³ + W_max.
- **Cost Model:** BIC (Binary Increase Congestion control) predecessor; window scale option (RFC 1323) for high-BDP networks.
- **Real Wall:** Hystart detects bandwidth probing phase; high-speed performance (10+ Gbps); competes poorly with BBR on bufferbloat links.
- **Cross-Domain Aliases:** cubic-convergence (control-numerical-opt), polynomial-growth (ml-training).

### [PRIM-049] tcp-ecn
- **Atom/Composite:** Composite
- **Definition:** ECN (Explicit Congestion Notification): router marks packets (CE codepoint) instead of dropping; receiver signals ECN to sender via ECE (ECN Echo); sender reduces rate.
- **Cost Model:** ECN requires L4S identifier (ECT(1)); ECN negotiation via SYN/SYN-ACK ECE/CWR flags.
- **Real Wall:** ECN on end-to-end path not always supported; DCTCP uses ECN for byte-granular congestion response.
- **Cross-Domain Aliases:** congestion-mark (control-numerical-opt), signal-feedback (information-theory-coding).

### [PRIM-050] tcp-window-scaling
- **Atom/Composite:** Primitive
- **Definition:** RFC 1323 window scaling: shift count in SYN/SYN-ACK increases receive window beyond 16-bit field (max 1 GB). Default shift = 14 (65 KB → 1 GB).
- **Cost Model:** Scale factor negotiation; enables high-BDP links (satellite, cellular, data center).
- **Real Wall:** Broken by some firewalls (misinterpret shift as unknown option); security concern (amplification attacks).
- **Cross-Domain Aliases:** capacity-window (control-numerical-opt), receiver-buffer-sizing (distributed-systems).

### [PRIM-051] tcp-timestamp
- **Atom/Composite:** Primitive
- **Definition:** TCP timestamps (RFC 1323): PAWS (Protection Against Wrapped Sequences) prevents data corruption from delayed/replayed segments on high-speed links.
- **Cost Model:** Timestamp option = 10 bytes; RTTM (RTT Measurement) from timestamp delta.
- **Real Wall:** Coarse-grained timestamps in some OS (1 ms resolution); timestamps must survive NAT.
- **Cross-Domain Aliases:** sequence-wrap-detection (information-theory-coding), time-stamp-ordering (database-streaming).

### [PRIM-052] tcp-fidelity
- **Atom/Composite:** Primitive
- **Definition:** TCP-Friendly Rate Control (TFRC): equation-based congestion control for non-TCP flows (RTP, DASH) that compete fairly with TCP without being overly aggressive.
- **Cost Model:** Throughput equation: X = s / (RTT·√(2·p/3) + 12·RTT·√(3·π·p/8)·p·(1+32·p²)); smooth rate changes.
- **Real Wall:** Doesn't react quickly to sudden capacity changes; better for long-running streaming than short bursts.
- **Cross-Domain Aliases:** fair-share-rate (control-numerical-opt), equation-based-pacing (information-theory-coding).

### [PRIM-053] quic-stream-multiplexing
- **Atom/Composite:** Composite
- **Definition:** QUIC: multiple logical streams within one connection (no head-of-line blocking); stream frames carry data; connection ID (CID) survives address change (mobility).
- **Cost Model:** 0-RTT and 1-RTT connection establishment (no SYN/SYN-ACK/ACK round trip); packet-level encryption (not byte-level like TLS).
- **Real Wall:** Connection migration (IP change → new path without reconnect); PATH_CHALLENGE/PATH_RESPONSE for path validation.
- **Cross-Domain Aliases:** http-streaming (networking), multi-stream-channel (distributed-systems).

### [PRIM-054] quic-connection-migration
- **Atom/Composite:** Primitive
- **Definition:** QUIC connection migration: client continues using same connection (same CID) after network change (WiFi→Cellular); no TCP connection break.
- **Cost Model:** PATH_CHALLENGE frames validate new path; PATH_RESPONSE confirms; min RTT on new path tracked separately.
- **Real Wall:** Middleboxes may not pass unknown UDP packets (QUIC is often blocked on corporate networks); path validation adds latency.
- **Cross-Domain Aliases:** session-migration (distributed-systems), transparent-handoff (signal-processing-rf).

### [PRIM-055] udp-datagram-service
- **Atom/Composite:** Primitive
- **Definition:** UDP: connectionless datagram service; no delivery guarantee, ordering, or flow control; minimal header (8 bytes).
- **Cost Model:** Zero overhead beyond checksum; per-datagram delivery; no connection state at sender.
- **Real Wall:** No congestion control (send at any rate); applications must handle loss, duplication, reordering; used for DNS, QUIC, VoIP, gaming.
- **Cross-Domain Aliases:** best-effort-unreliable (information-theory-coding), stateless-packet (distributed-systems).

### [PRIM-056] udp-multiplexing-demultiplexing
- **Atom/Composite:** Primitive
- **Definition:** UDP demultiplexes based on destination port; multiple applications can share same port (SO_REUSEPORT).
- **Cost Model:** O(1) port-based lookup; port space: 0–1023 (well-known), 1024–49151 (registered), 49152–65535 (ephemeral).
- **Real Wall:** Port exhaustion on high-connection servers; ephemeral port range varies by OS (Linux: 32768–60999).
- **Cross-Domain Aliases:** port-lookup (distributed-systems), packet-classification (networking).

## 5. Application / HTTP

### [PRIM-057] http-keepalive
- **Atom/Composite:** Primitive
- **Definition:** HTTP/1.1 persistent connection: TCP connection kept open for multiple request/response pairs; Connection: close header terminates.
- **Cost Model:** Saves TCP handshake overhead per request; pipelining (serial) allows out-of-order response delivery.
- **Real Wall:** Head-of-line blocking (pipelining still serializes requests); connection timeout (server closes after inactivity).
- **Cross-Domain Aliases:** session-persistence (distributed-systems), connection-pooling (database-streaming).

### [PRIM-058] http-2-multiplexing
- **Atom/Composite:** Composite
- **Definition:** HTTP/2: single TCP connection, multiple interleaved streams (binary frames); no head-of-line blocking; server push.
- **Cost Model:** HPACK header compression (static + dynamic table); stream priority (weight + dependency tree).
- **Real Wall:** TCP head-of-line blocking still present at transport layer; QUIC (HTTP/3) solves this at application layer.
- **Cross-Domain Aliases:** stream-multiplex (networking), frame-interleaving (information-theory-coding).

### [PRIM-059] http-3-quic-transport
- **Atom/Composite:** Composite
- **Definition:** HTTP/3 over QUIC: UDP-based transport; 0-RTT/1-RTT handshake; connection migration; loss recovery at stream level.
- **Cost Model:** HTTP/3 negotiation via Alt-Svc header or HTTP/2 ALTSVC frame; TLS 1.3 in QUIC (not separate layer).
- **Real Wall:** UDP port 443 often blocked on enterprise networks; middlebox interference with QUIC's wire image.
- **Cross-Domain Aliases:** http-streaming (networking), quic-stream-multiplex (networking).

### [PRIM-060] http-content-negotiation
- **Atom/Composite:** Primitive
- **Definition:** HTTP content negotiation: client advertises preferences (Accept, Accept-Language, Accept-Encoding, Accept-Charset) in request; server selects representation.
- **Cost Model:** Vary header tells caches which request headers affect response; cached responses vary by header combination.
- **Real Wall:** Caching complexity (must vary on multiple dimensions); server-side negotiation adds latency.
- **Cross-Domain Aliases:** format-negotiation (distributed-systems), adaptive-response (ml-training).

### [PRIM-061] http-caching
- **Atom/Composite:** Composite
- **Definition:** HTTP caching: response caching directives (Cache-Control, Expires, ETag, Last-Modified); conditional requests (If-None-Match, If-Modified-Since).
- **Cost Model:** Cache hit = 0 ms latency; cache miss = full RTT + processing; CDN edge caches extend origin cache lifetime.
- **Real Wall:** Stale-while-revalidate; must-revalidate; private vs. public caching; Vary: * complicates caching.
- **Cross-Domain Aliases:** result-cache (database-streaming), response-memoization (ml-training).

### [PRIM-062] http-state-machine
- **Atom/Composite:** Primitive
- **Definition:** HTTP finite state machine: request/response pairs over persistent connection; method semantics (GET idempotent, POST non-idempotent).
- **Cost Model:** State management via cookies (client-side) or session tokens; Cookie/Session header exchange.
- **Real Wall:** CSRF attacks (cross-site request forgery); SameSite cookie attribute mitigates; XSS bypasses same-origin policy.
- **Cross-Domain Aliases:** session-state (distributed-systems), state-transition (control-numerical-opt).

### [PRIM-063] websocket-full-duplex
- **Atom/Composite:** Composite
- **Definition:** WebSocket: persistent TCP connection with WebSocket handshake (HTTP Upgrade); bidirectional frames after CONNECT; ping/pong keepalive.
- **Cost Model:** Overhead: 2-byte frame header (4 bytes for masked client frames); binary or text frames.
- **Real Wall:** Proxy traversal (some corporate proxies close long-lived connections); no built-in routing/acknowledgment.
- **Cross-Domain Aliases:** persistent-channel (distributed-systems), bidirectional-stream (networking).

### [PRIM-064] http-3xx-redirect
- **Atom/Composite:** Primitive
- **Definition:** HTTP redirect (3xx): 301 (permanent), 302 (temporary), 307 (temporary, method preserved), 308 (permanent, method preserved); Location header specifies new URL.
- **Cost Model:** Extra RTT for redirect; permanent redirects should be cached by browsers; HSTS enforces HTTPS via 307.
- **Real Wall:** Redirect chains (A→B→C); chain depth tracked by browsers; CORS preflight (OPTIONS) for cross-origin.
- **Cross-Domain Aliases:** forwarding-hop (distributed-systems), path-rewrite (networking).

### [PRIM-065] tls-handshake
- **Atom/Composite:** Composite
- **Definition:** TLS 1.3: 1-RTT handshake (full handshake) + 0-RTT (early data, PSK); cipher suite negotiation; certificate authentication; key derivation (HKDF).
- **Cost Model:** 1-RTT = ClientHello → ServerHello → finished (2 round trips); 0-RTT = early data in ClientHello (replay risk).
- **Real Wall:** Post-quantum hybrid key exchange (X25519 + ML-KEM768); TLS 1.3 not supported by some legacy devices.
- **Cross-Domain Aliases:** authenticated-key-exchange (cryptography-hashing), secure-handshake (distributed-systems).

### [PRIM-066] tls-session-resumption
- **Atom/Composite:** Composite
- **Definition:** TLS session resumption: PSK (Pre-Shared Key) or session ticket; avoids full handshake; 0-RTT early data possible.
- **Cost Model:** Session ticket = encrypted state (no server-side state); PSK = shared secret established in prior handshake.
- **Real Wall:** 0-RTT early data is replayable (anti-replay required); stateful PSK requires server-side storage.
- **Cross-Domain Aliases:** session-cache (distributed-systems), key-reuse (cryptography-hashing).

## 6. DNS / Naming / Service Discovery

### [PRIM-067] dns-resolution
- **Atom/Composite:** Composite
- **Definition:** DNS: distributed hierarchical database; recursive resolver → root server → TLD (.com) → authoritative NS; A (IPv4), AAAA (IPv6), CNAME, MX, TXT records.
- **Cost Model:** DNS lookup latency: cache hit = μs, recursive = 10–100 ms; EDNS(0) client subnet (ECS) optimizes for CDN.
- **Real Wall:** DNSSEC provides authentication but not confidentiality; DNS over TLS (DoT, port 853) or DNS over HTTPS (DoH) encrypts queries.
- **Cross-Domain Aliases:** name-lookup (distributed-systems), hierarchical-resolution (information-theory-coding).

### [PRIM-068] dns-cache-poisoning
- **Atom/Composite:** Primitive
- **Definition:** DNS cache poisoning: attacker sends forged DNS response; transaction ID + source port randomization mitigates; Kaminsky attack (2008) exploited ID entropy.
- **Cost Model:** Attack complexity: 2¹⁶ × 2¹⁶ (transaction ID × source port); random source port increases difficulty.
- **Real Wall:** DNSSEC prevents poisoning (signed zones); 0x20 encoding (randomized letter case) provides entropy.
- **Cross-Domain Aliases:** cache-injection (distributed-systems), poisoning-attack (cryptography-hashing).

### [PRIM-069] anycast-dns
- **Atom/Composite:** Composite
- **Definition:** DNS root and TLD servers use anycast: same IP prefix advertised from multiple geographic locations; BGP routes clients to nearest node.
- **Cost Model:** Traffic distribution by AS path length + IGP metric; DDoS mitigation via traffic scrubbing at edges.
- **Real Wall:** Query patterns differ by resolver (recursive vs. stub); CDN anycast vs. DNS anycast serve different purposes.
- **Cross-Domain Aliases:** geographic-anycast (distributed-systems), nearest-server (networking).

### [PRIM-070] mdns-service-discovery
- **Atom/Composite:** Composite
- **Definition:** mDNS (Multicast DNS, RFC 6762): zero-configuration service discovery on local link;224.0.0.251 (IPv4) / ff02::fb (IPv6); announces and queries _service._proto.local.
- **Cost Model:** 1 query + N responses; TTL = 120 s; DnsServiceRegister/DnsServiceResolve for service browsing.
- **Real Wall:** Scopes: link-local only (not routed); Bonjour (Apple), Avahi (Linux), Chromecast use mDNS.
- **Cross-Domain Aliases:** local-discovery (distributed-systems), zero-conf-announce (networking).

### [PRIM-071] dnssec-chain-validation
- **Atom/Composite:** Composite
- **Definition:** DNSSEC: RRSIG (signed records), DS (delegation signer), DNSKEY (signing keys); chain of trust from root → TLD → zone.
- **Cost Model:** Validation: verify signature on each record set; TTL management for DS/DNSKEY changes (propagation delay).
- **Real Wall:** RRSIG expiry; key rollovers (KSK/ZSK separation); some resolvers don't validate (security theater).
- **Cross-Domain Aliases:** chain-of-trust (cryptography-hashing), authenticated-lookup (distributed-systems).

### [PRIM-072] dns-over-https
- **Atom/Composite:** Composite
- **Definition:** DoH (DNS over HTTPS, RFC 8484): DNS queries sent over HTTPS (port 443) to DoH server; GET or POST requests; JSON or wire-format responses.
- **Cost Model:** Hides DNS queries from network observers (privacy); uses HTTP/2 or HTTP/3 transport.
- **Real Wall:** Some networks block QUIC/HTTP/3 DoH; corporate DNS filtering bypassed by DoH; latency slightly higher than raw DNS.
- **Cross-Domain Aliases:** encrypted-lookup (cryptography-hashing), privacy-resolver (networking).

## 7. Security / Threat / Defense

### [PRIM-073] ddos-mitigation
- **Atom/Composite:** Composite
- **Definition:** DDoS mitigation: volumetric (scrubbing center absorbs traffic), protocol (challenge-response blocks spoofed), application-layer (rate limiting + CAPTCHA).
- **Cost Model:** Scrubbing center capacity (100s of Gbps); anycast distributes attack across PoPs; BGP flowspec routes null.
- **Real Wall:** Amplification attacks (NTP, DNS, Memcached); cryptomining coin hive; GRE flood; TCP state-exhaustion.
- **Cross-Domain Aliases:** traffic-scrubbing (distributed-systems), volumetric-filter (control-numerical-opt).

### [PRIM-074] bgp-flowspec
- **Atom/Composite:** Composite
- **Definition:** BGP Flowspec (RFC 5575): distributes traffic filtering rules via BGP; matches source/destination IP, port, protocol, DSCP, packet length; actions: drop, rate-limit, redirect.
- **Cost Model:** Flowspec rules distributed as NLRI; hardware TCAM at routers; convergence time = IGP convergence + BGP update.
- **Real Wall:** Not widely supported by all routers; rule cardinality limits; potential for rule injection attacks.
- **Cross-Domain Aliases:** policy-propagation (distributed-systems), traffic-filter (networking).

### [PRIM-075] rtbh-triggered-null-route
- **Atom/Composite:** Composite
- **Definition:** Remotely Triggered Black Hole (RTBH): BGP update with next-hop set to null interface; drops traffic at edge before it reaches target.
- **Cost Model:** BGP convergence time (minutes); Scrubbing center integration: trigger RTBH → redirect traffic to scrubbing center.
- **Real Wall:** Legitimate traffic also dropped; Flowspec more surgical; used for DDoS and network security incidents.
- **Cross-Domain Aliases:** blackhole-route (distributed-systems), traffic-null (networking).

### [PRIM-076] ids-signature-detection
- **Atom/Composite:** Composite
- **Definition:** Intrusion Detection (Snort/Suricata): pattern match on packet payload; rule language (Snort rule format); HTTP/decode预处理.
- **Cost Model:** Rule evaluation O(N·M) where N = rules, M = packet bytes; hyperscan for regex offload.
- **Real Wall:** Evasion: fragmentation, payload encoding, protocol confusion; false positive rate vs. true positive rate trade-off.
- **Cross-Domain Aliases:** pattern-match (information-theory-coding), anomaly-detection (ml-training).

### [PRIM-077] tls-inspection
- **Atom/Composite:** Composite
- **Definition:** TLS inspection: enterprise proxy intercepts TLS connection; re-encrypts with corporate cert; enables IDS/AV scanning of encrypted traffic.
- **Cost Model:** Certificate pinning bypass; BYOK (Bring Your Own Key) for customer-managed keys; performance overhead (latency + CPU).
- **Real Wall:** Privacy concerns; some applications pin certificates (browsers, mobile apps); U-turn traffic (internal-to-internal) handling.
- **Cross-Domain Aliases:** tls-intercept (cryptography-hashing), man-in-the-middle (distributed-systems).

### [PRIM-078] network-segmentation
- **Atom/Composite:** Primitive
- **Definition:** Network segmentation: split flat network into VLANs / subnets; microsegmentation (zero-trust): per-workload firewall rules; east-west vs. north-south traffic.
- **Cost Model:** Firewall rules scale with workload count; east-west flow visibility (NetFlow, sFlow).
- **Real Wall:** Overly permissive rules defeat segmentation; policy management complexity (thousands of rules).
- **Cross-Domain Aliases:** namespace-isolation (distributed-systems), access-control (cryptography-hashing).

## 8. Network Measurement / Telemetry

### [PRIM-079] netflow-collection
- **Atom/Composite:** Composite
- **Definition:** NetFlow v5/v9/IPFIX: export flow records (5-tuple + bytes/packets) from router/switch to collector; aggregation-based billing + security analytics.
- **Cost Model:** Flow cache size; template vs. data sets in IPFIX; sampling rate (1-in-N) reduces CPU/export overhead.
- **Real Wall:** Sampled NetFlow loses small flows (DDoS detection misses low-rate attacks); sFlow (statistical sampling) vs. NetFlow.
- **Cross-Domain Aliases:** flow-aggregation (database-streaming), traffic-accounting (information-theory-coding).

### [PRIM-080] sdn-controller-api
- **Atom/Composite:** Composite
- **Definition:** SDN controller (OpenFlow, ONOS, ODL): programmatic control of switch forwarding tables; northbound REST API for applications; southbound OpenFlow protocol.
- **Cost Model:** Flow table size (TCAM); reactive (packet-in) vs. proactive (pre-installed) flow rules; controller cluster for HA.
- **Real Wall:** Scale: thousands of switches; consistency vs. availability (CAP theorem for distributed controllers).
- **Cross-Domain Aliases:** centralized-control (distributed-systems), programmable-forwarding (networking).

### [PRIM-081] bgp-looking-glass
- **Atom/Composite:** Primitive
- **Definition:** BGP Looking Glass: public server that allows anyone to query BGP routes and router state (without authentication); reveals AS path, next-hop, MED, local preference.
- **Cost Model:** Read-only queries; used for route debugging, route hijack detection, AS path analysis.
- **Real Wall:** Doesn't expose full routing table; RIPE RIS (Routing Information Service) provides raw BGP data feeds.
- **Cross-Domain Aliases:** route-inspection (distributed-systems), policy-query (networking).

### [PRIM-082] tcpdump-packet-capture
- **Atom/Composite:** Primitive
- **Definition:** Packet capture (tcpdump, Wireshark): libpcap/winpcap/Npcap; kernel BPF filter reduces userspace copies; mmap ring buffer.
- **Cost Model:** Full packet capture at 10 Gbps = ~100 TB/day; data reduction: capture filters (BPF) + display filters + statistics.
- **Real Wall:** Storage cost; privacy (PCI/HIPAA compliance); CPU overhead for encryption (TLS/HTTPS not visible without MITM).
- **Cross-Domain Aliases:** traffic-sampling (signal-processing-rf), event-logging (distributed-systems).

### [PRIM-083] tcp-state-enumeration
- **Atom/Composite:** Primitive
- **Definition:** TCP state machine: CLOSED → SYN_SENT → ESTABLISHED → FIN_WAIT → CLOSED; half-open connections = SYN flood; TIME_WAIT = 2·MSL (60–120 s).
- **Cost Model:** Connection tracking table: O(N) for N concurrent connections; connection rate = new connections per second.
- **Real Wall:** SYN cookies (serverless TCP state during SYN flood); socket reuse; TCP_DEFER_ACCEPT.
- **Cross-Domain Aliases:** state-machine-tracking (control-numerical-opt), session-tracking (distributed-systems).

### [PRIM-084] bgp-route-monitor
- **Atom/Composite:** Composite
- **Definition:** BGP route monitoring: RouteViews (Oregon), RIPE RIS; BGP stream (BGPUpdate, withdrawal, peer up/down); route flap damping metrics.
- **Cost Model:** MRT format (RFC 6398);RIB dumps + updates; RouteViews = read-only looking glass + raw data feeds.
- **Real Wall:** Sampling rate (not full table at all peers); route collector location ≠ global view.
- **Cross-Domain Aliases:** as-path-tracking (networking), route-change-detection (distributed-systems).

### [PRIM-085] network-coding
- **Atom/Composite:** Primitive
- **Definition:** Random linear network coding over GF(2ⁿ): intermediate nodes linear-combine received packets; receivers solve linear system to recover original messages.
- **Cost Model:** Encoding/decoding O(N²) over GF(2⁸); requires finite field arithmetic; subspace coding for coded caching.
- **Real Wall:** Decoding complexity grows with generation size; coding coefficient overhead per packet; limited deployment (theoretical appeal, practical overhead).
- **Cross-Domain Aliases:** linear-coding (information-theory-coding), packet-combination (distributed-systems).

### [PRIM-086] tcp-bbr-pacing
- **Atom/Composite:** Primitive
- **Definition:** BBR pacing: send packets at pacing rate = BDP / RTprop (not cwnd-limited); pacing prevents burst-induced queue buildup.
- **Cost Model:** Pacing rate = bytes_per_rtt / RTprop; fq qdisc (fair queue) for per-flow pacing in Linux.
- **Real Wall:** Pacing vs. cwnd interaction; combined BBR+CUBIC not well-defined; pacing grain (timer vs. token bucket).
- **Cross-Domain Aliases:** rate-pacing (control-numerical-opt), smooth-transmission (signal-processing-rf).

### [PRIM-087] tcp-initial-congestion-window
- **Atom/Composite:** Primitive
- **Definition:** Initial CWND (IW): number of packets sender can transmit before first ACK. RFC 6928: IW = 10 MSS (~15 KB).
- **Cost Model:** IW too small → slow transfer for short flows; IW too large → packet loss + retransmit overhead.
- **Real Wall:** Modern Linux allows IW = 10–30 MSS; long-range networks (satellite) benefit from larger IW.
- **Cross-Domain Aliases:** initial-throttle (control-numerical-opt), warm-start (ml-training).

### [PRIM-088] tcp-keepalive
- **Atom/Composite:** Primitive
- **Definition:** TCP keepalive: after idle period (default 2 hours), probe packets (no data) detect dead peers; 9 probes × 75 s = 11 min 15 s before declaring dead.
- **Cost Model:** Keepalive timer overhead; consumes bandwidth; can trigger stateful firewall pinholes.
- **Real Wall:** Some NAT devices have shorter connection timeout than TCP keepalive; application-level keepalive (HTTP) more reliable.
- **Cross-Domain Aliases:** heartbeat-ping (distributed-systems), liveness-probe (networking).

### [PRIM-089] quic-udp-offload
- **Atom/Composite:** Primitive
- **Definition:** QUIC kernel bypass / UDP offload: GSO (Generic Segmentation Offload), CHECKSUM_OFFLOAD; userspace QUIC stacks (lsquic, quiche, ngtcp2).
- **Cost Model:** Kernel bypass reduces context switch overhead; GSO batches packets; DPDK for full userspace networking.
- **Real Wall:** GSO size must fit MTU; UDP checksum offload to NIC; IRQ coalescing for high-throughput NICs.
- **Cross-Domain Aliases:** kernel-bypass (distributed-systems), dma-transfer (signal-processing-rf).

### [PRIM-090] srh-segment-routing-ipv6
- **Atom/Composite:** Composite
- **Definition:** SRH (Segment Routing Header, RFC 8754): SRv6 SID encoded in IPv6 destination address + optional SRH; SID = locator (40 bits) + function (16 bits) + args (72 bits).
- **Cost Model:** Per-packet SID processing at each hop; SRH optional ( SID is in destination address, SRH allows more than 128 SIDs).
- **Real Wall:** Large SID tables; SRH processing in hardware requires Tofino or similar programmable ASICs.
- **Cross-Domain Aliases:** segment-routing (networking), source-routing (distributed-systems).

### [PRIM-091] tls-1-3-zero-rtt
- **Atom/Composite:** Primitive
- **Definition:** TLS 1.3 0-RTT: client sends early data using PSK; 0-RTT data is encrypted with early key; risk of replay attacks.
- **Cost Model:** Replay detection: anti-replay cache or non-repeating nonce; 0-RTT data limited by server policy.
- **Real Wall:** Replayable (no non-repudiation); TLS layer can't prevent application-level replay; HTTP/3 0-RTT is QUIC 0-RTT.
- **Cross-Domain Aliases:** early-data (distributed-systems), pre-shared-key (cryptography-hashing).

### [PRIM-092] bgp-community-annotation
- **Atom/Composite:** Primitive
- **Definition:** BGP communities (48-bit: AA:NN): transitive and non-transitive; used for route policy signaling between ASes (e.g., NO_EXPORT, NO_ADVERTISE, RTBH trigger).
- **Cost Model:** Route filtering and attribute modification based on community value; policy routing without changing next-hop.
- **Real Wall:** Community space is per-AS (AA = AS number); some ISPs strip communities; well-known communities defined in RFC 1997.
- **Cross-Domain Aliases:** route-annotation (distributed-systems), policy-tag (networking).

### [PRIM-093] tcp-syncookies
- **Atom/Composite:** Primitive
- **Definition:** SYN cookies: server encodes TCP state (MSS, window scale, timestamp) in SYN-ACK sequence number; no per-connection state during SYN flood.
- **Cost Model:** Sequence number = timestamp + MSS + wscale encoded in 32-bit ISN; cryptographic hash (secure cookie) prevents forgery.
- **Real Wall:** ECN bits not encoded (compatibility issue); MSS limited to 8 values; timestamp-based cookies (Mitchell's algorithm) for more features.
- **Cross-Domain Aliases:** stateless-connection (distributed-systems), client-puzzle (cryptography-hashing).

### [PRIM-094] sdn-data-plane-programming
- **Atom/Composite:** Composite
- **Definition:** P4 (Programming Protocol-independent Packet Processors): data plane programming; match-action tables; parsers for custom protocol headers; extern objects (checksum, hash, counters).
- **Cost Model:** Compiler (p4c) generates switch ASIC / FPGA / software target; table size + extern complexity determines hardware feasibility.
- **Real Wall:** Targets: Tofino ASIC, Tofino2, NPU, vSwitch (BMv2), eBPF; not all P4 programs compile to all targets.
- **Cross-Domain Aliases:** programmable-pipeline (signal-processing-rf), match-action (control-numerical-opt).

### [PRIM-095] bgp-hop-bypass
- **Atom/Composite:** Primitive
- **Definition:** BGP route hop bypass (hot potato routing): local policy minimizes cost to exit AS; ASes may prefer close exit despite longer AS path.
- **Cost Model:** Local preference overrides AS path length; exit router selection = IGP cost + BGP MED + pre-policy community.
- **Real Wall:** Hot potato routing changes with IGP metric changes; consistent exit (MED) helps peers predict exit router.
- **Cross-Domain Aliases:** exit-router-selection (networking), policy-based-routing (distributed-systems).

### [PRIM-096] tcp-accurate-sndbuf
- **Atom/Composite:** Primitive
- **Definition:** TCP send buffer auto-tuning: kernel dynamically sizes send buffer based on RTT, bandwidth, and measured loss; socket option SO_SNDBUF.
- **Cost Model:** Buffer too small = throughput limited by RTT (bandwidth-delay product); buffer too large = memory bloat.
- **Real Wall:** BDP = bandwidth × RTT; Linux autotuning: base = 16 KB, max = 6 MB (tcp_rmem[2]); TCP_NOTSENT_LOWAT socket option.
- **Cross-Domain Aliases:** buffer-sizing (control-numerical-opt), flow-control-tuning (networking).

### [PRIM-097] ipv6-dual-stack
- **Atom/Composite:** Composite
- **Definition:** Dual-stack: IPv4 and IPv6 simultaneous deployment; DNS A + AAAA records;Happy Eyeballs (RFC 6555): try both, use whichever succeeds first.
- **Cost Model:** Dual-stack overhead (two routing tables, two address families); IPv6 preference reduces dual-stack penalty.
- **Real Wall:** NAT64/DNS64 (IPv6-only client accessing IPv4-only server);464XLAT (carrier-grade NAT for IPv6-only mobile networks).
- **Cross-Domain Aliases:** protocol-duality (distributed-systems), address-family-transition (networking).

### [PRIM-098] tcp-timestamp-based-rtt
- **Atom/Composite:** Primitive
- **Definition:** TCP RTT measurement via timestamps: send TSval in each segment, receive TSecr echoed in ACK; RTT = ACK_time - TSval_of_segment.
- **Cost Model:** Per-packet RTT measurement (vs. per-ACK); RTTVAR computed from RTT samples (Jacobson/Karels); used for RTO calculation.
- **Real Wall:** Timestamp must be echoed; TSval not echoed on pure ACKs → cannot measure one-way delay.
- **Cross-Domain Aliases:** round-trip-sample (distributed-systems), latency-sampling (signal-processing-rf).

### [PRIM-099] ip-anycast-route-injection
- **Atom/Composite:** Composite
- **Definition:** IP anycast: announce same /24 (or larger) prefix from multiple PoPs via BGP; shortest AS path wins; traffic lands at nearest PoP.
- **Cost Model:** Route propagation delay; health checks must withdraw prefix at failed PoP; load distribution ≈ geographic routing.
- **Real Wall:** Per-flow routing (BGP is per-prefix, not per-flow); connection may land at different PoP mid-session; TTL = 1 for LAN anycast.
- **Cross-Domain Aliases:** geographic-announcement (distributed-systems), nearest-exit (networking).

### [PRIM-100] quic-connection-id
- **Atom/Composite:** Primitive
- **Definition:** QUIC Connection ID (CID): variable-length identifier; survives address change (migration); server can track client across network changes; not visible to middleboxes.
- **Cost Model:** CID length negotiated; client-selected CID (1–20 bytes); server-selected CID for server-to-client packets.
- **Real Wall:** CID required for connection migration; stateless reset token for connection termination; version negotiation uses CID=0.
- **Cross-Domain Aliases:** session-identifier (distributed-systems), connection-migration (networking).

## Appendix: Primitive Count

Total primitives in networking domain: **100**
