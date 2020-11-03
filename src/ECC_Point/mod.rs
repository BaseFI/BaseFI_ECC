use crate::BigIntegerUtils;
use BigIntegerUtils::BigInteger;

pub struct ECC_POINT
{
  pub  X:BigInteger,
  pub  Y:BigInteger,
  pub  Inf:bool,

}

impl Clone for ECC_POINT
{
    fn clone(&self) -> ECC_POINT 
    {
        let mut x = self.X;    
        let mut y = self.Y;    
        let mut inf=self.Inf;
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:inf,
        }

    }
}

impl ECC_POINT{
    pub fn Get_G_Point() ->ECC_POINT
    { 
         
        let mut x = BigInteger::static_from_base_16(String::from("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798"));       
        let mut y = BigInteger::static_from_base_16(String::from("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"));      
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }
        
    }
    pub fn Get_kG_Point() ->ECC_POINT
    { 
        let mut x = BigInteger::static_from_base_16(String::from("807BAF868A6AC6CFC192B3491C711EDC35B1E7DD7481410A52840F54C86EFB0A"));       
        let mut y = BigInteger::static_from_base_16(String::from("BED82BC634D0E219DDC1A0511CC56391ECA96869BC9A33231DA88D5172704A7A"));      
    
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }
        
    }

    pub fn new() ->ECC_POINT
    { 
         
        let mut x = BigInteger::new();       
        let mut y = BigInteger::new();      
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }
        
    }
    pub fn Add(p1:&ECC_POINT,p2:&ECC_POINT) ->ECC_POINT
    {  

        let mut k =BigInteger::new(); 
        let mut P =  BigInteger::static_from_base_16(String::from("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"));    
        let mut exp2 = BigInteger::static_from_base_16(String::from("2"));
        let mut exp3 = BigInteger::static_from_base_16(String::from("3"));   
        
        if p1.Inf && !p2.Inf
        {
        return p2.clone();
        }
        if p2.Inf && !p1.Inf
        {
        return p1.clone();
        }
        let zero_y_test =&(&p1.Y+&p2.Y )%&P;
        if (zero_y_test== BigInteger::new())
        {
            let mut inf_res=ECC_POINT::new();
            inf_res.Inf=true;
            return inf_res;
        }


       
        if p1.X==p2.X
        {
            k= BigInteger::static_from_base_16(String::from("3"));  
            k=(&k* (&p1.X));k=&k%&P;k=&k*(&p1.X);k=&k%&P;
            k= &k* &ECC_POINT::GetReciprocalModP(&(&exp2*(&p1.Y)));
            k=&k%(&P);
        }else
        {
            k= &((&p2.Y)+(&P))-(&p1.Y);
            k=&k%(&P);
            let mut  k_temp=&((&p2.X)+(&P))-(&p1.X);
            k_temp=&k_temp%(&P);
            k= &k* &ECC_POINT::GetReciprocalModP(&k_temp);
            k=k%(P);
        }
       
        let mut x = (&k)*(&k);x=&x+(&P);x=&x-(&p1.X);x=&x+(&P);x=&x-(&p2.X);  
        x=&x%(&P);
        let mut y = (&k)*&((&P)+&((&p1.X)-(&x)) );y=&y+ &((&P)-(&p1.Y));     
        y=&y%(&P);
        ECC_POINT
        {
            X:x,
            Y:y,
            Inf:false,
        }
                
    }
    pub fn Mul( p:&ECC_POINT, x: &BigInteger) ->ECC_POINT
    {
        let P = BigInteger::static_from_base_16(String::from("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"));  
        let mut x_avatar=x.clone();
        let mut cache:Vec<ECC_POINT>=Vec::new();
        let mut two = BigInteger::static_from_base_16(String::from("2"));
        let mut res=ECC_POINT::new();
        res.Inf=true;

        cache.push(p.clone());
        for i in (0..256)
        {
           //println!("{}",i);
            if(i!=0)
            {
            let last=cache.get(i-1).unwrap();
            let mut cur= ECC_POINT::Add(&last, &last);   
            //cur.%P;         
            cache.push(cur)
            }
            let cur=cache.get(i).unwrap();
            if ! x_avatar.is_even()
            {
                res=ECC_POINT::Add(&res , &cur);              
               
            }
            x_avatar=&x_avatar/&two;
        }
        return res;


    }
    pub fn GetReciprocalModP(x: &BigInteger)->BigInteger
    {        
        let mut res=BigInteger::one();
        let P = BigInteger::static_from_base_16(String::from("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"));  
        let P_avatar = BigInteger::static_from_base_16(String::from("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",));  
        let mut P_2=BigInteger::static_from_base_16(String::from("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2D")); 
        let mut exp2 = BigInteger::two();
        let mut two = BigInteger::static_from_base_16(String::from("2"));
        let mut cache:Vec<BigInteger>=Vec::new();
        let x_avatar=x.clone();
        cache.push(x_avatar);
        for i in (0..256)
        {
            //print!("{0}\n",i);
            if(i!=0)
            {
            let last=cache.get(i-1).unwrap();
            let mut cur=(last)*(last);
            cur=&cur% (&P);
            cache.push(cur)
            }
            let cur=cache.get(i).unwrap();
            if ! P_2.is_even()
            {
                res=&res*(&cur);              
                res=&res% (&P_avatar);
            }
            P_2=&P_2/&two;
        }
        return res;
    }

}
