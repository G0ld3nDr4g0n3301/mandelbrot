

use image::{RgbImage, Rgb};

fn main() {
    // Размеры изображения
    let width = 10000;
    let height = 15000;

    // Параметры множества Мандельброта
    let max_iterations = 100;
    let x_min = -2.0 ;
    let x_max = 1.0;
    let y_min = -1.5;
    let y_max = 1.5;

    // Создаем изображение
    let mut img = RgbImage::new(width, height);

    // Генерируем пиксели
    for j in 0..height {
        for i in 0..width {
            // Преобразуем координаты пикселя в комплексную плоскость
            let cx = x_min + (x_max - x_min) * i as f64 / width as f64;
            let cy = y_min + (y_max - y_min) * j as f64 / height as f64;

            // Вычисляем количество итераций до расходимости
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iterations = 0;

            while x * x + y * y <= 4.0 && iterations < max_iterations {
                let temp_x = x * x - y * y + cx;
                y = 2.0 * x * y + cy;
                x = temp_x;
                iterations += 1;
            }

            // Определяем цвет на основе количества итераций
            let color = if iterations < max_iterations {
                let c = (255 * iterations / max_iterations) as u8;
                Rgb([c, 0, 255 - c])
            } else {
                Rgb([0, 0, 0]) // черный цвет для точек внутри множества
            };

            // Устанавливаем пиксель в изображении
            img.put_pixel(i, j, color);
        }
    }

    // Сохраняем изображение в файл PNG
    img.save("mandelbrot.png").expect("Не удалось сохранить изображение");

    println!("Изображение множества Мандельброта сохранено в файл mandelbrot.png");
}


