use upscayl::tiling::Tiler;

#[test]
fn test_tile_computation_small_image() {
    let tiler = Tiler::new(200, 200, 512, 16);
    let tiles = tiler.compute_tiles();
    assert_eq!(tiles.len(), 1);
    assert_eq!(tiles[0].x, 0);
    assert_eq!(tiles[0].y, 0);
    assert_eq!(tiles[0].width, 200);
    assert_eq!(tiles[0].height, 200);
}

#[test]
fn test_tile_computation_large_image() {
    let tiler = Tiler::new(1000, 1000, 400, 20);
    let tiles = tiler.compute_tiles();
    assert!(tiles.len() > 1);

    for tile in &tiles {
        assert!(tile.width <= 400);
        assert!(tile.height <= 400);
        assert!(tile.x + tile.width <= 1000);
        assert!(tile.y + tile.height <= 1000);
    }
}

#[test]
fn test_tile_seam_feathering_blend() {
    let mut dest = image::RgbaImage::from_pixel(64, 32, image::Rgba([0, 0, 0, 255]));
    let tile = image::RgbaImage::from_pixel(32, 32, image::Rgba([200, 200, 200, 255]));

    Tiler::blend_tile(&mut dest, &tile, 16, 0, 8);

    // At the start of the overlap (dx=16, tx=0), pixel should be feathered between 0 and 200
    let seam_px = dest.get_pixel(16, 16);
    assert!(seam_px[0] > 0 && seam_px[0] < 200);
    // Past the overlap region (dx=30, tx=14), pixel should be full tile intensity
    let inner_px = dest.get_pixel(30, 16);
    assert_eq!(inner_px[0], 200);
}
