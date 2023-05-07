#[allow(dead_code)]

#[derive(Clone)]
pub struct Array2<T: Clone>{
    pub arr: Vec<T>,
    pub width_height: (usize, usize),
    pub col_row: (usize, usize),
}

impl<T: Clone> Array2<T> {
    // constructor for a 2D array in row major order
    pub fn new(v: Vec<T>, w_h: (usize, usize)) -> Self {
        Array2 {
            arr: v,
            width_height: w_h,
            col_row: (0, 0),
        }
    }
    // constructor for a 2D array with a polymorphic value copied to each element 
    pub fn blanck_state(val: T, w_h: (usize, usize)) -> Self {
        let d_vec = vec![val; w_h.0 * w_h.1];
        Array2 {
            arr: d_vec,
            width_height: w_h,
            col_row: (0, 0),
        }
    }
}

// Iterator for our Array2 structure 
impl<T: Clone> Iterator for Array2<T>{

    type Item = (usize, usize);

    // next function returning some tuple coordinates for our 2D array in row major order starting at (0,0)
    // returns the Option None if there is no next coordinate
    fn next(&mut self) -> Option<Self::Item> {
        self.col_row.0 = (self.col_row.0 + 1) % self.width_height.0;

        if self.col_row.0 == 0 {
            self.col_row.1 = (self.col_row.1 + 1) % self.width_height.1;

            if self.col_row.1 == 0 {
                return None
            }
        }
        Some(self.col_row)
    }
}

impl<T: Clone> Array2<T>{
    // function iter_row_major, calls the next function over and over until all element of the 2D array are iterated through
    // returns an interator over a vector of tuples containing the coordonates tuple returned by the next function at call
    // as well as a 1 dimensional index calculated from it
    pub fn iter_row_major(&mut self) -> impl Iterator <Item = ((usize, usize),T)> {

        let _len = self.width_height.0 * self.width_height.1;
        let mut xyi_path: Vec<((usize, usize),T)> = Vec::new();

        for _i in 0.._len{
            xyi_path.push(((self.col_row.0, self.col_row.1), self.get_mut((self.col_row.0, self.col_row.1)).cloned().unwrap()));
            
            self.next();
        }
        return xyi_path.into_iter();
    }
    pub fn iter_col_major(&mut self)-> impl Iterator <Item = ((usize, usize),T)>{ 

        let width = self.width_height.0;
        let height = self.width_height.1;

        let mut xyi_path: Vec<((usize, usize),T)> = Vec::new();
        let mut saved: (usize, usize) = (0, 0);
        // for every elements in a row
        for _z in 0..width {
            // for every elements in column get the index based on the x and y coordinates and push them into a vector as a tuple of 3 usizes
            for _i in 0..height{ 
                xyi_path.push(((self.col_row.0, self.col_row.1), self.get_mut((self.col_row.0, self.col_row.1)).cloned().unwrap()));
                for _z in 0..width{
                    self.next();         
                }
            }
            self.col_row = saved;
            self.next();
            saved = self.col_row;
        } 
        self.col_row = (0,0); // reset tracker and saved 
        return xyi_path.into_iter();
    }

// you probably do want to provide a function that allows access to an element by a pair of coordinates
    // function gets index from a  pair of coordinates for a 2D array
    pub fn get_index(&self, c_r: (usize, usize)) -> Option<usize>{
        let index = self.width_height.0 * c_r.1 + c_r.0;

        let max_index = self.width_height.0 * self.width_height.1;
        if index < max_index{
            Some(index)
        }else {None}
    }
    pub fn get_indexc(&self, c_r: (usize, usize)) -> Option<usize>{
        let index = self.width_height.1 * c_r.0 + c_r.1;

        let max_index = self.width_height.0 * self.width_height.1;
        if index < max_index{
            Some(index)
        }else {None}
    }
    pub fn get_index90(&self, c_r: (usize, usize)) -> Option<usize>{
        let index = self.width_height.1 * c_r.1 + c_r.0;

        let max_index = self.width_height.0 * self.width_height.1;
        if index < max_index{
            Some(index)
        }else {None}
    }
    // function gets a value from 2D array using pair of coordinates
    pub fn get(&self, c_r: (usize, usize)) -> Option<&T>{
        self.get_index(c_r).map(|index| &self.arr[index])
    }
    // function gets a mutable value from our 2D array using a pair of coordinates
    pub fn get_mut(& mut self, c_r: (usize, usize)) -> Option<&mut T>{
        self.get_index(c_r).map(move |index| &mut self.arr[index])
    }
    pub fn get_mutc(& mut self, c_r: (usize, usize)) -> Option<&mut T>{
        self.get_indexc(c_r).map(move |index| &mut self.arr[index])
    }
    pub fn get_mut90(& mut self, c_r: (usize, usize)) -> Option<&mut T>{
        self.get_index90(c_r).map(move |index| &mut self.arr[index])
    }

}

