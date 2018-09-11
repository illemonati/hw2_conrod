// #![windows_subsystem = "windows"]
//comment out the previous line for non windows use


#[macro_use]
extern crate conrod;
extern crate find_folder;

use conrod::backend::glium::glium::{self, Surface,glutin};
use conrod::backend::glium::glium::backend::glutin::glutin::GlContext;
use conrod::widget::{Text, Button};
use conrod::{widget, Positionable, Colorable, Widget, Sizeable};

fn main() {
    const WIDTH: f64 = 400f64;
    const HEIGHT: f64 = 200f64;

    let assets = find_folder::Search::KidsThenParents(3,5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets
        .join("fonts/Halo3/Halo3.ttf");

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("To Alex")
        .with_resizable(true)
        .with_dimensions(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT));
    let ctx = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    let display = glium::Display::new(window, ctx, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    widget_ids!(struct Ids {
         text,
         button,
    });
    ui.fonts.insert_from_file(font_path).unwrap();

    let ids = Ids::new(ui.widget_id_generator());

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let mut main_text = String::from("Gay, You be");
    'main: loop {
        {
            for input in ui.global_input().events(){
                println!("{:?}", input);
            }
            let mut uis = &mut ui.set_widgets();
            Text::new(&main_text)
                .middle_of(uis.window)
                .color(conrod::color::RED)
                .font_size(34)
                .set(ids.text, uis);
            Button::new()
                .bottom_right_of(uis.window)
                .color(conrod::color::GREEN)
                .w(uis.win_w / 8.0)
                .set(ids.button, uis);

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = uis.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                // target.clear_color((109/255)as f32,(207/255) as f32,(246/255) as f32, 1.0);
                target.clear_color(0.4274,0.81176,0.96471,1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }

        //handel events
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        for event in events{
            match conrod::backend::winit::convert_event(event.clone(), &display) {
                None => (),
                Some(input) => {
                    // let mut input_string = String::new();
                    // if (&input == &conrod::event::Input::Text(string::new())){
                    //     println!("{:?}", &input_string);
                    // }
                    ui.handle_event(input);
                }
            }
            match event {
                glium::glutin::Event::WindowEvent { event, ..} => match event {
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    // glium::glutin::WindowEvent::Resized(ls) => {ui.win_w=ls.width; ui.win_h=ls.height},
                    _ => (),
                },
                _ => (),
            }



        }


    }


}
