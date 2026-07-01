# AI Accelerator Primitives for Ordo

**Date:** 2026-06-20  
**Source:** 5 AI accelerator UAPI headers from Linux kernel  
**Total:** 156 defines, 82 structs, ~33 KB analysis

---

## Executive Summary

The Linux kernel exposes **5 major AI accelerator interfaces** through DRM (Direct Rendering Manager):

| Accelerator | Vendor | Target Use Case | Complexity |
|-------------|--------|-----------------|------------|
| **AMD XDNA** | AMD | Ryzen AI NPU (laptop/desktop) | High (32 structs) |
| **Intel VPU** | Intel | Movidius Myriad X (vision processing) | Medium-High (12 structs, 72 defines) |
| **Qualcomm QAIC** | Qualcomm | Cloud AI 100 (datacenter) | High (24 structs) |
| **ARM Ethos-U** | ARM | Embedded microNPUs (IoT/edge) | Low (8 structs, minimal API) |
| **Rocket** | Intel | Discrete GPU compute (Arc GPUs) | Low-Medium (6 structs) |

**Key Insight:** All follow the same architectural pattern:
1. **Buffer objects** - Allocate GPU/NPU memory
2. **Context creation** - Set up execution environment
3. **Command submission** - Queue work to hardware
4. **Synchronization** - Wait for completion via fences/eventfd
5. **Query results** - Retrieve output buffers

---

## Common Architectural Pattern

### Phase 1: Device Discovery & Opening

```rust
// All accelerators use DRM subsystem
let fd = open("/dev/dri/card0", O_RDWR)?;  // Primary GPU
let fd = open("/dev/dri/renderD128", O_RDWR)?;  // Render node (no display)

// Query device capabilities
let mut param = drm_ivpu_param {
    param: DRM_IVPU_PARAM_DEVICE_ID,
    value: 0,
};
ioctl(fd, DRM_IOCTL_IVPU_GET_PARAM, &mut param)?;
println!("Device ID: 0x{:x}", param.value);
```

### Phase 2: Buffer Object Allocation

```c
// AMD XDNA
struct amdxdna_drm_create_bo {
    __u32 size;           // Buffer size in bytes
    __u32 flags;          // BO_FLAGS_* 
    __u32 handle;         // Returned handle
    __u64 mapp_offset;    // For mmap()
};

// Intel VPU
struct drm_ivpu_bo_create {
    __u64 size;
    __u32 flags;          // BO_MAPPABLE, BO_DMA_MEM, etc.
    __u32 handle;
    __u64 offset;         // GPU virtual address
};

// Qualcomm QAIC
struct qaic_create_bo {
    __u32 size;
    __u32 flags;
    __u32 handle;
    __u64 offset;
};
```

**Buffer Types:**
- **Model weights** - Read-only, loaded once
- **Input tensors** - Write-only from CPU, read by NPU
- **Output tensors** - Read-only from CPU, written by NPU
- **Command buffers** - Executable instructions for NPU
- **Scratch memory** - Temporary workspace

### Phase 3: Hardware Context Creation

```c
// AMD XDNA - Most sophisticated context API
struct amdxdna_drm_create_hwctx {
    __u64 ext;              // Extension pointer
    __u64 ext_flags;        // Extension flags
    __u64 qos_p;            // QoS parameters pointer
    __u32 umq_bo;           // User mode queue BO handle
    __u32 log_buf_bo;       // Log buffer BO handle
    __u32 max_opc;          // Max outstanding commands
    __u32 num_tiles;        // NPU tile count
    __u32 mem_size;         // Context memory size
    __u32 umq_doorbell;     // Doorbell register offset
    __u32 handle;           // Returned context handle
    __u32 syncobj_handle;   // Sync object for fencing
};

// QoS parameters (performance tuning)
struct amdxdna_qos_info {
    __u32 gops;             // Giga-operations per second target
    __u32 fps;              // Frames per second target
    __u32 dma_bandwidth;    // DMA bandwidth requirement
    __u32 latency;          // Max latency in ms
    __u32 frame_exec_time;  // Expected frame execution time
    __u32 priority;         // QOS_* priority level
};

// Priority levels
#define AMDXDNA_QOS_REALTIME_PRIORITY  0x100
#define AMDXDNA_QOS_HIGH_PRIORITY      0x180
#define AMDXDNA_QOS_NORMAL_PRIORITY    0x200
#define AMDXDNA_QOS_LOW_PRIORITY       0x280
```

