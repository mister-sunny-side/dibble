//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`MapView`], [`AccountView`], and [`SocialFeedView`] components will be rendered when the current route matches
//! their respective route variants.
//!
//!
//! The [`BottomTabLayout`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes with a bottom tab bar navigation.

mod bottom_tab_layout;
pub use bottom_tab_layout::BottomTabLayout;

mod map_view;
pub use map_view::MapView;

mod account_view;
pub use account_view::AccountView;

mod social_feed_view;
pub use social_feed_view::SocialFeedView;
