pub(crate) struct CameraHandler{
    relation_pos_x:f32,
    relation_pos_y:f32,

    x_camera_pos:f32,
    y_camera_pos:f32,

}


impl CameraHandler{

    pub fn create() -> Self {

        let mut camera = Self {
            relation_pos_x: 0f32,
            relation_pos_y: 0f32,
            x_camera_pos: 0f32,
            y_camera_pos: 0f32,
        };

        camera
    }

    pub fn set_realation(&mut self ,relation_pos_x:f32, relation_pos_y:f32){
        if relation_pos_x != self.relation_pos_x || relation_pos_y != self.relation_pos_y
        {
            self.relation_pos_x = relation_pos_x;
            self.relation_pos_y = relation_pos_y;
        }
    }

    pub fn find_real_coordinate(&mut self,x:f32,y:f32) -> [f32;2]{
        [(x-self.x_camera_pos) / self.relation_pos_x , (y-self.y_camera_pos) / self.relation_pos_y]
    }

    pub fn get_camera_coordinate(&mut self) -> [f32;2]{
        [self.x_camera_pos, self.y_camera_pos]
    }

    pub fn set_camera_coordinate(&mut self,x:f32,y:f32){
        self.x_camera_pos = x;
        self.y_camera_pos = y;
    }

}