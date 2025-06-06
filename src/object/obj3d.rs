use glam::Vec3;


pub struct Transform3d {
    parrent_position: Vec3,
    pub global_position: Vec3,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}


pub struct Obj3d {
    pub transform: Transform3d,
    htransform: Transform3d,
}
impl Obj3d {
    
}