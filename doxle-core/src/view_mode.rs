
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    TwoD,   //2D floor plan view (orthogrpahic, top-down) 
    ThreeD,  //3D wall view (perspective, angled)
}

impl ViewMode {
    // Toggle between 2D and 3D modes
    pub fn toggle(&mut self){
        // Dereference the self before we can alter it
        *self = match self {
            ViewMode::TwoD => ViewMode::ThreeD,
            ViewMode::ThreeD => ViewMode::TwoD,
        };
    }

    pub fn is_2d(&self) -> bool {
        matches!(self, ViewMode::TwoD)
    }
    
    pub fn is_3d(&self) -> bool {
        matches!(self, ViewMode::ThreeD)
    }
}
