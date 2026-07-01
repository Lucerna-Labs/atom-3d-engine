"""
Memory object schema for the LoRA Memory Experiment.

Each memory is an episodic scene — something experienced, not something explained.
The model should feel like it has BEEN THERE, not that it READ ABOUT IT.
"""

from dataclasses import dataclass, field
from typing import Optional
from enum import Enum


class MemoryFlavor(Enum):
    SCENE = "scene"          # Full narrative scene with sensory detail
    PRACTICE = "practice"    # Practicing a skill, with mistakes and corrections
    DISCOVERY = "discovery"  # Moment of realization or insight


class MemoryDominance(Enum):
    IDENTITY = "identity"        # Core to who the reasoner is
    OPERATIONAL = "operational"   # Skill being actively used
    INTEGRATED = "integrated"     # Skill internalized, used without thinking


@dataclass
class Memory:
    id: str
    domain: str                    # Which domain this memory belongs to
    title: str
    age: Optional[int] = None      # Fictional age of the "experiencer" (for echo chains)
    year: Optional[int] = None     # Fictional year
    dominance: MemoryDominance = MemoryDominance.INTEGRATED
    load_bearing: bool = False     # Required by behavioral spec
    behaviors_encoded: list[str] = field(default_factory=list)  # Behavior IDs
    echoes_from: list[str] = field(default_factory=list)        # IDs of earlier memories this echoes
    echoes_to: list[str] = field(default_factory=list)          # IDs of later memories this echoes to
    sensory_anchor: Optional[str] = None   # Key sensory detail that grounds the memory
    emotional_signature: str = ""   # Brief emotional color
    body: str = ""                  # The actual memory text — episodic, scene-based
    review_status: str = "draft"    # draft, reviewed, final

    def to_dict(self) -> dict:
        return {
            "id": self.id,
            "domain": self.domain,
            "title": self.title,
            "age": self.age,
            "year": self.year,
            "dominance": self.dominance.value,
            "load_bearing": self.load_bearing,
            "behaviors_encoded": self.behaviors_encoded,
            "echoes_from": self.echoes_from,
            "echoes_to": self.echoes_to,
            "sensory_anchor": self.sensory_anchor,
            "emotional_signature": self.emotional_signature,
            "body": self.body,
            "review_status": self.review_status,
        }

    @classmethod
    def from_dict(cls, d: dict) -> "Memory":
        d["dominance"] = MemoryDominance(d.get("dominance", "integrated"))
        return cls(**d)


@dataclass
class ExperimentConfig:
    """Configuration for a simulation run."""
    name: str
    domains: list[str]                    # Which domains to include
    memories_per_behavior: int = 3        # How many memories per behavior
    echo_density: float = 0.3            # Fraction of memories with echo connections
    load_bearing_ratio: float = 0.2      # Fraction of memories that are load_bearing
    total_target: int = 250              # Target total memories
    flavor_override: Optional[str] = None  # Override domain's default flavor

    @property
    def total_behaviors(self) -> int:
        from simulation.domains import ALL_DOMAINS
        return sum(len(ALL_DOMAINS[d].behaviors) for d in self.domains)

    @property
    def estimated_memories(self) -> int:
        return self.total_behaviors * self.memories_per_behavior