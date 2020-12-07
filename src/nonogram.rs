use ggez::{
    event::MouseButton,
    graphics::{self, Color},
    nalgebra::Point2,
    Context, GameError, GameResult,
};
use graphics::{DrawMode, Mesh, Rect};
use std::fs;

pub type Nonogram = Vec<Vec<Block>>;

pub const BLOCK_CHAR: char = '#';
const BLOCK_SIZE: f32 = 35.0;
const BLOCK_COLOR: graphics::Color = graphics::WHITE;

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Empty,
    Filled,
    Crossed,
    Marked,
}

impl Block {
    pub fn to_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        match self {
            Block::Filled => Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE),
                BLOCK_COLOR,
            ),

            Block::Empty => Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, 0.0, 0.0),
                graphics::BLACK,
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

            Block::Marked => Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(0.0),
                Rect::new(0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE),
                graphics::Color::from_rgba(255, 255, 255, 128),
            ),
        }
    }

    pub fn from_mouse_button(&self, button: MouseButton) -> Self {
        let proposed_block = match button {
            MouseButton::Left => Block::Filled,
            MouseButton::Right => Block::Crossed,
            MouseButton::Middle => Block::Marked,
            MouseButton::Other(_) => Block::Filled,
        };

        if *self == proposed_block {
            Block::Empty
        } else {
            proposed_block
        }
    }
}

pub fn draw_nonogram(nonogram: &Nonogram, ctx: &mut Context) -> GameResult<()> {
    let (window_width, window_height) = graphics::drawable_size(ctx);
    let (half_window_width, half_window_height) = (window_width / 2.0, window_height / 2.0);

    let (width, height) = (
        nonogram[0].len() as f32 * BLOCK_SIZE,
        nonogram.len() as f32 * BLOCK_SIZE,
    );

    let top_left_x = half_window_width - width / 2.0;
    let top_left_y = half_window_height - height / 2.0;

    let params = graphics::DrawParam::default();

    let background = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(0.0, 0.0, width, height),
        Color::from_rgb(255 / 4, 255 / 4, 255 / 4),
    )?;

    graphics::draw(
        ctx,
        &background,
        params.dest(Point2::new(top_left_x, top_left_y)),
    )?;

    for (y, line) in nonogram.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            let mesh = block.to_mesh(ctx)?;
            graphics::draw(
                ctx,
                &mesh,
                params.dest(Point2::new(
                    top_left_x + x as f32 * BLOCK_SIZE,
                    top_left_y + y as f32 * BLOCK_SIZE,
                )),
            )?;
        }
    }

    Ok(())
}

pub fn translate_xy_to_rc(
    nonogram: &Nonogram,
    x: f32,
    y: f32,
    ctx: &mut Context,
) -> Option<(usize, usize)> {
    let (window_width, window_height) = graphics::drawable_size(ctx);
    let (half_window_width, half_window_height) = (window_width / 2.0, window_height / 2.0);

    let (width, height) = (
        nonogram[0].len() as f32 * BLOCK_SIZE,
        nonogram.len() as f32 * BLOCK_SIZE,
    );

    let top_left = (
        half_window_width - width / 2.0,
        half_window_height - height / 2.0,
    );

    let bottom_right = (
        half_window_width + width / 2.0,
        half_window_height + height / 2.0,
    );

    if x < top_left.0 || y < top_left.1 || x > bottom_right.0 || y > bottom_right.1 {
        return None;
    }

    let row = (y - top_left.1) / BLOCK_SIZE;
    let col = (x - top_left.0) / BLOCK_SIZE;

    Some((row as usize, col as usize))
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
