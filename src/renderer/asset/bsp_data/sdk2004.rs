use crate::common::mathlib::vector3::Vector3f;

struct Lump {
    filelen:    i32, // 4 bytes
    fileofs:    i32, // 4 bytes
    version:    i32, // 4 bytes
    fourCC:     [char; 4] // 4 bytes ()
} //Lump as actually 16 bytes

struct Header {
    ident:          i32,
    version:        i32,
    lumps:          [Lumps; 50],
    map_revision:   i32;
}

struct Face {
    planenum:   u16,
    side:       u8,
    onNode:     u8,
    firstedge:  i32,
    numedges:   i8,
    texinfo:    i8,
    dispinfo:   i8,
    surfaceFogVolumeID:     i8,
    styles:     [u8;4],
    light_ofs:  i32,
    area:       f32,
    LightmapTextureMinsInLuxels:    [i32; 2],
    LightmapTextureSizeInLuxels:    [i32, 2],
    origFace:       i32,
    numPrims:       u16,
    firstPrimsID:   u16,
    smoothingGroups:    u32
}

struct Node {
    planenum:   i32,
    children:   [i32; 2],
    mins:       [u16; 3],
    maxs:       [u16; 3],
    firstface:  u16,
    numfaces:   u16,
    area:       i16,
    padding:    i16
}

struct Leaf {
    contents:   i32,
    cluster:    i16,
    area:       i16,
    flags:      i16,
    mins:       [i16; 3],
    maxs:       [i16; 3],
    firstleafface:  u16,
    numleaffaces:   u16,
    firstleafbrush: u16,
    numleafbrushes: u16,
    leafWaterDataID: i16
}

struct Texinfo {
    textureVecs:    [[f32; 4]; 2],
    lightmapsVecs:  [[f32; 4]; 2],
    flags:      i32,
    texdata:    i32
}

struct Texdata {
    reflectivity:   Vector3f,
    nameStringTableID:  i32,
    width:  i32,
    height: i32,
    view_width: i32,
    view_height:    i32
}

pub struct BSP{


}
