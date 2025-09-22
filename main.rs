fn shared_bits(a: u32, b: u32) -> bool 
{
    if a == 0 || b == 0
    {
        return false;
    }
    else
    {
        let string1 = dec_to_binary(a);
        let string2 = dec_to_binary(b);
        
        if string1.is_empty() || string2.is_empty()
        {
            panic!("Strings are empty");
        }
        else
        {
            let mut match_indices: u8 = 0;
            

            for (index,ch) in string1.char_indices()
            {
                
                if ch == string2.chars().nth(index).expect("REASON")
                {
                    if ch == '1'
                    {
                        match_indices += 1;
                    }
                    
                }
            }

            if match_indices > 1
            {
                return true;
            }
            else
            {
                return false;
            }
        }
    }
    
   
}

use std::mem;
fn dec_to_binary(dec: u32) -> String
{
    let mut quotient : u32  = dec;
    let mut dividend : u32  = dec;
    let mut remainder: u32  = 0;


    let mut BCD     = Vec::new();
    let mut str_BCD = String::new();
    
    while quotient != 1
    {
        quotient  = dividend / 2;
        remainder = dividend % 2; 
        dividend  = quotient;
                
        BCD.push(remainder);
    }
            
    BCD.push(1);
    //BCD.reverse();

    // println!("{:?}", BCD);

    let mut BCD_full32 = vec![0;mem::size_of_val(&dec)*8];
    let max_len        = BCD_full32.len()-1;
    for (index, value) in BCD.iter().enumerate()
    {
        // println!("{}", value);
        BCD_full32[max_len-index] += value;
    }

    for i in BCD_full32.iter()
    {
        str_BCD.push_str(&i.to_string());
    }

    return str_BCD;
}



fn main ()
{
    println!("{}", shared_bits(0,0));
    // println!("{}", dec_to_binary(10));
}
