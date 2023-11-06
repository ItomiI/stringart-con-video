use std::{f32::consts::PI, fmt::Debug};
use image::{DynamicImage, GenericImageView, Luma};


pub fn process_image(input_image: DynamicImage,hilos:u32,clavos:u32,carpeta:&str,extension:&str,_nombre:&str,n:&mut u32) -> DynamicImage {
    let (width, height) = input_image.dimensions();
    let menos = 21.0;
    let radio;
    if width > height{
        radio = height/2;
    }else{
        radio = width/2;
    }

    let centro_x = width / 2;
    let centro_y = height / 2;

    let mut imgbn = input_image.to_luma8();
    let grosor_borde = 2; 

    draw_circle_border(&mut imgbn, centro_x, centro_y, radio, grosor_borde);

/////////////////////////
 
    let mut dibujo = image::GrayImage::from_pixel(imgbn.width(), imgbn.height(),Luma([255]));
    let radio = (radio-5) as f32;
    let mut v = vec![];

    for i in 1 ..= clavos{//la pos de cada clavo en un circulo imaginario BIEN ACA
        let x = f32::floor((imgbn.width()/2) as f32 + radio * f32::cos(((360.0/clavos as f32)* i as f32) * PI / 180.0 + PI/2.0)) as u32;
        let y = f32::floor((imgbn.height()/2) as f32 + radio *f32::sin(((360.0/clavos as f32)* i as f32 ) * PI / 180.0 + PI/2.0)) as u32;
        let p = Punto::new(i,x,y);
        v.push(p);
        dibujo.put_pixel(x, y, Luma([0]));
    }
    
    let mut ultimo_lugar = v[0].clone();//ultimo lugar vector
    for h  in 0..hilos{//cantidad de lineas
        
        let start = ultimo_lugar;
        let pixelss: Vec<Punto>;//vector de sumas de negro (cada linea)
        let mut max_encontrado:Punto = Punto::new(1000,0,0);//la suma maxima encontrada

        for punto_del_circulo in &v{//punto del circulo
            let pixels = pixel_linea(start,*punto_del_circulo);
          
            let mut temp_suma: f32 = 0.0;
        
            for pixel in &pixels{//punto de la linea..... cambiar la forma de sumar, talvez que saque por cada coso que toca
                let a = imgbn.get_pixel(pixel.x, pixel.y);
                temp_suma += (u8::MAX-a[0]) as f32;
            }
            if temp_suma > menos*pixels.len() as f32{
                temp_suma -=menos*pixels.len() as f32
            }else {
                temp_suma=0.0;
            }
            if temp_suma as u32 >= max_encontrado.get_intensidad() as u32{
                max_encontrado.id = punto_del_circulo.id;
                max_encontrado.x = punto_del_circulo.x;
                max_encontrado.y = punto_del_circulo.y;
                max_encontrado.set_intensidad(temp_suma as u32);         
            }  
        }
        if max_encontrado.intensida < 1200{
            println!("se consumio en el hilo: {}",h);
            *n = h-1;
            break;
        }
        pixelss = pixel_linea(start, max_encontrado);

        for p in pixelss{
  
            let mut pixel_img = imgbn.get_pixel(p.x, p.y).clone();
            if pixel_img[0] <= 235{
                pixel_img[0] +=20;
            }
            imgbn.put_pixel(p.x, p.y, pixel_img);

            let mut pixel_dib = dibujo.get_pixel(p.x, p.y).clone();
            if pixel_dib[0] >= 20{
                pixel_dib[0]-=20;
            }
            dibujo.put_pixel(p.x, p.y, pixel_dib);
        }
        ultimo_lugar = max_encontrado;
        let mifile = format!("imagen{}.{}",h,extension);
        let elpath = format!("{}{}",carpeta,mifile);
        if !std::path::Path::new(&carpeta).exists() {
            std::fs::create_dir(&carpeta).unwrap();
        }
        dibujo.save(elpath).unwrap();
    } 
    *n = hilos-1;
    DynamicImage::ImageLuma8(dibujo)
    
}

fn draw_circle_border(image: &mut image::GrayImage, centro_x: u32, centro_y: u32, radio: u32, grosor_borde: u32) {

    for y in 0..image.height() {
        for x in 0..image.width() {

            let dx = x as f32 - centro_x as f32;//longitud entre pixel y centro
            let dy = y as f32 - centro_y as f32;
            let distancia_al_centro: f32 = f32::sqrt(dx * dx + dy * dy);
            let distancia_max = (radio + grosor_borde) as f32;
            let distancia_min = radio as f32;
            
            if  distancia_al_centro > distancia_min as f32 && distancia_al_centro < distancia_max {
                image.put_pixel(x, y, Luma([0]));
                
            }else if distancia_al_centro > distancia_min{
                image.put_pixel(x, y, Luma([255]));
            }
        }
    }
}

fn pixel_linea(point1: Punto, point2: Punto) -> Vec<Punto> {
    let (x1, y1) = point1.condenadas();
    let (x2, y2) = point2.condenadas();

    let mut pixels_on_line: Vec<Punto> = Vec::new();

    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let mut x = x1 as i32;
    let mut y = y1 as i32;
    let x_inc = if x2 >= x1 { 1 } else { -1 };
    let y_inc = if y2 >= y1 { 1 } else { -1 };
    let mut error = if dx > dy { dx } else { -dy } / 2;
    let mut prev_x = x;
    let mut prev_y = y;

    let mut id = 0;
    loop {
        
        pixels_on_line.push(Punto::new(id,x as u32, y as u32));
        
        if x == x2 as i32 && y == y2 as i32 {
            break;
        }

        let error2 = error * 2;

        if error2 > -dx {
            error -= dy;
            x += x_inc;
        }
        if error2 < dy {
            error += dx;
            y += y_inc;
        }

        // Si hay cambio en la dirección vertical o horizontal, almacenar el píxel anterior
        if x != prev_x || y != prev_y {
            pixels_on_line.push(Punto::new(id,prev_x as u32, prev_y as u32));
            prev_x = x;
            prev_y = y;
        }
        id+=1;
    }

    pixels_on_line
}


#[derive(Clone, Copy,Debug)]
struct Punto {
    id:u32,
    x:u32,
    y:u32,
    intensida:u32
}
impl PartialEq for Punto {
    fn eq(&self, other: &Self) -> bool {
       self.x == other.x && self.y == other.y
    }
}
impl Punto {
    pub fn new(id:u32,x:u32,y:u32)-> Punto{
        Punto { id, x, y, intensida:0 }
    }
    pub fn condenadas(&self)->(u32,u32){
        (self.x,self.y)
    }

    pub fn get_intensidad(& self)->u32{
        self.intensida
    }
    pub fn set_intensidad(&mut self,n:u32)->u32{
        self.intensida = n;
        self.intensida
    }
}
