/// Missile Defense system

 
/// Update enemy missile position
/// Update interceptor position
/// Detect enemy with radar and send to interceptor
/// Update interceptor's goal
///     - predict interception location
///     - update heading to target new interception location


/// Plan:
/// Create way to represent enemy missile
///     structs for position, heading
///     constructor
///     trait for the non-unique methods
/// Model the flight of the missile (run a loop that runs the functions mentioned above)
/// 
/// Create way to represent radar
/// Implement tracking of the missile
/// 
/// Create way to represent interceptor missile
/// Model the flight of the missile

/// All distances are in meters and all time is in seconds
/// Heading is represented as a normalized vector (length 1)


use plotters::prelude::*;
use std::f64::consts::PI;




fn main() {
    // Create enemy missile
    // Based off YJ-62
    let enemy_pos = Position { x: 500_000.0, y: 100_000.0, z: 250_000.0 };
    let enemy_heading = Heading { x: -1.0, y: 0.0, z: 0.0 }; // facing straight down the y axis
    let enemy_vel = Velocity { speed: 360.0, heading: enemy_heading }; // 950 km/h
    let enemy_target_pos = Position { x: 0.0, y: 100_000.0, z: 250_000.0 };
    let enemy_detonation_dist = 3.0;
    let mut enemy = EnemyMissile::new(enemy_pos, enemy_vel, enemy_target_pos, enemy_detonation_dist, None);

    //println!("Starting Position: {:?}", enemy.pos);
    //enemy.update_pos(5.0); // 5 seconds
    //println!("Updated Position: {:?}", enemy.pos);

    // Add test for distance travelled


    // let enemy reach target
    //loop {
    //    enemy.update_pos(0.005);
    //}

    // enemy is now simulated


    // Create interceptor missile
    // Based off RIM-174 Standard ERAM
    let interceptor_pos = Position { x: 0.0, y: 0.0, z: 200_000.0 };
    let interceptor_heading = Heading { x: 0.0, y: 1.0, z: 0.0 }; // facing straight up
    let interceptor_vel = Velocity { speed: 500.0, heading: interceptor_heading }; // 4285 km/h
    let interceptor_target_pos = enemy.pos.clone();
    let interceptor_detonation_dist = 10.0;
    let interceptor_max_accel = MissileAccel { _forward: 30.0, _backward: 20.0, _roll: 20.0, _attitude: 40.0 };
    let interceptor_flight_range = 370_000.0; // 370 km
    let mut interceptor = InterceptorMissile::new(interceptor_pos, interceptor_vel, interceptor_detonation_dist, interceptor_target_pos, interceptor_max_accel, interceptor_flight_range);

    // interceptor is now modeled, implement the flight logic
 
    // simulate flight
    //let time_step = 0.005;
    //loop {
    //    interceptor.update_pos(time_step);
    //    enemy.update_pos(time_step);
    //    interceptor.update_vel(enemy.pos.clone());
    //
    //    //println!("Enemy Position: {:?}", &enemy.pos);
    //    //println!("Interceptor Position: {:?}", &interceptor.pos);
    //}

    // Chat gpt. 
    let root = BitMapBackend::gif("missile_paths.gif", (1920, 1080), 250).unwrap().into_drawing_area();

    // Setup chart with 3D projection
    let x_range = 0.0..500_000.0;
    let y_range = 0.0..500_000.0;
    let z_range = 0.0..500_000.0;
    // Simulation parameters
    let time_step = 0.005;

    // Data collection
    let mut enemy_positions = Vec::new();
    let mut interceptor_positions = Vec::new();

    let mut iter_count = 0;
    loop {
        //println!("{:?}", enemy.pos.clone());
        // match example
        match enemy.update_pos(time_step) {
            MissileState::Normal => (), // show what happens when we don't have this arm
            MissileState::Exploded => {
                // We can put braces and add many lines of code
                break;
            }
        }
        // the same as the match, but when we only care about 1 variant
        if let MissileState::Exploded = interceptor.update_pos(time_step) {
            break;
        }
        enemy.update_vel();
        interceptor.update_vel(enemy.pos.clone());

        enemy_positions.push((enemy.pos.x, enemy.pos.y, enemy.pos.z));
        interceptor_positions.push((interceptor.pos.x, interceptor.pos.y, interceptor.pos.z));


        if (iter_count % 5_000 == 0) | 
            ((abs_dist(&interceptor.pos, &enemy.pos) < 10_000.0) & 
             (iter_count % 1_000 == 0)) {
            // draw frame

            root.fill(&WHITE).unwrap();

            let mut chart = ChartBuilder::on(&root)
                .caption("Missile Paths", ("sans-serif", 30))
                .margin(20)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_3d(x_range.clone(), y_range.clone(), z_range.clone())
                .unwrap();

            chart.with_projection(|mut pb| {
                //pb.pitch = 0.25;
                //pb.yaw = 0.25;
                pb.pitch = 0. - (iter_count as f64 / 250_000.); // explain why casting after the
                                                              // division is bad (integer division)
                pb.yaw = 1. - (iter_count as f64 / 150_000.);
                pb.scale = 1.;
                pb.into_matrix()
            });

            chart.configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .max_light_lines(3)
                .draw()
                .unwrap();

            // Plot enemy missile path
            chart.draw_series(LineSeries::new(
                enemy_positions.iter().map(|&(x, y, z)| (x, y, z)),
                &RED,
            )).unwrap();

            // Plot interceptor missile path
            chart.draw_series(LineSeries::new(
                interceptor_positions.iter().map(|&(x, y, z)| (x, y, z)),
                &BLUE,
            )).unwrap();
            
            // Plot missiles
            

            // Configure and draw chart
            chart.configure_series_labels()
                .background_style(&WHITE.mix(0.8))
                .border_style(&BLACK)
                .draw()
                .unwrap();

            // Save the plot to a file
            root.present().unwrap();
        }
        iter_count += 1;
    }
    println!("simulated {iter_count} positions");
}