**Intel VPU Context Parameters:**
```c
#define DRM_IVPU_PARAM_CONTEXT_PRIORITY    6  // Deprecated
#define DRM_IVPU_PARAM_CONTEXT_ID          7  // Context identifier
#define DRM_IVPU_PARAM_UNIQUE_INFERENCE_ID 10 // Unique inference session ID

// Priority levels
#define DRM_IVPU_CONTEXT_PRIORITY_IDLE      0
#define DRM_IVPU_CONTEXT_PRIORITY_NORMAL    1
#define DRM_IVPU_CONTEXT_PRIORITY_FOCUS     2
#define DRM_IVPU_CONTEXT_PRIORITY_REALTIME  3
```

### Phase 4: Command Submission

#### AMD XDNA - Explicit Command Buffer

```c
struct amdxdna_cmd_submit {
    struct amdxdna_cmd_header header;
    struct amdxdna_cmd_buffer buffers[AMDXDNA_MAX_BUFFERS];
    struct amdxdna_cmd_fence fence;
};

struct amdxdna_cmd_header {
    __u32 type;             // Command type
    __u32 flags;            // Command flags
    __u32 size;             // Total command size
    __u32 rsvd;
};

struct amdxdna_cmd_buffer {
    __u32 bo_handle;        // Buffer object handle
    __u32 flags;            // Buffer flags
    __u64 offset;           // Offset within BO
    __u64 size;             // Buffer size
};

struct amdxdna_cmd_fence {
    __u64 seq_num;          // Sequence number
    __u32 syncobj_handle;   // Sync object handle
    __u32 flags;            // Fence flags
};

// Command types
#define AMDXDNA_CMD_SUBMIT_RUN    0x01  // Execute inference
#define AMDXDNA_CMD_SUBMIT_WAIT   0x02  // Wait for completion
#define AMDXDNA_CMD_SUBMIT_QUERY  0x03  // Query status
```

#### Intel VPU - Job-Based Submission

```c
struct drm_ivpu_submit {
    __u32 ctx_id;           // Context ID
    __u32 engine_id;        // Engine type
    __u64 job_ptr;          // Pointer to job structure
    __u64 fence_ptr;        // Fence for completion
};

struct drm_ivpu_job {
    __u32 flags;            // Job flags
    __u32 priority;         // Job priority
    __u64 cmd_buffer_addr;  // Command buffer GPU address
    __u32 cmd_buffer_size;
    __u32 num_bos;          // Number of buffer objects
    __u64 bo_handles_ptr;   // Array of BO handles
};

// Engine types
#define IVPU_ENGINE_COMPUTE   0  // Neural compute engine
#define IVPU_ENGINE_DECODE    1  // Video decode
#define IVPU_ENGINE_ENCODE    2  // Video encode
```

#### Qualcomm QAIC - Manage + Execute Model

```c
// Step 1: Manage session (load model, configure)
struct qaic_manage {
    __u32 type;             // QAIC_MANAGE_* type
    __u32 size;             // Message size
    __u64 msg_ptr;          // Message pointer
};

// Manage message types
#define QAIC_TRANS_ACTIVATE_FROM_USR    7   // Load model
#define QAIC_TRANS_DEACTIVATE_FROM_USR  10  // Unload model
#define QAIC_TRANS_STATUS_FROM_USR      12  // Query status

// Step 2: Execute inference
struct qaic_execute_bo {
    __u32 bo_handle;        // Command buffer BO
    __u32 flags;            // Execution flags
};

// Step 3: Wait for completion
struct qaic_wait_bo {
    __u32 bo_handle;
    __u32 flags;
    __s64 timeout_ns;       // Timeout in nanoseconds
};
```

