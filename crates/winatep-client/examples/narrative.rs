//! An example that runs a test on Narrative Select+Edit

use std::time::{Duration, Instant};

use snafu::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use winatep_client::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("{source}"))]
    Client { source: winatep_client::Error },

    #[snafu(display("Could not find image"))]
    CouldNotFindImage,
}

impl From<winatep_client::Error> for Error {
    fn from(source: winatep_client::Error) -> Self {
        Error::Client { source }
    }
}

/// Images used for searching and assertions.
struct NarrativeDemo {
    /// Small logo
    logo_small: ImageBuffer,
    /// Tab logo, selected
    logo_tab_selected: ImageBuffer,
    /// Tab logo, deselected
    logo_tab_deselected: ImageBuffer,
    /// Highlighted project sidebar button
    projects_button: ImageBuffer,
    /// Button to create a new project from the projects tab
    new_project_button: ImageBuffer,
    /// Choose a folder link
    choose_a_folder_link: ImageBuffer,
    /// Finder folder to use as a test project
    test_set_finder_folder: ImageBuffer,
    /// Finder folder to use as a test project, when pre-selected
    test_set_finder_folder_selected: ImageBuffer,
    /// New project next button
    new_project_next: ImageBuffer,
    /// Choose your project type
    choose_your_project_type: ImageBuffer,
}

impl Default for NarrativeDemo {
    fn default() -> Self {
        NarrativeDemo {
            logo_small: image::open("demo/narrative-logo-small.png").unwrap().into(),
            logo_tab_selected: image::open("demo/narrative-logo-tab-selected.png")
                .unwrap()
                .into(),
            logo_tab_deselected: image::open("demo/narrative-logo-tab-deselected.png")
                .unwrap()
                .into(),
            projects_button: image::open("demo/narrative-projects-highlighted.png")
                .unwrap()
                .into(),
            new_project_button: image::open("demo/narrative-new-project-button.png")
                .unwrap()
                .into(),
            choose_a_folder_link: image::open("demo/narrative-choose-a-folder.png")
                .unwrap()
                .into(),
            test_set_finder_folder: image::open("demo/narrative-test-set-finder-folder.png")
                .unwrap()
                .into(),
            test_set_finder_folder_selected: image::open(
                "demo/narrative-test-set-finder-folder-selected.png",
            )
            .unwrap()
            .into(),
            new_project_next: image::open("demo/narrative-new-project-next.png")
                .unwrap()
                .into(),
            choose_your_project_type: image::open("demo/narrative-choose-your-project-type.png")
                .unwrap()
                .into(),
        }
    }
}

#[allow(unused)]
fn draw_bounding_boxes<'a>(
    img: &mut image::RgbImage,
    boxes: impl IntoIterator<Item = &'a (Vec2, Vec2)>,
    width: f32,
) {
    /// Signed distance field of a point from a box centered at the origin
    fn sd_box(p: Vec2, b: Vec2) -> f32 {
        let d = p.abs() - b;
        d.max(Vec2::splat(0.0)).length() + d.max_element().min(0.0)
    }

    fn distance_to_bounding_box(p: Vec2, (min, max): (Vec2, Vec2)) -> f32 {
        let b = max - min;
        let center_point = (min + max) / 2.0;
        let p = p - center_point;
        sd_box(p, b)
    }

    fn distance_from_boxes<'a>(
        p: Vec2,
        boxes: impl IntoIterator<Item = &'a (Vec2, Vec2)>,
    ) -> Option<f32> {
        boxes
            .into_iter()
            .map(|bb| distance_to_bounding_box(p, *bb))
            .min_by(|a, b| a.total_cmp(b))
    }

    let boxes = boxes.into_iter().collect::<Vec<_>>();
    for (x, y, image::Rgb([r, g, b])) in img.enumerate_pixels_mut() {
        let p = Vec2::new(x as f32, y as f32);
        let distance = distance_from_boxes(p, boxes.iter().copied()).unwrap();
        if distance > 0.0 && distance <= width {
            let shadow_percent = distance / width;
            *r = (*r as f32 * shadow_percent).round() as u8;
            *g = (*g as f32 * shadow_percent).round() as u8;
            *b = (*b as f32 * shadow_percent).round() as u8;
        }
    }
}

