extern crate nalgebra_glm as glm;

use flo_draw::*;
use flo_canvas::*;
use futures::prelude::*;
use futures::executor;

fn clip_line(start: &glm::Vec4, end: &glm::Vec4, near: f32, far: f32) -> Option<(glm::Vec4, glm::Vec4)> {
    if (start.z > near && end.z > near) || (start.z < far && end.z < far) {
        return None;
    } 

    let fixed_start = if start.z > near {
        let t = (near - start.z) / (end.z - start.z);
        start + t * (end - start)
    } else if start.z < far {
        let t = (far - start.z) / (end.z - start.z);
        start + t * (end - start)
    } else {
        *start
    };

    let fixed_end = if end.z > near {
        let t = (near - start.z) / (end.z - start.z);
        start + t * (end - start)
    } else if end.z < far {
        let t = (far - start.z) / (end.z - start.z);
        start + t * (end - start)
    } else {
        *end
    };

    Some((fixed_start, fixed_end))
}

fn main() {
    let vertices = [
        glm::vec4(-0.5, -0.5, -0.5, 1.0),
        glm::vec4(-0.5, -0.5,  0.5, 1.0),
        glm::vec4(-0.5,  0.5, -0.5, 1.0),
        glm::vec4(-0.5,  0.5,  0.5, 1.0),
        glm::vec4( 0.5, -0.5, -0.5, 1.0),
        glm::vec4( 0.5, -0.5,  0.5, 1.0),
        glm::vec4( 0.5,  0.5, -0.5, 1.0),
        glm::vec4( 0.5,  0.5,  0.5, 1.0)
    ];

    let edges = [
        [0, 1],
        [0, 2],
        [3, 1],
        [3, 2],
        [4, 5],
        [4, 6],
        [7, 5],
        [7, 6],
        [0, 4],
        [1, 5],
        [2, 6],
        [3, 7]
    ];

    with_2d_graphics(move || {
        let (canvas, mut events) = create_drawing_window_with_events("Cube");

        let mut rotation = glm::vec3(0.0, 0.0, 0.0);
        let mut scaling = glm::vec3(1.0, 1.0, 1.0);
        let mut scaling_lock = false;
        let mut translation = glm::vec3(0.0, 0.0, 0.0);

        fn render_cube(
            gc: &mut Vec<Draw>,
            vertices: &[glm::Vec4; 8],
            edges: &[[usize; 2]; 12],
            rotation: &glm::Vec3,
            scaling: &glm::Vec3,
            translation: &glm::Vec3
        ) {
            let eye = glm::vec3(0.0, 0.0, 4.0);
            let center = glm::vec3(0.0, 0.0, 0.0);
            let up = glm::normalize(&(center - eye + glm::vec3(0.0, 1.0, 0.0)));
        
            let near_plane = 1.0;
            let far_plane = 20.0;
        
            let model = 
                glm::translate(
                    &glm::scale(
                        &glm::rotate_z(
                            &glm::rotate_y(
                                &glm::rotate_x(
                                        &glm::identity(),
                                    rotation.x), 
                                rotation.y),
                            rotation.z),
                        &scaling),
                    &translation);
            let view = glm::look_at(&eye, &center, &up);
            let perspective = glm::perspective(1.0, glm::pi::<f32>() / 3.0, near_plane, far_plane);
        
            let world_space = vertices.map(|v| view * model * v);

            gc.clear_layer();
            gc.line_width(0.005);
            gc.new_path();

            for e in edges {
                if let Some((v1, v2)) = clip_line(&world_space[e[0]], &world_space[e[1]], -near_plane, -far_plane) {
                    let (p1, p2) = (perspective * v1, perspective * v2);
                    let (p1, p2) = ((p1 / p1.w).xy(), (p2 / p2.w).xy());

                    gc.move_to(p1.x, p1.y);
                    gc.line_to(p2.x, p2.y);
                }
            }

            gc.stroke();
        }

        canvas.draw(|gc| {
            gc.canvas_height(2.0);
            gc.center_region(-1.0, -1.0, 1.0, 1.0);

            render_cube(gc, &vertices, &edges, &rotation, &scaling, &translation);
        });

        executor::block_on(async move {
            while let Some(event) = events.next().await {
                match event {
                    DrawEvent::KeyDown(_, Some(Key::KeyDown)) => {
                        rotation.x += 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyUp)) => {
                        rotation.x -= 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyLeft)) => {
                        rotation.y -= 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyRight)) => {
                        rotation.y += 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyPgDown)) => {
                        rotation.z -= 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyPgUp)) => {
                        rotation.z += 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyL)) => {
                        translation.x += 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyJ)) => {
                        translation.x -= 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyI)) => {
                        translation.y += 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyK)) => {
                        translation.y -= 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyU)) => {
                        translation.z += 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyO)) => {
                        translation.z -= 0.01;
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyA)) => {
                        if scaling_lock {
                            scaling = scaling.add_scalar(-0.01);
                        } else {
                            scaling.x -= 0.01;
                        }
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyD)) => {
                        if scaling_lock {
                            scaling = scaling.add_scalar(0.01);
                        } else {
                            scaling.x += 0.01;
                        }
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyW)) => {
                        if scaling_lock {
                            scaling = scaling.add_scalar(0.01);
                        } else {
                            scaling.y += 0.01;
                        }
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyS)) => {
                        if scaling_lock {
                            scaling = scaling.add_scalar(-0.01);
                        } else {
                            scaling.y -= 0.01;
                        }
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyQ)) => {
                        if scaling_lock {
                            scaling = scaling.add_scalar(-0.01);
                        } else {
                            scaling.z -= 0.01;
                        }
                    },
                    DrawEvent::KeyDown(_, Some(Key::KeyE)) => {
                        if scaling_lock {
                            scaling = scaling.add_scalar(0.01);
                        } else {
                            scaling.z += 0.01;
                        }
                    },
                    DrawEvent::KeyUp(_, Some(Key::KeyL)) => {
                        scaling_lock = !scaling_lock;
                    },
                    DrawEvent::KeyUp(_, Some(Key::KeyR)) => {
                        rotation = glm::vec3(0.0, 0.0, 0.0);
                        scaling = glm::vec3(1.0, 1.0, 1.0);
                        translation = glm::vec3(0.0, 0.0, 0.0);
                        scaling_lock = false;
                    },
                    _ => {}
                };

                canvas.draw(|gc| render_cube(gc, &vertices, &edges, &rotation, &scaling, &translation));
            }
        });
    });
}
