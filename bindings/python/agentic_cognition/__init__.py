"""AgenticCognition — Longitudinal user modeling for AI agents.

Python bindings for the AgenticCognition Rust library.
"""

__version__ = "0.1.0"

from .cognition import CognitionEngine, Model, Belief, Prediction

__all__ = [
    "CognitionEngine",
    "Model",
    "Belief",
    "Prediction",
    "__version__",
]
