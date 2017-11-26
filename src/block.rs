use vertex::Vertex;
use std::hash::{Hash, Hasher};
use glium::vertex::VertexBuffer;
use glium::backend::Facade;
use color::Color;
use color;
use space::{Position, Direction, get_normal_for_triangle, UP, DOWN, NORTH, EAST, SOUTH, WEST};

/// Size of a block (in metres)
const BLOCK_SIZE: f32 = 0.5;

/// Vertices of a cube
///
/// ordering is important - so that the correct faces get culled
const CUBE_VERTICES: [Position; 24] = [
    // face
    Position { x: -BLOCK_SIZE, y: -BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: -BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: -BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    // face
    Position { x: BLOCK_SIZE, y: -BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: -BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: -BLOCK_SIZE },
    // face
    Position { x: BLOCK_SIZE, y: -BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: -BLOCK_SIZE, y: -BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: -BLOCK_SIZE, y: BLOCK_SIZE, z: -BLOCK_SIZE },
    // face
    Position { x: -BLOCK_SIZE, y: -BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: -BLOCK_SIZE, y: -BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: -BLOCK_SIZE, y: BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: -BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    // face
    Position { x: -BLOCK_SIZE, y: -BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: -BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: -BLOCK_SIZE, y: -BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: -BLOCK_SIZE, z: BLOCK_SIZE },
    // face
    Position { x: -BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: BLOCK_SIZE },
    Position { x: -BLOCK_SIZE, y: BLOCK_SIZE, z: -BLOCK_SIZE },
    Position { x: BLOCK_SIZE, y: BLOCK_SIZE, z: -BLOCK_SIZE },
];

fn cube_normals() -> [Direction; 6] {
    [
        get_normal_for_triangle(CUBE_VERTICES[0], CUBE_VERTICES[1], CUBE_VERTICES[2]),
        get_normal_for_triangle(CUBE_VERTICES[4], CUBE_VERTICES[5], CUBE_VERTICES[6]),
        get_normal_for_triangle(CUBE_VERTICES[8], CUBE_VERTICES[9], CUBE_VERTICES[10]),
        get_normal_for_triangle(CUBE_VERTICES[12], CUBE_VERTICES[13], CUBE_VERTICES[14]),
        get_normal_for_triangle(CUBE_VERTICES[16], CUBE_VERTICES[17], CUBE_VERTICES[18]),
        get_normal_for_triangle(CUBE_VERTICES[20], CUBE_VERTICES[21], CUBE_VERTICES[22]),
    ]
}

/// Create a vertex buffer for a cube centred at (x, y, z)
pub fn make_cube<F: ? Sized>(facade: &F, position: &Position, color: Color) -> VertexBuffer<Vertex> where F: Facade {
    let x = position[0];
    let y = position[1];
    let z = position[2];
    let normals = cube_normals();
    VertexBuffer::new(facade, &[
        Vertex::new([x + CUBE_VERTICES[0][0], y + CUBE_VERTICES[0][1], z + CUBE_VERTICES[0][2]], color, normals[0].into()),
        Vertex::new([x + CUBE_VERTICES[1][0], y + CUBE_VERTICES[1][1], z + CUBE_VERTICES[1][2]], color, normals[0].into()),
        Vertex::new([x + CUBE_VERTICES[2][0], y + CUBE_VERTICES[2][1], z + CUBE_VERTICES[2][2]], color, normals[0].into()),
        Vertex::new([x + CUBE_VERTICES[3][0], y + CUBE_VERTICES[3][1], z + CUBE_VERTICES[3][2]], color, normals[0].into()),

        Vertex::new([x + CUBE_VERTICES[4][0], y + CUBE_VERTICES[4][1], z + CUBE_VERTICES[4][2]], color, normals[1].into()),
        Vertex::new([x + CUBE_VERTICES[5][0], y + CUBE_VERTICES[5][1], z + CUBE_VERTICES[5][2]], color, normals[1].into()),
        Vertex::new([x + CUBE_VERTICES[6][0], y + CUBE_VERTICES[6][1], z + CUBE_VERTICES[6][2]], color, normals[1].into()),
        Vertex::new([x + CUBE_VERTICES[7][0], y + CUBE_VERTICES[7][1], z + CUBE_VERTICES[7][2]], color, normals[1].into()),

        Vertex::new([x + CUBE_VERTICES[8][0], y + CUBE_VERTICES[8][1], z + CUBE_VERTICES[8][2]], color, normals[2].into()),
        Vertex::new([x + CUBE_VERTICES[9][0], y + CUBE_VERTICES[9][1], z + CUBE_VERTICES[9][2]], color, normals[2].into()),
        Vertex::new([x + CUBE_VERTICES[10][0], y + CUBE_VERTICES[10][1], z + CUBE_VERTICES[10][2]], color, normals[2].into()),
        Vertex::new([x + CUBE_VERTICES[11][0], y + CUBE_VERTICES[11][1], z + CUBE_VERTICES[11][2]], color, normals[2].into()),

        Vertex::new([x + CUBE_VERTICES[12][0], y + CUBE_VERTICES[12][1], z + CUBE_VERTICES[12][2]], color, normals[3].into()),
        Vertex::new([x + CUBE_VERTICES[13][0], y + CUBE_VERTICES[13][1], z + CUBE_VERTICES[13][2]], color, normals[3].into()),
        Vertex::new([x + CUBE_VERTICES[14][0], y + CUBE_VERTICES[14][1], z + CUBE_VERTICES[14][2]], color, normals[3].into()),
        Vertex::new([x + CUBE_VERTICES[15][0], y + CUBE_VERTICES[15][1], z + CUBE_VERTICES[15][2]], color, normals[3].into()),

        Vertex::new([x + CUBE_VERTICES[16][0], y + CUBE_VERTICES[16][1], z + CUBE_VERTICES[16][2]], color, normals[4].into()),
        Vertex::new([x + CUBE_VERTICES[17][0], y + CUBE_VERTICES[17][1], z + CUBE_VERTICES[17][2]], color, normals[4].into()),
        Vertex::new([x + CUBE_VERTICES[18][0], y + CUBE_VERTICES[18][1], z + CUBE_VERTICES[18][2]], color, normals[4].into()),
        Vertex::new([x + CUBE_VERTICES[19][0], y + CUBE_VERTICES[19][1], z + CUBE_VERTICES[19][2]], color, normals[4].into()),

        Vertex::new([x + CUBE_VERTICES[20][0], y + CUBE_VERTICES[20][1], z + CUBE_VERTICES[20][2]], color, normals[5].into()),
        Vertex::new([x + CUBE_VERTICES[21][0], y + CUBE_VERTICES[21][1], z + CUBE_VERTICES[21][2]], color, normals[5].into()),
        Vertex::new([x + CUBE_VERTICES[22][0], y + CUBE_VERTICES[22][1], z + CUBE_VERTICES[22][2]], color, normals[5].into()),
        Vertex::new([x + CUBE_VERTICES[23][0], y + CUBE_VERTICES[23][1], z + CUBE_VERTICES[23][2]], color, normals[5].into()),
    ]).unwrap()
}

#[derive(Debug)]
pub struct BlockType {
    pub id: u8,
    pub color: Color,
}

impl Hash for BlockType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for BlockType {
    fn eq(&self, other: &BlockType) -> bool {
        self.id == other.id
    }
}

impl Eq for BlockType {}

pub static BLOCKS: [&BlockType; 2] = [
    &BlockType {
        id: 0,
        color: color::GREEN,
    },
    &BlockType {
        id: 1,
        color: color::BROWN,
    },
];