#[allow(unused)]
fn save_image(img: image::RgbImage, path: impl AsRef<std::path::Path>) {
    let path = std::path::PathBuf::from("demo/output").join(path.as_ref());
    log::debug!("saving image to '{}'", path.display());
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    img.save(path).unwrap();
}

fn _test_draw_boxes() {
    let mut img = image::RgbImage::from_pixel(100, 100, image::Rgb([255, 255, 255]));
    draw_bounding_boxes(
        &mut img,
        [
            (Vec2::splat(20.0), Vec2::splat(30.0)),
            (Vec2::splat(80.0), Vec2::new(85.0, 90.0)),
        ]
        .iter(),
        5.0,
    );
    save_image(img, "border-test.png");
}

/// Attempts to find an image within the given screen within a certain timeout.
///
/// Returns the most likely position of the center of the image in absolute logical coordinates.
async fn find_image_in_screen_with_timeout(
    session: &mut Session,
    screen: &Screen,
    imgs: impl IntoIterator<Item = ImageBuffer>,
) -> Result<Vec2, Error> {
    let imgs = imgs.into_iter().collect::<Vec<_>>();
    let start = Instant::now();
    while start.elapsed().as_secs_f32() < TIMEOUT_SECONDS {
        for img in imgs.iter() {
            let mut locations = session
                .find_image_in_screen(
                    &screen.name,
                    img.clone(),
                    FindImageQuality::Standard,
                    FindImageFilter::Standard,
                )
                .await?;
            if let Some(rect) = locations.pop() {
                let relative_to_screen_pixels = rect.center();
                let relative_to_screen_logical = relative_to_screen_pixels / screen.scale_factor;
                let abs = screen.bounds().min + relative_to_screen_logical;
                return Ok(abs);
            }
        }
    }

    log::error!("could not find the image within {TIMEOUT_SECONDS} seconds");
    CouldNotFindImageSnafu.fail()
}

/// Attempts to find any of the given images within all screens within a certain timeout.
///
/// Returns the most likely position of the center of the first image found, in absolute
/// logical coordinates, along with the screen it was found in.
async fn find_image_with_timeout(
    session: &mut Session,
    imgs: impl IntoIterator<Item = ImageBuffer>,
) -> Result<(Vec2, Screen), Error> {
    let imgs = imgs.into_iter().collect::<Vec<_>>();
    let screens = session.get_screens().await?;
    let start = Instant::now();
    while start.elapsed().as_secs_f32() < TIMEOUT_SECONDS {
        for screen in screens.iter() {
            for img in imgs.iter() {
                let mut locations = session
                    .find_image_in_screen(
                        &screen.name,
                        img.clone(),
                        FindImageQuality::Standard,
                        FindImageFilter::Standard,
                    )
                    .await?;
                if let Some(rect) = locations.pop() {
                    let relative_to_screen_pixels = rect.center();
                    let relative_to_screen_logical =
                        relative_to_screen_pixels / screen.scale_factor;
                    let abs = screen.bounds().min + relative_to_screen_logical;
                    return Ok((abs, screen.clone()));
                }
            }
        }
    }

    log::error!("could not find the image within {TIMEOUT_SECONDS} seconds");
    CouldNotFindImageSnafu.fail()
}

/// Find the image, mouse to it, click on it, return the current mouse position.
async fn click_on_image_in_screen_with_timeout(
    session: &mut Session,
    screen: &Screen,
    imgs: impl IntoIterator<Item = ImageBuffer>,
) -> Result<Vec2, Error> {
    let point = find_image_in_screen_with_timeout(session, screen, imgs).await?;
    let current_mouse = session.get_mouse_location().await?;
    let distance = current_mouse.distance(point);
    session
        .mouse_path(
            [current_mouse, point],
            Coordinate::Abs,
            distance / PIXELS_PER_SECOND,
        )
        .await?;
    session.sleep(Duration::from_millis(100)).await;
    session.mouse_left_click().await?;
    session.sleep(Duration::from_millis(100)).await;
    Ok(session.get_mouse_location().await?)
}

/// Speed of the cursor
const PIXELS_PER_SECOND: f32 = 1000.0;

