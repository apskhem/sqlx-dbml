//! Generated by sqlx-dbml 0.1.0-beta.2

pub mod cake {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub id: i32,
		pub name: Option<String>,
	}
}

pub mod vendor {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub id: i32,
		pub name: String,
		pub fruit_id: Option<i32>,
	}
}

pub mod fruit {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub id: i32,
		pub name: String,
		pub cake_id: Option<i32>,
	}
}

pub mod filling {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub id: i32,
		pub name: String,
	}
}

pub mod cake_filling {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub cake_id: i32,
		pub filling_id: i32,
	}
}
