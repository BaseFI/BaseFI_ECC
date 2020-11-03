
const BigInteger_Max_Cap:usize=18;
const BigInteger_Cell_Size:usize=4;
pub struct BigInteger
{
pub  Data: [u32;BigInteger_Max_Cap],
}
impl BigInteger
{
    pub fn new( )->BigInteger
    {
        BigInteger{
            Data:[0;BigInteger_Max_Cap],
        }
    }
    #[inline(always)]
    pub fn one( )->BigInteger
    {
        let mut res=BigInteger{
            Data:[0;BigInteger_Max_Cap],
        };
        res.Data[0]=1;

        res
    }
    #[inline(always)]
    pub fn two( )->BigInteger
    {
        let mut res=BigInteger{
            Data:[0;BigInteger_Max_Cap],
        };
        res.Data[0]=2;

        res
    }
    pub fn fill(&mut self, index :usize ,data :Vec<u32>)
    {
        let mut i=index;
        while(i<data.len())
        {
            self.Data[i]=  *data.get(i).unwrap();

            i+=1;
        }
    }
    pub fn from_base_16(&mut self,s: String)
    {
        let mut c_array=s.as_bytes();
        let mut index=0;
        let c_len=c_array.len();
        while (index<c_len)
        {
            let mut c:u8=c_array[c_len-1-index];
            
            if(c>=48 && c<=57)
            {
                c-=48;               
            }
            if(c>=65 && c<=70)
            {
                c-=55;               
            }
            if(c>=97 && c<=102)
            {
                c-=87;               
            }
            let mut  data_index=index/2;
            data_index=(data_index%(BigInteger_Max_Cap*BigInteger_Cell_Size))/BigInteger_Cell_Size;
            if(index%(BigInteger_Cell_Size*2)==0)
            {
                self.Data[data_index]=c as u32;
            }
            if(index%(BigInteger_Cell_Size*2)==1)
            {
                self.Data[data_index]+=16*(c as u32);
            }
            if(index%(BigInteger_Cell_Size*2)==2)
            {
                self.Data[data_index]+=16*16*(c as u32);
            }
            if(index%(BigInteger_Cell_Size*2)==3)
            {
                self.Data[data_index]+=16*16*16*(c as u32);
            }
            if(index%(BigInteger_Cell_Size*2)==4)
            {
                self.Data[data_index]+=16*16*16*16*c as u32;
            }
            if(index%(BigInteger_Cell_Size*2)==5)
            {
                self.Data[data_index]+=16*16*16*16*16*(c as u32);
            }
            if(index%(BigInteger_Cell_Size*2)==6)
            {
                self.Data[data_index]+=16*16*16*16*16*16*(c as u32);
            }
            if(index%(BigInteger_Cell_Size*2)==7)
            {
                self.Data[data_index]+=16*16*16*16*16*16*16*(c as u32);
            }

            

            index+=1;
        }

    }
    pub fn static_from_base_16(s: String) ->BigInteger
    {
        let mut res=BigInteger::new();
        res.from_base_16(s);
        res

    }
    pub fn to_base_16(&mut self)->String
    {
        let str_array=['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];
        let mut index=0;
        let mut res:Vec<u8>=Vec::new();
        //res.push('0' as u8 );
        //res.push('x' as u8 );
        //let res=String ::from("");
        while (index<BigInteger_Max_Cap)
        {
            let mut d:u32= self.Data[BigInteger_Max_Cap-1-index] as u32;
            let mut byte_index=0;
            while byte_index<=3
            {
                let mut byte_i=(d/(1<<8*(3-byte_index)) ) as usize;
                d= d-byte_i as u32*(1<<8*(3-byte_index)) ;
                let l=str_array[byte_i%16];
                let h=str_array[byte_i/16];
                res.push(h as u8 );
                res.push(l as u8 );
                byte_index+=1;
            }
            
            index+=1;
        }

        let resbuild=String::from_utf8(res);
        return match(resbuild){
            Ok(_)=>resbuild.unwrap(),
            Err(_) =>String::from("")
            

        };
    }
    /*
    pub fn shift(&mut self,i:i64)
    {
        let temp:BigInteger=self.clone();
        let mut index:usize=0;


        let byte_mask:u8=0xff;
        let i_rotation :i64 =i;
        let mut src_index=0 as i64;
        src_index = i_rotation;
        let src_byte_i=src_index/8;
        let src_bit_i=src_index%8;
        
        while (index<(BigInteger_Max_Cap)){
                self.Data[index]=0;
            
                if((src_byte_i+index as i64 )>=0 && (src_byte_i+index as i64 )<BigInteger_Max_Cap as i64)
                {
                    self.Data[index]= temp.Data[(src_byte_i+index as i64 ) as usize];
                }
                else
                {
                    self.Data[index]=0;
                }
                
            
                index +=1;
        }
        index=0;
        let temp2=self.clone();
        while (index<(BigInteger_Max_Cap)){
            if(src_bit_i!=0)
            {

                if(i_rotation<0)
                {   
                    //bigger
                    self.Data[index]<<=-src_bit_i;
                    if(index>=1){
                        self.Data[index]&= !(byte_mask>>(8-src_bit_i));
                        self.Data[index]|= temp2.Data[index-1]>>(8+src_bit_i);
                    }
                }else
                {
                    //smaller
                    self.Data[index]>>=src_bit_i;
                    if(index< BigInteger_Max_Cap-1){
                        self.Data[index]&= !(byte_mask<<(src_bit_i));
                        self.Data[index]|= temp2.Data[index+1]<<(src_bit_i);
                    }
                }
            }
                index +=1;
        }

    }*/
    #[inline(always)]
    pub fn compare(a:&BigInteger,b:&BigInteger)->i64{

        let mut index=0;
        let mut res:i64=0;
       
        //let res=String ::from("");
        while (index<BigInteger_Max_Cap)
        {
           
            if a.Data[BigInteger_Max_Cap-1-index]>b.Data[BigInteger_Max_Cap-1-index]
            {
                return 1;
            }
            if a.Data[BigInteger_Max_Cap-1-index]<b.Data[BigInteger_Max_Cap-1-index]
            {
                return -1;
            }
            index+=1;
        }
        return 0;

    }
    pub fn is_even(&self)->bool{
        if (self.Data[0]&0x01==1){return false}else{return true};
    }

}
impl Copy for BigInteger
{
  
}
impl Clone for BigInteger
{
    fn clone(&self) -> Self {
        let mut index=0;
        
        let mut res= BigInteger::new();
        while (index<BigInteger_Max_Cap)
        {
            res.Data[index]=self.Data[index];
            index+=1;
        }
        res
    }
}
impl std::ops::Add for BigInteger
{
    type Output = Self;
    fn add(self, other: Self) -> BigInteger {
        
        let  a=&self;
        let  b=&other;
        
        let mut temp:u64=0;
        let mut res= BigInteger::new();
        let mut index=0;
        while (index<BigInteger_Max_Cap)
        {
            temp+=a.Data[index] as u64;
                
            temp+=b.Data[index] as u64;
            res.Data[index]=(temp% ((1 as u64) <<(BigInteger_Cell_Size*8)) ) as u32;
            temp=temp/((1 as u64) <<(BigInteger_Cell_Size*8)) ;
            index+=1;
        }

        return res;
    }


}
impl std::ops::Add for &BigInteger
{
    type Output = BigInteger;
    fn add(self, other: Self) -> BigInteger {
        
        let  a=self;
        let  b=other;
        
        let mut temp:u64=0;
        let mut res= BigInteger::new();
        let mut index=0;
        while (index<BigInteger_Max_Cap)
        {
            temp+=a.Data[index] as u64;
                
            temp+=b.Data[index] as u64;
            res.Data[index]=(temp% ((1 as u64) <<(BigInteger_Cell_Size*8))) as u32;
            temp=temp/((1 as u64) <<(BigInteger_Cell_Size*8));
            index+=1;
        }

        return res;
    }


}
impl std::ops::Sub for BigInteger
{
    type Output = Self;
    fn sub(self, other: Self) -> BigInteger {
        
        let mut a=self.clone();
        let mut b=other.clone();
        let one= BigInteger::one();
        let mut temp:u64=0;
        let mut res= BigInteger::new();
        let mut index=0;
        while (index<BigInteger_Max_Cap)
        {
            b.Data[index]=!b.Data[index];
            index+=1;
        }
        res=a+b;
        res=res+one;
        return res;
    }


}
impl std::ops::Sub for &BigInteger
{
    type Output = BigInteger;
    fn sub(self, other: Self) -> BigInteger {
        
        let  a=self;
        let  b=other;
        let one= BigInteger::one();
        let mut temp:u64=0;
        let mut res= BigInteger::new();
        let mut index=0;
        while (index<BigInteger_Max_Cap)
        {
            res.Data[index]=!b.Data[index];
            index+=1;
        }
        res=a+&res;
        res=&res+&one;
        return res;
    }


}
impl std::ops::Rem for BigInteger
{
    type Output = Self;
    fn rem(self, other: Self) -> BigInteger {
        
        let one= BigInteger::one();
        //let mut temp:u64=0;
        let mut res: BigInteger;
        let mut index=0;
        let mut cache:Vec<BigInteger>=Vec::new();
        let mut temp_bi=other.clone();
        
        while (index<BigInteger_Max_Cap/2* BigInteger_Cell_Size*8)
        {
            cache.push(temp_bi.clone());
            temp_bi=&temp_bi+&temp_bi;
            index+=1;
        }
        index=BigInteger_Max_Cap/2*BigInteger_Cell_Size*8-1;
        let mut divided=self.clone();
        let mut quotient= BigInteger::new();
        while (index>=0)
        {   quotient=&quotient+&quotient;
            if (BigInteger::compare(&divided,&cache[index])>=0)
            {
                quotient=&quotient+&one;
                divided=&divided-&cache[index];
            }
            if (index==0)
            {
                break;
            }
            index-=1;
        }


        res= divided;
        return res;
    }
}
impl std::ops::Rem for &BigInteger
{
    type Output = BigInteger;
    fn rem(self, other: Self) -> BigInteger {
     //let mut a=self.clone();
        //let mut b=other.clone();
        let one= BigInteger::one();
        //let mut temp:u64=0;
        let mut res: BigInteger;
        let mut index=0;
        let mut cache:Vec<BigInteger>=Vec::new();
        let mut temp_bi=other.clone();
        
        while (index<BigInteger_Max_Cap/2*BigInteger_Cell_Size*8)
        {
            cache.push(temp_bi.clone());
            temp_bi=&temp_bi+&temp_bi;
            index+=1;
        }
        index=BigInteger_Max_Cap/2*BigInteger_Cell_Size*8-1;
        let mut divided=self.clone();
        let mut quotient= BigInteger::new();
        while (index>=0)
        {   quotient=&quotient+&quotient;
            if (BigInteger::compare(&divided,&cache[index])>=0)
            {
                quotient=&quotient+&one;
                divided=&divided-&cache[index];
            }
            if (index==0)
            {
                break;
            }
            index-=1;
        }


        res= divided;
        return res;
    }


}

impl std::ops::Div for BigInteger
{
    type Output = Self;
    fn div(self, other: Self) -> BigInteger {
        
        let mut a=self.clone();
        let mut b=other.clone();
        let one= BigInteger::one();
        let mut temp:u64=0;
        let mut res= BigInteger::new();
        let mut index=0;
        let mut cache:Vec<BigInteger>=Vec::new();
        let mut temp_bi=other.clone();
        while (index<BigInteger_Max_Cap/2*BigInteger_Cell_Size*8)
        {
            cache.push(temp_bi.clone());
            temp_bi=&temp_bi+&temp_bi;
            index+=1;
        }
        index=BigInteger_Max_Cap/2*BigInteger_Cell_Size*8-1;
        let mut divided=self.clone();
        let mut quotient= BigInteger::new();
        while (index>=0)
        {   quotient=&quotient+&quotient;
            if (BigInteger::compare(&divided,&cache[index])>=0)
            {
                quotient=&quotient+&one;
                divided=&divided-&cache[index];
            }
            if (index==0)
            {
                break;
            }
            index-=1;
        }


        res= quotient;
        return res;
    }


}   

impl std::ops::Div for &BigInteger
{
    type Output = BigInteger;
    fn div(self, other: Self) -> BigInteger {
        
        //let mut a=self.clone();
        //let mut b=other.clone();
        let one= BigInteger::one();
        //let mut temp:u64=0;
        let mut res: BigInteger;
        let mut index=0;
        let mut cache:Vec<BigInteger>=Vec::new();
        let mut temp_bi=other.clone();
        while (index<BigInteger_Max_Cap/2*BigInteger_Cell_Size*8)
        {
            cache.push(temp_bi.clone());
            temp_bi=&temp_bi+&temp_bi;
            index+=1;
        }
        index=BigInteger_Max_Cap/2*BigInteger_Cell_Size*8-1;
        let mut divided=self.clone();
        let mut quotient= BigInteger::new();
        while (index>=0)
        {   quotient=&quotient+&quotient;
            if (BigInteger::compare(&divided,&cache[index])>=0)
            {
                quotient=&quotient+&one;
                divided=&divided-&cache[index];
            }
            if (index==0)
            {
                break;
            }
            index-=1;
        }


        res= quotient;
        return res;
    }


}   


impl std::ops::Mul for BigInteger
{
    type Output = Self;
    fn mul(mut self, other: Self) -> BigInteger {
        
        //let mut a=self.clone();
        //let mut b=other.clone();
        //let one= BigInteger::one();
        //let mut temp:u64=0;
        let mut res= BigInteger::new();
        let mut index=0;
        let mut cache:Vec<BigInteger>=Vec::new();
        let mut temp_bi=other.clone();
        while (index<BigInteger_Max_Cap/2*BigInteger_Cell_Size*8)
        {
            cache.push(temp_bi.clone());
            temp_bi=&temp_bi+&temp_bi;
            index+=1;
        }
        //let mut mul_res=BigInteger::new();

        index= 0;
        let mut index_mask:u32=1;
        let mut multiplied:BigInteger=self.clone();
        while (index<BigInteger_Max_Cap/2*BigInteger_Cell_Size*8)
        {
            //res=res.clone()+res.clone();
            //println!("{}",res.to_base_16());
            let data_byte =multiplied.Data[(index/32)];
            let bit_zero=data_byte & (index_mask <<(index%32));
            if(bit_zero!=0)
            {
                res=&res+&cache[index];
            }

            
            index +=1;
           //if()
        }
        
        return res;
    }


}   

impl std::ops::Mul for &BigInteger
{
    type Output = BigInteger;
    fn mul(self, other: Self) -> BigInteger {
        
        //let mut a=self.clone();
        //let mut b=other.clone();
        //let one= BigInteger::one();
        //let mut temp:u64=0;
        let mut res= BigInteger::new();
        let mut index=0;
        let mut cache:Vec<BigInteger>=Vec::new();
        let mut temp_bi=other.clone();
        while (index<BigInteger_Max_Cap/2*BigInteger_Cell_Size*8)
        {
            cache.push(temp_bi.clone());
            temp_bi=&temp_bi+&temp_bi;
            index+=1;
        }
        //let mut mul_res=BigInteger::new();

        index= 0;
        let mut index_mask:u32=1;
        let mut multiplied:BigInteger=self.clone();
        while (index<BigInteger_Max_Cap/2*BigInteger_Cell_Size*8)
        {
            //res=res.clone()+res.clone();
            let data_byte =multiplied.Data[(index/32)];
            let bit_zero=data_byte & (index_mask <<(index%32));
            if(bit_zero!=0)
            {
                res=&res+&cache[index];
            }

            
            index +=1;
           //if()
        }
        
        return res;
    }


}   


impl std::cmp::PartialEq for BigInteger
{
    fn eq(&self, other: &Self) -> bool {
       
        if (BigInteger::compare(self, other)==0)
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}