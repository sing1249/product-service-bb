use crate::model::Product;
use crate::configuration::Settings;

// The store-front vue.config.js fetches the products from this file, so updated the data.rs with Best Buy products. 

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Smart 4K UHD TV".to_string(),
            price: 499.99,
            description: "Experience stunning clarity with the Best Buy Smart 4K UHD TV, featuring HDR support and built-in streaming apps.".to_string(),
            image: "/smart_tv.jpg".to_string(),
        },
        Product {
            id: 2,
            name: "Wireless Noise-Cancelling Headphones".to_string(),
            price: 299.99,
            description: "Immerse yourself in pure sound with these wireless noise-cancelling headphones, designed for long-lasting comfort and superior audio quality.".to_string(),
            image: "/headphones.jpg".to_string(),
        },
        Product {
            id: 3,
            name: "Gaming Laptop".to_string(),
            price: 1199.99,
            description: "Dominate your gaming sessions with the high-performance Best Buy Gaming Laptop, equipped with a powerful GPU and fast refresh rate.".to_string(),
            image: "/gaming_laptop.jpg".to_string(),
        },
        Product {
            id: 4,
            name: "Smart Home Speaker".to_string(),
            price: 79.99,
            description: "Control your smart home with ease using the Best Buy Smart Home Speaker, featuring voice assistant integration and rich sound.".to_string(),
            image: "/smart_speaker.jpg".to_string(),
        },
        Product {
            id: 5,
            name: "Fitness Tracker Watch".to_string(),
            price: 149.99,
            description: "Track your fitness goals with the Best Buy Fitness Tracker, featuring heart rate monitoring and GPS tracking.".to_string(),
            image: "/fitness_tracker.jpg".to_string(),
        },
        Product {
            id: 6,
            name: "Wireless Gaming Mouse".to_string(),
            price: 89.99,
            description: "Enhance your gaming experience with the precision and comfort of the Best Buy Wireless Gaming Mouse.".to_string(),
            image: "/gaming_mouse.jpg".to_string(),
        },
        Product {
            id: 7,
            name: "Smart Thermostat".to_string(),
            price: 199.99,
            description: "Save energy and stay comfortable with the Best Buy Smart Thermostat, compatible with voice control systems.".to_string(),
            image: "/smart_thermostat.jpg".to_string(),
        },
        Product {
            id: 8,
            name: "4K Ultra HD Projector".to_string(),
            price: 899.99,
            description: "Bring the theater experience home with the Best Buy 4K Ultra HD Projector, supporting HDR and wide color gamut.".to_string(),
            image: "/4k_projector.jpg".to_string(),
        },
    ]
}
