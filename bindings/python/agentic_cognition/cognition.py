"""Python interface to AgenticCognition.

Uses ctypes FFI to call the Rust library.
"""

import ctypes
import json
import os
from dataclasses import dataclass
from typing import Optional, List, Dict, Any


@dataclass
class Model:
    """A living user model."""
    id: str
    health: float = 0.0
    confidence: float = 0.0
    lifecycle_stage: str = "Birth"
    evidence_count: int = 0


@dataclass
class Belief:
    """A single belief in the model."""
    id: str
    content: str
    domain: str
    confidence: float
    state: str = "Forming"


@dataclass
class Prediction:
    """A preference prediction."""
    id: str
    item: str
    predicted_preference: float
    confidence: float
    reasoning: List[str]


class CognitionEngine:
    """Main interface to AgenticCognition.

    Example:
        engine = CognitionEngine()
        model = engine.create_model()
        belief = engine.add_belief(model.id, "I value honesty", "values", 0.9)
        prediction = engine.predict_preference(model.id, "remote work")
    """

    def __init__(self, storage_path: Optional[str] = None):
        """Initialize the cognition engine.

        Args:
            storage_path: Path to storage directory. Defaults to ~/.agentic/cognition/
        """
        self._storage = storage_path or os.path.expanduser("~/.agentic/cognition")
        self._lib = self._load_library()

    def _load_library(self):
        """Load the native library via ctypes."""
        lib_name = "libagentic_cognition_ffi"

        # Try common library paths
        for ext in [".dylib", ".so", ".dll"]:
            for prefix in ["", "lib"]:
                for path in [
                    os.path.join(os.path.dirname(__file__), f"{prefix}{lib_name}{ext}"),
                    os.path.join(os.path.dirname(__file__), "..", "..", "target", "release", f"{prefix}agentic_cognition_ffi{ext}"),
                    os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug", f"{prefix}agentic_cognition_ffi{ext}"),
                ]:
                    if os.path.exists(path):
                        return ctypes.CDLL(path)

        return None  # Library not found, will use subprocess fallback

    def create_model(self) -> Model:
        """Create a new user model."""
        if self._lib and hasattr(self._lib, 'acog_create_model'):
            result = self._lib.acog_create_model()
            if result:
                data = json.loads(ctypes.string_at(result).decode())
                self._lib.acog_free_string(result)
                return Model(id=data["model_id"])

        # Fallback: use CLI
        import subprocess
        result = subprocess.run(
            ["acog", "model", "create", "--storage", self._storage],
            capture_output=True, text=True
        )
        if result.returncode == 0:
            data = json.loads(result.stdout)
            return Model(id=data.get("model_id", ""))
        raise RuntimeError(f"Failed to create model: {result.stderr}")

    def add_belief(
        self,
        model_id: str,
        content: str,
        domain: str = "other",
        confidence: float = 0.5,
    ) -> Belief:
        """Add a belief to a model."""
        import subprocess
        result = subprocess.run(
            [
                "acog", "belief", "add", model_id, content,
                "--domain", domain,
                "--confidence", str(confidence),
                "--storage", self._storage,
            ],
            capture_output=True, text=True
        )
        if result.returncode == 0:
            data = json.loads(result.stdout)
            return Belief(
                id=data.get("belief_id", ""),
                content=content,
                domain=domain,
                confidence=confidence,
            )
        raise RuntimeError(f"Failed to add belief: {result.stderr}")

    def get_vitals(self, model_id: str) -> Dict[str, Any]:
        """Get model vital signs."""
        import subprocess
        result = subprocess.run(
            ["acog", "model", "vitals", model_id, "--storage", self._storage],
            capture_output=True, text=True
        )
        if result.returncode == 0:
            return json.loads(result.stdout)
        raise RuntimeError(f"Failed to get vitals: {result.stderr}")

    def predict_preference(self, model_id: str, item: str) -> Prediction:
        """Predict preference for an item."""
        import subprocess
        result = subprocess.run(
            ["acog", "predict", "preference", model_id, item, "--storage", self._storage],
            capture_output=True, text=True
        )
        if result.returncode == 0:
            data = json.loads(result.stdout)
            return Prediction(
                id=data.get("prediction_id", ""),
                item=data.get("item", item),
                predicted_preference=data.get("predicted_preference", 0.5),
                confidence=data.get("confidence", 0.0),
                reasoning=data.get("reasoning", []),
            )
        raise RuntimeError(f"Failed to predict: {result.stderr}")

    def soul_reflection(self, model_id: str) -> Dict[str, Any]:
        """Perform soul reflection."""
        import subprocess
        result = subprocess.run(
            ["acog", "model", "soul", model_id, "--storage", self._storage],
            capture_output=True, text=True
        )
        if result.returncode == 0:
            return json.loads(result.stdout)
        raise RuntimeError(f"Failed to reflect: {result.stderr}")
