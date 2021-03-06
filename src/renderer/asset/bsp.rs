use crate::io::importer::pak::PakEntry;
use crate::common:mathlib::vector3::Vector3f;

trait BSPFunction {
    fn from_pak(file_data: PakEntry) -> Self;
    fn open(filepath: String) -> Self;

    fn render_level(Self, pos: Vector3f);
    fn render_faces(Self, index: u32);
    fn render_leafs(Self, index: u32);
    fn render_nodes(Self, index: u32, cameraLeaf: i32, cameraPos: Vector3f);
    fn traverse_tree(Self, pos: Vector3f, nodeIndex: i32) -> i32;
    fn pre_renderer();

    fn copy_lump<T>(Self, lump_flags: u32, dest: &Vec<T>) -> u32;
}