#### ARM Ethos-U - Minimalist Approach

```c
// Ethos-U has simplest API - just submit command stream
struct drm_ethosu_submit {
    __u64 jobs;             // Pointer to job array
    __u32 job_count;        // Number of jobs
    __u32 pad;
};

struct drm_ethosu_job {
    __u32 cmd_bo;           // Command stream BO handle
    __u32 sram_size;        // SRAM usage
    __u32 region_bo_handles[ETHOSU_MAX_REGIONS];  // Memory regions
};

#define ETHOSU_MAX_REGIONS  8  // Max memory regions per job
```

### Phase 5: Synchronization

All accelerators support multiple sync mechanisms:

#### Fence-Based Sync (AMD XDNA)

```c
struct amdxdna_cmd_fence {
    __u64 seq_num;          // Monotonic sequence number
    __u32 syncobj_handle;   // DRM sync object
    __u32 flags;
};

// Wait on fence
ioctl(fd, DRM_IOCTL_SYNC_OBJ_WAIT, &sync_wait);
```

#### Eventfd Signaling (Intel VPU)

```c
// Create eventfd for async notification
int efd = eventfd(0, EFD_CLOEXEC);

// Associate with submission
struct drm_ivpu_fence {
    __u32 ctx_id;
    __u32 fd;               // Eventfd FD
    __u64 value;            // Signal value
};

// When job completes, eventfd becomes readable
struct pollfd pfd = { .fd = efd, .events = POLLIN };
poll(&pfd, 1, timeout_ms);
```

#### Polling (Qualcomm QAIC)

```c
struct qaic_wait_bo {
    __u32 bo_handle;
    __s64 timeout_ns;       // -1 = infinite, 0 = non-blocking
};

// Non-blocking check
wait.timeout_ns = 0;
ioctl(fd, DRM_IOCTL_QAIC_WAIT_BO, &wait);
if (errno == EAGAIN) {
    // Not ready yet
}
```

### Phase 6: Performance Metrics

**Intel VPU - Metric Streaming:**

```c
// Start metric collection
struct drm_ivpu_metric_streamer_start {
    __u32 ctx_id;
    __u32 period_us;        // Sampling period in microseconds
};

// Get collected metrics
struct drm_ivpu_metric_streamer_get_data {
    __u32 ctx_id;
    __u32 size;             // Buffer size
    __u64 data_ptr;         // Output buffer
};

// Metrics include:
// - NPU utilization %
// - Memory bandwidth
// - Power consumption
// - Temperature
// - Frame execution time
```

**Qualcomm QAIC - Per-BO Stats:**

```c
struct qaic_perf_stats_bo {
    __u32 bo_handle;
    __u32 flags;
    __u64 cycles;           // Execution cycles
    __u64 bytes_read;       // Bytes read from memory
    __u64 bytes_written;    // Bytes written to memory
    __u64 latency_ns;       // Total latency
};
```

---

## Ordo Integration Patterns

### Pattern 1: Unified Accelerator Trait