struct EnemyMissile {
    pos: Position,
    vel: Velocity,
    detonation_dist: f64,
    target_pos: Position,
    theta: f64,
}

#[derive(Debug, PartialEq, Clone)]
struct Position {
    x: f64,
    y: f64, 
    z: f64
}

struct Velocity {
    speed: f64, 
    heading: Heading
}

#[derive(Debug)]
struct Heading {
    x: f64,
    y: f64,
    z: f64
}


// At this point, compile the code and see the warnings for unconstructed structs

impl EnemyMissile {
    fn new(pos: Position, vel: Velocity, target_pos: Position, detonation_dist: f64, theta: Option<f64>) -> Self {
        let theta = theta.unwrap_or(0.0);
        EnemyMissile { pos, vel, target_pos, detonation_dist, theta }
    }

    // don't implement this right after the constructor, move on and show construction
    fn update_vel(&mut self) {
        //println!("Updating enemy heading");
        //println!("Before update: {:?}", self.vel.heading);
        self.theta += PI / 10_000.0; // our change rate (affects frequency of our missile's oscillation)
        //let amplitude = 5.0;
        let adjacent = self.theta.cos(); // The x-component of the radius line when at angle
        
        let slope = adjacent;
        //let slope = -(adjacent.sin()); // The slope of the wave (and our missile) when at that point
        //println!("Slope of enemy missile: {slope}");

        self.vel.heading.y = slope; // Set our missile's heading to the computed angle
        //println!("After update: {:?}", self.vel.heading);
        //println!("")
    }
}

// Do the first section of main() to contruct and print something about our missile


trait UpdateMissile {
    // we wouldn't need all these getters and setters if we just did the basic impl for each struct
    // instead of a trait. It is just to demonstrate traits
    fn get_pos(&self) -> &Position;
    fn get_vel(&self) -> &Velocity;
    fn get_target_pos(&self) -> &Position;
    fn get_detonation_dist(&self) -> f64;

    fn set_pos(&mut self, new_pos: Position);
    fn explode(&self);

    /// Calculates the required heading to get from current_pos to target_pos.
    /// Finds the vector that connects the two points and then normalizes it to
    /// a vector with magnitude 1
    fn get_new_heading(current_pos: &Position, target_pos: &Position) -> Heading {
        let x_diff = target_pos.x - current_pos.x;
        let y_diff = target_pos.y - current_pos.y;
        let z_diff = target_pos.z - current_pos.z;
        let raw_vector = Heading { x: x_diff, y: y_diff, z: z_diff }; // the vector that connects
                                                                      // the two points
        let magnitude = (raw_vector.x*raw_vector.x + raw_vector.y*raw_vector.y + raw_vector.z*raw_vector.z).sqrt();
        let new_x = raw_vector.x / magnitude;
        let new_y = raw_vector.y / magnitude;
        let new_z = raw_vector.z / magnitude;
        let normalized_vector = Heading { x: new_x, y: new_y, z: new_z };
        normalized_vector
    }

    
    fn update_pos(&mut self, time_step: f64) -> MissileState {
        let velocity = self.get_vel();
        let distance_travelled = velocity.speed * time_step;
        let Heading { x: x_heading, y: y_heading, z: z_heading } = velocity.heading;
        //println!("{x_heading}, {y_heading}, {z_heading}");

        let position = self.get_pos();
        let new_pos = Position {
            x: position.x + (distance_travelled * x_heading),
            y: position.y + (distance_travelled * y_heading),
            z: position.z + (distance_travelled * z_heading),
        };

        // Check if we have gotten close enough to our target
        let target_pos = self.get_target_pos();
        let detonation_dist = self.get_detonation_dist();
        if abs_dist(&new_pos, &target_pos) < detonation_dist { // go define abs_dist()
            // We are close enough for detonation
            self.explode();
            return MissileState::Exploded; // compare return methods
        }
        
        self.set_pos(new_pos);
        MissileState::Normal
    }

}

