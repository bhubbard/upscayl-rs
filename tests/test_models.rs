use upscayl::models::ModelRegistry;

#[test]
fn test_all_models_registered() {
    let models = ModelRegistry::all();
    assert!(models.len() >= 7);

    let standard = ModelRegistry::find("upscayl-standard-4x").expect("Standard model should exist");
    assert_eq!(standard.scale, 4);
    assert_eq!(standard.param_file, "upscayl-standard-4x.param");
    assert_eq!(standard.bin_file, "upscayl-standard-4x.bin");

    let ultrasharp = ModelRegistry::find("ultrasharp-4x").expect("UltraSharp model should exist");
    assert_eq!(ultrasharp.scale, 4);

    let digital_art = ModelRegistry::find("digital-art-4x").expect("Digital art model should exist");
    assert_eq!(digital_art.scale, 4);
}

#[test]
fn test_model_scale_lookup() {
    assert_eq!(ModelRegistry::get_scale("upscayl-standard-4x"), 4);
    assert_eq!(ModelRegistry::get_scale("remacri-4x"), 4);
    assert_eq!(ModelRegistry::get_scale("nonexistent-model"), 4); // default fallback
}
