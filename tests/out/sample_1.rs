//! Generated by sqlx-dbml 0.1.0-beta.2

pub mod users {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub id: i32,
		pub age: i32,
		pub username: String,
		pub role: String,
		pub created_at: types::chrono::NaiveDateTime,
		pub referral_id: Option<i32>,
	}
}

pub mod posts {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub id: i32,
		pub title: String,
		pub body: String,
		pub user_id: i32,
		pub status: super::PostStatus,
		pub created_at: types::chrono::NaiveDateTime,
	}
}

pub mod orders {
	use sqlx::*;

	#[derive(FromRow)]
	pub struct Model {
		pub id: i32,
		pub status: String,
	}
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "post_status")]
pub enum PostStatus {
	Draft,
	Published,
	Private,
}
