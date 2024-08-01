pub struct Body {
    // struct represents a rectangle
    x_start: f64,
    y_start: f64,
    x_end: f64,
    y_end: f64,
}

impl Body {
    pub fn new(x_start: f64, y_start: f64, x_end: f64, y_end: f64)->Self {
        Body { x_start, y_start, x_end, y_end }
    }

    pub fn collides_with(&self, other: &Body)->bool {
        if self.x_start > other.x_end {
            return false
        } if self.x_end < other.x_start {
            return false
        }
        // x_coordinates overlap

        if self.y_start > other.y_end {
            return false
        } if self.y_end < other.y_start {
            return false
        }
        //y coordinates overlap too:
        return true
    }

    pub fn x_end(&self)->f64 {
        return self.x_end
    }
    pub fn y_end(&self)->f64 {
        return self.y_end
    }
    
    pub fn x_start(&self)->f64 {
        return self.x_start
    }
    pub fn y_start(&self)->f64 {
        return self.y_start
    }

    pub fn move_x(&mut self, delta: f64) {
        * &mut self.x_end += delta;
        * &mut self.x_start += delta;
    }
    pub fn move_y(&mut self, delta: f64) {
        * &mut self.y_end += delta;
        * &mut self.y_start += delta;
    }
}

#[cfg(test)]
mod tests {
    use super::Body;

    #[test]
    fn collison1() {
        /*
            bottom right corner with top left corner
         */
        let rectangle1: Body = Body::new(0., 0., 5., 5.);
        let rectangle2: Body = Body::new(3., 3., 7., 7.);
        assert!(rectangle1.collides_with(&rectangle2));
        assert!(rectangle2.collides_with(&rectangle1));
    }
    
    #[test]
    fn collison2() {
        /*
            bottom left corner with top right corner
         */
        let rectangle1: Body = Body::new(0., 3., 5., 7.);
        let rectangle2: Body = Body::new(3., 0., 7., 5.);
        assert!(rectangle1.collides_with(&rectangle2));
        assert!(rectangle2.collides_with(&rectangle1));
    }

    #[test]
    fn collison3() {
        /*
            top overlaps whole bottom
         */
        let rectangle1: Body = Body::new(3., 0., 5., 5.);
        let rectangle2: Body = Body::new(0., 3., 7., 7.);
        assert!(rectangle1.collides_with(&rectangle2));
        assert!(rectangle2.collides_with(&rectangle1));
    }
    
    #[test]
    fn collison4() {
        /*
            top overlaps whole bottom
         */
        let rectangle1: Body = Body::new(0., 0., 7., 7.);
        let rectangle2: Body = Body::new(3., 3., 5., 5.);
        assert!(rectangle1.collides_with(&rectangle2));
        assert!(rectangle2.collides_with(&rectangle1));
    }

    #[test]
    fn collison5() {
        /*
            contains
         */
        let rectangle1: Body = Body::new(3., 0., 5., 5.);
        let rectangle2: Body = Body::new(0., 3., 7., 7.);
        assert!(rectangle1.collides_with(&rectangle2));
        assert!(rectangle2.collides_with(&rectangle1));
        assert!(rectangle2.collides_with(&rectangle2)); // check case both rectangles are identical
    }
    
    #[test]
    fn collison6() {
        /*
            cross vult
         */
        let rectangle1: Body = Body::new(2., 0., 3., 7.);
        let rectangle2: Body = Body::new(0., 2., 5., 2.);
        assert!(rectangle1.collides_with(&rectangle2));
        assert!(rectangle2.collides_with(&rectangle1));
    }

    #[test]
    fn collison7() {
        /*
            big left collides right
         */
        let rectangle1: Body = Body::new(0., 0., 5., 5.);
        let rectangle2: Body = Body::new(3., 3., 7., 4.);
        assert!(rectangle1.collides_with(&rectangle2));
        assert!(rectangle2.collides_with(&rectangle1));
    }
    
    #[test]
    fn collison8() {
        /*
            big right collides left
         */
        let rectangle1: Body = Body::new(0., 2., 5., 3.);
        let rectangle2: Body = Body::new(3., 0., 7., 7.);
        assert!(rectangle1.collides_with(&rectangle2));
        assert!(rectangle2.collides_with(&rectangle1));

        let rectangle3: Body = Body::new(0., 0., 5., 7.);
        assert!(rectangle3.collides_with(&rectangle2));
        assert!(rectangle3.collides_with(&rectangle2));
    }

    #[test]
    fn no_collison1() {
        /*
            diagonal
         */
        let rectangle1: Body = Body::new(0., 0., 3., 3.);
        let rectangle2: Body = Body::new(4., 3., 7., 7.);
        assert!(!rectangle1.collides_with(&rectangle2));
        assert!(!rectangle2.collides_with(&rectangle1));
    }


    #[test]
    fn no_collison2() {
        /*
            flipped diagonal
         */
        let rectangle1: Body = Body::new(0., 5., 3., 3.);
        let rectangle2: Body = Body::new(4., 0., 3., 2.);
        assert!(!rectangle1.collides_with(&rectangle2));
        assert!(!rectangle2.collides_with(&rectangle1));
    }

    #[test]
    fn no_collison3() {
        /*
            big high ground
         */
        let rectangle1: Body = Body::new(0., 0., 7., 2.);
        let rectangle2: Body = Body::new(2., 3., 5., 5.);
        assert!(!rectangle1.collides_with(&rectangle2));
        assert!(!rectangle2.collides_with(&rectangle1));
    }


    #[test]
    fn no_collison4() {
        /*
            small high ground
         */
        let rectangle1: Body = Body::new(2., 0., 4., 2.);
        let rectangle2: Body = Body::new(0., 3., 7., 7.);
        assert!(!rectangle1.collides_with(&rectangle2));
        assert!(!rectangle2.collides_with(&rectangle1));
    }
}

