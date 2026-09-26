use crate::error::OpenWAError;
use std::future::Future;

/// Default limit for paginated queries when not explicitly specified.
pub const DEFAULT_PAGE_LIMIT: u32 = 50;

/// Collect all items across all pages for a paginated API endpoint.
///
/// Calls `fetch_page(offset, limit)` repeatedly until an empty or partial page
/// indicates that all records have been retrieved, or until `max_items` is reached.
///
/// # Example
///
/// ```no_run
/// # use openwa::{OpenWAClient, pagination::fetch_all_pages};
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = OpenWAClient::new("http://localhost:3000", "key")?;
/// let all_sessions = fetch_all_pages(50, None, |offset, limit| {
///     let client = &client;
///     async move {
///         client.sessions().list(Some(openwa::ListSessionsQuery {
///             limit: Some(limit),
///             offset: Some(offset),
///             name: None,
///         })).await
///     }
/// }).await?;
/// # Ok(())
/// # }
/// ```
pub async fn fetch_all_pages<T, F, Fut>(
    page_limit: u32,
    max_items: Option<usize>,
    mut fetch_page: F,
) -> Result<Vec<T>, OpenWAError>
where
    F: FnMut(u32, u32) -> Fut,
    Fut: Future<Output = Result<Vec<T>, OpenWAError>>,
{
    let limit = if page_limit == 0 {
        DEFAULT_PAGE_LIMIT
    } else {
        page_limit
    };
    let mut offset: u32 = 0;
    let mut results: Vec<T> = Vec::new();

    loop {
        let page = fetch_page(offset, limit).await?;
        let count = page.len();
        if count == 0 {
            break;
        }

        results.extend(page);

        if let Some(max) = max_items {
            if results.len() >= max {
                results.truncate(max);
                break;
            }
        }

        if (count as u32) < limit {
            break;
        }

        offset += count as u32;
    }

    Ok(results)
}

/// An asynchronous paginator that lazily yields items from paginated endpoints one-by-one.
pub struct Paginator<T, F, Fut> {
    fetch_page: F,
    page_limit: u32,
    current_offset: u32,
    buffer: std::collections::VecDeque<T>,
    exhausted: bool,
    _marker: std::marker::PhantomData<Fut>,
}

impl<T, F, Fut> Paginator<T, F, Fut>
where
    F: FnMut(u32, u32) -> Fut,
    Fut: Future<Output = Result<Vec<T>, OpenWAError>>,
{
    /// Create a new lazy paginator.
    pub fn new(page_limit: u32, fetch_page: F) -> Self {
        let limit = if page_limit == 0 {
            DEFAULT_PAGE_LIMIT
        } else {
            page_limit
        };
        Self {
            fetch_page,
            page_limit: limit,
            current_offset: 0,
            buffer: std::collections::VecDeque::new(),
            exhausted: false,
            _marker: std::marker::PhantomData,
        }
    }

    /// Retrieve the next item, fetching the next page from the server if necessary.
    pub async fn next_item(&mut self) -> Result<Option<T>, OpenWAError> {
        if let Some(item) = self.buffer.pop_front() {
            return Ok(Some(item));
        }

        if self.exhausted {
            return Ok(None);
        }

        let page = (self.fetch_page)(self.current_offset, self.page_limit).await?;
        let count = page.len();
        if count == 0 {
            self.exhausted = true;
            return Ok(None);
        }

        if (count as u32) < self.page_limit {
            self.exhausted = true;
        }

        self.current_offset += count as u32;
        self.buffer.extend(page);

        Ok(self.buffer.pop_front())
    }
}
