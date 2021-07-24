mod utils;

use wasm_bindgen::prelude::*;

//// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//
//#[wasm_bindgen]
//extern {
//    fn alert(s: &str);
//}
//
//#[wasm_bindgen]
//pub fn greet() {
//    alert("Hello, wasm-game-of-life!");
//}

extern crate web_sys;
macro_rules! log {
	( $( $t:tt )* ) => {
		web_sys::console::log_1(&format!( $( $t )* ).into());
	}
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
	pub fn set_width(&mut self, width: u32) {
		self.width = width;
		self.cells = (0..width*self.height).map(|_i| Cell::Dead).collect();
	}

	pub fn set_height(&mut self, height: u32) {
		self.height = height;
		self.cells = (0..height*self.width).map(|_i| Cell::Dead).collect();
	}

	pub fn new() -> Universe {
		utils::set_panic_hook(); //添加钩子函数，用于调试

		let w = 64;
		let h = 64;

		let cells = (0..w*h).map(|i| {
			if i%2 == 0 || i%7 == 0 {
				Cell::Alive
			} else {
				Cell::Dead
			}
		}).collect();

		Universe {
			width: w,
			height: h,
			cells: cells,
		}
	}

	pub fn render(&self) -> String {
		self.to_string()
	}

	//获取到一维数组中的索引
	fn get_index(&self, row: u32, col: u32)	 -> usize {
		(row * self.width + col) as usize
	}

	//获取活着的邻居的个数
	//相邻的步进是-1, 0, 1
	fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
		let mut count = 0;
		
		//for delta_row in [-1, 0, 1].iter().cloned() {
		for delta_row in [self.height-1, 0, 1].iter().cloned() {
			//for delta_col in [-1, 0, 1].iter().cloned() {
			for delta_col in [self.width-1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue;
				}
				
				let neighbor_row = (row + delta_row)%self.height;
				let neighbor_col = (col + delta_col)%self.width;
				let idx = self.get_index(neighbor_row, neighbor_col);
				count += self.cells[idx] as u8;
			}	
		}

		count
	}
	
	//下一个时间点，宇宙的变化
	pub fn tick(&mut self) {
		let mut next = self.cells.clone();
		
		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_index(row, col);
				let cell = self.cells[idx];
				let live_neighbors = self.live_neighbor_count(row, col);
				
				//变化之前打印一下情况
				log!(
					"cell [{}, {}] is initally {:?} and has {} live neighbors",
					row,
					col,
					cell,
					live_neighbors
				);
				
				let next_cell = match(cell, live_neighbors) {
					(Cell::Alive, x) if x < 2 => Cell::Dead,
					(Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
					(Cell::Alive, x) if x > 3 => Cell::Dead,
					(Cell::Dead, 3)  => Cell::Alive,
					(otherwise, _)  => otherwise,
				};
				
				log!("it becomes {:?}", next_cell );

				next[idx] = next_cell;
			}
		}
		self.cells = next;
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn cells(&self) -> *const Cell {
		self.cells.as_ptr()
	}
}

use std::fmt;

impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				let symbol = if cell == Cell::Dead {
					'◻'
				} else {
					'◼'
				};

				write!(f, "{}", symbol)?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}

//没有#[wasm_bindgen]
impl Universe {
	pub fn get_cells(&self) -> &[Cell] {
		&self.cells
	}

	pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
		for (row , col) in cells.iter().cloned() {
			let idx = self.get_index(row, col);
			self.cells[idx] = Cell::Alive;
		}
	}
}
