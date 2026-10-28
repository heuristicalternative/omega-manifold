#![allow(dead_code)]

// Double Category 2-Cell Execution Grid for policy verification and workflow state transitions.

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HorizontalProcess {
    pub id: String,
    pub source: String,
    pub target: String,
    pub payload: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VerticalMorphism {
    pub id: String,
    pub source: String,
    pub target: String,
    pub policy: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DoubleCell2Cell {
    pub top: HorizontalProcess,
    pub bottom: HorizontalProcess,
    pub left: VerticalMorphism,
    pub right: VerticalMorphism,
}

impl DoubleCell2Cell {
    pub fn new(
        top: HorizontalProcess,
        bottom: HorizontalProcess,
        left: VerticalMorphism,
        right: VerticalMorphism,
    ) -> Self {
        Self { top, bottom, left, right }
    }

    /// Validate that boundary conditions match across the 2-cell grid.
    pub fn verify_2_cell_coherence(&self) -> bool {
        self.top.source == self.bottom.source
            && self.top.target == self.bottom.target
            && self.left.source == self.right.source
            && self.left.target == self.right.target
            && self.top.source == self.left.source
            && self.top.target == self.left.target
    }

    /// Horizontal composition law: top and bottom processes must share endpoints.
    pub fn horizontal_composition_valid(&self) -> bool {
        self.top.source == self.bottom.source && self.top.target == self.bottom.target
    }

    /// Vertical composition law: left and right morphisms must share endpoints.
    pub fn vertical_composition_valid(&self) -> bool {
        self.left.source == self.right.source && self.left.target == self.right.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hp(id: &str, src: &str, tgt: &str) -> HorizontalProcess {
        HorizontalProcess {
            id: id.to_string(),
            source: src.to_string(),
            target: tgt.to_string(),
            payload: "state".to_string(),
        }
    }

    fn vm(id: &str, src: &str, tgt: &str) -> VerticalMorphism {
        VerticalMorphism {
            id: id.to_string(),
            source: src.to_string(),
            target: tgt.to_string(),
            policy: "zk-proof".to_string(),
        }
    }

    #[test]
    fn test_coherent_2_cell() {
        let cell = DoubleCell2Cell::new(
            hp("top", "A", "B"),
            hp("bottom", "A", "B"),
            vm("left", "A", "B"),
            vm("right", "A", "B"),
        );
        assert!(cell.verify_2_cell_coherence());
        assert!(cell.horizontal_composition_valid());
        assert!(cell.vertical_composition_valid());
    }

    #[test]
    fn test_incoherent_2_cell() {
        let cell = DoubleCell2Cell::new(
            hp("top", "A", "B"),
            hp("bottom", "A", "C"),
            vm("left", "A", "B"),
            vm("right", "A", "B"),
        );
        assert!(!cell.verify_2_cell_coherence());
        assert!(!cell.horizontal_composition_valid());
    }

    #[test]
    fn test_vertical_incoherence() {
        let cell = DoubleCell2Cell::new(
            hp("top", "A", "B"),
            hp("bottom", "A", "B"),
            vm("left", "A", "B"),
            vm("right", "A", "C"),
        );
        assert!(!cell.verify_2_cell_coherence());
        assert!(!cell.vertical_composition_valid());
    }
}
