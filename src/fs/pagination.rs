use super::file::File;

#[derive(Debug)]
pub enum PaginationError {
    InvalidPageNumber,
    InvalidPageSize,
    PageOutOfBounds,
}

pub fn paginate(
    items: &[File],
    page: usize,
    page_size: usize,
) -> Result<Vec<File>, PaginationError> {
    if page < 1 {
        return Err(PaginationError::InvalidPageNumber);
    }

    if page_size == 0 {
        return Err(PaginationError::InvalidPageSize);
    }

    let total_pages = items.len().div_ceil(page_size);
    if page > total_pages {
        return Err(PaginationError::PageOutOfBounds);
    }

    let start = (page - 1) * page_size;
    let end = std::cmp::min(start + page_size, items.len());

    Ok(items[start..end].to_vec())
}