```rust
/// Common interface for all AI accelerators
pub trait AiAccelerator: Send + Sync {
    /// Allocate buffer object on accelerator
    fn alloc_buffer(&self, size: usize, flags: BufferFlags) -> Result<BufferHandle>;
    
    /// Create execution context with QoS parameters
    fn create_context(&self, qos: QosParams) -> Result<ContextHandle>;
    
    /// Submit command buffer for execution
    fn submit(&self, ctx: ContextHandle, cmd: CommandBuffer) -> Result<FenceHandle>;
    
    /// Wait for completion
    fn wait(&self, fence: FenceHandle, timeout: Duration) -> Result<CompletionStatus>;
    
    /// Query performance metrics
    fn get_metrics(&self, ctx: ContextHandle) -> Result<AcceleratorMetrics>;
    
    /// Get device info
    fn device_info(&self) -> DeviceInfo;
}

pub struct DeviceInfo {
    pub vendor: AcceleratorVendor,
    pub device_id: u32,
    pub revision: u32,
    pub memory_size: u64,
    pub num_compute_units: u32,
    pub max_clock_mhz: u32,
    pub supported_precisions: Vec<Precision>,
}

pub enum AcceleratorVendor {
    Amd,
    Intel,
    Qualcomm,
    Arm,
}

pub struct QosParams {
    pub gops_target: u32,      // Giga-ops per second
    pub fps_target: u32,       // Frames per second
    pub latency_budget_ms: u32,
    pub priority: Priority,
    pub dma_bandwidth_mbps: u32,
}

pub enum Priority {
    Realtime,
    High,
    Normal,
    Low,
    Idle,
}
```

### Pattern 2: Model Lifecycle Management

```rust
pub struct ModelSession {
    accelerator: Box<dyn AiAccelerator>,
    ctx: ContextHandle,
    model_weights: BufferHandle,
    input_buffers: Vec<BufferHandle>,
    output_buffers: Vec<BufferHandle>,
    scratch_space: BufferHandle,
}

impl ModelSession {
    /// Load neural network model onto accelerator
    pub fn load_model(
        accelerator: &dyn AiAccelerator,
        model_path: &Path,
        qos: QosParams,
    ) -> Result<Self> {
        // Create context with QoS
        let ctx = accelerator.create_context(qos)?;
        
        // Allocate buffer for model weights
        let model_size = std::fs::metadata(model_path)?.len();
        let weights_buf = accelerator.alloc_buffer(model_size as usize, BufferFlags::READ_ONLY)?;
        
        // Map buffer and load weights
        let mut mapped = weights_buf.map::<u8>()?;
        let mut file = File::open(model_path)?;
        file.read_exact(&mut mapped)?;
        drop(mapped);  // Unmap
        
        // Allocate I/O buffers
        let input_buffers = vec![
            accelerator.alloc_buffer(INPUT_SIZE, BufferFlags::CPU_WRITE)?;
            NUM_INPUT_BUFFERS
        ];
        let output_buffers = vec![
            accelerator.alloc_buffer(OUTPUT_SIZE, BufferFlags::CPU_READ)?;
            NUM_OUTPUT_BUFFERS
        ];
        
        // Allocate scratch space for intermediate results
        let scratch = accelerator.alloc_buffer(SCRATCH_SIZE, BufferFlags::RW)?;
        
        Ok(Self {
            accelerator: Box::new(accelerator),
            ctx,
            model_weights: weights_buf,
            input_buffers,
            output_buffers,
            scratch_space: scratch,
        })
    }
    
    /// Run inference on input data
    pub fn infer(&self, input: &[f32]) -> Result<Vec<f32>> {
        // Copy input to first input buffer
        let mut in_map = self.input_buffers[0].map::<f32>()?;
        in_map.copy_from_slice(input);
        drop(in_map);
        
        // Build command buffer
        let cmd = CommandBuffer::new()
            .load_weights(&self.model_weights)
            .set_input(&self.input_buffers[0])
            .set_output(&self.output_buffers[0])
            .set_scratch(&self.scratch_space)
            .execute();
        
        // Submit and wait
        let fence = self.accelerator.submit(self.ctx, cmd)?;
        self.accelerator.wait(fence, Duration::from_secs(5))?;
        
        // Read output
        let out_map = self.output_buffers[0].map::<f32>()?;
        Ok(out_map.to_vec())
    }
    
    /// Batch inference with multiple inputs
    pub fn infer_batch(&self, inputs: &[Vec<f32>]) -> Result<Vec<Vec<f32>>> {
        // Submit all inputs concurrently
        let mut fences = Vec::new();
        for (i, input) in inputs.iter().enumerate() {
            let buf_idx = i % self.input_buffers.len();
            
            // Copy input
            let mut in_map = self.input_buffers[buf_idx].map::<f32>()?;
            in_map.copy_from_slice(input);
            drop(in_map);
            
            // Build and submit command
            let cmd = CommandBuffer::new()
                .load_weights(&self.model_weights)
                .set_input(&self.input_buffers[buf_idx])
                .set_output(&self.output_buffers[buf_idx])
                .set_scratch(&self.scratch_space)
                .execute();
            
            let fence = self.accelerator.submit(self.ctx, cmd)?;
            fences.push((fence, buf_idx));
        }
        
        // Wait for all completions
        let mut results = Vec::with_capacity(inputs.len());
        for (fence, buf_idx) in fences {
            self.accelerator.wait(fence, Duration::from_secs(5))?;
            let out_map = self.output_buffers[buf_idx].map::<f32>()?;
            results.push(out_map.to_vec());
        }
        
        Ok(results)
    }
}
```

