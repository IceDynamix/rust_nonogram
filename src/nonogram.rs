use ggez::{graphics, nalgebra::Point2, Context, GameError, GameResult};
use std::fs;

pub type Nonogram = Vec<Vec<Block>>;

pub const BLOCK_CHAR: char = '#';
const BLOCK_SIZE: f32 = 35.0;
const BLOCK_COLOR: graphics::Color = graphics::WHITE;

#[derive(Debug, Clone)]
pub enum Block {
    Empty,
    Filled,
    Crossed,
    Marked,
}

impl Block {
    pub fn to_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        match self {
            Block::Filled => graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE),
                BLOCK_COLOR,
            ),
            Block::Empty => graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE),
                graphics::Color::from_rgb(50, 50, 50),
            ),
            Block::Crossed => graphics::MeshBuilder::new()
                .line(
                    &[Point2::new(0.0, 0.0), Point2::new(BLOCK_SIZE, BLOCK_SIZE)],
                    2.0,
                    BLOCK_COLOR,
                )?
                .line(
                    &[Point2::new(BLOCK_SIZE, 0.0), Point2::new(0.0, BLOCK_SIZE)],
                    2.0,
                    BLOCK_COLOR,
                )?
                .build(ctx),
            Block::Marked => graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(0.0),
                graphics::Rect::new(0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE),
                BLOCK_COLOR,
            ),
        }
    }

    pub fn update() -> GameResult<()> {
        Ok(())
    }
}

pub fn draw_nonogram(nonogram: &Nonogram, ctx: &mut Context) -> GameResult<()> {
    let (window_width, window_height) = graphics::drawable_size(ctx);

    let nonogram_height = nonogram.len() as f32 * BLOCK_SIZE;
    let nonogram_width = nonogram[0].len() as f32 * BLOCK_SIZE;

    let center_x = window_width / 2.0 - nonogram_width / 2.0;
    let center_y = window_height / 2.0 - nonogram_height / 2.0;

    let params = graphics::DrawParam::default();
    for (y, line) in nonogram.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            let mesh = block.to_mesh(ctx)?;
            graphics::draw(
                ctx,
                &mesh,
                params.dest(Point2::new(
                    center_x + x as f32 * BLOCK_SIZE,
                    center_y + y as f32 * BLOCK_SIZE,
                )),
            )?;
        }
    }
    Ok(())
}

pub fn load_nonogram_from_file(filename: &str) -> GameResult<Nonogram> {
    let file_content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => return Err(GameError::FilesystemError(e.to_string())),
    };

    let lines = file_content.lines();
    let width = lines.clone().map(|line| line.len()).max().unwrap_or(0);

    // height == 0 implies width == 0
    if width == 0 {
        return Err(GameError::ResourceLoadError(
            "Nonogram has invalid dimensions".to_string(),
        ));
    }

    let data: Nonogram = lines.map(|line| parse_line(line, width)).collect();

    Ok(data)
}

fn parse_line(line: &str, width: usize) -> Vec<Block> {
    let mut data: Vec<Block> = vec![Block::Empty; width];
    for (i, c) in line.chars().enumerate() {
        data[i] = match c {
            BLOCK_CHAR => Block::Filled,
            _ => Block::Empty,
        };
    }
    data
}
