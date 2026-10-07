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