### Pattern 3: Multi-Accelerator Orchestration

```rust
/// Manages multiple accelerators for parallel inference
pub struct AcceleratorPool {
    devices: Vec<Box<dyn AiAccelerator>>,
    sessions: HashMap<DeviceId, ModelSession>,
    scheduler: WorkScheduler,
}

impl AcceleratorPool {
    /// Discover all available AI accelerators
    pub fn discover() -> Result<Self> {
        let mut devices = Vec::new();
        
        // Scan DRM render nodes
        for i in 0..16 {
            let path = format!("/dev/dri/renderD{}", 128 + i);
            if let Ok(fd) = OpenOptions::new().read(true).write(true).open(&path) {
                // Probe device type
                match Self::probe_device(fd)? {
                    AcceleratorType::AmdXdna => {
                        devices.push(Box::new(AmdXdnaDevice::from_fd(fd)));
                    }
                    AcceleratorType::IntelVpu => {
                        devices.push(Box::new(IntelVpuDevice::from_fd(fd)));
                    }
                    AcceleratorType::QualcommQaic => {
                        devices.push(Box::new(QualcommQaicDevice::from_fd(fd)));
                    }
                    AcceleratorType::ArmEthosu => {
                        devices.push(Box::new(ArmEthosuDevice::from_fd(fd)));
                    }
                    _ => {}  // Unknown or unsupported
                }
            }
        }
        
        Ok(Self {
            devices,
            sessions: HashMap::new(),
            scheduler: WorkScheduler::new(),
        })
    }
    
    /// Load model onto all available accelerators
    pub fn load_model_everywhere(&mut self, model_path: &Path) -> Result<()> {
        for device in &self.devices {
            let qos = QosParams {
                gops_target: device.device_info().max_gops,
                fps_target: 30,
                latency_budget_ms: 50,
                priority: Priority::Normal,
                dma_bandwidth_mbps: 10000,
            };
            
            let session = ModelSession::load_model(device.as_ref(), model_path, qos)?;
            self.sessions.insert(device.device_info().device_id, session);
        }
        
        Ok(())
    }
    
    /// Distribute batch across all accelerators
    pub fn infer_distributed(&self, inputs: Vec<Vec<f32>>) -> Result<Vec<Vec<f32>>> {
        // Partition inputs across devices
        let chunk_size = (inputs.len() + self.devices.len() - 1) / self.devices.len();
        let chunks: Vec<_> = inputs.chunks(chunk_size).collect();
        
        // Spawn inference on each device
        let handles: Vec<_> = chunks.iter().enumerate().map(|(i, chunk)| {
            let device = &self.devices[i % self.devices.len()];
            let session = &self.sessions[&device.device_info().device_id];
            let chunk = chunk.to_vec();
            
            std::thread::spawn(move || {
                let mut results = Vec::new();
                for input in chunk {
                    let result = session.infer(&input)?;
                    results.push(result);
                }
                Ok::<_, Error>(results)
            })
        }).collect();
        
        // Collect results
        let mut all_results = Vec::new();
        for handle in handles {
            let results = handle.join().unwrap()?;
            all_results.extend(results);
        }
        
        Ok(all_results)
    }
}
```

