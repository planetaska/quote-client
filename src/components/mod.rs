//! Component modules for the quote client application.

pub mod about;
pub mod app;
pub mod create_quote_form;
pub mod home;
pub mod navigation;
pub mod quote_card;
pub mod quote_display;
pub mod quotes_list;
pub mod quotes_page;
pub mod random_quote;
pub mod show_quote;

pub use about::About;
pub use app::App;
pub use create_quote_form::CreateQuoteForm;
pub use home::Home;
pub use navigation::Navigation;
pub use quote_card::QuoteCard;
pub use quote_display::QuoteDisplay;
pub use quotes_list::QuotesList;
pub use quotes_page::QuotesPage;
pub use random_quote::RandomQuote;
pub use show_quote::ShowQuote;
