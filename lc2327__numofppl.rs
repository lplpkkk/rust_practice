struct Sol;
impl Sol{
    pub fn ppl(n:i32,delay:i32,forget:i32) -> i32{
        const MOD:i64=1_000_000_007;
        let n= n as usize;
        let delay = delay as usize;
        let forget = forget as usize;
        
        let mut dp=vec![0i64;n+1];
        dp[1]=1;
        let mut share=0i64;
        
        for day in 2..=n{
            if day>=delay+1{
                share+=dp[day-delay];
            }
            
            if day>=forget+1{
                share-=dp[day-forget];
            }
            
            share=(share+MOD)%MOD;
            dp[day]=share;
        }
        
        let mut result=0i64;
        for day in n-forget+1..=n{
            result=(result+dp[day])%MOD;
        }
        
        result as i32
    }
}

fn main() {
    assert_eq!(Sol::ppl(6,2,4),5);
    println!("pass");
    assert_eq!(Sol::ppl(6,2,4),6);
}