enum MissileState {
    Normal,
    Exploded,
}

impl UpdateMissile for EnemyMissile {
    fn get_pos(&self) -> &Position { &self.pos }
    fn get_vel(&self) -> &Velocity { &self.vel }
    fn get_target_pos(&self) -> &Position { &self.target_pos }
    
    fn get_detonation_dist(&self) -> f64 { self.detonation_dist }
    fn set_pos(&mut self, new_pos: Position) { self.pos = new_pos }
    fn explode(&self) {
        println!("oof");
    }

    // Test flight of missile, then go to the interceptor


}


/// Calculates the absolute distance between two 'Position' structs
fn abs_dist(pos_1: &Position, pos_2: &Position) -> f64 {
    let dx = pos_1.x - pos_2.x;
    let dy = pos_1.y - pos_2.y;
    let dz = pos_1.z - pos_2.z;

    let distance = (dx*dx + dy*dy + dz*dz).sqrt();

    distance
}


// our interceptor missile will have a max acceleration and a range variable too


struct InterceptorMissile {
    pos: Position,
    vel: Velocity,
    detonation_dist: f64,
    target_pos: Position,
    _max_accel: MissileAccel,
    _flight_range: f64,
}

/// Acceleration capabilities for our missile in each dimension.
/// Measured in m/s^2.
struct MissileAccel {
    _forward: f64,
    _backward: f64,
    _roll: f64,
    _attitude: f64, // pitch and yaw control
}

impl InterceptorMissile {
    fn new(pos: Position, vel: Velocity, detonation_dist: f64, target_pos: Position, _max_accel: MissileAccel, _flight_range: f64) -> Self {
        InterceptorMissile { pos, vel, detonation_dist, target_pos, _max_accel, _flight_range }
    }


    fn update_vel(&mut self, new_target_pos: Position) {
        // update speed (not yet)
        
        // update heading
        let current_pos = &self.pos;
        let new_goal_heading = InterceptorMissile::get_new_heading(&current_pos, &new_target_pos); // change this at some
                                                                             // point so that the
                                                                             // target point is
                                                                             // predicted based on
                                                                             // the enemy missile's
                                                                             // path and not just
                                                                             // on where it is
                                                                             // currently
        // TODO: let max_heading_correction = ; // Take into account the max acceleration to adjust the
                                       // heading as much toward ouselfselfr target heading as possible

        // update target position
        self.target_pos = new_target_pos;
        // update heading for the new target_pos
        self.vel.heading = new_goal_heading;
    }
}


impl UpdateMissile for InterceptorMissile {
    fn get_pos(&self) -> &Position { &self.pos }
    fn get_vel(&self) -> &Velocity { &self.vel }
    fn get_target_pos(&self) -> &Position { &self.target_pos }
    
    fn get_detonation_dist(&self) -> f64 { self.detonation_dist }
    fn set_pos(&mut self, new_pos: Position) { self.pos = new_pos }
    fn explode(&self) {
        println!("Boom! Intercepted");
    }
    // we have implemented the common stuff. Go back to enemy to implement the update_vel()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_update_pos() {
        let pos = position { x: 50_000.0, y: 500_000.0, z: 100_000.0 };
        let heading = heading { x: 0.0, y: -1.0, z: 0.0 }; // facing straight down the y axis
        let vel = Velocity { speed: 700.0, heading: heading }; // 700 m/s
        let target_pos = Position { x: 50_000.0, y: 0.0, z: 100_000.0 };
        let detonation_dist = 3.0;
        let mut missile = EnemyMissile::new(pos, vel, target_pos, detonation_dist);

        
        assert_eq!(missile.pos, Position { x: 50000.0, y: 500000.0, z: 100000.0 });
        let _ = missile.update_pos(5.0);
        assert_eq!(missile.pos, Position { x: 50000.0, y: 496500.0, z: 100000.0 });
    }

    #[test]
    fn test_abs_dist() {
        let pos_1 = Position { x: 0.0, y: 0.0, z: 0.0 };
        let pos_2 = Position { x: 0.0, y: 3.0, z: 4.0};

        assert_eq!(5.0, abs_dist(&pos_1, &pos_2));
    }
}