### Pattern 4: Subagent Integration

```rust
/// Ordo subagent that uses AI accelerators
pub struct AiSubagent {
    pool: Arc<Mutex<AcceleratorPool>>,
    model_registry: ModelRegistry,
    active_inferences: AtomicUsize,
}

#[ordo_message]
pub enum AiSubagentMessage {
    /// Load a new model
    LoadModel {
        model_id: String,
        model_path: PathBuf,
        precision: Precision,
    },
    
    /// Run inference
    Infer {
        model_id: String,
        input: Tensor,
        priority: Priority,
        reply_to: Sender<Result<Tensor>>,
    },
    
    /// Batch inference
    InferBatch {
        model_id: String,
        inputs: Vec<Tensor>,
        priority: Priority,
        reply_to: Sender<Result<Vec<Tensor>>>,
    },
    
    /// Query device status
    GetDeviceStatus {
        reply_to: Sender<Vec<DeviceInfo>>,
    },
    
    /// Get performance metrics
    GetMetrics {
        model_id: String,
        reply_to: Sender<AcceleratorMetrics>,
    },
}

impl SubagentHandler for AiSubagent {
    async fn handle(&self, msg: AiSubagentMessage) -> Result<()> {
        match msg {
            AiSubagentMessage::LoadModel { model_id, model_path, precision } => {
                // Download/prepare model if needed
                let prepared_path = self.model_registry.prepare_model(&model_path, precision).await?;
                
                // Load onto all accelerators
                let mut pool = self.pool.lock().await;
                pool.load_model_everywhere(&prepared_path)?;
                
                info!("Model {} loaded onto {} accelerators", 
                      model_id, pool.devices.len());
            }
            
            AiSubagentMessage::Infer { model_id, input, priority, reply_to } => {
                self.active_inferences.fetch_add(1, Ordering::Relaxed);
                
                let pool = self.pool.clone();
                let model_id = model_id.clone();
                
                tokio::task::spawn_blocking(move || {
                    let pool = pool.lock().await;
                    // Find best device based on priority
                    let result = pool.infer_single(&model_id, input.as_slice(), priority);
                    
                    let _ = reply_to.send(result);
                    // Note: can't decrement counter here due to async
                });
            }
            
            AiSubagentMessage::InferBatch { model_id, inputs, priority, reply_to } => {
                let pool = self.pool.clone();
                
                tokio::task::spawn_blocking(move || {
                    let pool = pool.lock().await;
                    let result = pool.infer_distributed(inputs.into_iter()
                        .map(|t| t.as_slice().to_vec())
                        .collect());
                    
                    let _ = reply_to.send(result);
                });
            }
            
            AiSubagentMessage::GetDeviceStatus { reply_to } => {
                let pool = self.pool.lock().await;
                let devices: Vec<_> = pool.devices.iter()
                    .map(|d| d.device_info())
                    .collect();
                
                let _ = reply_to.send(Ok(devices));
            }
            
            AiSubagentMessage::GetMetrics { model_id, reply_to } => {
                let pool = self.pool.lock().await;
                // Aggregate metrics from all devices
                let metrics = pool.get_aggregate_metrics(&model_id)?;
                let _ = reply_to.send(Ok(metrics));
            }
        }
        
        Ok(())
    }
}
```

---

## Specific Vendor Details

### AMD XDNA (Ryzen AI)

**Unique Features:**
- **Hardware contexts** with explicit state tracking (IDLE → ACTIVE)
- **QoS parameters** (GOPS, FPS, latency, DMA bandwidth, priority)
- **Tile-based architecture** (num_tiles field)
- **User mode queue** with doorbell ringing
- **Sync objects** integrated with DRM scheduler

