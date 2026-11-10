use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstHexModule {
    pub name: &'static str,
    pub ast_kind: &'static str,
    pub hex_coordinates: [i32; 6],
    pub hyperplane_valid: bool,
    pub metric_tensor_type: &'static str,
}

pub const OMEGA_MASTER_CATALOG: &[AstHexModule] = &[
    AstHexModule {
        name: "Point 0 (Source / Singularity)",
        ast_kind: "RootAxiom",
        hex_coordinates: [0, 0, 0, 0, 0, 0],
        hyperplane_valid: true,
        metric_tensor_type: "Identity (Delta)",
    },
    AstHexModule {
        name: "RhizomeOS P2P Mesh & Gossip Plane",
        ast_kind: "ControlFlow",
        hex_coordinates: [12, -4, -8, 2, -2, 0],
        hyperplane_valid: true,
        metric_tensor_type: "Isotropic Flat",
    },
    AstHexModule {
        name: "Radicle-Style DAG Lineage & Clocks",
        ast_kind: "DataBinding",
        hex_coordinates: [-8, 14, -6, 4, -2, -2],
        hyperplane_valid: true,
        metric_tensor_type: "Causal Exponential",
    },
    AstHexModule {
        name: "Riemannian Manifold Gluing (H1=0)",
        ast_kind: "Constraint",
        hex_coordinates: [6, 6, -12, 0, 0, 0],
        hyperplane_valid: true,
        metric_tensor_type: "Overlap Jacobian",
    },
    AstHexModule {
        name: "Mutual Credit Graph Clearing",
        ast_kind: "Transform",
        hex_coordinates: [-10, 2, 8, -4, 6, -2],
        hyperplane_valid: true,
        metric_tensor_type: "Weighted Graph Flow",
    },
    AstHexModule {
        name: "AIMC / Memristive Crossbars",
        ast_kind: "Transform",
        hex_coordinates: [15, -15, 0, 5, -5, 0],
        hyperplane_valid: true,
        metric_tensor_type: "Conductance Gradient",
    },
    AstHexModule {
        name: "JIT Holographic Circuits",
        ast_kind: "Transform",
        hex_coordinates: [-18, 12, 6, -6, 12, -6],
        hyperplane_valid: true,
        metric_tensor_type: "Acoustic-Optical Coupled",
    },
    AstHexModule {
        name: "AST-to-HEX Compiler Pipeline",
        ast_kind: "Transform",
        hex_coordinates: [4, -10, 6, -2, 4, -2],
        hyperplane_valid: true,
        metric_tensor_type: "Depth-Density Exponential",
    },
];

pub fn validate_hyperplanes() -> bool {
    OMEGA_MASTER_CATALOG.iter().all(|m| {
        let sum: i32 = m.hex_coordinates.iter().sum();
        sum == 0 && m.hyperplane_valid
    })
}

// Newly added ASTX Coordinate Module for Google Docs Archive Synchronization
pub const ASTX_ARCHIVE_MODULE: AstHexModule = AstHexModule {
    name: "Google Docs Archive ASTX Ingestion Substrate",
    ast_kind: "DataBinding",
    hex_coordinates: [10, -5, -5, 2, -2, 0],
    hyperplane_valid: true,
    metric_tensor_type: "Archive Hyper-Tensor",
};