/// Time to wait before giving up on an image search
const TIMEOUT_SECONDS: f32 = 15.0;

async fn run() -> Result<(), Error> {
    log::info!("running from directory {:#?}", std::env::current_dir());

    let images = NarrativeDemo::default();

    let mut session = Session::new("ws://127.0.0.1:31337/driver").await?;
    let screens = session.get_screens().await?;
    log::info!("screens: {screens:#?}");
    let main_screen = session.get_main_screen().await?;
    log::info!("main_screen: {main_screen:#?}");

    // Command-tab to open spotlight
    session.key_down(Key::Meta).await?;
    session.key_click(Key::Space).await?;
    session.sleep(Duration::from_millis(500)).await;
    session.key_up(Key::Meta).await?;
    session.pause().await;

    // Type in "narrative select"
    session.type_text("Narrative Select").await?;

    let current_screen = {
        let mut found_screen: Option<Screen> = None;
        let current_mouse = session.get_mouse_location().await?;
        for screen in screens.iter() {
            if screen.contains_abs_point(current_mouse) {
                found_screen = Some(screen.clone());
                break;
            }
        }
        found_screen.unwrap()
    };
    let small_logo_point = click_on_image_in_screen_with_timeout(
        &mut session,
        &current_screen,
        [images.logo_small.clone()],
    )
    .await?;
    log::info!("found Narrative Select + Edit spotlight logo at {small_logo_point}");

    // Find the screen that Narrative Select is now running in.
    //
    // This is trickier than the previous move because Narrative could be in one of two
    // states, so we search for two buttons...
    log::info!("looking for the screen Narrative Select + Edit is running in");
    let (narrative_logo_tab, narrative_screen) = find_image_with_timeout(
        &mut session,
        [
            // one in the deselected state
            images.logo_tab_deselected.clone(),
            // one in selected state
            images.logo_tab_selected.clone(),
        ],
    )
    .await?;
    log::info!("...found the screen: {narrative_screen:#?}");
    let current_mouse = session.get_mouse_location().await?;
    session
        .mouse_path([current_mouse, narrative_logo_tab], Coordinate::Abs, 1.0)
        .await?;
    session.mouse_left_click().await?;

    // Find the projects button
    log::info!("looking for the projects button on the side bar");
    let _projects_button_point = click_on_image_in_screen_with_timeout(
        &mut session,
        &narrative_screen,
        [images.projects_button],
    )
    .await?;

    // Create a new project
    log::info!("looking for the new projects '+' button");
    let _new_project_button_point = click_on_image_in_screen_with_timeout(
        &mut session,
        &narrative_screen,
        [images.new_project_button.clone()],
    )
    .await?;

    log::info!("looking for 'Choose a folder'");
    let _choose = click_on_image_in_screen_with_timeout(
        &mut session,
        &narrative_screen,
        [images.choose_a_folder_link.clone()],
    )
    .await?;

    log::info!("looking for a known test set in the finder");
    let _finder_folder = click_on_image_in_screen_with_timeout(
        &mut session,
        &narrative_screen,
        [
            images.test_set_finder_folder.clone(),
            images.test_set_finder_folder_selected.clone(),
        ],
    )
    .await?;

    session.key_click(Key::Return).await?;

    log::info!("looking for the 'Next' button");
    let _next_point = click_on_image_in_screen_with_timeout(
        &mut session,
        &narrative_screen,
        [images.new_project_next.clone()],
    )
    .await?;

    log::info!("looking for 'Choose your project type'");
    let _chose_cull_pre_edit = click_on_image_in_screen_with_timeout(
        &mut session,
        &narrative_screen,
        [images.choose_your_project_type.clone()],
    )
    .await?;

    // Now we should be looking at the project as it is scanning
    log::info!("done!");

    Ok(())
}

fn main() {
    // env_logger::builder().init();
    let perfetto_layer = tracing_perfetto::PerfettoLayer::new(std::sync::Mutex::new(
        std::fs::File::create("trace.pftrace").unwrap(),
    ));
    let filter_layer = tracing_subscriber::EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(perfetto_layer)
        .with(filter_layer)
        .with(fmt_layer)
        .init();
    async_std::task::block_on(run()).unwrap();
}