**Best For:** Laptop/desktop AI workloads with Ryzen AI NPUs

**Key Structs:** 32 (most complex API)

### Intel VPU (Movidius)

**Unique Features:**
- **Multiple engines** (compute, decode, encode)
- **Metric streaming** for real-time performance monitoring
- **Command queues** with explicit create/destroy
- **User pointer BOs** (zero-copy from userspace)
- **Preemption support** with dedicated buffer

**Best For:** Vision processing, video analytics, edge AI

**Key Defines:** 72 (most configuration options)

### Qualcomm QAIC (Cloud AI 100)

**Unique Features:**
- **Manage protocol** with typed transactions (21 transaction types)
- **Slice-based memory** allocation
- **Partial execution** for large models
- **Performance stats** per buffer object
- **Semaphore-based synchronization** (init/inc/dec/wait)

**Best For:** Datacenter inference, large model serving

**Key Structs:** 24, Transaction types: 21

### ARM Ethos-U (Embedded)

**Unique Features:**
- **Minimalist API** (only 8 structs)
- **SRAM-constrained** operation
- **Region-based memory** management (max 8 regions)
- **Command stream** direct submission

**Best For:** Microcontrollers, IoT devices, ultra-low-power edge

**Complexity:** Lowest (designed for simplicity)

### Rocket (Intel Arc GPU Compute)

**Unique Features:**
- **Task-based submission** (regcmd arrays)
- **Input/output BO separation**
- **Explicit prepare/fini** lifecycle
- **DMA address exposure** for advanced use cases

**Best For:** GPU compute workloads on Intel Arc GPUs

**Complexity:** Low-Medium

---

## Comparison Table

| Feature | AMD XDNA | Intel VPU | Qualcomm QAIC | ARM Ethos-U | Rocket |
|---------|----------|-----------|---------------|-------------|--------|
| **Structs** | 32 | 12 | 24 | 8 | 6 |
| **Defines** | 26 | 72 | 49 | 1 | 8 |
| **Context QoS** | ✅ Full | ✅ Priority | ❌ | ❌ | ❌ |
| **Metric Streaming** | ❌ | ✅ Yes | Per-BO | ❌ | ❌ |
| **Multi-Engine** | ❌ | ✅ 3 engines | ❌ | ❌ | ❌ |
| **User Pointer BOs** | ❌ | ✅ Yes | ❌ | ❌ | ❌ |
| **Partial Execution** | ❌ | ❌ | ✅ Yes | ❌ | ❌ |
| **Tile Architecture** | ✅ Yes | ❌ | ❌ | ❌ | ❌ |
| **Command Queues** | UMQ | ✅ Explicit | ❌ | ❌ | ❌ |
| **Sync Objects** | ✅ DRM | ✅ Eventfd | Polling | ❌ | ❌ |

---

## Files Generated

```
F:\OPENCLAW-PROJECTS\linux-primitives-extracted\output\high-priority\
├── amdxdna_accel_analysis.md    (32 structs - most complex)
├── ethosu_accel_analysis.md     (8 structs - simplest)
├── ivpu_accel_analysis.md       (72 defines - most config options)
├── qaic_accel_analysis.md       (24 structs, 21 transaction types)
└── rocket_accel_analysis.md     (6 structs - task-based)
```

---

## Next Steps for Ordo

1. **Implement `AiAccelerator` trait** for each vendor backend
2. **Build model loader** with ONNX/TensorFlow/PyTorch support
3. **Create command buffer builder** DSL for easy kernel definition
4. **Integrate with Ordo bus** as dedicated AI subagent
5. **Add automatic device selection** based on workload characteristics
6. **Implement fallback to CPU** (via mistral.rs or ONNX Runtime) when no accelerator available
7. **Build benchmarking suite** to measure GOPS/Watt across devices

This gives Ordo **hardware-accelerated ML inference** across the full spectrum: embedded (Ethos-U), edge (VPU), desktop (XDNA), and datacenter (QAIC)!
