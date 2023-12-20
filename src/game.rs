extern crate sdl2;

mod mine;

use std::path::Path;
use sdl2::mouse::MouseButton;
use mine::*;
use sdl2::event::Event;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color};
use std::time::Duration;

pub fn game(x_case: i32 , y_case: i32 , number_mine: i32) {
    

    let width: i32 = 32*x_case;
    let height: i32 = 32*y_case;
    let mut first = true;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Minesweeper", width as u32, height  as u32 + 50)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
    let texture_creator = canvas.texture_creator();
    let texture = vec![texture_creator.load_texture("assets/Minesweeper_simple_texture_atlas.png").unwrap()];

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut all_rct: Vec<Vec<Case>> = vec![];
    let mut _time = 0;
    let mut mine_number = number_mine;
    let font_path: &Path = Path::new("./assets/PKMN_RBYGSC.ttf");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);
        for a in 0..y_case {
            let mut stock: Vec<Case> = Vec::new();
            for e in 0..x_case {
                let  case = Case::new((width/x_case)*e , (height/y_case)*a +50, (width/x_case) as u32 , (height/y_case) as u32);
                stock.push(case)
            }
            all_rct.push(stock);
        }
        setup_mines(&mut all_rct , number_mine, y_case , x_case);
        setup_number(&mut all_rct, y_case , x_case);
        let mut stop = false;
        let mut redo_mouse_ckick = false;
        let mut redo_left_ckick = false; 
        let texture_location: [Rect; 14] = texture_location();
    let backgroud_color = Color::RGB(192, 192, 192);
        
    'running: loop {
        remove_see(&mut all_rct);
        let state = event_pump.mouse_state();
        for event in event_pump.poll_iter() {
            
            match event {
                Event::MouseButtonUp { 
                    mouse_btn: MouseButton::Left,
                    ..
                } => if !stop{
                    redo_left_ckick = false;
                    unhidden(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, &mut stop , y_case , x_case ,&mut first , number_mine);
                },
                Event::MouseButtonUp { 
                    mouse_btn: MouseButton::Right,
                    ..
                } => if !stop {
                    toggle_flag(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, y_case , x_case, &mut mine_number);
                },
                Event::MouseButtonUp { 
                    mouse_btn: MouseButton::Middle,
                    ..
                } => if !stop {
                    redo_mouse_ckick = false;
                    remove_see(&mut all_rct);
                    if_all_flag_unhidden(&mut all_rct, state.x() , state.y() , width/x_case , height/y_case , &mut stop, y_case , x_case)
                },
                Event::MouseButtonDown { 
                    mouse_btn: MouseButton::Middle,
                    ..
                } => if !stop {
                    redo_mouse_ckick = true;
                    see_selected(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, y_case , x_case);
                },
                Event::MouseButtonDown { 
                    mouse_btn: MouseButton::Left,
                    ..
                } => if !stop {
                    redo_left_ckick = true;
                    see_click(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, y_case , x_case);
                },



                Event::KeyUp { 
                    keycode: Some(Keycode::A),
                    ..
                } => if !stop{
                    redo_left_ckick = false;
                    unhidden(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, &mut stop , y_case , x_case ,&mut first , number_mine);
                },
                Event::KeyUp { 
                    keycode: Some(Keycode::E),
                    ..
                } => if !stop {
                    toggle_flag(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, y_case , x_case, &mut mine_number);
                },
                Event::KeyUp { 
                    keycode: Some(Keycode::Z),
                    ..
                } => if !stop {
                    redo_mouse_ckick = false;
                    remove_see(&mut all_rct);
                    if_all_flag_unhidden(&mut all_rct, state.x() , state.y() , width/x_case , height/y_case , &mut stop, y_case , x_case)
                },
                Event::KeyDown { 
                    keycode: Some(Keycode::Z),
                    ..
                } => if !stop {
                    redo_mouse_ckick = true;
                    see_selected(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, y_case , x_case);
                },
                Event::KeyDown { 
                    keycode: Some(Keycode::A),
                    ..
                } => if !stop {
                    redo_left_ckick = true;
                    see_click(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, y_case , x_case);
                },


                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    reset_all(&mut all_rct);
                    setup_mines(&mut all_rct , number_mine, y_case , x_case);
                    setup_number(&mut all_rct, y_case , x_case);
                    mine_number = number_mine;
                    stop = false;
                    first = true;
                },

                Event::Quit { .. }
              | Event::KeyDown {
                keycode: Some(Keycode::Escape),
             ..
             } => break 'running,
                _ => {}
            }
        }
        if redo_mouse_ckick {
            see_selected(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, y_case , x_case);
        } else if redo_left_ckick {
            see_click(&mut all_rct , state.x() , state.y() , width/x_case , height/y_case, y_case , x_case);
        }
        
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
      
        
        // Render
        render(&mut canvas, backgroud_color, &texture , &mut all_rct , texture_location,mine_number,_time,width,&font,&texture_creator).unwrap();

        
        std::thread::sleep(Duration::from_millis(16));
        canvas.present();
    }
}