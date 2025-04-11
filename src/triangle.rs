pub struct Triangle{
    iso:bool,
    scalene:bool,
    equi:bool,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // todo!(
        //     "Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid."
        // );
        
        let mut arr:Vec<u64>=Vec::new();
        for i in 0..sides.len(){
            arr.push(sides[i]);
        }
        arr.sort();
        if arr[0]==0 || arr[1]==0 || arr[2]==0{
            return None;
        }
        if arr[0]==arr[1] && arr[1]==arr[2] && arr[0] != 0{
            Some(Triangle{
                iso:true,
                scalene:false,
                equi:true,
            })
        }
        else if (arr[0]==arr[1] && arr[1] != arr[2]) || (arr[0]!= arr[1] && arr[1] == arr[2]){
            if arr[0]+arr[1]>=arr[2] && arr[1]+arr[2]>=arr[0] && arr[0]+arr[2]>=arr[1]{
                Some(Triangle{
                    iso:true,
                    scalene: false,
                    equi:false,
                })
            }else{
                None
            }
        }else{
            if arr[0]+arr[1]>=arr[2] && arr[1]+arr[2]>=arr[0] && arr[0]+arr[2]>=arr[1]{
                Some(Triangle{
                    iso:false,
                    scalene:true,
                    equi:false,
                })
            }else{
                None
            }
            
        }
        
        
    }

    pub fn is_equilateral(&self) -> bool {
        // todo!("Determine if the Triangle is equilateral.");
        self.equi
    }

    pub fn is_scalene(&self) -> bool {
        // todo!("Determine if the Triangle is scalene.");
        self.scalene
    }

    pub fn is_isosceles(&self) -> bool {
        // todo!("Determine if the Triangle is isosceles.");
        self.iso
    }
}
