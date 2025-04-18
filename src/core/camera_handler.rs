pub(crate) struct CameraHandler{
    main_x_coordinate:f32,
    main_y_coordinate:f32,

    x_camera_pos:f32,
    y_camera_pos:f32,

}


impl CameraHandler{

    pub fn create() -> Self {

        let mut camera = Self {
            main_x_coordinate: 0f32,
            main_y_coordinate: 0f32,
            x_camera_pos: 0f32,
            y_camera_pos: 0f32,
        };

        camera
    }

    pub fn find_real_coordinate(&mut self,x:f32,y:f32) -> [f32;2]{
        [x-self.x_camera_pos , y-self.y_camera_pos]
    }

    pub fn get_camera_coordinate(&mut self) -> [f32;2]{
        [self.x_camera_pos, self.y_camera_pos]
    }

    pub fn set_camera_coordinate(&mut self,x:f32,y:f32){
        self.x_camera_pos = x;
        self.y_camera_pos = y;
    }

}