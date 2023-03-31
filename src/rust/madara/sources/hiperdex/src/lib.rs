#![no_std]
use aidoku::{
	error::Result, prelude::*, std::defaults::defaults_get, std::String, std::Vec, Chapter,
	DeepLink, Filter, Listing, Manga, MangaPageResult, Page,
};

use madara_template::template;

fn get_data() -> template::MadaraSiteData {
	let url = match defaults_get("sourceURL").as_string() {
		Ok(url_str) => url_str.read(),
		Err(_) => panic!("missing sourceURL"),
	};
	let data: template::MadaraSiteData = template::MadaraSiteData {
		base_url: url,
		alt_ajax: true,
		..Default::default()
	};
	data
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	template::get_manga_list(filters, page, get_data())
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	template::get_manga_listing(get_data(), listing, page)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	template::get_manga_details(id, get_data())
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	template::get_chapter_list(id, get_data())
}

#[get_page_list]
fn get_page_list(id: String) -> Result<Vec<Page>> {
	template::get_page_list(id, get_data())
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
	template::handle_url(url, get_data())
}