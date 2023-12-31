//! Generated by sqlx-dbml 0.1.0-beta.2

pub mod sal_emp {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub name: String,
		pub pay_by_quarter: Vec<i32>,
		pub schedule: Option<Vec<Vec<String>>>,
	}
}

pub mod tictactoe {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub squares: Vec<Vec<i32>>,
	}
}
