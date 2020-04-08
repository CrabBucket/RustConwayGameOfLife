extern crate tetra;
extern crate rand;
extern crate rayon;

use rayon::prelude::*;

use rand::rngs::ThreadRng;
use rand::{Rng};
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, MouseButton};
use tetra::math::Vec2;
use tetra::time;
use tetra::window;
use tetra::{Context, ContextBuilder, State};
use std::sync::{Mutex, Arc};
use std::thread;


// NOTE: Using a high number here yields worse performance than adding more bunnies over
// time - I think this is due to all of the RNG being run on the same tick...
//const CELL_WIDTH: usize = 1000;
const WIDTH: i32 = 2560;
const HEIGHT: i32 = 1400;
const PIXEL_WIDTH: i32 = 20;

#[derive(Copy,Clone)]
struct Cell {
    position: (i32,i32),
    state: CellState,
    
}

#[derive(Copy,Clone)]
enum CellState{
    On,
    Off,
}



impl Cell {
    fn new(x: i32, y: i32) -> Cell {
        let position = (x,y);
        

        Cell {
            position: position,
            state: CellState::On,
            
        }
    }
    
}

struct GameState {
    black_pixel: Texture,
    white_pixel: Texture,
    cells: Vec<Vec<Cell>>,
    timer: i32,
    update: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let black_pixel = Texture::new(ctx, "./src/test/pixel.png")?;
        let white_pixel = Texture::new(ctx, "./src/test/whitepixel.png")?;
        
        let mut cells = Vec::with_capacity(WIDTH as usize);
        
        for x in 0..(WIDTH/PIXEL_WIDTH) +1{
            let mut cellstack = Vec::with_capacity(HEIGHT as usize);
            for y in 0..(WIDTH/PIXEL_WIDTH) +1{
            //println!("X:{} ,Y:{} ",x%10,x/10);
                cellstack.push(Cell::new((x*20),y*20));
            }
            cells.push(cellstack);
        }

        Ok(GameState {
            black_pixel,
            white_pixel,
            cells,
            timer: 0,
            update: true,
        })
    }
}


impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.timer != 0 {
            self.timer -= 1;
        }
        if input::is_key_down(ctx,input::Key::Q) && self.timer == 0{
            self.timer = 30;
            self.update = !self.update;
        }
        if input::is_mouse_button_down(ctx, MouseButton::Left) {
            //self.click_timer=10;
            let mouse_x = tetra::input::get_mouse_x(ctx);
            let mouse_y = tetra::input::get_mouse_y(ctx);
            
            //println!("Mouse x: {} Mouse y: {}",mouse_x,mouse_y);
            let clickedcellstack = self.cells.get_mut((mouse_x/20f32).floor() as usize).unwrap();
            let mut clickedcell = clickedcellstack.get_mut((mouse_y.floor()/20f32) as usize).unwrap();
            let mut clickedcellstate = clickedcell;
            match clickedcellstate.state{
                CellState::On => {
                    //clickedcellstate.state = CellState::Off;
                }
                CellState::Off => {
                    clickedcellstate.state = CellState::On;
                }
            }

        }
        let tempvec = self.cells.clone();
        
        //tempvec.get(100).unwrap().get((0-1 + 1) as usize).unwrap();
        for cellstack in &mut self.cells{
            if !self.update {
                break;
            }
            //Update Cell State
            
            cellstack.par_iter_mut().for_each(|cell|{
                //println!("{}",rayon::current_num_threads());
                
                let xindex = cell.position.0/20;
                let yindex = cell.position.1/20;
                let mut count = 0;

                for xpos in 0..3{
                    for ypos in 0..3{
                        if xpos+xindex-1<0 || xpos+xindex-1 > WIDTH/PIXEL_WIDTH || ypos+yindex-1 < 0 || ypos+yindex-1 > HEIGHT/PIXEL_WIDTH || (xpos == 1 && ypos ==1) {
                            continue;
                        }else{
                            //println!("xindex: {}, xpos: {}, yindex: {}, ypos: {}",xindex,xpos,yindex,ypos);
                            match tempvec.get((xindex-1 + xpos) as usize).unwrap().get((yindex-1 + ypos) as usize).unwrap().state {
                                CellState::On =>{
                                    //println!("test");
                                    count+=1;
                                }
                                CellState::Off =>{
                                    
                                }
                            }
                        }
                    }
                }
                match cell.state {
                    CellState::On =>{
                        if count == 2 || count == 3 {

                        }else{
                            cell.state = CellState::Off;
                        }
                    }
                    CellState::Off =>{
                        if count == 3 {
                            cell.state = CellState::On;
                        }
                    }
                }

            })
        }
        if input::is_key_down(ctx,input::Key::A){
            fn set_cell_state(cells: &mut Vec<Vec<Cell>>, x: i32, y: i32, state: bool) {
                
                let mut cell = cells.get_mut(x as usize).unwrap().get_mut(y as usize).unwrap();
                match cell.state {
                    CellState::On=>{
                        //println!("State: {}", "On");
                    }
                    CellState::Off=>{
                        //println!("State: {}", "Off");
                    }
                }
                if(state){
                    cell.state = CellState::On;
                }else{
                    cell.state = CellState::Off;
                }
            }

            for x in 1..4{
                for y in 1..1{
                    set_cell_state(self.cells.as_mut(),x,y,true);
                }
            }

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        //thread::sleep(std::time::Duration::from_millis(2));
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        
        for cellstack in &self.cells {
            
            for cell in cellstack{
                
                match cell.state{
                    CellState::On =>{
                        graphics::draw(ctx, &self.black_pixel, Vec2::new(cell.position.0 as f32,cell.position.1 as f32));
                    }
                    CellState::Off => {
    
                    }
                }
                
            }
        }
        graphics::draw(ctx, &self.white_pixel, Vec2::new(tetra::input::get_mouse_x(ctx),tetra::input::get_mouse_y(ctx)));
        // for cell in &self.cells {
        //     match &cell.state{
                // CellState::On =>{
                //     graphics::draw(ctx, &self.texture, Vec2::new(cell.position.0 as f32,cell.position.1 as f32));
                // }
                // CellState::Off => {

                // }
        //     }
            
        // }

        window::set_title(
            ctx,
            &format!(
                "Michael Bench Mark - {} Michaels - {:.0} FPS",
                self.cells.len()*self.cells.get(0).unwrap().len(),
                time::get_fps(ctx)
            ),
        );

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("GammyMark", WIDTH, HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
