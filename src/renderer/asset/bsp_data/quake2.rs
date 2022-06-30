use crate::common::mathlib::vector3::Vector3f;

enum LumpIndex
{
	LUMP_ENTITIES,
	LUMP_PLANES,
	LUMP_VERTEXES,
	LUMP_VISIBILITY,
	LUMP_NODES,
	LUMP_TEXINFO,
	LUMP_FACES,
	LUMP_LIGHTING,
	LUMP_LEAFS,
	LUMP_LEAFFACES,
	LUMP_LEAFBRUSHES,
	LUMP_EDGES,
	LUMP_SURFEDGES,
	LUMP_MODEL,
	LUMP_BRUSHES,
	LUMP_BRUSHSIDES,
	LUMP_POP,
	LUMP_AREAS,
	LUMP_AREAPORTALS,
	HEADER_LUMPS // =19
};

struct Lump {
    offset: i32,
    length: i32,
}

struct Header {
    ident:      i32,
    version:    i32,
    lumps:      [Array; 19]
}

struct Model {
    mins:   Vector3f,
    maxs:   Vector3f,
    origin: Vector3f,
    headnode:   i32,
    firstface:  i32,
    numfaces:   i32
}

struct Shader {
    shader: String,
    surfaceFlags:   i32,
    contentFlags:   i32,
}

struct Edge {
    v: [u16;2],
}

struct Face {
    planenum:   u16,
    side:   i16,
    firstedge:  i32,
    numedges:   i16,
    texinfo:    i16,
    styles: [u8; 4],
    lightofs:   i32
}

struct Plane {
    normal: Vector3,
    dist:   f32,
    types: i32
}

struct Node {
    planenum:   i32,
    children:   [i32; 2],
    mins:   [i16; 3],
    maxs:   [i16; 3],
    firstface:  u16,
    numface:    u16
}

struct Leaf {
    contents:   i32,
    cluster:    i16,
    area:       i16,
    mins:       [i16; 3],
    maxs:       [i16; 3],
    firstleafface:  u16,
    numleaffaces:   u16,
    firstleafbrush: u16,
    numleafbrushes: u16,
}

struct Texinfo {
    textureVecs:    [[f32; 4]; 2],
    flags:  i32,
    value:  i32,
    texture:    [i8; 32],
    nexttexinfo:    i32
}

struct BrushSide {
    planenum:   u16,
    texinfo:    i16
}

struct Brush {
    firstside:  i32,
    numsides:   i32,
    contents:   i32
}

struct Fog {
    shader: [i8; 64],
    brushnum:   i32,
    visibleside:    i32

}

struct Drawvert {
    vec:    Vector3f,
    st: [f32; 2],
    lightmap: [[f32; 2]; 4],
    normal: Vector3f,
    color:  [[u8; 4]; 4]
}

enum SurfaceType
{
	MST_BAD,
	MST_PLANAR,
	MST_PATCH,
	MST_TRIANGLE_SOUP,
	MST_FLARE,
	MST_FOLIAGE
}

struct GridPoint {
    ambient:    [[u8; 3]; 4],
    directed:   [[u8; 3]; 4],
    styles:     [u8; 4],
    latlong:    [u8; 2]
}

struct DrawSurface {
    shader_num:  i32,
    fognum: i32,
    surfacetype:    i32,
    firstVet:   i32,
    numVerts:   i32,
    firstIndex: i32,
    numIndexes: i32,
    lightmapStyles: [u8; 4],
    vertexStyles:   [u8; 4],
    ligthmapNum:    [i32; 4],
    lightmapX:  [i32; 4],
    lightmapY:  [i32; 4],
    ligthmapWidth:  i32,
    lightmapHeight: i32,
    lightmapOrigin: Vector3f,
    lightmapVec:    Vector3f,
    patchWidth: i32,
    patchHeight:    i32
}

struct Advertissment {
    cellid: i32,
    normal: Vector3f,
    rect:   [Vector3f; 4],
    model:  [char; 64],
}

struct Epair {
    next:   Option<Box<Epair>>,
    key:    String,
    value:  String
}

struct Entity {
    origin: Vector3f,
    firstbrush: i32,
    numnrush:   i32,
    epairs:     Option<Box<Vec<Epair>>>,
    areaportalnum:  i32,
    portalareas:    [i32; 2],

}

struct Vis {
    numcluster: i32,
    bitofs:     [[i32 ;2]; 8]
}

struct AreaPortal {
    portalnum:  i32,
    otherare:   i32
}

struct Area {
    numareaportal:  i32,
    firstareaportal:    i32
}

pub struct BSP{

}
