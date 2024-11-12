#![no_std]

mod helper;
mod parser;
extern crate alloc;

use aidoku::{
    error::Result,
    prelude::{format, get_chapter_list, get_manga_details, get_manga_list, get_page_list},
    std::{
        net::{HttpMethod, Request},
        *,
    },
    Chapter, Filter, FilterType, Manga, MangaPageResult, Page,
};
use alloc::string::ToString;
use helper::{change_page, genre_id_from_filter, BASE_URL};

use parser::{parse_chapter_list, parse_manga, parse_page_list, parse_search};

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
    let mut manga_arr: Vec<Manga> = Vec::new();

    let mut manga_title = String::new();
    let mut artist_name = String::new();
    let mut status: i64 = 0;
    let mut sort_order = String::from("last-added"); // Default to "latest"

    let mut included_tags: Vec<i64> = Vec::new();
    let mut excluded_tags: Vec<i64> = Vec::new();

    for filter in filters {
        match filter.kind {
            FilterType::Title => manga_title = filter.value.as_string()?.read(),
            FilterType::Author => artist_name = filter.value.as_string()?.read(),
            FilterType::Genre => {
                let object_id = filter.object.get("id").as_string()?.read();
                let object_value = genre_id_from_filter(&object_id);

                match filter.value.as_int().unwrap_or(-1) {
                    0 => excluded_tags.push(object_value),
                    1 => included_tags.push(object_value),
                    _ => continue,
                }
            }
            FilterType::Select => match filter.name.as_str() {
                "Status" => status = filter.value.as_int()?,
                "Sort" => {  // Matches JSON "Sort" key
                    sort_order = match filter.value.as_int().unwrap_or(-1) {
                        0 => String::from("last-added"),   // "Newest" option
                        1 => String::from("oldest"),       // "Oldest" option
                        2 => String::from("most-popular"), // "Most Popular" option
                        3 => String::from("least-popular"),// "Least Popular" option
                        _ => String::from("last-added"),   // Default to "latest"
                    };
                }
                _ => continue,
            },
            _ => continue,
        }
    }

    // Construct the URL based on `sort_order` without using advanced search
    let url = format!("{BASE_URL}/hentai-list/all/any/all/{}/{}", sort_order, page);

    let mut has_next = false;

    // Direct request for sorted manga list
    if let Ok(html) = Request::new(url.clone(), HttpMethod::Get).html() {
        manga_arr = parse_search(&html);

        // Check if there's a next page
        has_next = html.select(".pagination").select("a#js-linkNext").attr("href").is_some();
    }

    Ok(MangaPageResult {
        manga: manga_arr,
        has_more: has_next,
    })
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
    let manga_url = format!("{BASE_URL}/{id}");

    let html = Request::new(manga_url, HttpMethod::Get).html()?;
    parse_manga(id, html)
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
    let url = format!("{BASE_URL}/{id}");
    let html = Request::new(url, HttpMethod::Get).html()?;
    parse_chapter_list(html)
}

#[get_page_list]
fn get_page_list(id: String, chapter: String) -> Result<Vec<Page>> {
    let url = format!("{BASE_URL}/{id}/{chapter}/1");
    let html = Request::new(url, HttpMethod::Get).html()?;
    parse_page_list(html)
}
