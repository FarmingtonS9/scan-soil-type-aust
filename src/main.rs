//Standard crates

//Third-party crates
use image::ColorType;
use image::GenericImageView;
use image::Rgba;

//Constants
const RAW_IMG_FILE: &str = r"foo.png";
const ALL_PIXELS_CLASSIFIED_IMG: &str = r"bar.png";

const VERTOSOL: [u8; 4] = [204, 204, 179, 255];
const SODOSOL: [u8; 4] = [153, 102, 25, 255];
const DERMOSOL: [u8; 4] = [179, 204, 51, 255];
const CHROMOSOL: [u8; 4] = [230, 179, 77, 255];
const FERROSOL: [u8; 4] = [255, 102, 128, 255];
const KUROSOL: [u8; 4] = [104, 100, 10, 255];
const TENOSOL: [u8; 4] = [102, 204, 102, 255];
const KANDOSOL: [u8; 4] = [255, 255, 153, 255];
const HYDROSOL: [u8; 4] = [128, 255, 217, 255];
const RUDOSOL: [u8; 4] = [204, 255, 102, 255];
const CALCARASOL: [u8; 4] = [255, 204, 204, 255];
const ORGANOSOL: [u8; 4] = [255, 102, 51, 255];
//const ANTHROPOSOL: [u8; 4] = []; Can't find value on map
const WHITE: [u8; 4] = [255, 255, 255, 255]; //Used to mark water
const BLACK: [u8; 4] = [0, 0, 0, 255];

fn main() {
    //Open png file
    let img = image::open(RAW_IMG_FILE).expect("Could not open file!"); //Image is 24-bit depth (8-bit per channel). No alpha.

    //Get dimensions for limiting the scan range.
    let (width, height) = img.dimensions(); //Tuple of dimension w x h

    //Getting colour type
    let colour_type = img.color();

    //Calculate number of pixels
    let num_of_pixels: usize = (width * height).try_into().unwrap();

    println!(
        "Image details: Width: {}px, Height: {}px;\nNumber of pixels: {};\nHas colour? {};\nAlpha channel? {};\nChannel count? {};\nBits per channel? {}",
        width,
        height,
        num_of_pixels,
        colour_type.has_color(),
        colour_type.has_alpha(),
        colour_type.channel_count(),
        colour_type.bits_per_pixel(),
    );

    //Scan each pixel and determine appropriate allocation in struct. Start at top left
    //Each scanned pixel, output the RGB value and position (x, y)
    let mut pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut vertosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut sodosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut dermosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut chromosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut ferrosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut kurosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut tenosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut kandosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut hydrosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut rudosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut calcarasol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut organosol_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut white_pixel_vec: Vec<PixelInfo> = Vec::new();
    let mut other_pixel_vec: Vec<PixelInfo> = Vec::new();

    for pixel in img.pixels() {
        let pixel_info = PixelInfo {
            x: pixel.0,
            y: pixel.1,
            colour: pixel.2,
        };
        pixel_vec.push(pixel_info)
    }

    //display_pixel_list(pixel_vec);
    pixel_vec = change_colour_value_of_unassigned_pixel(pixel_vec);

    for temp_pix in pixel_vec.iter_mut() {
        match temp_pix.colour.0 {
            VERTOSOL => vertosol_pixel_vec.push(*temp_pix),
            SODOSOL => sodosol_pixel_vec.push(*temp_pix),
            DERMOSOL => dermosol_pixel_vec.push(*temp_pix),
            CHROMOSOL => chromosol_pixel_vec.push(*temp_pix),
            FERROSOL => ferrosol_pixel_vec.push(*temp_pix),
            KUROSOL => kurosol_pixel_vec.push(*temp_pix),
            TENOSOL => tenosol_pixel_vec.push(*temp_pix),
            KANDOSOL => kandosol_pixel_vec.push(*temp_pix),
            HYDROSOL => hydrosol_pixel_vec.push(*temp_pix),
            RUDOSOL => rudosol_pixel_vec.push(*temp_pix),
            CALCARASOL => calcarasol_pixel_vec.push(*temp_pix),
            ORGANOSOL => organosol_pixel_vec.push(*temp_pix),
            WHITE => white_pixel_vec.push(*temp_pix),
            _ => {
                //This arm collects unassigned pixels.
                other_pixel_vec.push(*temp_pix)
            }
        }
    }

    let number_of_pixels_counted = vertosol_pixel_vec.len()
        + sodosol_pixel_vec.len()
        + dermosol_pixel_vec.len()
        + chromosol_pixel_vec.len()
        + ferrosol_pixel_vec.len()
        + kurosol_pixel_vec.len()
        + tenosol_pixel_vec.len()
        + kandosol_pixel_vec.len()
        + hydrosol_pixel_vec.len()
        + rudosol_pixel_vec.len()
        + calcarasol_pixel_vec.len()
        + organosol_pixel_vec.len()
        + white_pixel_vec.len();

    println!(
        "Unclassified pixels: {:?}",
        unclassified_num_of_pixels(num_of_pixels, number_of_pixels_counted)
    );
}

fn assign_pixels_to_category(image: (u32, u32, Rgba<u8>)) {}

fn display_pixel_list(pixel_vec: Vec<PixelInfo>) {
    for pixel in pixel_vec.iter() {
        println!(
            "x:{}, y:{}, colour value: {:?}",
            pixel.x, pixel.y, pixel.colour.0
        );
    }
}

//fn find_location_of_unassigned_pixels();

fn change_colour_value_of_unassigned_pixel(mut pixel_vec: Vec<PixelInfo>) -> Vec<PixelInfo> {
    let mut num_of_changes: u32 = 0;
    //Take in the full vector of pixels of the image.
    //Compare each pixel to the constants above
    for pixel in pixel_vec.iter_mut() {
        let colour_value = pixel.colour.0;

        match colour_value {
            VERTOSOL | SODOSOL | DERMOSOL | CHROMOSOL | FERROSOL | KUROSOL | TENOSOL | KANDOSOL
            | HYDROSOL | RUDOSOL | CALCARASOL | WHITE => {
                continue;
            }
            _ => {
                pixel.colour.0 = VERTOSOL; //If pixel is found to not be any of the constants, change the colour field of the PixelInfo to the colour constant that came before it. (Simple, easy fox for now).
                num_of_changes += 1;
            }
        }
    }
    //Output updated vector of pixels
    println!("Number of changes made: {}", num_of_changes);
    pixel_vec
}

fn unclassified_num_of_pixels(num_of_pixels: usize, number_of_pixels_counted: usize) -> usize {
    num_of_pixels - number_of_pixels_counted
}

#[derive(Clone, Copy)]
struct PixelInfo {
    x: u32,
    y: u32,
    colour: Rgba<u8>,
}

impl PixelInfo {
    //Convert PixelInfo to ClassifiedPixel based on colour.

    //Get colour value at coordinate
}

struct ClassifiedPixel {
    x: u32,
    y: u32,
    colour: ColorType,
    classification: Classification,
}

//List of soil types found in Australia as classified by TERN.
enum Classification {
    Vertosol,
    Sodosol,
    Dermosol,
    Chromosol,
    Ferrosol,
    Kurosol,
    Tenosol,
    Kandosol,
    Hydrosol,
    Podosol,
    Calcarasol,
    Organosol,
    Anthroposol,
    White,
    Unknown,
}
